use crypto_utils::sha::{Algorithm, CryptographicHash};
use serde::Serialize;
use uuid::Uuid;

/// SQL user entry
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

impl User {
    /// The function create a unique UUID and compute SHA-512 hash from salted user password
    /// and returns the [User] type.
    ///
    /// **Note**: This function **does not** create a user in database!
    pub fn new(username: &str, password: &str, gen_uuid: bool) -> Self {
        // change username to lowercase
        let username = username.to_lowercase();

        // salting the password
        let password = format!("{username}${password}");

        // hash the password using SHA-512 algorithm and encode it into String.
        let password = hex::encode(CryptographicHash::hash(
            Algorithm::SHA512,
            password.as_bytes(),
        ));

        // generate UUID
        let id = if gen_uuid {
            Uuid::new_v4().to_string()
        } else {
            "none".to_string()
        };

        Self {
            id,
            username,
            password,
        }
    }

    /// The function returns the directory where the user files is located.
    pub fn user_dir(&self, storage: &str) -> String {
        format!("{storage}/{username}", username = self.username)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Check if the username is in lowercase.
    #[test]
    fn test_username_is_lowercase() {
        // example user data
        let username = "MeDZiK";
        let password = "password";

        // expected username
        let username_expected = "medzik";

        let user = User::new(username, password, false);

        // username validation
        assert_eq!(user.username, username_expected)
    }

    /// Check if the password is hashed.
    #[test]
    fn test_password_hashed() {
        // example user data
        let username = "username";
        let password = "password";

        let user = User::new(username, password, false);

        assert_ne!(user.password, password)
    }

    #[test]
    fn test_user_dir() {
        let storage_dir: &str = "/storage";

        let user = User::new("medzik", "whatever", false);

        let user_dir = user.user_dir(storage_dir);

        assert_eq!(user_dir, "/storage/medzik");
    }
}
