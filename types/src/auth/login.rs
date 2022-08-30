//! HTTP `/auth/login` Request and Response types

use serde::{Deserialize, Serialize};

/// HTTP `/auth/login` Request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    pub username: String,
    pub password: String,
}

/// HTTP `/auth/login` Response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    pub access_token: String,
}
