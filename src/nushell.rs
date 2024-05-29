use nu_protocol::PipelineData;

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
    #[error("couldn't load std: {0}")]
    LoadingStd(miette::ErrReport),
    #[error(r#"environment variable is not valid UTF-8"#)]
    InvalidEnvVar,
}

impl Nushell {
    pub fn new() -> Result<Nushell, Error> {
        let engine_state = nu_cmd_lang::create_default_context();
        let mut engine_state = nu_command::add_shell_command_context(engine_state);

        let mut pwd = false;

        for (key, value) in std::env::vars_os() {
            if key == "PWD" { pwd = true; }

            engine_state.add_env_var(
                key.into_string().map_err(|_| Error::InvalidEnvVar)?,
                nu_protocol::Value::string(
                    value.into_string().map_err(|_| Error::InvalidEnvVar)?,
                    nu_protocol::Span::unknown()
                )
            )
        }

        // If they don't have the PWD environment variable for some reason, default to their home
        // directory. If they don't have a home directory, or if the path isn't valid UTF-8, they're
        // SOL for now.
        if !pwd {
            if let Some(home) = dirs::home_dir() {
                if let Some(home) = home.to_str() {
                    engine_state.add_env_var(
                        String::from("PWD"),
                        nu_protocol::Value::string(
                            home.to_string(),
                            nu_protocol::Span::unknown(),
                        )
                    )
                }
            }
        }

        nu_std::load_standard_library(&mut engine_state).map_err(Error::LoadingStd)?;

        Ok(Nushell { engine_state, stack: nu_protocol::engine::Stack::new() })
    }

    pub fn evaluate(&mut self, command: &str) -> Result<PipelineData, Error> {
        let mut working_set = nu_protocol::engine::StateWorkingSet::new(&self.engine_state);
        let block = nu_parser::parse(&mut working_set, None, command.as_bytes(), false);

        // TODO Handle both parse errors and parse warnings here (both fields in StateWorkingSet).

        if !working_set.parse_errors.is_empty() {
            return Err(working_set.parse_errors.remove(0).into());
        }

        self.engine_state.merge_delta(working_set.render())?;

        nu_engine::eval_block::<nu_protocol::debugger::WithoutDebug>(
            &self.engine_state,
            &mut self.stack,
            &block,
            PipelineData::Empty
        )
            .map_err(Error::from)
    }
}