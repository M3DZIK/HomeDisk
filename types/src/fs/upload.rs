use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pagination {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    pub uploaded: bool,
}
