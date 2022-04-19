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

    let response = match db.create_user(&user).await {
        Ok(_) => {
            let token = Token::new(config.jwt.secret.as_bytes(), Claims::new(user.id)).unwrap();

            Response::LoggedIn {
                access_token: token.encoded,
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
