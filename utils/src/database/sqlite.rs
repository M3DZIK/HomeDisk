use log::debug;
use rusqlite::Connection;

use crate::crypto;

use super::{user, Error};

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn open() -> Result<Self, Error> {
        debug!("opening SQLite database");

        let conn = Connection::open("homedisk.db")?;
        Ok(Self { conn })
    }

    pub fn create_user(&mut self, user: user::User) -> Result<usize, Error> {
        debug!("creating user - {}", user.username);

        // hash password using sha512
        let password = crypto::hasher("sha512", user.password)?;

        Ok(self.conn.execute(
            "INSERT INTO user (username, password) VALUES (?1, ?2)",
            [user.username, password],
        )?)
    }
}
