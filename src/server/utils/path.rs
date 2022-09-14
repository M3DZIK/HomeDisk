use axum::extract::Query;
use serde::Deserialize;

use crate::server::error::{Error, Result};

pub type PathQuery = Query<Path>;

#[derive(Debug, Clone, Deserialize)]
pub struct Path {
    pub path: String,
}

pub fn validate_path(path: PathQuery) -> Result<String> {
    let path = path.path.clone();

    // `path` can't contain `..`
    // to prevent attack attempts because by using a `..` you can access the previous folder
    if path.contains("..") {
        return Err(Error::InvalidPath);
    }

    // `path` can't contain `~`
    // to prevent attack attempts because `~` can get up a directory on `$HOME`
    if path.contains('~') {
        return Err(Error::InvalidPath);
    }

    Ok(path)
}
