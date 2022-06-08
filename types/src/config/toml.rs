use std::fs;

use anyhow::Result;

use crate::option_return;

use super::types::Config;

impl Config {
    /// Parse configuration file
    /// ```ignore,rust
    /// use homedisk_types::config::Config;
    ///
    /// let config = Config::parse().unwrap();
    /// ```
    pub fn parse() -> Result<Config> {
        // get file path of config file
        let config_dir = option_return!(dirs::config_dir(), "find config dir")?;
        let config_path = format!("{}/homedisk/config.toml", config_dir.to_string_lossy());

        // read file content to string
        let config = fs::read_to_string(config_path)?;

        // parse config and return it
        Ok(toml::from_str(&config)?)
    }
}
