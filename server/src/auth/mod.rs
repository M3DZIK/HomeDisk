pub mod login;

pub fn app() -> axum::Router {
    use axum::routing::post;

    axum::Router::new().route("/login", post(login::handle))
}
