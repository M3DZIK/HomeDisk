use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum Error {
    #[error("file already exists")]
    FileAlreadyExists,

    #[error("file already exists")]
    WriteFile(String),

    #[error("base64 - {0}")]
    Base64(String),

    #[error("unknow error")]
    UnknowError(String),
}
