use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub http: ConfigHTTP,
    pub jwt: ConfigJWT,
    pub storage: ConfigStorage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigHTTP {
    /// HTTP Host
    pub host: String,
    /// Port HTTP Port
    pub port: u16,
    /// CORS Domaing (e.g ["site1.example.com", "site2.example.com"])
    pub cors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigJWT {
    /// JWT Secret string
    pub secret: String,
    /// Token expiers time in seconds
    pub expires: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigStorage {
    /// Directory where user files will be stored
    pub path: String,
}
