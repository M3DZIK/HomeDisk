use hex::encode;
use uuid::Uuid;

use crate::crypto::CryptographicHash;

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

impl User {
    /// Create a new User type (note **this doesn't create a new user in the database!**)
    ///
    /// This function creates a unique UUID for the user and creates a password hash using SHA-512
    /// ```
    /// use homedisk_utils::database::User;
    ///
    /// let user = User::new("medzik", "SuperSecretPassword123!");
    /// ```
    pub fn new(username: &str, password: &str) -> Self {
        // create user UUID
        let sha1_name = CryptographicHash::hash("SHA-1", username.as_bytes()).unwrap();
        let id = Uuid::new_v5(&Uuid::NAMESPACE_X500, &sha1_name).to_string();

        let password = encode(CryptographicHash::hash("SHA-512", password.as_bytes()).unwrap());

        Self {
            id,
            username: username.to_string(),
            password,
        }
    }
}
