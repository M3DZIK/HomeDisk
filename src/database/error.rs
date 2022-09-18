use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("User not found")]
    UserNotFound,
    #[error("Failed to open database: {0}")]
    OpenDatabase(sqlx::Error),
    #[error("Failed to open connection with database: {0}")]
    ConnectDatabase(sqlx::Error),
    #[error("Failed to get row: {0}")]
    GetRow(sqlx::Error),
    #[error("Failed to create all required tables: {0}")]
    CreateTables(sqlx::Error),
    #[error("Failed to execute the query: {0}")]
    Execute(sqlx::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
