mod api;
pub mod error;
pub mod utils;

use anyhow::anyhow;
use axum::{routing::get, Extension, Router, Server};
use tracing::info;

use crate::{config::Config, database::Database};

pub async fn start_server(config: Config, db: Database) -> anyhow::Result<()> {
    info!(
        "🚀 Server has launched on http://{}:{}",
        config.http.host, config.http.port
    );

    let host = format!("{}:{}", config.http.host, config.http.port);

    let app = Router::new()
        .nest("/api", api::app())
        .route("/", get(api::health))
        .layer(Extension(config))
        .layer(Extension(db));

    Server::bind(&host.parse()?)
        .serve(app.into_make_service())
        .await?;

    Err(anyhow!("Server unexpected stopped!"))
}
