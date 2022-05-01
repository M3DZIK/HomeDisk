pub mod list;
pub mod upload;

pub fn app() -> axum::Router {
    use axum::routing::post;

    axum::Router::new()
        .route("/list", post(list::handle))
        .route("/upload", post(upload::handle))
}

pub fn validate_path(path: &str) -> Result<(), homedisk_types::errors::ServerError> {
    use homedisk_types::errors::{FsError, ServerError};

    // `path` cannot contain `..`
    // to prevent attack attempts because by using a `..` you can access the previous folder
    if path.contains("..") {
        return Err(ServerError::FsError(FsError::ReadDir(
            "the `path` must not contain `..`".to_string(),
        )));
    }

    Ok(())
}
