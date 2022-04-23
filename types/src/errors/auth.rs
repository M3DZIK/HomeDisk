use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum Error {
    #[error("user not found")]
    UserNotFound,

    #[error("user already exists")]
    UserAlreadyExists,

    #[error("generate jwt token")]
    TokenGenerate,

    #[error("invalid jwt token")]
    InvalidToken,

    #[error("unknow error")]
    UnknowError(String),
}
