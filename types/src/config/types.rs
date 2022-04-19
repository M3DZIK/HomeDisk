use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub http: ConfigHTTP,
    pub jwt: ConfigJWT,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigHTTP {
    pub host: String,
    pub port: u16,
    pub cors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigJWT {
    pub secret: String,
}
