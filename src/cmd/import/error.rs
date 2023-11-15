use std::fmt;

#[derive(Debug)]
pub enum ImportError {
    InvalidConfiguration,
    InvalidCluster(String),
}

impl std::error::Error for ImportError {}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImportError::InvalidConfiguration => write!(f, "invalid configuration provided"),
            ImportError::InvalidCluster(s) => write!(f, "{s}"),
        }
    }
}
