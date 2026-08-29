use crate::config::Config;
use crate::handlers;
use axum::http::{HeaderValue, header};
use axum::serve::ListenerExt;
use axum::{Router, routing::get};
use log::{debug, error, info};
use std::time::Duration;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::timeout::TimeoutLayer;

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
        .route("/healthz", get(handlers::health))
        .route("/blog", get(handlers::blog_list))
        .route("/blog/{slug}", get(handlers::blog_post))
        .route("/sitemap.xml", get(handlers::sitemap))
        .route("/robots.txt", get(handlers::robots_txt))
        .route("/rss.xml", get(handlers::rss_feed))
        .nest_service(
            "/static",
            ServiceBuilder::new()
                .layer(SetResponseHeaderLayer::if_not_present(
                    header::CACHE_CONTROL,
                    HeaderValue::from_static("public, max-age=86400"),
                ))
                .service(static_service),
        )
        .fallback(handlers::not_found)
        .layer(
            ServiceBuilder::new()
                .layer(TimeoutLayer::new(Duration::from_secs(15)))
                // 500 instead of a dropped connection
                .layer(CatchPanicLayer::new())
                .layer(CompressionLayer::new()),
        );

    info!("Router configured with {} routes", 7);
    router
}

/// Resolve when the process is asked to stop, so in-flight requests finish before the
/// listener closes. SIGTERM comes from an orchestrator, Ctrl-C from a developer.
async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(e) = tokio::signal::ctrl_c().await {
            error!("Failed to install Ctrl-C handler: {e}");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut signal) => {
                signal.recv().await;
            }
            Err(e) => {
                error!("Failed to install SIGTERM handler: {e}");
                std::future::pending::<()>().await;
            }
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Received Ctrl-C, shutting down"),
        _ = terminate => info!("Received SIGTERM, shutting down"),
    }
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
            // a compressed body goes out as several chunks, and without this Nagle
            // holds the last one for the delayed ACK: ~40 ms per keep-alive request
            l.tap_io(|stream| {
                if let Err(e) = stream.set_nodelay(true) {
                    error!("Failed to set TCP_NODELAY: {e}");
                }
            })
        }
        Err(e) => {
            error!("Failed to bind TCP listener to {addr}: {e}");
            return Err(e.into());
        }
    };

    info!("Starting HTTP server...");
    info!("Server is ready to accept connections on {addr}");

    match axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
    {
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
