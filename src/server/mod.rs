mod api;
pub mod error;
pub mod utils;

use std::{
    path::{Path, PathBuf},
    process::exit,
};

use anyhow::anyhow;
use axum::{
    extract::Host,
    handler::HandlerWithoutStateExt,
    http::{HeaderValue, StatusCode, Uri},
    response::Redirect,
    routing::get,
    Extension, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    BoxError,
};
use tracing::{debug, error, info};

use crate::{config::Config, database::Database};

pub async fn start_server(config: Config, db: Database) -> anyhow::Result<()> {
    let host = if config.http.enable_https {
        format!("{}:{}", config.http.host, config.http.https_port)
    } else {
        format!("{}:{}", config.http.host, config.http.http_port)
    };

    if config.http.enable_https {
        // check if tls cert and key file exists
        if !Path::new(&config.http.tls_cert).exists() || !Path::new(&config.http.tls_key).exists() {
            error!("TLS cert or/and key file not found!");
            exit(1);
        }

        // start http redirect to https
        tokio::spawn(redirect_http_to_https(config.clone()));
    }

    info!("🚀 Server has launched on https://{host}");

    // change the type from Vec<String> to Vec<HeaderValue> so that the http server can correctly detect CORS hosts
    let origins = config
        .http
        .cors
        .iter()
        .map(|e| e.parse().expect("Failed to parse CORS hosts"))
        .collect::<Vec<HeaderValue>>();

    let app = Router::new()
        .nest("/api", api::app())
        .route("/", get(api::health))
        .layer(CorsLayer::new().allow_origin(AllowOrigin::list(origins)))
        .layer(Extension(config.clone()))
        .layer(Extension(db));

    // if https is enabled, run it, otherwise run server http
    if config.http.enable_https {
        // start https server
        let tls_config = RustlsConfig::from_pem_file(
            PathBuf::from("").join("").join(&config.http.tls_cert),
            PathBuf::from("").join("").join(&config.http.tls_key),
        )
        .await
        .unwrap();

        axum_server::bind_rustls(host.parse()?, tls_config)
            .serve(app.into_make_service())
            .await?;
    } else {
        // start http server
        axum::Server::bind(&host.parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    }

    Err(anyhow!("Server unexpected stopped!"))
}

async fn redirect_http_to_https(config: Config) {
    fn make_https(host: String, uri: Uri, config: Config) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(
            &config.http.http_port.to_string(),
            &config.http.https_port.to_string(),
        );
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let host = format!("{}:{}", config.http.host, config.http.http_port);

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, config) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "Failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            },
        }
    };

    debug!("🚀 HTTPS redirect listening on http://{host}");

    axum::Server::bind(&host.parse().unwrap())
        .serve(redirect.into_make_service())
        .await
        .unwrap();
}
