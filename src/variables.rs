/// Default SQLite file path for the database.
pub const DEFAULT_DATABASE_FILE: &str = "homedisk.db";
/// Default configuration file.
pub const DEFAULT_CONFIG_FILE: &str = "config.toml";
/// Default configuration file content.
pub static DEFAULT_CONFIG_CONTENT: &[u8] = include_bytes!("../config.toml");
