use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    UnknownAlgorithm(&'static str, String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnknownAlgorithm(typ, named) => write!(f, "unknown {} algorithm {}", typ, named),
        }
    }
}
