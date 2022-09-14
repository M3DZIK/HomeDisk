use thiserror::Error;

/// Database Error
#[derive(Debug, Error)]
pub enum Error {
    #[error("user not found")]
    UserNotFound,
    #[error("failed to open database: {0}")]
    OpenDatabase(sqlx::Error),
    #[error("failed to connect to the database: {0}")]
    ConnectDatabase(sqlx::Error),
    #[error("failed to get row: {0}")]
    GetRow(sqlx::Error),
    #[error("failed to create all required tables: {0}")]
    CreateTables(sqlx::Error),
    #[error("failed to execute the query: {0}")]
    Execute(sqlx::Error),
}

/// Custom Result alias for a [enum@Error].
pub type Result<T> = std::result::Result<T, Error>;
