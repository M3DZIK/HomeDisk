mod init;

use homedisk_utils::{config::Config, database::Database};

#[tokio::main]
async fn main() {
    init::init();

    let _config = Config::parse().expect("parse configuration file");

    let _db = Database::open().expect("open SQLite database");
}
