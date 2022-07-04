use crypto_utils::jsonwebtoken::{Token, TokenData};
use homedisk_types::errors::{AuthError, ServerError};

/// Validate user token
pub fn validate_jwt(secret: &[u8], token: &str) -> Result<TokenData, ServerError> {
    match Token::decode(secret, token.to_string()) {
        // if success return claims
        Ok(claims) => Ok(claims),
        // invalid token
        Err(_) => Err(ServerError::AuthError(AuthError::InvalidToken)),
    }
}

#[cfg(test)]
mod tests {
    use homedisk_database::User;

    use crate::middleware::create_token;

    use super::validate_jwt;

    const USERNAME: &str = "username";
    const PASSWORD: &str = "password";

    const SECRET: &[u8] = b"secret";
    const INVALID_SECRET: &[u8] = b"invalid secret";

    /// Test a token validation
    #[test]
    fn validate_token() {
        let user = User::new(USERNAME, PASSWORD);

        let token = create_token(&user, SECRET, 1).unwrap();

        validate_jwt(SECRET, &token).unwrap();
    }

    /// Test a token validation (invalid secret)
    #[test]
    fn validate_token_invalid_secret() {
        let user = User::new(USERNAME, PASSWORD);

        let token = create_token(&user, SECRET, 1).unwrap();

        validate_jwt(INVALID_SECRET, &token).unwrap_err();
    }
}
