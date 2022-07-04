//! HTTP `/auth/login` Request and Response types

use serde::{Deserialize, Serialize};

/// HTTP `/auth/login` Request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    /// Username
    pub username: String,
    /// Unencrypted user password
    pub password: String,
}

/// HTTP `/auth/login` Response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Response {
    LoggedIn {
        /// User access token
        access_token: String,
    },
}
