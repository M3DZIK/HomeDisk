use axum::{Extension, Json};
use homedisk_types::{
    auth::login::{Request, Response},
    config::types::Config,
    errors::{AuthError, ServerError},
    token::{Claims, Token},
};
use homedisk_utils::database::{Database, User};

pub async fn handle(
    db: Extension<Database>,
    config: Extension<Config>,
    Json(request): Json<Request>,
) -> Result<Json<Response>, ServerError> {
    let user = User::new(&request.username, &request.password);

    let response = match db.find_user(&user.username, &user.password).await {
        Ok(res) => {
            let token = Token::new(config.jwt.secret.as_bytes(), Claims::new(res.id)).unwrap();

            Response::LoggedIn {
                access_token: token.encoded,
            }
        }

        Err(e) => {
            use homedisk_utils::database::Error;
            match e {
                Error::UserNotFound => return Err(ServerError::AuthError(AuthError::UserNotFound)),
                _ => {
                    return Err(ServerError::AuthError(AuthError::UnknowError(
                        e.to_string(),
                    )))
                }
            }
        }
    };

    Ok(Json(response))
}
