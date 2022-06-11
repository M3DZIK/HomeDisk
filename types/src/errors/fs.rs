use serde::{Deserialize, Serialize};

/// `/fs/*` Error
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum Error {
    /// File doesn't exists.
    #[error("file doesn't exists")]
    FileDoesNotExist,
    /// File already exists.
    #[error("file already exists")]
    FileAlreadyExists,
    /// Error when parsing multipart.
    #[error("unexpected multipart error")]
    MultipartError,
    /// Failed to create a file.
    #[error("create file - {0}")]
    CreateFile(String),
    /// Failed to create a directory.
    #[error("create dir - {0}")]
    CreateDirectory(String),
    /// Failed to delete file.
    #[error("delete file - {0}")]
    DeleteFile(String),
    /// Failed to delete directory.
    #[error("delete dir - {0}")]
    DeleteDirectory(String),
    /// Failed to write content to file.
    #[error("write file - {0}")]
    WriteFile(String),
    /// Failed decoding base64.
    #[error("base64 - {0}")]
    Base64(String),
    /// Error when paths in directory.
    #[error("read dir - {0}")]
    ReadDirectory(String),
    /// Other error.
    #[error("other error - {0}")]
    Other(String),
}
