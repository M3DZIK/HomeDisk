use std::{fs, path::Path};

use axum::{Extension, Json};
use serde::Serialize;

use crate::{
    config::Config,
    server::{
        error::*,
        utils::{
            path::{validate_path, PathQuery},
            token::Token,
        },
    },
};

pub async fn delete(
    Extension(config): Extension<Config>,
    path: PathQuery,
    Token(user): Token,
) -> Result<Json<Response>> {
    let path = validate_path(path)?;

    let path = format!("{}/{}", user.user_dir(&config.storage.path), path);
    let path = Path::new(&path);

    if !path.exists() {
        return Err(Error::NotFound);
    }

    if path.is_dir() {
        fs::remove_dir(path).map_err(|_| Error::DeleteDirectory)?;
    } else {
        fs::remove_file(path).map_err(|_| Error::DeleteFile)?;
    }

    Ok(Json(Response { success: true }))
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub success: bool,
}
