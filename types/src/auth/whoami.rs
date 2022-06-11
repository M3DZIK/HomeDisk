//! HTTP `/auth/whoami` Response type

use serde::{Deserialize, Serialize};

/// HTTP `/auth/whoami` Response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    /// Logged user username
    pub username: String,
}
