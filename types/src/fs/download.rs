//! HTTP `/fs/download` Request type

use serde::{Deserialize, Serialize};

/// HTTP `/fs/download` Request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    /// Path of file to be download
    pub path: String,
}
