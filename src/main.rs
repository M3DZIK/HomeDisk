//! HomeDisk cloud server
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//!
//! [![github]](https://github.com/MedzikUser/HomeDisk)
//!
//! ## Configure
//!
//! Go to [config] module

use std::{fs::File, path::Path};

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

#[tokio::main]
async fn main() {
    logger::init();

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
