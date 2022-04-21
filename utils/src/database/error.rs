use std::fmt;

use crate::crypto;

#[derive(Debug)]
pub enum Error {
    UserNotFound,
    Crypto(crypto::Error),
    SQLx(sqlx::Error),
    Io(std::io::Error),
}

impl From<crypto::Error> for Error {
    fn from(err: crypto::Error) -> Self {
        Error::Crypto(err)
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::SQLx(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UserNotFound => write!(f, "user not found"),
            Error::Crypto(err) => write!(f, "crypto error: {}", err),
            Error::SQLx(err) => write!(f, "sqlx error: {}", err),
            Error::Io(err) => write!(f, "error: {}", err),
        }
    }
}
