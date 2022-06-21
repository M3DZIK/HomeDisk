#![doc(html_root_url = "https://homedisk-doc.medzik.xyz")]

mod auth;
mod error;
mod fs;
mod middleware;

use axum::{http::HeaderValue, routing::get, Extension, Router, Server};
use homedisk_database::Database;
use homedisk_types::config::Config;
use log::{debug, info};
use tower_http::cors::{AllowOrigin, CorsLayer};

/// Handle `/health-check` requests
async fn health_check() -> &'static str {
    "I'm alive!"
}

/// Start HTTP server
pub async fn serve_http(
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
        .nest("/api/auth", auth::app())
        .nest("/api/fs", fs::app())
        .layer(CorsLayer::new().allow_origin(AllowOrigin::list(origins)))
        .layer(Extension(db))
        .layer(Extension(config));

    // bind the provided address and serve Router
    Server::bind(&host.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
