use log::debug;
use rusqlite::Connection;

use super::{user, Error};

pub struct Database {
    pub conn: Connection,
}

impl Database {
    /// Open SQLite Database file
    /// ```
    /// use homedisk_utils::database::Database;
    ///
    /// Database::open("/tmp/homedisk.db").unwrap();
    /// ```
    pub fn open(path: &str) -> Result<Self, Error> {
        debug!("opening SQLite database");

        let conn = Connection::open(path)?;

        Ok(Self { conn })
    }

    /// Create new User
    /// ```
    /// use std::fs;
    ///
    /// use rusqlite::Connection;
    /// use homedisk_utils::database::{Database, User};
    ///
    /// let db = Database { conn: Connection::open_in_memory().unwrap() };
    /// let user = User::new("medzik", "SuperSecretPassword123").unwrap();
    ///
    /// db.conn.execute(&fs::read_to_string("../tables.sql").unwrap(), []).unwrap();
    ///
    /// db.create_user(user).unwrap();
    /// ```
    pub fn create_user(&self, user: user::User) -> Result<usize, Error> {
        debug!("creating user - {}", user.username);

        Ok(self.conn.execute(
            "INSERT INTO user (id, username, password) VALUES (?1, ?2, ?3)",
            [user.id, user.username, user.password],
        )?)
    }
}
