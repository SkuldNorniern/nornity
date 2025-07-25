use log::{debug, info};
use std::path::PathBuf;

#[cfg(feature = "hot-reload")]
use crate::routes;
#[cfg(feature = "hot-reload")]
use notify::{RecursiveMode, Watcher};
#[cfg(feature = "hot-reload")]
use tokio::sync::mpsc;

/// Setup file watching for hot reload
pub async fn setup_file_watcher(content_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "hot-reload")]
    {
        info!("Setting up file watcher for hot reload");
        debug!("Watching directory: {content_dir:?}");

        let (tx, mut rx) = mpsc::channel(100);
        let blog_store = routes::get_blog_store();

        // Spawn file watcher in a separate task
        tokio::spawn(async move {
            debug!("Starting file watcher task");
            let mut watcher = match notify::recommended_watcher(move |res| {
                if let Ok(event) = res {
                    debug!("File system event detected: {:?}", event);
                    let _ = tx.blocking_send(event);
                }
            }) {
                Ok(w) => w,
                Err(e) => {
                    log::error!("Failed to create file watcher: {}", e);
                    return;
                }
            };

            if let Err(e) = watcher.watch(&content_dir, RecursiveMode::NonRecursive) {
                log::error!("Failed to watch content directory: {}", e);
                return;
            }

            info!("File watcher started successfully");

            // Keep the watcher alive
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });

        // Handle file change events
        tokio::spawn(async move {
            info!("Starting file change handler");
            while let Some(event) = rx.recv().await {
                match event.kind {
                    notify::EventKind::Create(_)
                    | notify::EventKind::Modify(_)
                    | notify::EventKind::Remove(_) => {
                        info!("Content directory changed, reloading posts...");
                        debug!("Event details: {:?}", event);
                        match blog_store.load_posts().await {
                            Ok(_) => {
                                let post_count = blog_store.get_all_posts().len();
                                info!("Successfully reloaded {} posts", post_count);
                            }
                            Err(e) => {
                                log::error!("Failed to reload posts: {}", e);
                            }
                        }
                    }
                    _ => {
                        debug!("Ignoring event: {:?}", event.kind);
                    }
                }
            }
        });
    }

    #[cfg(not(feature = "hot-reload"))]
    {
        info!("Hot reload disabled - file watching not available");
        debug!("Content directory: {content_dir:?}");
    }

    Ok(())
}
