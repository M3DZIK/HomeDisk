//! [Configuration file types](Config)

use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub http: ConfigHTTP,
    pub jwt: ConfigJWT,
    pub storage: ConfigStorage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigHTTP {
    pub host: String,
    pub http_port: u16,
    pub https_port: u16,
    pub cors: Vec<String>,
    pub tls_cert: String,
    pub tls_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigJWT {
    pub secret: String,
    pub expires: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigStorage {
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
