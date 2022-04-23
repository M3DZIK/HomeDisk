use axum::{Extension, Json};
use homedisk_database::{Database, Error, User};
use homedisk_types::{
    auth::login::{Request, Response},
    config::types::Config,
    errors::{AuthError, ServerError},
};
use rust_utilities::crypto::jsonwebtoken::{Claims, Token};

pub async fn handle(
    db: Extension<Database>,
    config: Extension<Config>,
    request: Json<Request>,
) -> Result<Json<Response>, ServerError> {
    let user = User::new(&request.username, &request.password);

    let response = match db.find_user(&user.username, &user.password).await {
        Ok(res) => {
            let token = Token::new(
                config.jwt.secret.as_bytes(),
                Claims::new(res.id, config.jwt.expires),
            )
            .unwrap();

            Response::LoggedIn {
                access_token: token.encoded,
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
