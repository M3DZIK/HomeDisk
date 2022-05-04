use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum Error {
    #[error("file already exists")]
    FileAlreadyExists,

    #[error("unexpected multipart error")]
    MultipartError,

    #[error("create file - {0}")]
    CreateFile(String),

    #[error("write file - {0}")]
    WriteFile(String),

    #[error("base64 - {0}")]
    Base64(String),

    #[error("read dir - {0}")]
    ReadDir(String),

    #[error("unknow error")]
    UnknowError(String),
}
