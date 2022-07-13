use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{AuthError, FsError};

/// HTTP Server Error
#[derive(Debug, Clone, Serialize, Deserialize, Error)]
#[serde(tag = "error", content = "error_message", rename_all = "kebab-case")]
pub enum Error {
    #[error("auth error: {0}")]
    AuthError(#[from] AuthError),
    #[error("fs error: {0}")]
    FsError(#[from] FsError),
    #[error("too may requests, please slow down")]
    TooManyRequests,
    #[error("missing json in Content-Type header")]
    MissingJsonContentType,
    #[error("failed to deserialize json")]
    JsonDataError,
    #[error("syntax error in json")]
    JsonSyntaxError,
    #[error("failed to extract the request body")]
    BytesRejection,
    #[error("other error: {0}")]
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ResponseError {
    Error(String)
}

impl Error {
    fn into_response(self) -> ResponseError {
        ResponseError::Error(self.to_string())
    }
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        use axum::http::StatusCode;

        let status = match self {
            Self::AuthError(ref err) => match err {
                AuthError::TokenGenerate => StatusCode::INTERNAL_SERVER_ERROR,
                AuthError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
                _ => StatusCode::BAD_REQUEST,
            },
            Self::FsError(ref err) => match err {
                FsError::CreateFile(_) => StatusCode::INTERNAL_SERVER_ERROR,
                FsError::CreateDirectory(_) => StatusCode::INTERNAL_SERVER_ERROR,
                FsError::DeleteFile(_) => StatusCode::INTERNAL_SERVER_ERROR,
                FsError::DeleteDirectory(_) => StatusCode::INTERNAL_SERVER_ERROR,
                FsError::WriteFile(_) => StatusCode::INTERNAL_SERVER_ERROR,
                FsError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
                _ => StatusCode::BAD_REQUEST,
            },
            Self::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            Self::BytesRejection => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        };

        let mut response = axum::Json(self.into_response()).into_response();
        *response.status_mut() = status;

        response
    }
}
