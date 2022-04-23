use std::fmt;

#[derive(Debug)]
pub enum Error {
    UserNotFound,
    SQLx(sqlx::Error),
    Io(std::io::Error),
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
            Error::SQLx(err) => write!(f, "sqlx error: {}", err),
            Error::Io(err) => write!(f, "std::io error: {}", err),
        }
    }
}
