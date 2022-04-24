pub mod upload;

pub fn app() -> axum::Router {
    use axum::routing::post;

    axum::Router::new().route("/upload", post(upload::handle))
}
