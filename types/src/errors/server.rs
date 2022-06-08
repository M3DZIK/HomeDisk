use serde::{Deserialize, Serialize};

use super::{AuthError, FsError};

/// HTTP Server Error
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
#[serde(tag = "error", content = "error_message", rename_all = "kebab-case")]
pub enum Error {
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

    #[error("unexpected error - {0}")]
    Other(String),
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        use axum::http::StatusCode;

        let status = match self {
            Self::AuthError(ref err) => match err {
                AuthError::UserNotFound => StatusCode::BAD_REQUEST,
                AuthError::UserAlreadyExists => StatusCode::NOT_ACCEPTABLE,
                AuthError::UsernameTooShort => StatusCode::NOT_ACCEPTABLE,
                AuthError::UsernameTooLong => StatusCode::NOT_ACCEPTABLE,
                AuthError::PasswordTooShort => StatusCode::NOT_ACCEPTABLE,
                AuthError::TokenGenerate => StatusCode::INTERNAL_SERVER_ERROR,
                AuthError::InvalidToken => StatusCode::BAD_REQUEST,
                AuthError::UnknownError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Self::FsError(ref err) => match err {
                FsError::FileAlreadyExists => StatusCode::BAD_REQUEST,
                FsError::FileDoesNotExist => StatusCode::BAD_REQUEST,
                FsError::MultipartError => StatusCode::BAD_REQUEST,
                FsError::CreateFile(_) => StatusCode::INTERNAL_SERVER_ERROR,
                FsError::CreateDirectory(_) => StatusCode::INTERNAL_SERVER_ERROR,
                FsError::DeleteFile(_) => StatusCode::INTERNAL_SERVER_ERROR,
                FsError::DeleteDirectory(_) => StatusCode::INTERNAL_SERVER_ERROR,
                FsError::WriteFile(_) => StatusCode::INTERNAL_SERVER_ERROR,
                FsError::Base64(_) => StatusCode::BAD_REQUEST,
                FsError::ReadDir(_) => StatusCode::BAD_REQUEST,
                FsError::UnknownError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
