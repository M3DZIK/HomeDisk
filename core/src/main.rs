use homedisk_utils::{database::Database, config::Config};

#[tokio::main]
async fn main() {
    init();

    let _config = Config::parse().expect("parse configuration file");

    let _db = Database::connect().await.expect("open SQLite database");
}

fn init() {
    better_panic::install();
}
