use crypto_utils::sha::{Algorithm, CryptographicHash};
use uuid::Uuid;

/// SQL user table
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    /// UUID of the user
    pub id: String,
    /// Username
    pub username: String,
    /// Encryped user password
    pub password: String,
}

impl User {
    /// The function create a unique user UUID and create SHA-512 hash from salted user password
    /// and returns the [User] type.
    ///
    /// **Note: This doesn't create a user in the database!**
    ///
    /// ```
    /// use homedisk_types::database::User;
    ///
    /// let user = User::new("medzik", "SuperSecretPassword123!");
    /// ```
    pub fn new(username: &str, password: &str) -> Self {
        // change username to lowercase
        let username = username.to_lowercase();

        // salting the password
        let password = format!("{username}${password}");

        // hash password using SHA-512 and encode it to String from Vec<u8>
        let password = hex::encode(CryptographicHash::hash(
            Algorithm::SHA512,
            password.as_bytes(),
        ));

        // generate a user UUID
        let id_sha1 = CryptographicHash::hash(
            Algorithm::SHA1,
            (format!("{username}${password}")).as_bytes(),
        );
        let id = Uuid::new_v5(&Uuid::NAMESPACE_X500, &id_sha1).to_string();

        // return `User`
        Self {
            id,
            username,
            password,
        }
    }

    /// The function returns the directory where the user file is located.
    ///
    /// ```
    /// use homedisk_types::database::User;
    ///
    /// let user = User::new("medzik", "whatever");
    ///
    /// let dir = user.user_dir("/storage"); // will return `/storage/medzik`
    ///
    /// assert_eq!(dir, "/storage/medzik")
    /// ```
    pub fn user_dir(&self, storage: &str) -> String {
        // get a user storage path
        let path = format!(
            "{path}/{username}",
            path = storage,
            username = self.username,
        );

        // return user storage path
        path
    }
}

#[cfg(test)]
mod tests {
    use crypto_utils::sha::{Algorithm, CryptographicHash};

    use super::User;

    /// Check if the username is in lowercase
    #[test]
    fn check_username_is_in_lowercase() {
        // example user data
        let username = "mEDZIk";
        let password = "password";

        // username in lowercase (expected username)
        let username_expected = "medzik";

        // create a new `User` type
        let user = User::new(username, password);

        // username validation with expected username
        assert_eq!(user.username, username_expected)
    }

    /// Check that the password is a checksum with a salt
    #[test]
    fn check_if_password_is_hashed_and_salted() {
        // example user data
        let username = "username";
        let password = "password";

        // create a new `User` type
        let user = User::new(username, password);

        // expected password salt (string)
        let password_expected_salt = format!("{username}${password}");

        // expected password (hashed)
        let password_expected = hex::encode(CryptographicHash::hash(
            Algorithm::SHA512,
            password_expected_salt.as_bytes(),
        ));

        // password validation with expected password salt
        assert_eq!(user.password, password_expected)
    }
}
