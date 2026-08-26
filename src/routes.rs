use crate::blog::BlogStore;
use crate::config::Config;
use crate::templates::TemplateEngine;
use log::debug;
use std::sync::OnceLock;

/// Global blog store instance
static BLOG_STORE: OnceLock<BlogStore> = OnceLock::new();

/// Global template engine instance
static TEMPLATE_ENGINE: OnceLock<TemplateEngine> = OnceLock::new();

/// Global config instance
static CONFIG: OnceLock<Config> = OnceLock::new();

/// Get the global blog store instance
pub fn get_blog_store() -> &'static BlogStore {
    BLOG_STORE.get_or_init(|| {
        debug!("Initializing global blog store");
        BlogStore::new(std::path::PathBuf::from("content"))
    })
}

/// Load every template and store it in the global instance.
///
/// Called once from application startup. Returns an error instead of panicking so a
/// missing template stops the process with a readable message at boot, rather than
/// taking down the first request that happens to need it.
pub fn init_template_engine(
    config: &Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    debug!("Initializing global template engine");
    let engine = TemplateEngine::new(&config.template_dir)?;
    // A second call would mean startup ran twice; keep the first engine either way.
    let _ = TEMPLATE_ENGINE.set(engine);
    Ok(())
}

/// Get the global template engine instance.
///
/// `init_template_engine` runs during startup and the process exits if it fails, so by
/// the time any request handler calls this the engine is present.
pub fn get_template_engine() -> &'static TemplateEngine {
    TEMPLATE_ENGINE
        .get()
        .expect("template engine is initialized during startup")
}

/// Get the global config instance
pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| {
        debug!("Initializing global config");
        Config::from_file_or_env()
    })
}
