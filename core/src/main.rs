mod init;

use homedisk_utils::{
    config::Config,
    database::{Database, User},
};

#[tokio::main]
async fn main() {
    init::init();

    let _config = Config::parse().expect("parse configuration file");

    let _db = Database::open("homedisk.db").expect("open SQLite database");

    let user = User::new("medzik", "password").unwrap();

    println!("{:?}", user);

    _db.create_user(user).unwrap();
}
