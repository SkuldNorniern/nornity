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

use log::{debug, error, info, warn};
use logger::Logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger FIRST, before any other code
    match Logger::init() {
        Ok(_) => {
            info!("Logger initialized successfully");
            debug!("This is a debug message test");
            warn!("This is a warning message test");
            error!("This is an error message test");
        }
        Err(e) => {
            eprintln!("Failed to initialize logger: {e}");
            return Err(format!("Failed to initialize logger: {e}").into());
        }
    }

    // Load configuration
    let config = config::Config::default();
    debug!("Configuration loaded: {config:?}");

    // Initialize application
    app::init_app(&config).await?;

    // Run server
    if let Err(e) = server::run_server(config).await {
        error!("Server error: {e}");
        return Err(e);
    }

    Ok(())
}
