use sqlx::{Connection, SqliteConnection};

pub struct Database {
    pub conn: SqliteConnection,
}

impl Database {
    pub async fn connect() -> Result<Self, sqlx::Error> {
        let conn = SqliteConnection::connect("homedisk.db").await?;

        Ok(Self { conn })
    }
}
