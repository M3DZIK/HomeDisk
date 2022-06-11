use axum::{Extension, Json};
use axum_auth::AuthBearer;
use homedisk_database::{Database, Error};
use homedisk_types::{
    auth::whoami::Response,
    config::Config,
    errors::{AuthError, ServerError},
};

use crate::middleware::validate_jwt;

/// Handle `/auth/whoami` requests
pub async fn handle(
    db: Extension<Database>,
    config: Extension<Config>,
    AuthBearer(token): AuthBearer,
) -> Result<Json<Response>, ServerError> {
    // validate user token
    let token = validate_jwt(config.jwt.secret.as_bytes(), &token)?;

    // search for a user in database
    let response = match db.find_user_by_id(token.claims.sub).await {
        Ok(res) => Response {
            username: res.username,
        },

        // error while searching for a user
        Err(err) => match err {
            // user not found
            Error::UserNotFound => return Err(ServerError::AuthError(AuthError::UserNotFound)),
            // other error
            _ => return Err(ServerError::AuthError(AuthError::Other(err.to_string()))),
        },
    };

    Ok(Json(response))
}
