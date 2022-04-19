use std::fs;

use anyhow::Result;

use crate::option_return;

use super::types::Config;

impl Config {
    /// parse configuration file
    pub fn parse() -> Result<Config> {
        // configuration file path
        let config_dir = option_return!(dirs::config_dir(), "get config dir")?;

        let config_path = format!("{}/homedisk/config.toml", config_dir.to_string_lossy());

        let config = fs::read_to_string(config_path)?;
        Ok(toml::from_str(&config)?)
    }
}
