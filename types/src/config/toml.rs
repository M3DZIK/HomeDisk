use std::fs;

use anyhow::Result;

use crate::option_return;

use super::types::Config;

impl Config {
    /// Parse configuration file
    pub fn parse() -> Result<Config> {
        // config file path
        let config_dir = option_return!(dirs::config_dir(), "get config dir")?;
        let config_path = format!("{}/homedisk/config.toml", config_dir.to_string_lossy());

        // read file
        let config = fs::read_to_string(config_path)?;

        // parse config and return it
        Ok(toml::from_str(&config)?)
    }
}
