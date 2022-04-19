pub mod login;
pub mod register;

pub fn app() -> axum::Router {
    use axum::routing::post;

    axum::Router::new()
        .route("/login", post(login::handle))
        .route("/register", post(register::handle))
}
