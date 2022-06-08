use serde::{Deserialize, Serialize};

/// `/fs/*` Error
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum Error {
    #[error("file already exists")]
    FileAlreadyExists,

    #[error("file doesn't exists")]
    FileDoesNotExist,

    #[error("unexpected multipart error")]
    MultipartError,

    #[error("create file - {0}")]
    CreateFile(String),

    #[error("create dir - {0}")]
    CreateDirectory(String),

    #[error("delete file - {0}")]
    DeleteFile(String),

    #[error("delete dir - {0}")]
    DeleteDirectory(String),

    #[error("write file - {0}")]
    WriteFile(String),

    #[error("base64 - {0}")]
    Base64(String),

    #[error("read dir - {0}")]
    ReadDir(String),

    #[error("unknown error - {0}")]
    UnknownError(String),
}
