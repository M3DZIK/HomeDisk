use log::debug;
use sqlx::{sqlite::SqliteQueryResult, Executor, SqlitePool};

use super::{user, Error};

#[derive(Debug, Clone)]
pub struct Database {
    pub conn: SqlitePool,
}

impl Database {
    /// Open SQLite Database file
    /// ```ignore
    /// use homedisk_utils::database::Database;
    ///
    /// Database::open("sqlite::memory:").await?;
    /// ```
    pub async fn open(path: &str) -> Result<Self, Error> {
        debug!("opening SQLite database");

        let conn = SqlitePool::connect(path).await?;

        Ok(Self { conn })
    }

    /// Create new User
    /// ```ignore
    /// use homedisk_utils::database::{Database, User};
    ///
    /// let user = User::new("medzik", "SuperSecretPassword123");
    /// db.create_user(&user).await?;
    /// ```
    pub async fn create_user(&self, user: &user::User) -> Result<SqliteQueryResult, Error> {
        debug!("creating user - {}", user.username);

        let query = sqlx::query("INSERT INTO user (id, username, password) VALUES (?, ?, ?)")
            .bind(&user.id)
            .bind(&user.username)
            .bind(&user.password);

        Ok(self.conn.execute(query).await?)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use sqlx::Executor;

    use crate::database::{Database, User};

    async fn open_db() -> Database {
        Database::open("sqlite::memory:").await.expect("open db")
    }

    #[tokio::test]
    async fn open_db_in_memory() {
        open_db().await;
    }

    #[tokio::test]
    async fn create_user() {
        let db = open_db().await;

        // create user table
        db.conn
            .execute(sqlx::query(
                &fs::read_to_string("../tables.sql").expect("open tables file"),
            ))
            .await
            .expect("create tables");

        // create new user
        let user = User::new("medzik", "SuperSecretPassword123");
        db.create_user(&user).await.expect("create user");
    }
}
