#[derive(Debug, PartialEq)]
pub enum Error {
    IO(String, std::io::ErrorKind),
    Deserialize(String),
    UnknownVersion(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IO(str, kind) => write!(f, "IO error {kind} encountered: {str}"),
            Error::Deserialize(str) => {
                write!(f, "deserialization error encountered: {str}")
            }
            Error::UnknownVersion(version) => write!(f, "encountered unknown version {version}"),
        }
    }
}
