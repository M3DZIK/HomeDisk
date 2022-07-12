use crypto_utils::jsonwebtoken::{Claims, Token};
use homedisk_database::{Database, User};
use homedisk_types::errors::{AuthError, ServerError};

/// Create user token
pub fn create_token(user: &User, secret: &[u8], expires: i64) -> Result<String, ServerError> {
    let token = Token::new(secret, Claims::new(&user.id, expires))
        .map_err(|_| ServerError::AuthError(AuthError::TokenGenerate))?;

    Ok(token.encoded)
}

/// Search for a user
pub async fn find_user(db: &Database, user_id: &str) -> Result<User, ServerError> {
    match db.find_user_by_id(user_id).await {
        // if success return user
        Ok(user) => Ok(user),
        // errors
        Err(err) => match err {
            // user not found
            homedisk_database::Error::UserNotFound => {
                Err(ServerError::AuthError(AuthError::UserNotFound))
            },
            // other error
            _ => Err(ServerError::AuthError(AuthError::Other(err.to_string()))),
        },
    }
}

#[cfg(test)]
mod tests {
    use homedisk_database::User;

    use super::create_token;

    const SECRET: &[u8] = b"secret";

    const USERNAME: &str = "username";
    const PASSWORD: &str = "password";

    /// Test a token creation
    #[test]
    fn test_create_token() {
        let secret = SECRET;
        let user = User::new(USERNAME, PASSWORD);

        create_token(&user, secret, 1).unwrap();
    }
}
