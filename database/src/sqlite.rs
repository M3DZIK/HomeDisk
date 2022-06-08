use futures_util::TryStreamExt;
use log::debug;
use sqlx::{sqlite::SqliteQueryResult, Executor, Row, SqlitePool};

use super::{Error, User};

#[derive(Debug, Clone)]
pub struct Database {
    /// Sqlite Connection Pool
    pub conn: SqlitePool,
}

impl Database {
    /// Open SQLite Database file
    /// ```ignore,rust
    /// use homedisk_database::Database;
    ///
    /// Database::open("sqlite::memory:").await?;
    /// ```
    pub async fn open(path: &str) -> Result<Self, Error> {
        debug!("Opening SQLite database");

        let conn = SqlitePool::connect(path).await?;

        Ok(Self { conn })
    }

    /// Create new User
    /// ```ignore,rust
    /// use homedisk_database::{Database, User};
    ///
    /// let user = User::new("username", "password");
    /// db.create_user(&user).await?;
    /// ```
    pub async fn create_user(&self, user: &User) -> Result<SqliteQueryResult, Error> {
        debug!("Creating user - {}", user.username);

        let query = sqlx::query("INSERT INTO user (id, username, password) VALUES (?, ?, ?)")
            .bind(&user.id)
            .bind(&user.username)
            .bind(&user.password);

        Ok(self.conn.execute(query).await?)
    }

    /// Search for a user
    /// ```ignore,rust
    /// use homedisk_database::{Database, User};
    ///
    /// let user = User::new("username", "password");
    ///
    /// db.find_user(&user.username, &user.password).await?;
    /// ```
    pub async fn find_user(&self, username: &str, password: &str) -> Result<User, Error> {
        debug!("Searching for a user - {}", username);

        let query =
            sqlx::query_as::<_, User>("SELECT * FROM user WHERE username = ? AND password = ?")
                .bind(username)
                .bind(password);

        let mut stream = self.conn.fetch(query);

        let row = stream.try_next().await?.ok_or(Error::UserNotFound)?;

        let id = row.try_get("id")?;
        let username = row.try_get("username")?;
        let password = row.try_get("password")?;

        Ok(User {
            id,
            username,
            password,
        })
    }

    /// Search for a user by UUID
    /// ```ignore,rust
    /// use homedisk_database::{Database, User};
    ///
    /// let user = User::new("username", "password");
    ///
    /// db.find_user_by_id(&user.id).await?;
    /// ```
    pub async fn find_user_by_id(&self, id: String) -> Result<User, Error> {
        debug!("Searching for a user by UUID - {}", id);

        let query = sqlx::query_as::<_, User>("SELECT * FROM user WHERE id = ?").bind(id);

        let mut stream = self.conn.fetch(query);

        let row = stream.try_next().await?.ok_or(Error::UserNotFound)?;

        let id = row.try_get("id")?;
        let username = row.try_get("username")?;
        let password = row.try_get("password")?;

        Ok(User {
            id,
            username,
            password,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use sqlx::Executor;

    use crate::{Database, User};

    /// Utils to open database in tests    
    async fn open_db() -> Database {
        Database::open("sqlite::memory:").await.expect("open db")
    }

    /// Utils to create a new user in tests
    async fn new_user(db: &Database) {
        // create user table
        db.conn
            .execute(sqlx::query(
                &fs::read_to_string("../tables.sql").expect("open tables file"),
            ))
            .await
            .expect("create tables");

        // create new user
        let user = User::new("medzik", "Qwerty1234!");
        db.create_user(&user).await.expect("create user");
    }

    /// Open database in memory
    #[tokio::test]
    async fn open_db_in_memory() {
        open_db().await;
    }

    /// Create a new user
    #[tokio::test]
    async fn create_user() {
        let db = open_db().await;

        new_user(&db).await;
    }

    /// Search for a user
    #[tokio::test]
    async fn find_user() {
        let db = open_db().await;

        new_user(&db).await;

        let user = User::new("medzik", "Qwerty1234!");

        let res = db
            .find_user(&user.username, &user.password)
            .await
            .expect("find user");

        assert_eq!(res.password, user.password)
    }

    /// Search for a user with an invalid password to see if the user is returned (it shouldn't be)
    #[tokio::test]
    async fn find_user_wrong_password() {
        let db = open_db().await;

        new_user(&db).await;

        let user = User::new("medzik", "wrong password 123!");

        let err = db
            .find_user(&user.username, &user.password)
            .await
            .unwrap_err();

        assert_eq!(err.to_string(), "user not found")
    }

    /// Search for a user who does not exist
    #[tokio::test]
    async fn find_user_wrong_username() {
        let db = open_db().await;

        new_user(&db).await;

        let user = User::new("not_exists_user", "secret password of a not existing user");

        let err = db
            .find_user(&user.username, &user.password)
            .await
            .unwrap_err();

        assert_eq!(err.to_string(), "user not found")
    }

    /// Search for a user by UUID
    #[tokio::test]
    async fn find_user_by_id() {
        let db = open_db().await;

        new_user(&db).await;

        let user = User::new("medzik", "Qwerty1234!");

        let res = db.find_user_by_id(user.id).await.expect("find user");

        assert_eq!(res.password, user.password)
    }

    /// Search for a user by UUID who does not exist
    #[tokio::test]
    async fn find_user_wrong_id() {
        let db = open_db().await;

        new_user(&db).await;

        let other_user = User::new("other_user", "my secret passphrase");

        let err = db.find_user_by_id(other_user.id).await.unwrap_err();

        assert_eq!(err.to_string(), "user not found")
    }
}
