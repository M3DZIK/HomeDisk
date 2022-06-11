use homedisk_database::{Database, User};
use homedisk_types::errors::{AuthError, ServerError};
use rust_utilities::crypto::jsonwebtoken::{Claims, Token};

/// Create user token
pub fn create_token(user: &User, secret: &[u8], expires: i64) -> Result<String, ServerError> {
    let token = Token::new(secret, Claims::new(user.id.clone(), expires));

    match token {
        Ok(token) => Ok(token.encoded),
        Err(_) => Err(ServerError::AuthError(AuthError::TokenGenerate)),
    }
}

/// Search for a user
pub async fn find_user(db: Database, user_id: String) -> Result<User, ServerError> {
    match db.find_user_by_id(user_id).await {
        // if success return user
        Ok(user) => Ok(user),
        // errors
        Err(err) => match err {
            // user not found
            homedisk_database::Error::UserNotFound => {
                Err(ServerError::AuthError(AuthError::UserNotFound))
            }
            // other error
            _ => Err(ServerError::AuthError(AuthError::Other(err.to_string()))),
        },
    }
}

#[cfg(test)]
mod tests {
    use homedisk_database::User;

    use super::create_token;

    /// Test a token creation
    #[test]
    fn test_create_token() {
        let secret = b"secret";
        let user = User::new("username", "password");

        create_token(&user, secret, 1).unwrap();
    }
}
