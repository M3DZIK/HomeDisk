use std::fmt;

use crate::crypto;

#[derive(Debug)]
pub enum Error {
    Crypto(crypto::Error),
    Rusqlite(rusqlite::Error),
}

impl From<crypto::Error> for Error {
    fn from(err: crypto::Error) -> Self {
        Error::Crypto(err)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Error::Rusqlite(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Crypto(err) => write!(f, "crypto error: {}", err),
            Error::Rusqlite(err) => write!(f, "rusqlite error: {}", err),
        }
    }
}
