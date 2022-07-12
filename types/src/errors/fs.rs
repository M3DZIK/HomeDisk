use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error)]
pub enum Error {
    #[error("file doesn't exists")]
    FileDoesNotExist,
    #[error("file already exists")]
    FileAlreadyExists,
    #[error("unexpected multipart error")]
    MultipartError,
    #[error("failed to create create a file - {0}")]
    CreateFile(String),
    #[error("failed to create a directory - {0}")]
    CreateDirectory(String),
    #[error("failed to delete file: {0}")]
    DeleteFile(String),
    #[error("failed to delete directory: {0}")]
    DeleteDirectory(String),
    #[error("failed to write content to file: {0}")]
    WriteFile(String),
    #[error("failed to decode base64: {0}")]
    Base64(String),
    #[error("failed to read directory: {0}")]
    ReadDirectory(String),
    #[error("other error - {0}")]
    Other(String),
}
