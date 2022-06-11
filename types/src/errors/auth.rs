use serde::{Deserialize, Serialize};

/// HTTP `/auth/*` Errors
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum Error {
    /// User not found!
    /// Username or Password incorrect.
    #[error("user not found")]
    UserNotFound,
    /// Cannot create a user because username already exists.
    #[error("user already exists")]
    UserAlreadyExists,
    /// Username is too short.
    #[error("username is too short")]
    UsernameTooShort,
    /// Username is too long.
    #[error("username is too long")]
    UsernameTooLong,
    /// Password is too short.
    #[error("password is too short")]
    PasswordTooShort,
    /// Failed to generate user token.
    #[error("generate jwt token")]
    TokenGenerate,
    /// Incorrect user token.
    #[error("invalid jwt token")]
    InvalidToken,
    /// Other error.
    #[error("other error - {0}")]
    Other(String),
}
