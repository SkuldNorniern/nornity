use crate::config::Config;
use crate::routes;
use crate::watcher;
use log::{debug, error, info, warn};

/// Initialize the blog store
pub async fn init_blog_store(
    content_dir: &std::path::PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing blog store from {content_dir:?}");

    let blog_store = routes::get_blog_store();
    debug!("Blog store instance retrieved");

    match blog_store.load_posts().await {
        Ok(_) => {
            let post_count = blog_store.get_all_posts().len();
            info!("Successfully loaded {post_count} blog posts");
        }
        Err(e) => {
            error!("Failed to load posts: {e}");
            return Err(format!("Failed to load posts: {e}").into());
        }
    }

    Ok(())
}

/// Initialize the application
pub async fn init_app(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Nornity application initialization");

    info!("Starting Nornity server...");
    debug!(
        "Configuration: host={:?}, port={}, static_dir={}, content_dir={:?}",
        config.host, config.port, config.static_dir, config.content_dir
    );

    // Initialize blog store
    init_blog_store(&config.content_dir).await?;

    // Setup file watching for hot reload
    if config.content_dir_exists() {
        info!("Content directory exists, enabling hot reload");
        watcher::setup_file_watcher(config.content_dir.clone()).await?;
        info!("File watching enabled for hot reload");
    } else {
        warn!(
            "Content directory does not exist: {:?}, hot reload disabled",
            config.content_dir
        );
    }

    info!("Application initialization completed successfully");
    Ok(())
}
