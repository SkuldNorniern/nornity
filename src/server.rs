use crate::config::Config;
use crate::handlers;
use axum::{Router, routing::get};
use log::{debug, error, info};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

/// Build the application router
pub fn build_app(config: &Config) -> Router {
    info!("Building application router");
    let static_service = ServeDir::new(&config.static_dir);
    debug!(
        "Static file service configured for directory: {}",
        config.static_dir
    );

    let router = Router::new()
        .route("/", get(handlers::homepage))
        .route("/blog", get(handlers::blog_list))
        .route("/blog/{slug}", get(handlers::blog_post))
        .route("/sitemap.xml", get(handlers::sitemap))
        .route("/robots.txt", get(handlers::robots_txt))
        .nest_service("/static", static_service);

    info!("Router configured with {} routes", 5);
    router
}

/// Run the server
pub async fn run_server(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting server initialization");

    let app = build_app(&config);
    info!("Application routes configured");

    let addr = config.socket_addr();
    info!("Server will listen on {addr}");
    debug!("Full address: {addr:?}");

    let listener = match TcpListener::bind(addr).await {
        Ok(l) => {
            info!("TCP listener bound successfully to {addr}");
            l
        }
        Err(e) => {
            error!("Failed to bind TCP listener to {addr}: {e}");
            return Err(e.into());
        }
    };

    info!("Starting HTTP server...");
    info!("Server is ready to accept connections on {addr}");

    match axum::serve(listener, app).await {
        Ok(_) => {
            info!("Server shutdown gracefully");
            Ok(())
        }
        Err(e) => {
            error!("Server error: {e}");
            Err(e.into())
        }
    }
}
