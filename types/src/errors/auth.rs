use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error)]
pub enum Error {
    #[error("user not found")]
    UserNotFound,
    #[error("user already exists")]
    UserAlreadyExists,
    #[error("username is too short")]
    UsernameTooShort,
    #[error("username is too long")]
    UsernameTooLong,
    #[error("password is too short")]
    PasswordTooShort,
    #[error("failed to generate jwt token")]
    TokenGenerate,
    #[error("invalid jwt token")]
    InvalidToken,
    #[error("other error - {0}")]
    Other(String),
}
