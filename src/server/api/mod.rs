mod auth;
mod fs;

use axum::routing::*;

pub async fn health() -> &'static str {
    "I am working!"
}

pub fn app() -> Router {
    Router::new()
        .nest("/auth", auth::app())
        .nest("/fs", fs::app())
        .route("/health", get(health))
}
