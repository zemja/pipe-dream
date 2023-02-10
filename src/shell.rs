use nu_protocol::PipelineData;
use s_macro::s;

pub struct Nushell {
    engine_state: nu_protocol::engine::EngineState,
    stack: nu_protocol::engine::Stack,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to parse command: {0:?}")]
    Parse(#[from] nu_parser::ParseError),
    #[error("failed to evaluate command: {0:?}")]
    Shell(#[from] nu_protocol::ShellError),
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),
    #[error("path is not valid UTF-8")]
    InvalidPath,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Nushell {
    pub fn new() -> Result<Nushell> {
        let mut nushell = Nushell {
            // Infuriating that this function just prints an error message if it fails and doesn't
            // return a Result. But the only failure is something to do with plugins, so fingers
            // crossed that's not my problem.
            engine_state: nu_command::create_default_context(),
            stack: nu_protocol::engine::Stack::new()
        };

        // TODO Is this really the best way to handle errors here? (CDing to the current directory on startup.) See nu_cli::util::get_init_cwd().
        if let Some(pwd) = std::env::current_dir().ok().or_else(dirs::home_dir) {
            nushell.stack.add_env_var(
                s!("PWD"),
                nu_protocol::Value::String {
                    val: pwd.into_os_string().into_string().map_err(|_| Error::InvalidPath)?,
                    span: nu_protocol::Span::unknown()
                }
            );
        }

        Ok(nushell)
    }

    pub fn evaluate(&mut self, command: &str) -> Result<PipelineData> {
        let mut working_set = nu_protocol::engine::StateWorkingSet::new(&self.engine_state);
        let block = match nu_parser::parse(&mut working_set, None, command.as_bytes(), false, &[]) {
            (_, Some(err)) => return Err(err.into()),
            (block, _) => block,
        };

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