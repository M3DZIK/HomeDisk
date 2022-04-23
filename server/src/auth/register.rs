use axum::{extract::rejection::JsonRejection, Extension, Json};
use homedisk_database::{Database, User};
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

    let response = match db.create_user(&user).await {
        Ok(_) => {
            let token = create_token(user, config.jwt.secret.as_bytes(), config.jwt.expires)?;

            Response::LoggedIn {
                access_token: token,
            }
        }

        Err(e) => {
            if e.to_string().contains("UNIQUE constraint failed") {
                return Err(ServerError::AuthError(AuthError::UserAlreadyExists));
            }

            return Err(ServerError::AuthError(AuthError::UnknowError(
                e.to_string(),
            )));
        }
    };

    Ok(Json(response))
}
