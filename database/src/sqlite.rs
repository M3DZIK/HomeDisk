use futures_util::TryStreamExt;
use log::debug;
use sqlx::{sqlite::SqliteQueryResult, Executor, Row, SqlitePool};

use super::{Error, User};

/// SQL Database
#[derive(Debug, Clone)]
pub struct Database {
    /// SQLite Connection Pool
    pub conn: SqlitePool,
}

impl Database {
    /// Open a SQLite database
    /// ```no_run
    /// use homedisk_database::Database;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     // open database in memory
    ///     Database::open("sqlite::memory:").await?;
    ///
    ///     // open database from file
    ///     Database::open("path/to/database.db").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn open(path: &str) -> Result<Self, Error> {
        debug!("Opening SQLite database");

        // create a database pool
        let conn = SqlitePool::connect(path).await?;

        // return `Database`
        Ok(Self { conn })
    }

    /// Create all required tabled for HomeDisk
    /// ```
    /// use homedisk_database::Database;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     // open database in memory
    ///     let db = Database::open("sqlite::memory:").await?;
    ///
    ///     // create tables
    ///     db.create_tables().await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_tables(&self) -> Result<SqliteQueryResult, Error> {
        let query = sqlx::query(include_str!("../../tables.sql"));

        Ok(self.conn.execute(query).await?)
    }

    /// Create a new User
    /// ```no_run
    /// use homedisk_database::{Database, User};
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     // open database
    ///     let db = Database::open("path/to/database.db").await?;
    ///
    ///     // create `User` type
    ///     let user = User::new("username", "password");
    ///
    ///     // create a user in database
    ///     db.create_user(&user).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_user(&self, user: &User) -> Result<SqliteQueryResult, Error> {
        debug!("Creating user - {}", user.username);

        // insert user to a database
        let query = sqlx::query("INSERT INTO user (id, username, password) VALUES (?, ?, ?)")
            .bind(&user.id)
            .bind(&user.username)
            .bind(&user.password);

        // execute query and return output
        Ok(self.conn.execute(query).await?)
    }

    /// Search for a user
    /// ```
    /// use homedisk_database::{Database, User};
    ///
    /// #[tokio::main]
    /// async fn find_user() -> anyhow::Result<()> {
    ///     // open database
    ///     let db = Database::open("path/to/database.db").await?;
    ///
    ///     // create `User` type
    ///     let user = User::new("username", "password");
    ///
    ///     // create a user in database
    ///     db.create_user(&user).await?;
    ///
    ///     // search for a user
    ///     db.find_user(&user).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn find_user(&self, user: &User) -> Result<User, Error> {
        debug!("Searching for a user - {}", user.username);

        // create query request to database
        let query =
            sqlx::query_as::<_, User>("SELECT * FROM user WHERE username = ? AND password = ?")
                .bind(&user.username)
                .bind(&user.password);

        // fetch query
        let mut stream = self.conn.fetch(query);

        // get rows from query
        let row = stream.try_next().await?.ok_or(Error::UserNotFound)?;

        // get `id` row
        let id = row.try_get("id")?;
        // get `username` row
        let username = row.try_get("username")?;
        // get `password` row
        let password = row.try_get("password")?;

        // return `User`
        Ok(User {
            id,
            username,
            password,
        })
    }

    /// Search for a user by UUID
    /// ```
    /// use homedisk_database::{Database, User};
    ///
    /// #[tokio::main]
    /// async fn find_user_by_id() -> anyhow::Result<()> {
    ///     // open database
    ///     let db = Database::open("path/to/database.db").await?;
    ///
    ///     // create `User` type
    ///     let user = User::new("username", "password");
    ///
    ///     // create a user in database
    ///     db.create_user(&user).await?;
    ///
    ///     // search for a user using UUID
    ///     db.find_user_by_id(&user.id).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn find_user_by_id(&self, id: &str) -> Result<User, Error> {
        debug!("Searching for a user by UUID - {}", id);

        // create query request to database
        let query = sqlx::query_as::<_, User>("SELECT * FROM user WHERE id = ?").bind(id);

        // fetch query
        let mut stream = self.conn.fetch(query);

        // get rows from query
        let row = stream.try_next().await?.ok_or(Error::UserNotFound)?;

        // get `id` row
        let id = row.try_get("id")?;
        // get `username` row
        let username = row.try_get("username")?;
        // get `password` row
        let password = row.try_get("password")?;

        // return `User`
        Ok(User {
            id,
            username,
            password,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Database, User};

    const USERNAME: &str = "medzik";
    const PASSWORD: &str = "SuperSecretPassword123!";

    /// Utils to open database in tests
    async fn open_db() -> Database {
        Database::open("sqlite::memory:").await.expect("open db")
    }

    /// Utils to create a new user in tests
    async fn new_user(db: &Database) {
        // create tables
        db.create_tables().await.expect("create tables");

        // create new user
        let user = User::new(USERNAME, PASSWORD);
        db.create_user(&user).await.expect("create user");
    }

    /// Test a create user
    #[tokio::test]
    async fn create_user() {
        let db = open_db().await;
        new_user(&db).await;
    }

    // Test a search for a user
    #[tokio::test]
    async fn find_user() {
        let db = open_db().await;

        new_user(&db).await;

        let user = User::new(USERNAME, PASSWORD);

        let user = db.find_user(&user).await.unwrap();

        assert_eq!(user.username, USERNAME)
    }

    // Test a search for a user by id
    #[tokio::test]
    async fn find_user_by_id() {
        let db = open_db().await;
        new_user(&db).await;

        let user = User::new(USERNAME, PASSWORD);

        let user = db.find_user_by_id(&user.id).await.unwrap();

        assert_eq!(user.username, USERNAME)
    }

    /// Test a search for a user with an invalid password to see if the user is returned (it shouldn't be)
    #[tokio::test]
    async fn find_user_wrong_password() {
        let db = open_db().await;

        new_user(&db).await;

        let user = User::new(USERNAME, "wrong password 123!");

        let err = db.find_user(&user).await.unwrap_err();

        assert_eq!(err.to_string(), "user not found")
    }

    /// Test a search for a user who does not exist
    #[tokio::test]
    async fn find_user_wrong_username() {
        let db = open_db().await;

        new_user(&db).await;

        let user = User::new("not_exists_user", PASSWORD);

        let err = db.find_user(&user).await.unwrap_err();

        assert_eq!(err.to_string(), "user not found")
    }

    /// Test a search for a user by UUID who does not exist
    #[tokio::test]
    async fn find_user_wrong_id() {
        let db = open_db().await;

        new_user(&db).await;

        let other_user = User::new("not_exists_user", "my secret passphrase");

        let err = db.find_user_by_id(&other_user.id).await.unwrap_err();

        assert_eq!(err.to_string(), "user not found")
    }
}
