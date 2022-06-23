use thiserror::Error;

/// Database Error
#[derive(Debug, Error)]
pub enum Error {
    /// Username or Password incorrect.
    #[error("user not found")]
    UserNotFound,
    /// [sqlx::Error](https://docs.rs/sqlx/latest/sqlx/enum.Error.html)
    #[error("sqlx error - {0}")]
    SQLx(sqlx::Error),
    /// [std::io::Error]
    #[error("std::io error - {0}")]
    StdIo(std::io::Error),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::SQLx(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::StdIo(err)
    }
}

/// Custom Result alias for a [enum@Error].
pub type Result<T> = std::result::Result<T, Error>;
