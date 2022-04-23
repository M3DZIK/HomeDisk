use axum::{extract::rejection::JsonRejection, Extension, Json};
use homedisk_database::{Database, Error, User};
use homedisk_types::{
    auth::login::{Request, Response},
    config::types::Config,
    errors::{AuthError, ServerError},
};

use crate::middleware::{create_token, validate_json};

pub async fn handle(
    db: Extension<Database>,
    config: Extension<Config>,
    request: Result<Json<Request>, JsonRejection>,
) -> Result<Json<Response>, ServerError> {
    let request = validate_json::<Request>(request)?;

    let user = User::new(&request.username, &request.password);

    let response = match db.find_user(&user.username, &user.password).await {
        Ok(res) => {
            let token = create_token(res, config.jwt.secret.as_bytes(), config.jwt.expires)?;

            Response::LoggedIn {
                access_token: token,
            }
        }

        Err(err) => match err {
            Error::UserNotFound => return Err(ServerError::AuthError(AuthError::UserNotFound)),
            _ => {
                return Err(ServerError::AuthError(AuthError::UnknowError(
                    err.to_string(),
                )))
            }
        },
    };

    Ok(Json(response))
}
