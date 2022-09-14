use std::{fs, time::SystemTime};

use axum::{Extension, Json};
use byte_unit::Byte;
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

pub async fn list(
    Extension(config): Extension<Config>,
    path: PathQuery,
    Token(user): Token,
) -> Result<Json<Response>> {
    let path = validate_path(path)?;

    let mut response = Response::default();

    let path = format!("{}/{}", user.user_dir(&config.storage.path), path);

    let paths = fs::read_dir(&path).map_err(|_| Error::FailedReadDirectory)?;

    for dir_entry in paths {
        let dir_entry = dir_entry.unwrap();

        let metadata = dir_entry.metadata().unwrap();

        let name = dir_entry
            .path()
            .display()
            .to_string()
            .replace(&format!("{path}/"), "")
            .replace(&path, "");

        if metadata.is_dir() {
            // TODO: add size and modification date
            response.dirs.push(Entry::new(name, 0.to_string(), 0))
        } else {
            let size = Byte::from_bytes(metadata.len().into())
                .get_appropriate_unit(true)
                .to_string();

            let modified = metadata
                .modified()
                .unwrap()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            response.files.push(Entry::new(name, size, modified));
        }
    }

    Ok(Json(response))
}

#[derive(Debug, Default, Serialize)]
pub struct Response {
    pub dirs: Vec<Entry>,
    pub files: Vec<Entry>,
}

#[derive(Debug, Serialize)]
pub struct Entry {
    pub name: String,
    pub size: String,
    pub modified: u64,
}

impl Entry {
    pub fn new(name: String, size: String, modified: u64) -> Self {
        Self {
            name,
            size,
            modified,
        }
    }
}
