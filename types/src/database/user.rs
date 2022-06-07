use rust_utilities::crypto::sha::{encode, Algorithm, CryptographicHash};
use uuid::Uuid;

/// SQL `user` Table
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

impl User {
    /// **Note this doesn't create a new user in the database!**
    ///
    /// This function creates a unique UUID for a user and creates a password hash using SHA-512
    /// and returns in the User type
    /// ```
    /// use homedisk_types::database::User;
    ///
    /// let user = User::new("medzik", "SuperSecretPassword123!");
    /// ```
    pub fn new(username: &str, password: &str) -> Self {
        // change username to lowercase
        let username = username.to_lowercase();

        // create user UUID
        let sha1_name = CryptographicHash::hash(Algorithm::SHA1, username.as_bytes());
        let id = Uuid::new_v5(&Uuid::NAMESPACE_X500, &sha1_name).to_string();

        // hash password using SHA-512
        let password = encode(CryptographicHash::hash(
            Algorithm::SHA512,
            password.as_bytes(),
        ));

        Self {
            id,
            username,
            password,
        }
    }

    /// User directory
    /// function returns the directory where the user file is located
    /// e.g.
    /// ```
    /// use homedisk_types::database::User;
    ///
    /// let user = User::new("medzik", "whatever");
    ///
    /// let dir = user.user_dir("/home/homedisk"); // will return  `/home/homedisk/medzik`
    ///
    /// assert_eq!(dir, "/home/homedisk/medzik")
    /// ```
    pub fn user_dir(&self, storage: &str) -> String {
        let path = format!(
            "{path}/{username}",
            path = storage,
            username = self.username,
        );

        path
    }
}

#[cfg(test)]
mod tests {
    use super::User;

    /// Check if the username has been changed to lowercase
    #[test]
    fn check_username_is_in_lowercase() {
        let user = User::new("MEdzIk", "SuperSecretPassword123!");

        assert_eq!(user.username, "medzik")
    }

    /// Check that the password is a checksum
    #[test]
    fn check_if_password_is_hashed() {
        let password = "password";
        let user = User::new("test", password);

        assert!(user.password != password)
    }
}
