//! HTTP `/fs/delete` Request type

use serde::{Deserialize, Serialize};

/// `/fs/delete` Request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    /// Path of file/directory to delete
    pub path: String,
}
