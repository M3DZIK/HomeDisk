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

pub async fn create_dir(
    Extension(config): Extension<Config>,
    path: PathQuery,
    Token(user): Token,
) -> Result<Json<Response>> {
    let path = validate_path(path)?;

    let path = format!("{}/{}", user.user_dir(&config.storage.path), path);

    if Path::new(&path).exists() {
        return Err(Error::AlreadyExists);
    }

    fs::create_dir_all(path).map_err(|_| Error::CreateDirectory)?;

    Ok(Json(Response { success: true }))
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub success: bool,
}
