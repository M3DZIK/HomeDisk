pub mod list;
pub mod upload;

pub fn app() -> axum::Router {
    use axum::routing::post;

    axum::Router::new()
        .route("/list", post(list::handle))
        .route("/upload", post(upload::handle))
}
