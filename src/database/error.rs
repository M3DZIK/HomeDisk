use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    #[error("User not found")]
    UserNotFound,
    #[error("Failed to open database: {0}")]
    OpenDatabase(String),
    #[error("Failed to open connection with database: {0}")]
    ConnectDatabase(String),
    #[error("Failed to get row: {0}")]
    GetRow(String),
    #[error("Failed to create all required tables: {0}")]
    CreateTables(String),
    #[error("Failed to execute the query: {0}")]
    Execute(String),
}

pub type Result<T> = std::result::Result<T, Error>;
