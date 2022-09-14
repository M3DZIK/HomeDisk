mod login;
mod register;
mod whoami;

use axum::routing::*;

pub fn app() -> Router {
    Router::new()
        .route("/login", post(login::login))
        .route("/register", post(register::register))
        .route("/whoami", get(whoami::whoami))
}
