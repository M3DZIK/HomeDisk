use homedisk_types::errors::{AuthError, ServerError};
use jsonwebtoken::TokenData;
use rust_utilities::crypto::jsonwebtoken::{Claims, Token};

pub fn validate_jwt(secret: &[u8], token: &str) -> Result<TokenData<Claims>, ServerError> {
    match Token::decode(secret, token.to_string()) {
        Ok(claims) => Ok(claims),
        Err(_) => Err(ServerError::AuthError(AuthError::InvalidToken)),
    }
}
