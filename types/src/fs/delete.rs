//! HTTP `/fs/delete` Request type

use serde::{Deserialize, Serialize};

/// HTTP `/fs/delete` Request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    /// Path of file/directory to be delete
    pub path: String,
}
