//! HTTP `/fs/upload` Request and Response types

use serde::{Deserialize, Serialize};

/// HTTP `/fs/upload` Queries
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pagination {
    /// Path where the file will be uploaded
    pub path: String,
}
