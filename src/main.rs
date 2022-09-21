//! HomeDisk cloud server
//!
//! [Source code available on GitHub](https://github.com/MedzikUser/HomeDisk)
//!
//! ## Configure
//!
//! Go to [config] module

use std::{fs::File, io::Write, path::Path};

use config::Config;
use tracing::{info, warn};

use crate::database::Database;

#[path = "./types/config.rs"]
mod config;
mod database;
mod logger;
mod server;

/// Default SQLite file path for the database.
pub const DATABASE_FILE: &str = "homedisk.db";
/// Default configuration file.
pub const CONFIG_FILE: &str = "config.toml";
/// Default configuration file content.
pub static DEFAULT_CONFIG_CONTENT: &'static [u8] = include_bytes!("../config.toml");

#[tokio::main]
async fn main() {
    logger::init();

    // if configuration file doesn't exist, create it
    if !Path::new(CONFIG_FILE).exists() {
        warn!("Configuration file doesn't exists.");

        let mut file =
            File::create(CONFIG_FILE).expect("notrace - Failed to create configuration file");

        file.write_all(DEFAULT_CONFIG_CONTENT)
            .expect("notrace - Failed to write default configuration to config file");

        info!("Created default configuration file.");

        std::process::exit(0);
    }

    let config = Config::parse(CONFIG_FILE).expect("notrace - Failed to parse configuration file");

    // open database connection
    let db =
        // if database file doesn't exists create it
        if !Path::new(DATABASE_FILE).exists() {
            warn!("Database file doesn't exists.");
            info!("Creating database file...");

            // create an empty database file
            File::create(DATABASE_FILE).expect("notrace - Failed to create a database file");

            // open database file
            let db = Database::open(DATABASE_FILE)
                .await
                .expect("notrace - Failed to open database file");

            // create tables in the database
            db.create_tables()
                .await
                .expect("notrace - Failed to create tables in the database");

            db
        }
        // if database file exists
        else {
            Database::open(DATABASE_FILE)
                .await
                .expect("notrace - Failed to open database file")
        };

    server::start_server(config, db)
        .await
        .expect("notrace - HTTP Server error");
}
