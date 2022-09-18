use std::str::FromStr;

use futures_util::TryStreamExt;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteQueryResult, SqliteRow},
    ConnectOptions, Executor, Row, SqlitePool,
};
use tracing::{debug, info, log::LevelFilter};

use super::{
    error::{Error, Result},
    User,
};

/// SQLite database
#[derive(Clone)]
pub struct Database {
    /// Sqlite connection pool
    pub pool: SqlitePool,
}

impl Database {
    /// Open a SQLite database
    pub async fn open(path: &str) -> Result<Self> {
        debug!("Opening SQLite database: {}", path);

        // sqlite connection options
        let mut options =
            SqliteConnectOptions::from_str(path).map_err(|e| Error::OpenDatabase(e.to_string()))?;

        // set log level to Debug
        options.log_statements(LevelFilter::Debug);

        // create a database pool
        let pool = SqlitePool::connect_with(options)
            .await
            .map_err(|e| Error::ConnectDatabase(e.to_string()))?;

        info!("Connected to database!");

        Ok(Self { pool })
    }

    /// Create all required tables for HomeDisk.
    pub async fn create_tables(&self) -> Result<SqliteQueryResult> {
        self.pool
            .execute(include_str!("../../tables.sql"))
            .await
            .map_err(|e| Error::CreateTables(e.to_string()))
    }

    /// Create new user in the database.
    pub async fn create_user(&self, user: &User) -> Result<SqliteQueryResult> {
        debug!("Creating user - {}", user.username);

        // build sql query
        let query = sqlx::query("INSERT INTO user (id, username, password) VALUES (?, ?, ?)")
            .bind(&user.id)
            .bind(&user.username)
            .bind(&user.password);

        self.pool
            .execute(query)
            .await
            .map_err(|e| Error::Execute(e.to_string()))
    }

    /// Search for a user.
    pub async fn find_user(&self, user: &User) -> Result<User> {
        debug!("Searching for a user - {}", user.username);

        // create query request to database
        let query = sqlx::query("SELECT * FROM user WHERE username = ? AND password = ?")
            .bind(&user.username)
            .bind(&user.password);

        // fetch query
        let mut stream = self.pool.fetch(query);

        // get rows from query
        let row = stream
            .try_next()
            .await
            .map_err(|e| Error::Execute(e.to_string()))?
            .ok_or(Error::UserNotFound)?;

        Self::find(row)
    }

    /// Search for a user by UUID.
    pub async fn find_user_by_id(&self, id: &str) -> Result<User> {
        debug!("Searching for a user by UUID - {}", id);

        // create query request to database
        let query = sqlx::query("SELECT * FROM user WHERE id = ?").bind(id);

        // fetch query
        let mut stream = self.pool.fetch(query);

        // get rows from query
        let row = stream
            .try_next()
            .await
            .map_err(|e| Error::Execute(e.to_string()))?
            .ok_or(Error::UserNotFound)?;

        Self::find(row)
    }

    fn find(row: SqliteRow) -> Result<User> {
        // get `id` row
        let id = row
            .try_get("id")
            .map_err(|e| Error::GetRow(e.to_string()))?;
        // get `username` row
        let username = row
            .try_get("username")
            .map_err(|e| Error::GetRow(e.to_string()))?;
        // get `password` row
        let password = row
            .try_get("password")
            .map_err(|e| Error::GetRow(e.to_string()))?;

        Ok(User {
            id,
            username,
            password,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const USERNAME: &str = "medzik";
    const PASSWORD: &str = "SuperSecretPassword123!";

    async fn open_db() -> Database {
        Database::open("sqlite::memory:")
            .await
            .expect("Failed to open database in memory")
    }

    async fn new_user() -> Database {
        let db = open_db().await;

        // create tables
        db.create_tables().await.expect("create tables");

        // create new user
        let user = User::new(USERNAME, PASSWORD, true);
        db.create_user(&user).await.expect("create user");

        db
    }

    #[tokio::test]
    async fn test_create_user() {
        new_user().await;
    }

    #[tokio::test]
    async fn test_find_user() {
        let db = new_user().await;

        let user = db
            .find_user(&User::new(USERNAME, PASSWORD, false))
            .await
            .unwrap();

        assert_eq!(user.username, USERNAME)
    }

    #[tokio::test]
    async fn test_find_user_wrong_password() {
        let db = new_user().await;

        let err = db
            .find_user(&User::new(USERNAME, "wrong password 123!", false))
            .await
            .unwrap_err();

        assert_eq!(err, Error::UserNotFound)
    }
}
