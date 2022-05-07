use std::path::PathBuf;
use std::{fs, io};

use crate::fs::validate_path;
use axum::extract::Query;
use axum::{extract::rejection::JsonRejection, Extension, Json};
use axum_auth::AuthBearer;
use byte_unit::Byte;
use homedisk_database::Database;
use homedisk_types::fs::list::DirInfo;
use homedisk_types::fs::upload::Pagination;
use homedisk_types::{
    config::types::Config,
    errors::{FsError, ServerError},
    fs::list::{FileInfo, Request, Response},
};

use crate::middleware::{find_user, validate_json, validate_jwt};

pub async fn handle(
    Extension(db): Extension<Database>,
    Extension(config): Extension<Config>,
    AuthBearer(token): AuthBearer,
    query: Query<Pagination>,
) -> Result<Vec<u8>, ServerError> {
    // validate token
    let token = validate_jwt(config.jwt.secret.as_bytes(), &token)?;

    // validate the `path` can be used
    validate_path(&query.path)?;

    // search for a user by UUID from a token
    let user = find_user(db, token.claims.sub).await?;

    // directory where the file will be placed
    let path = format!(
        "{user_dir}/{req_dir}",
        user_dir = user.user_dir(&config.storage.path),
        req_dir = query.path
    );

    // read file content
    let content = fs::read(path).unwrap();

    Ok(content)
}
