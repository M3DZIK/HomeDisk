use std::{fs, io, path::PathBuf, time::SystemTime};

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

/// Get directory size on disk (size of all files in directory).
fn dir_size(path: impl Into<PathBuf>) -> io::Result<u64> {
    fn dir_size(mut dir: fs::ReadDir) -> io::Result<u64> {
        dir.try_fold(0, |acc, file| {
            let file = file?;
            let size = match file.metadata()? {
                data if data.is_dir() => dir_size(fs::read_dir(file.path())?)?,
                data => data.len(),
            };
            Ok(acc + size)
        })
    }

    dir_size(fs::read_dir(path.into())?)
}

pub async fn list(
    Extension(config): Extension<Config>,
    path: PathQuery,
    Token(user): Token,
) -> Result<Json<Response>> {
    let path = validate_path(path)?;

    let mut response = Response::default();

    let path = format!("{}/{}", user.user_dir(&config.storage.path), path);

    let mut paths = fs::read_dir(&path).map_err(|_| Error::FailedReadDirectory)?;

    while let Some(Ok(entry)) = paths.next() {
        let metadata = entry.metadata().unwrap();

        let name = entry
            .path()
            .display()
            .to_string()
            .replace(&format!("{path}/"), "")
            .replace(&path, "");

        let modified = metadata
            .modified()
            .unwrap()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if metadata.is_dir() {
            let size = Byte::from_bytes(dir_size(entry.path()).unwrap().into())
                .get_appropriate_unit(true)
                .to_string();

            response.dirs.push(Entry::new(name, size, modified))
        } else {
            let size = Byte::from_bytes(metadata.len().into())
                .get_appropriate_unit(true)
                .to_string();

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
