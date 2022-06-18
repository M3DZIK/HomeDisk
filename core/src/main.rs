use homedisk_database::Database;
use homedisk_server::serve_http;
use homedisk_types::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // init better_panic
    better_panic::install();
    // init logger
    init_logger()?;

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

    // format host ip and port
    let host = format!(
        "{host}:{port}",
        host = config.http.host,
        port = config.http.port
    );

    // start http server
    serve_http(host, origins, db, config).await?;

    Ok(())
}

/// Init logger
fn init_logger() -> anyhow::Result<()> {
    use std::fs::File;

    use log::LevelFilter;
    use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("logs.log").expect("create logs file"),
        ),
    ])?;

    Ok(())
}
