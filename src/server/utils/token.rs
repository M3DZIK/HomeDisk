use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use crypto_utils::jsonwebtoken;

use crate::{
    config::Config,
    database::{error::Error as DatabaseError, Database, User},
    server::error::Error,
};

pub struct Token(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for Token
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let extensions = &req.extensions;

        let config = extensions.get::<Config>().unwrap();
        let db = extensions.get::<Database>().unwrap();

        // Get authorisation header
        let authorisation = req
            .headers
            .get(AUTHORIZATION)
            .ok_or(Error::MissingHeader("Authorization"))?
            .to_str()
            .map_err(|_| Error::InvalidToken)?;

        // Check that its a well-formed bearer
        let split = authorisation.split_once(' ');

        let token = match split {
            Some((name, contents)) if name == "Bearer" => contents.to_string(),
            _ => return Err(Error::MissingBearer),
        };

        let token = match jsonwebtoken::Token::decode(config.jwt.secret.as_bytes(), token) {
            Ok(token) => token,
            Err(_) => return Err(Error::InvalidToken),
        };

        let user = match db.find_user_by_id(&token.claims.sub).await {
            Ok(user) => user,
            Err(err) => match err {
                DatabaseError::UserNotFound => return Err(Error::UserNotFound),
                _ => return Err(Error::Database),
            },
        };

        Ok(Self(user))
    }
}
