pub mod auth;
pub mod fs;
pub mod middleware;

mod error;

use axum::{http::HeaderValue, routing::get, Extension, Router, Server};
use homedisk_database::Database;
use homedisk_types::config::types::Config;
use log::{debug, info};
use tower_http::cors::{AllowOrigin, CorsLayer};

/// Handle `/health-check` requests
async fn health_check() -> &'static str {
    "I'm alive!"
}

pub async fn run_http_server(
    host: String,
    origins: Vec<HeaderValue>,
    db: Database,
    config: Config,
) -> error::Result<()> {
    debug!("Starting http server");
    info!("Website available at: http://{host}");

    // create http Router
    let app = Router::new()
        .route("/health-check", get(health_check))
        .nest("/auth", auth::app())
        .nest("/fs", fs::app())
        .layer(CorsLayer::new().allow_origin(AllowOrigin::list(origins)))
        .layer(Extension(db))
        .layer(Extension(config));

    // bind the provided address and serve Router
    Server::bind(&host.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
