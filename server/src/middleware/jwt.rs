use homedisk_database::User;
use homedisk_types::errors::{AuthError, ServerError};
use rust_utilities::crypto::jsonwebtoken::{Claims, Token};

pub fn create_token(user: &User, secret: &[u8], expires: i64) -> Result<String, ServerError> {
    let token = Token::new(secret, Claims::new(user.id.clone(), expires));

    match token {
        Ok(token) => Ok(token.encoded),
        Err(_) => Err(ServerError::AuthError(AuthError::TokenGenerate)),
    }
}
