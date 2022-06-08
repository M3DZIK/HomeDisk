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
        Ok(user) => Ok(user),
        Err(err) => match err {
            homedisk_database::Error::UserNotFound => {
                Err(ServerError::AuthError(AuthError::UserNotFound))
            }
            _ => Err(ServerError::AuthError(AuthError::UnknownError(
                err.to_string(),
            ))),
        },
    }
}

#[cfg(test)]
mod tests {
    use homedisk_database::User;

    use super::create_token;

    #[test]
    fn test_create_token() {
        let secret = b"secret";
        let user = User::new("username", "password");

        create_token(&user, secret, 1).unwrap();
    }
}
