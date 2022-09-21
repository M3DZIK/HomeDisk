//! HomeDisk cloud server
//!
//! [Source code available on GitHub](https://github.com/MedzikUser/HomeDisk)
//!
//! ## Configure
//!
//! Go to [config] module

use std::{fs::File, io::Write, path::Path};

use clap::Parser;
use config::Config;
use tracing::{info, warn};

use crate::database::Database;

mod config;
mod database;
mod logger;
mod server;
mod variables;

pub use variables::*;

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
)]
struct Cli {
    #[clap(short = 'c', long = "config", help = "Configuration file path", default_value = DEFAULT_CONFIG_FILE, display_order = 1)]
    config: String,
    #[clap(short = 'd', long = "database", help = "SQLite database path", default_value = DEFAULT_DATABASE_FILE, display_order = 2)]
    database: String,
}

#[tokio::main]
async fn main() {
    logger::init();

    let args = Cli::parse();

    let config_path = args.config;
    let database_path = &args.database;

    // if configuration file doesn't exist, create it
    if !Path::new(&config_path).exists() {
        warn!("Configuration file doesn't exists.");

        let mut file =
            File::create(config_path).expect("notrace - Failed to create configuration file");

        file.write_all(DEFAULT_CONFIG_CONTENT)
            .expect("notrace - Failed to write default configuration to config file");

        info!("Created configuration file. Exiting...");

        std::process::exit(0);
    }

    let config = Config::parse(&config_path).expect("notrace - Failed to parse configuration file");

    // open database connection
    let db =
        // if database file doesn't exists create it
        if !Path::new(&database_path).exists() {
            warn!("Database file doesn't exists.");
            info!("Creating database file...");

            // create an empty database file
            File::create(database_path).expect("notrace - Failed to create a database file");

            // open database file
            let db = Database::open(database_path)
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
            Database::open(database_path)
                .await
                .expect("notrace - Failed to open database file")
        };

    server::start_server(config, db)
        .await
        .expect("notrace - HTTP Server error");
}
