mod app;
mod blog;
mod components;
mod config;
mod handlers;
mod logger;
mod routes;
mod server;
mod template_helpers;
mod templates;
mod watcher;

use log::{debug, error, info};
use logger::Logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Logger::init() {
        Ok(_) => {
            info!("Logger initialized successfully");
        }
        Err(e) => {
            eprintln!("Failed to initialize logger: {e}");
            return Err(format!("Failed to initialize logger: {e}").into());
        }
    }

    // handlers read this same instance, so parsing again here would give two configs
    let config = routes::get_config();
    debug!("Configuration loaded: {config:?}");

    app::init_app(config).await?;

    if let Err(e) = server::run_server(config.clone()).await {
        error!("Server error: {e}");
        return Err(e);
    }

    Ok(())
}
