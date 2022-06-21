use axum::{extract::rejection::JsonRejection, Extension, Json};
use homedisk_database::{Database, Error, User};
use homedisk_types::{
    auth::login::{Request, Response},
    config::Config,
    errors::{AuthError, ServerError},
};

use crate::middleware::{create_token, validate_json};

pub async fn handle(
    Extension(db): Extension<Database>,
    Extension(config): Extension<Config>,
    request: Result<Json<Request>, JsonRejection>,
) -> Result<Json<Response>, ServerError> {
    // validate json request
    let request = validate_json::<Request>(request)?;

    // create `User` type
    let user = User::new(&request.username, &request.password);

    // search for a user in database
    let response = match db.find_user(&user).await {
        Ok(user) => {
            // create user token
            let token = create_token(&user, config.jwt.secret.as_bytes(), config.jwt.expires)?;

            // Reponse user token
            Response::LoggedIn {
                access_token: token,
            }
        }

        // error while searching for a user
        Err(err) => {
            return match err {
                // user not found
                Error::UserNotFound => Err(ServerError::AuthError(AuthError::UserNotFound)),
                // other error
                _ => Err(ServerError::AuthError(AuthError::Other(err.to_string()))),
            };
        }
    };

    Ok(Json(response))
}
