use axum::{Extension, Json};
use homedisk_types::{
    auth::login::{Request, Response},
    errors::{AuthError, ServerError},
};
use homedisk_utils::database::{Database, User};

pub async fn handle(
    db: Extension<Database>,
    Json(request): Json<Request>,
) -> Result<Json<Response>, ServerError> {
    let response = match db
        .create_user(User::new(&request.username, &request.password))
        .await
    {
        Ok(_) => Response::LoggedIn {
            access_token: "access_token".to_string(),
            refresh_token: "refresh_token".to_string(),
        },

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
