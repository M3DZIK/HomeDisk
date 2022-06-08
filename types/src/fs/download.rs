//! HTTP `/fs/download` Request type

use serde::{Deserialize, Serialize};

/// `/fs/download` Request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    /// Path of file to download
    pub path: String,
}
