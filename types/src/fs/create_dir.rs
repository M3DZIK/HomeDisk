//! HTTP `/fs/createdir` Request and Response types

use serde::{Deserialize, Serialize};

/// `/fs/createdir` Request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    pub path: String,
}

/// `/fs/createdir` Reponse
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    /// Directory created?
    pub created: bool,
}
