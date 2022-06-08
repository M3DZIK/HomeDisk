//! HTTP `/fs/upload` Request and Response types

use serde::{Deserialize, Serialize};

/// `/fs/upload` Queries
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pagination {
    /// Path where the file will be uploaded
    pub path: String,
}

/// `/fs/upload` Response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    /// The file has been uploaded?
    pub uploaded: bool,
}
