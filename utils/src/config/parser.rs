use std::fs;

use serde::{Deserialize, Serialize};

use super::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub http: ConfigHTTP,
    pub jwt: ConfigJWT,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigHTTP {
    pub host: String,
    pub cors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigJWT {
    pub secret: String,
}

impl Config {
    /// parse configuration file
    pub fn parse() -> Result<Config, Error> {
        // configuration file path
        let config_dir = if dirs::config_dir() == None {
            return Err(Error::UnknowConfigDir());
        } else {
            dirs::config_dir().unwrap()
        };

        let config_path = format!("{}/homedisk/config.toml", config_dir.to_string_lossy());

        let config = fs::read_to_string(config_path)?;
        Ok(toml::from_str(&config)?)
    }
}
