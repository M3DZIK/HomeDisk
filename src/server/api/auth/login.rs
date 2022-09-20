use axum::{Extension, Json};
use crypto_utils::jsonwebtoken::{Claims, Token};
use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    database::{error::Error as DatabaseError, Database, User},
    server::{
        error::*,
        utils::ratelimit::{check_limit_login, ClientIp},
    },
};

pub async fn login(
    Extension(db): Extension<Database>,
    Extension(config): Extension<Config>,
    ClientIp(ip): ClientIp,
    request: Json<Request>,
) -> Result<Json<Response>> {
    // check rate limit
    check_limit_login(&ip)?;

    let user = User::new(&request.username, &request.password, false);

    let response = match db.find_user(&user).await {
        Ok(user) => {
            let token = Token::new(
                config.jwt.secret.as_bytes(),
                Claims::new(&user.id, config.jwt.expires),
            )
            .map_err(|_| Error::GenerateToken)?;

            Response {
                access_token: token.encoded,
            }
        },
        Err(err) => {
            return match err {
                DatabaseError::UserNotFound => Err(Error::UserNotFound),
                _ => Err(Error::Database),
            };
        },
    };

    Ok(Json(response))
}

#[derive(Debug, Deserialize)]
pub struct Request {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub access_token: String,
}
