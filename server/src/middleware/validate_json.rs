use axum::{extract::rejection::JsonRejection, Json};
use homedisk_types::errors::ServerError;

pub fn validate_json<Typ>(
    payload: Result<Json<Typ>, JsonRejection>,
) -> Result<Json<Typ>, ServerError> {
    match payload {
        Ok(payload) => Ok(payload),
        Err(JsonRejection::MissingJsonContentType(_)) => Err(ServerError::MissingJsonContentType),
        Err(JsonRejection::JsonDataError(_)) => Err(ServerError::JsonDataError),
        Err(JsonRejection::JsonSyntaxError(_)) => Err(ServerError::JsonSyntaxError),
        Err(JsonRejection::BytesRejection(_)) => Err(ServerError::BytesRejection),
        Err(err) => Err(ServerError::Other(err.to_string())),
    }
}
