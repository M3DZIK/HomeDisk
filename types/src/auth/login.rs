//! `/auth/login` Request and Response types

use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// `/auth/login` Request
#[derive(Debug, Serialize, Deserialize, Clone, Zeroize, ZeroizeOnDrop)]
pub struct Request {
    /// Username
    pub username: String,
    /// Unencrypted user password
    pub password: String,
}

/// `/auth/login` Response
#[derive(Debug, Serialize, Deserialize, Clone, Zeroize, ZeroizeOnDrop)]
pub enum Response {
    LoggedIn {
        /// Token of a user
        access_token: String,
    },
}
