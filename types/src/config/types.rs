//! Configuration file types

use std::fs;

use serde::{Deserialize, Serialize};

use crate::option_return;

/// Config type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Configure HTTP settings
    pub http: ConfigHTTP,
    /// Configure Json Web Token settings
    pub jwt: ConfigJWT,
    /// Configure storage settings
    pub storage: ConfigStorage,
}

/// HTTP config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigHTTP {
    /// HTTP Host
    pub host: String,
    /// Port HTTP Port
    pub port: u16,
    /// [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) Domains (e.g ["site1.example.com", "site2.example.com"])
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

#[cfg(feature = "config")]
impl Config {
    /// Parse configuration file.
    ///
    /// ```no_run
    /// use homedisk_types::config::Config;
    ///
    /// let config = Config::parse().unwrap();
    /// ```
    pub fn parse() -> anyhow::Result<Config> {
        // get file path of config file
        let config_dir = option_return!(dirs::config_dir(), "find config dir")?;
        let config_path = format!("{}/homedisk/config.toml", config_dir.to_string_lossy());

        // read file content to string
        let config = fs::read_to_string(config_path)?;

        // parse config and return it
        Ok(toml::from_str(&config)?)
    }
}
