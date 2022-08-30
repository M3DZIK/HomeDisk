use axum::{extract::rejection::JsonRejection, Json};
use homedisk_types::errors::ServerError;

/// Validate json request
pub fn validate_json<T>(payload: Result<Json<T>, JsonRejection>) -> Result<Json<T>, ServerError> {
    match payload {
        // if success return payload
        Ok(payload) => Ok(payload),
        // mission json in Content-Type Header
        Err(JsonRejection::MissingJsonContentType(_)) => Err(ServerError::InvalidContentType),
        // failed to deserialize json
        Err(JsonRejection::JsonDataError(_)) => Err(ServerError::JsonDataError),
        // syntax error in json
        Err(JsonRejection::JsonSyntaxError(_)) => Err(ServerError::JsonSyntaxError),
        // failed to extract the request body
        Err(JsonRejection::BytesRejection(_)) => Err(ServerError::BytesRejection),
        // other error
        Err(err) => Err(ServerError::Other(err.to_string())),
    }
}
