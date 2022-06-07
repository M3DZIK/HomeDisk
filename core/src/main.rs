mod init;

use homedisk_database::Database;
use homedisk_server::run_http_server;
use homedisk_types::config::types::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init::init()?;

    // parse config
    let config = Config::parse()?;

    // open database connection
    let db = Database::open("homedisk.db").await?;

    // change the type from Vec<String> to Vec<HeaderValue> so that the http server can correctly detect CORS hosts
    let origins = config
        .http
        .cors
        .iter()
        .map(|e| e.parse().expect("parse CORS hosts"))
        .collect();

    let host = format!(
        "{host}:{port}",
        host = config.http.host,
        port = config.http.port
    );

    // start http server
    run_http_server(host, origins, db, config).await?;

    Ok(())
}
