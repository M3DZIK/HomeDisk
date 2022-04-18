use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum Error {
    #[error("user not found")]
    UserNotFound,

    #[error("user already exists")]
    UserAlreadyExists,
}
