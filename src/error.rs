// This is an error wrapping all possible errors in this crate.
#[derive(Debug)]
pub enum Error {
    // Variant for error messages reported within this crate.
    Message(String),

    // Variants that wrap error messages from crates (either std crates
    // or external ones).
    IO(std::io::Error),
    YAML(serde_yaml::Error),
    JSON(serde_json::Error),
    URLParse(url::ParseError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Message(msg) => write!(f, "{msg}"),
            Error::IO(err) => write!(f, "IO error encountered: {err}"),
            Error::YAML(err) => write!(f, "YAML (de-)serialize error: {err}"),
            Error::JSON(err) => write!(f, "JSON (de-)serialize error: {err}"),
            Error::URLParse(err) => write!(f, "failed to parse URL: {err}"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Error {
        Error::YAML(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::JSON(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::URLParse(err)
    }
}
