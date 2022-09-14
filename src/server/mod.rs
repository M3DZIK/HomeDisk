mod api;
pub mod error;
pub mod utils;

use anyhow::anyhow;
use axum::{http::HeaderValue, routing::get, Extension, Router, Server};
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing::info;

use crate::{config::Config, database::Database};

pub async fn start_server(config: Config, db: Database) -> anyhow::Result<()> {
    info!(
        "🚀 Server has launched on http://{}:{}",
        config.http.host, config.http.port
    );

    // change the type from Vec<String> to Vec<HeaderValue> so that the http server can correctly detect CORS hosts
    let origins = config
        .http
        .cors
        .iter()
        .map(|e| e.parse().expect("Failed to parse CORS hosts"))
        .collect::<Vec<HeaderValue>>();

    let host = format!("{}:{}", config.http.host, config.http.port);

    let app = Router::new()
        .nest("/api", api::app())
        .route("/", get(api::health))
        .layer(CorsLayer::new().allow_origin(AllowOrigin::list(origins)))
        .layer(Extension(config))
        .layer(Extension(db));

    Server::bind(&host.parse()?)
        .serve(app.into_make_service())
        .await?;

    Err(anyhow!("Server unexpected stopped!"))
}
