//! HTTP `/fs/createdir` Request and Response types

use serde::{Deserialize, Serialize};

/// HTTP `/fs/createdir` Request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    /// Path to directory wich will be created
    pub path: String,
}
