mod create_dir;
mod delete;
mod list;
mod upload;

use axum::routing::*;

pub fn app() -> Router {
    Router::new()
        .route("/list", get(list::list))
        .route("/createDir", get(create_dir::create_dir))
        .route("/delete", delete(delete::delete))
        .route("/upload", post(upload::upload))
}
