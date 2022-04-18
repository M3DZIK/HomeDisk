use axum::Json;
use homedisk_types::{
    auth::auth::login::{Request, Response},
    errors::{AuthError, ServerError},
};

pub async fn handle(Json(_request): Json<Request>) -> Result<Json<Response>, ServerError> {
    Err(ServerError::AuthError(AuthError::UserAlreadyExists))
}
