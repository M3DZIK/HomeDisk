// HTTP Error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // axum::Error
    #[error("axum error - {0}")]
    Axum(axum::Error),
    // hyper::Error
    #[error("hyper error - {0}")]
    Hyper(hyper::Error),
    // std::net::AddrParseError
    #[error("std::net::AddrParseError - {0}")]
    AddrParseError(std::net::AddrParseError),
}

/// Custom Result
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
