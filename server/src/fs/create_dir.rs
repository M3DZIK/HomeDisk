use std::fs;

use axum::{extract::rejection::JsonRejection, Extension, Json};
use axum_auth::AuthBearer;
use homedisk_database::Database;
use homedisk_types::fs::create_dir::Request;
use homedisk_types::{
    config::Config,
    errors::{FsError, ServerError},
};

use crate::fs::validate_path;
use crate::middleware::{find_user, validate_json, validate_jwt};

/// Handle `/fs/createdir` requests
pub async fn handle(
    Extension(db): Extension<Database>,
    Extension(config): Extension<Config>,
    AuthBearer(token): AuthBearer,
    request: Result<Json<Request>, JsonRejection>,
) -> Result<(), ServerError> {
    // validate json request
    let Json(request) = validate_json::<Request>(request)?;

    // validate user token
    let token = validate_jwt(config.jwt.secret.as_bytes(), &token)?;

    // validate the `path` can be used
    validate_path(&request.path)?;

    // search for a user by UUID from a token
    let user = find_user(&db, &token.claims.sub).await?;

    // directory where the file will be placed
    let path = format!(
        "{user_dir}/{req_dir}",
        user_dir = user.user_dir(&config.storage.path),
        req_dir = request.path
    );

    // create directories
    fs::create_dir_all(path)
        .map_err(|err| ServerError::FsError(FsError::CreateDirectory(err.to_string())))?;

    // send a blank Response
    Ok(())
}
