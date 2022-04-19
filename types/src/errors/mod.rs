mod auth;

pub use auth::Error as AuthError;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
#[serde(tag = "error", content = "error-message", rename_all = "kebab-case")]
pub enum ServerError {
    #[error("auth error: {0}")]
    AuthError(#[from] AuthError),

    #[error("too may requests, please slow down")]
    TooManyRequests,
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        use axum::http::StatusCode;

        let status = match self {
            Self::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            Self::AuthError(ref err) => match err {
                AuthError::UserNotFound => StatusCode::BAD_REQUEST,
                AuthError::UserAlreadyExists => StatusCode::NOT_ACCEPTABLE,
                AuthError::UnknowError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
        };

        let mut response = axum::Json(self).into_response();
        *response.status_mut() = status;

        response
    }
}
