use homedisk_utils::database;

#[tokio::main]
async fn main() {
    init();

    database::Database::connect().await.unwrap();
}

fn init() {
    better_panic::install();
}
