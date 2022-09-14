use std::{fs, io::Write, path::Path};

use axum::{extract::Multipart, Extension, Json};
use futures_util::TryStreamExt;
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

pub async fn upload(
    Extension(config): Extension<Config>,
    path: PathQuery,
    Token(user): Token,
    mut multipart: Multipart,
) -> Result<Json<Response>> {
    let path = validate_path(path)?;

    let path = format!("{}/{}", user.user_dir(&config.storage.path), path);
    let path = Path::new(&path);

    if path.exists() {
        return Err(Error::AlreadyExists);
    }

    // create a directory where the file will be placed
    // e.g. path ==> `/secret/files/images/screenshot.png`
    // directories up to `{storage dir}/{username}/secret/files/images/` will be created
    if let Some(prefix) = path.parent() {
        fs::create_dir_all(&prefix).map_err(|_| Error::CreateDirectory)?
    }

    let field = multipart
        .next_field()
        .await
        .map_err(|_| Error::Multipart)?
        .ok_or(Error::Multipart)?;

    let file = fs::File::create(path).map_err(|_| Error::CreateFile)?;

    field
        .try_fold((file, 0), |(mut file, written_len), bytes| async move {
            file.write_all(bytes.as_ref())
                .expect("failed to write chunk to file");

            Ok((file, written_len + bytes.len() as u64))
        })
        .await
        .map_err(|_| Error::WriteFile)?;

    Ok(Json(Response { success: true }))
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub success: bool,
}
