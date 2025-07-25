use crate::blog::BlogStore;
use crate::templates::TemplateEngine;
use log::debug;
use std::sync::OnceLock;

/// Global blog store instance
static BLOG_STORE: OnceLock<BlogStore> = OnceLock::new();

/// Global template engine instance
static TEMPLATE_ENGINE: OnceLock<TemplateEngine> = OnceLock::new();

/// Get the global blog store instance
pub fn get_blog_store() -> &'static BlogStore {
    BLOG_STORE.get_or_init(|| {
        debug!("Initializing global blog store");
        BlogStore::new(std::path::PathBuf::from("content"))
    })
}

/// Get the global template engine instance
pub fn get_template_engine() -> &'static TemplateEngine {
    TEMPLATE_ENGINE.get_or_init(|| {
        debug!("Initializing global template engine");
        TemplateEngine::new().expect("Failed to initialize template engine")
    })
}
