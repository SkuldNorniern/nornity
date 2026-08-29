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

/// Load every template into the global instance. Called once at startup so a missing
/// template stops the process there instead of panicking inside the first request.
pub fn init_template_engine(
    config: &Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    debug!("Initializing global template engine");
    let engine = TemplateEngine::new(&config.template_dir)?;
    let _ = TEMPLATE_ENGINE.set(engine);
    Ok(())
}

/// Present for every handler: startup calls `init_template_engine` and exits on error.
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
