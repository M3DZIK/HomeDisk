//! HTTP `/auth/login` Request and Response types

use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// HTTP `/auth/login` Request
#[derive(Debug, Serialize, Deserialize, Clone, Zeroize, ZeroizeOnDrop)]
pub struct Request {
    /// Username
    pub username: String,
    /// Unencrypted user password
    pub password: String,
}

/// HTTP `/auth/login` Response
#[derive(Debug, Serialize, Deserialize, Clone, Zeroize, ZeroizeOnDrop)]
pub enum Response {
    #[allow(missing_docs)]
    LoggedIn {
        /// User access token
        access_token: String,
    },
}
