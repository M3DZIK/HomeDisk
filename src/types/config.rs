//! [Configuration file types](Config).
use std::fs;

use serde::{Deserialize, Serialize};

/// Settings configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// HTTP Settings.
    pub http: ConfigHTTP,
    /// Json Web Token Settings.
    pub jwt: ConfigJWT,
    /// Storage Settings.
    pub storage: ConfigStorage,
}

/// HTTP Settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigHTTP {
    /// HTTP Host.
    pub host: String,
    /// Port HTTP Port.
    pub port: u16,
    /// [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) Domains (e.g ["site1.example.com", "site2.example.com"]).
    pub cors: Vec<String>,
}

/// Json Web Token Settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigJWT {
    /// JWT Secret string (used to sign tokens).
    pub secret: String,
    /// Token expiration time in hours.
    pub expires: i64,
}

/// Storage Settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigStorage {
    /// Directory where user files will be stored.
    pub path: String,
}

impl Config {
    /// Parse configuration file.
    pub fn parse(path: &str) -> anyhow::Result<Self> {
        let config_str = fs::read_to_string(path)?;

        let config = toml::from_str(&config_str)?;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test default configuration file.
    #[test]
    fn test_config() {
        Config::parse("./config.toml").expect("Failed to parse configuration file");
    }
}
