use nu_protocol::PipelineData;
use s_macro::s;

pub struct Nushell {
    engine_state: nu_protocol::engine::EngineState,
    stack: nu_protocol::engine::Stack,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to parse command: {0:?}")]
    Parse(#[from] nu_protocol::ParseError),
    #[error("failed to evaluate command: {0:?}")]
    Shell(#[from] nu_protocol::ShellError),
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),
    #[error(r#"environment variable is not valid UTF-8"#)]
    InvalidEnvVar,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Nushell {
    pub fn new() -> Result<Nushell> {
        let mut nushell = Nushell {
            // Infuriating that this function just prints an error message if it fails and doesn't
            // return a Result. But the only failure is something to do with plugins, so fingers
            // crossed that's not my problem.
            engine_state: nu_command::add_shell_command_context(
                nu_cmd_lang::create_default_context()
            ),
            stack: nu_protocol::engine::Stack::new()
        };

        let mut pwd = false;

        for (key, value) in std::env::vars_os() {
            if key == "PWD" { pwd = true; }

            nushell.stack.add_env_var(
                key.into_string().map_err(|_| Error::InvalidEnvVar)?,
                nu_protocol::Value::String {
                    val: value.into_string().map_err(|_| Error::InvalidEnvVar)?,
                    internal_span: nu_protocol::Span::unknown()
                }
            )
        }

        // If they don't have the PWD environment variable for some reason, default to their home
        // directory. If they don't have a home directory, or if the path isn't valid UTF-8, they're
        // SOL for now.
        if !pwd {
            if let Some(home) = dirs::home_dir() {
                if let Some(home) = home.to_str() {
                    nushell.stack.add_env_var(
                        s!("PWD"),
                        nu_protocol::Value::String {
                            val: home.to_string(),
                            internal_span: nu_protocol::Span::unknown(),
                        }
                    )
                }
            }
        }

        Ok(nushell)
    }

    pub fn evaluate(&mut self, command: &str) -> Result<PipelineData> {
        let mut working_set = nu_protocol::engine::StateWorkingSet::new(&self.engine_state);
        let block = nu_parser::parse(&mut working_set, None, command.as_bytes(), false);
        self.engine_state.merge_delta(working_set.render())?;

        nu_engine::eval_block(
            &self.engine_state,
            &mut self.stack,
            &block,
            PipelineData::Empty,
            false,
            false
        )
            .map_err(Error::from)
    }
}