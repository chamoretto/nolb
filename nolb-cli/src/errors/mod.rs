use crate::config::file_format::FileFormat;
use std::ffi::OsString;
use std::fmt::Formatter;
use std::io::ErrorKind;

pub(crate) type CliResult<T> = Result<T, CliCommandError>;

#[derive(Debug, derive_new::new)]
pub(crate) struct Warning {
    config_file_path: OsString,
    description: String,
    suggestion: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub(crate) enum CliCommandError {
    #[error("errors found during validating or parsing config: {0}")]
    Config(ConfigError),
    #[error("unable to find any valid config at current work directory")]
    NoConfig,
    #[error("error communicating with remote hosts: {0}")]
    Network(String),
    #[error("I/O error: {0}")]
    Io(#[source] std::io::Error),
    #[error("error returned from the OS: {0}")]
    Os(#[source] std::io::Error),
    #[error("unable to find file by the given path `{path:?}`: {source}")]
    FileNotFound {
        path: Option<String>,
        #[source]
        source: std::io::Error,
    },
}

impl From<std::io::Error> for CliCommandError {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            ErrorKind::NotFound => CliCommandError::FileNotFound { path: None, source: e },

            ErrorKind::ConnectionRefused
            | ErrorKind::ConnectionReset
            | ErrorKind::ConnectionAborted
            | ErrorKind::NotConnected
            | ErrorKind::AddrNotAvailable => CliCommandError::Network(e.to_string()),

            ErrorKind::PermissionDenied
            | ErrorKind::AddrInUse
            | ErrorKind::BrokenPipe
            | ErrorKind::AlreadyExists
            | ErrorKind::WouldBlock
            | ErrorKind::InvalidInput
            | ErrorKind::InvalidData
            | ErrorKind::TimedOut
            | ErrorKind::WriteZero
            | ErrorKind::Interrupted
            | ErrorKind::Unsupported
            | ErrorKind::UnexpectedEof
            | ErrorKind::OutOfMemory
            | ErrorKind::Other => CliCommandError::Os(e),

            _ => CliCommandError::Os(e),
        }
    }
}

macro impl_from_for_ser_de_errors($path_to_error: path) {
    impl From<$path_to_error> for CliCommandError {
        fn from(e: $path_to_error) -> CliCommandError {
            CliCommandError::Config(ConfigError::Deserialization(e.to_string()))
        }
    }
}

impl_from_for_ser_de_errors!(serde_json::error::Error);
impl_from_for_ser_de_errors!(serde_yaml::Error);
impl_from_for_ser_de_errors!(toml::de::Error);

#[derive(Debug, thiserror::Error)]
pub(crate) enum ConfigError {
    /// This variant always comes from ['serde'](serde)
    #[error("error during validation: {0}")]
    Deserialization(String),
    /// This variant represents logical errors in configuration files
    #[error("DSL error: {0:?}")]
    Logical(Vec<LogicalError>),
}

#[derive(Debug, derive_new::new)]
pub(crate) struct LogicalError {
    config_file_path: OsString,
    // format: FileFormat,
    elements_tree_sequence: Vec<String>,
    description: String,
    suggestion: Option<String>,
}

impl std::fmt::Display for LogicalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let suggestion = match &self.suggestion {
            None => "".to_string(),
            Some(s) => {
                format!("\nHelp: {}", s)
            },
        };

        let mut path = String::new();
        for (index, element) in self.elements_tree_sequence.iter().enumerate() {
            path.push('\n');
            path.push_str(&"\t".repeat(index));
            path.push_str(element);
        }

        write!(
            f,
            "Error during config's DSL check.\nSource file: '{}'\nDescription: {}\nPath to wrong property: {}{}",
            self.config_file_path.to_string_lossy(),
            // self.format,
            self.description,
            path,
            suggestion
        )
    }
}
