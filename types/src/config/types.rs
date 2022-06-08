//! Configuration file types

use serde::{Deserialize, Serialize};

/// Config type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub http: ConfigHTTP,
    pub jwt: ConfigJWT,
    pub storage: ConfigStorage,
}

/// HTTP config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigHTTP {
    /// HTTP Host
    pub host: String,
    /// Port HTTP Port
    pub port: u16,
    /// CORS Domains (e.g ["site1.example.com", "site2.example.com"])
    pub cors: Vec<String>,
}

/// Json Web Token config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigJWT {
    /// JWT Secret string (used to sign tokens)
    pub secret: String,
    /// Token expiration time in hours
    pub expires: i64,
}

/// Storage config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigStorage {
    /// Directory where user files will be stored
    pub path: String,
}
