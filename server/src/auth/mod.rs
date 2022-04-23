pub mod login;
pub mod register;
pub mod whoami;

pub fn app() -> axum::Router {
    use axum::routing::post;

    axum::Router::new()
        .route("/login", post(login::handle))
        .route("/register", post(register::handle))
        .route("/whoami", post(whoami::handle))
}
