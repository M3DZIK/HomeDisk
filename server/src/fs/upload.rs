use std::{fs, path::Path};

use crate::fs::validate_path;
use axum::{extract::rejection::JsonRejection, Extension, Json};
use axum_auth::AuthBearer;
use homedisk_database::Database;
use homedisk_types::{
    config::types::Config,
    errors::{FsError, ServerError},
    fs::upload::{Request, Response},
};

use crate::middleware::{find_user, validate_json, validate_jwt};

pub async fn handle(
    Extension(db): Extension<Database>,
    Extension(config): Extension<Config>,
    AuthBearer(token): AuthBearer,
    request: Result<Json<Request>, JsonRejection>,
) -> Result<Json<Response>, ServerError> {
    let Json(request) = validate_json::<Request>(request)?;
    let token = validate_jwt(config.jwt.secret.as_bytes(), &token)?;

    // validate the `path` can be used
    validate_path(&request.path)?;

    // search for a user by UUID from a token
    let user = find_user(db, token.claims.sub).await?;

    // get file content
    let content = base64::decode(request.content)
        .map_err(|err| ServerError::FsError(FsError::Base64(err.to_string())))?;

    // directory where the file will be placed
    let dir = format!(
        "{user_dir}/{req_dir}",
        user_dir = user.user_dir(&config.storage.path),
        req_dir = request.path
    );
    let path = Path::new(&dir);

    // check if the file currently exists to avoid overwriting it
    if path.exists() {
        return Err(ServerError::FsError(FsError::FileAlreadyExists));
    }

    // create a directory where the file will be placed
    // e.g. path ==> `/secret/files/images/screenshot.png`
    // directories up to `/home/homedisk/{username}/secret/files/images/` will be created
    match path.parent() {
        Some(prefix) => fs::create_dir_all(&prefix).unwrap(),
        None => (),
    }

    // write file
    fs::write(&path, &content)
        .map_err(|err| ServerError::FsError(FsError::WriteFile(err.to_string())))?;

    Ok(Json(Response { uploaded: true }))
}
