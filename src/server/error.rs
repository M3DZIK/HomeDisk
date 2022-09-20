use serde::Serialize;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum Error {
    // `/auth` error
    #[error("Too many request, please slow down.")]
    RateLimit,
    #[error("User not found")]
    UserNotFound,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Failed to generate access token")]
    GenerateToken,
    #[error("An error occurred in database request")]
    Database,
    #[error("Username is too short")]
    UsernameTooShort,
    #[error("Username is too short")]
    UsernameTooLong,
    #[error("Password is too short")]
    PasswordTooShort,
    #[error("Invalid token")]
    InvalidToken,
    #[error("`{0}` header is missing")]
    MissingHeader(&'static str),
    #[error("`Authorization` header must be a bearer token")]
    MissingBearer,

    // `/fs` error
    #[error("Invalid `path` query parameter")]
    InvalidPath,
    #[error("Failed to read files from directory")]
    FailedReadDirectory,
    #[error("Failed to create directory")]
    CreateDirectory,
    #[error("Failed to create file")]
    CreateFile,
    #[error("File or directory already exists")]
    AlreadyExists,
    #[error("File or directory does not exists")]
    NotFound,
    #[error("Failed to delete directory (directory is not empty?)")]
    DeleteDirectory,
    #[error("Failed to delete file")]
    DeleteFile,
    #[error("Multipart error")]
    Multipart,
    #[error("Failed write to file")]
    WriteFile,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum ResponseError {
    Error(String),
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        use axum::http::StatusCode;

        #[cfg(debug_assertions)]
        error!("Error: {:?}", self);

        let status = match self {
            Error::RateLimit => StatusCode::TOO_MANY_REQUESTS,
            Error::GenerateToken => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Database => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        };

        let mut response = axum::Json(ResponseError::Error(self.to_string())).into_response();
        *response.status_mut() = status;

        response
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
