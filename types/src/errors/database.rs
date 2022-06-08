/// Database Error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("user not found")]
    UserNotFound,
    /// sqlx::Error
    #[error("sqlx error - {0}")]
    SQLx(sqlx::Error),
    /// std::io::Error
    #[error("std::io error - {0}")]
    StdIo(std::io::Error),
}

/// sqlx::Error
impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::SQLx(err)
    }
}

/// std::io::Error
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::StdIo(err)
    }
}
