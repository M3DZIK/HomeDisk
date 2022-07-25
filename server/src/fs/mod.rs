mod create_dir;
mod delete;
mod download;
mod list;
mod upload;

pub fn app() -> axum::Router {
    use axum::routing::{delete, get, post};

    axum::Router::new()
        .route("/list", post(list::handle))
        .route("/upload", post(upload::handle))
        .route("/delete", delete(delete::handle))
        .route("/download", get(download::handle))
        .route("/createdir", post(create_dir::handle))
}
