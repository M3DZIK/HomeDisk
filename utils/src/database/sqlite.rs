use log::debug;
use sqlx::{sqlite::SqliteQueryResult, Executor, SqlitePool};

use super::{user, Error};

#[derive(Debug, Clone)]
pub struct Database {
    pub conn: SqlitePool,
}

impl Database {
    /// Open SQLite Database file
    /// ```
    /// use homedisk_utils::database::Database;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     Database::open("sqlite::memory:").await.unwrap();
    /// }
    /// ```
    pub async fn open(path: &str) -> Result<Self, Error> {
        debug!("opening SQLite database");

        let conn = SqlitePool::connect(path).await?;

        Ok(Self { conn })
    }

    /// Create new User
    /// ```
    /// use std::fs;
    ///
    /// use sqlx::Executor;
    /// use homedisk_utils::database::{Database, User};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let db = Database::open("sqlite::memory:").await.unwrap();
    ///
    ///     db.conn.execute(sqlx::query(&fs::read_to_string("../tables.sql").unwrap())).await.unwrap();
    ///
    ///     let user = User::new("medzik", "SuperSecretPassword123");
    ///     db.create_user(user).await.unwrap();
    /// }
    /// ```
    pub async fn create_user(&self, user: user::User) -> Result<SqliteQueryResult, Error> {
        debug!("creating user - {}", user.username);

        let query = sqlx::query("INSERT INTO user (id, username, password) VALUES (?, ?, ?)")
            .bind(user.id)
            .bind(user.username)
            .bind(user.password);

        Ok(self.conn.execute(query).await?)
    }
}
