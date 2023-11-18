use std::fmt;

#[derive(Debug)]
pub enum CmdError {
    CommandNotFound,
}

impl std::error::Error for CmdError {}

impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CmdError::CommandNotFound => write!(f, "command not found"),
        }
    }
}

#[derive(Debug)]
pub enum ImportError {
    InvalidConfiguration,
    FileExists(String),
    InvalidCluster(String),
}

impl std::error::Error for ImportError {}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImportError::InvalidConfiguration => write!(f, "invalid configuration provided"),
            ImportError::InvalidCluster(s) => write!(f, "{s}"),
            ImportError::FileExists(s) => write!(f, "file {s} aready exists"),
        }
    }
}
