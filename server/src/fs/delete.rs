use std::{fs, path::Path};

use axum::extract::Query;
use axum::Extension;
use axum_auth::AuthBearer;
use homedisk_database::Database;
use homedisk_types::{
    config::Config,
    errors::{FsError, ServerError},
    fs::delete::Request,
};

use crate::fs::validate_path;
use crate::middleware::{find_user, validate_jwt};

/// Handle `/fs/delete` requests
pub async fn handle(
    Extension(db): Extension<Database>,
    Extension(config): Extension<Config>,
    AuthBearer(token): AuthBearer,
    query: Query<Request>,
) -> Result<(), ServerError> {
    // validate user token
    let token = validate_jwt(config.jwt.secret.as_bytes(), &token)?;

    // validate the `path` can be used
    validate_path(&query.path)?;

    // search for a user by UUID from a token
    let user = find_user(db, token.claims.sub).await?;

    // path to the file
    let path = format!(
        "{user_dir}/{request_path}",
        user_dir = user.user_dir(&config.storage.path),
        request_path = query.path
    );
    let path = Path::new(&path);

    // if file does not exist return error
    if !path.exists() {
        return Err(ServerError::FsError(FsError::FileDoesNotExist));
    }

    // delete file
    if path.is_file() {
        fs::remove_file(&path)
            // return error
            .map_err(|err| ServerError::FsError(FsError::DeleteFile(err.to_string())))?;
    }
    // delete directory
    else if path.is_dir() {
        fs::remove_dir(&path)
            // return error
            .map_err(|err| ServerError::FsError(FsError::DeleteDirectory(err.to_string())))?;
    }

    // send a blank Response
    Ok(())
}
