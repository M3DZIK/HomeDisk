use std::fmt;

#[derive(Debug)]
pub enum Error {
    Axum(axum::Error),
    Hyper(hyper::Error),
    AddrParseError(std::net::AddrParseError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<axum::Error> for Error {
    fn from(err: axum::Error) -> Self {
        Error::Axum(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::Hyper(err)
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Self {
        Error::AddrParseError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Axum(err) => write!(f, "axum error: {}", err),
            Error::Hyper(err) => write!(f, "hyper error: {}", err),
            Error::AddrParseError(err) => write!(f, "AddrParse error: {}", err),
        }
    }
}
