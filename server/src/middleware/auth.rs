use homedisk_types::errors::{AuthError, ServerError};
use jsonwebtoken::TokenData;
use rust_utilities::crypto::jsonwebtoken::{Claims, Token};

/// Validate user token
pub fn validate_jwt(secret: &[u8], token: &str) -> Result<TokenData<Claims>, ServerError> {
    match Token::decode(secret, token.to_string()) {
        Ok(claims) => Ok(claims),
        Err(_) => Err(ServerError::AuthError(AuthError::InvalidToken)),
    }
}

#[cfg(test)]
mod tests {
    use homedisk_database::User;

    use crate::middleware::create_token;

    use super::validate_jwt;

    #[test]
    fn validate_token() {
        let secret = b"secret";
        let user = User::new("username", "password");

        let token = create_token(&user, secret, 1).unwrap();

        validate_jwt(secret, &token).unwrap();
    }
}
