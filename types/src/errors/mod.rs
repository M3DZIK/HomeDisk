mod auth;
mod fs;

pub use auth::Error as AuthError;
pub use fs::Error as FsError;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
#[serde(tag = "error", content = "error-message", rename_all = "kebab-case")]
pub enum ServerError {
    #[error("auth error: {0}")]
    AuthError(#[from] AuthError),

    #[error("fs error: {0}")]
    FsError(#[from] FsError),

    #[error("too may requests, please slow down")]
    TooManyRequests,

    #[error("missing json content type")]
    MissingJsonContentType,

    #[error("error deserialize json")]
    JsonDataError,

    #[error("json syntax error")]
    JsonSyntaxError,

    #[error("failed to extract the request body")]
    BytesRejection,

    #[error("unexcepted error")]
    Other(String),
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        use axum::http::StatusCode;

        let status = match self {
            Self::AuthError(ref err) => match err {
                AuthError::UserNotFound => StatusCode::BAD_REQUEST,
                AuthError::UserAlreadyExists => StatusCode::NOT_ACCEPTABLE,
                AuthError::TokenGenerate => StatusCode::INTERNAL_SERVER_ERROR,
                AuthError::InvalidToken => StatusCode::BAD_REQUEST,
                AuthError::UnknowError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Self::FsError(ref err) => match err {
                FsError::FileAlreadyExists => StatusCode::BAD_REQUEST,
                FsError::WriteFile(_) => StatusCode::INTERNAL_SERVER_ERROR,
                FsError::Base64(_) => StatusCode::BAD_REQUEST,
                FsError::UnknowError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Self::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            Self::MissingJsonContentType => StatusCode::BAD_REQUEST,
            Self::JsonDataError => StatusCode::BAD_REQUEST,
            Self::JsonSyntaxError => StatusCode::BAD_REQUEST,
            Self::BytesRejection => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let mut response = axum::Json(self).into_response();
        *response.status_mut() = status;

        response
    }
}
