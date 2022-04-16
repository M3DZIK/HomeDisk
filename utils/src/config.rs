use std::fs;

use anyhow::Error;
use serde::{Deserialize, Serialize};

// configuration file path
static CONFIG_PATH: &str = "config.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub http: ConfigHTTP,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigHTTP {
    pub host: String,
    pub port: u32,
    pub cors: String,
}

impl Config {
    /// parse configuration file
    pub fn parse() -> Result<Config, Error> {
        let config = fs::read_to_string(CONFIG_PATH)?;
        Ok(toml::from_str(&config)?)
    }
}
