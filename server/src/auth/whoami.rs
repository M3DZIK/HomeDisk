use axum::{Extension, Json};
use axum_auth::AuthBearer;
use homedisk_database::{Database, Error};
use homedisk_types::{
    auth::whoami::Response,
    config::types::Config,
    errors::{AuthError, ServerError},
};

use crate::middleware::validate_jwt;

pub async fn handle(
    db: Extension<Database>,
    config: Extension<Config>,
    AuthBearer(token): AuthBearer,
) -> Result<Json<Response>, ServerError> {
    // validate user token
    let token = validate_jwt(config.jwt.secret.as_bytes(), &token)?;

    let response = match db.find_user_by_id(token.claims.sub).await {
        Ok(res) => Response {
            username: res.username,
        },

        Err(err) => match err {
            Error::UserNotFound => return Err(ServerError::AuthError(AuthError::UserNotFound)),
            _ => {
                return Err(ServerError::AuthError(AuthError::UnknownError(
                    err.to_string(),
                )))
            }
        },
    };

    Ok(Json(response))
}
