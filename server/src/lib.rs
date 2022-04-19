pub mod auth;
mod error;

use axum::{http::HeaderValue, routing::get, Extension, Router, Server};
use homedisk_utils::{config::Config, database::Database};
use log::{debug, info};
use tower_http::cors::{CorsLayer, Origin};

async fn health_check() -> &'static str {
    "I'm alive!"
}

pub async fn serve(
    host: String,
    origins: Vec<HeaderValue>,
    db: Database,
    config: Config,
) -> error::Result<()> {
    debug!("starting http server");
    info!("Website available at: http://{host}");

    let app = Router::new()
        .route("/health-check", get(health_check))
        .nest("/auth", auth::app())
        .layer(CorsLayer::new().allow_origin(Origin::list(origins)))
        .layer(Extension(db))
        .layer(Extension(config));

    Server::bind(&host.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
