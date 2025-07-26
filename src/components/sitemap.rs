use crate::blog::BlogStore;
use chrono::{DateTime, Utc};
use log::{debug, info};

/// Sitemap entry representing a URL in the sitemap
#[derive(Debug, Clone)]
pub struct SitemapEntry {
    pub url: String,
    pub last_modified: DateTime<Utc>,
    pub change_frequency: ChangeFrequency,
    pub priority: f32,
}

/// Change frequency for sitemap entries
#[derive(Debug, Clone)]
pub enum ChangeFrequency {
    Always,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Never,
}

impl ChangeFrequency {
    fn as_str(&self) -> &'static str {
        match self {
            ChangeFrequency::Always => "always",
            ChangeFrequency::Hourly => "hourly",
            ChangeFrequency::Daily => "daily",
            ChangeFrequency::Weekly => "weekly",
            ChangeFrequency::Monthly => "monthly",
            ChangeFrequency::Yearly => "yearly",
            ChangeFrequency::Never => "never",
        }
    }
}

/// Sitemap generator for the website
pub struct SitemapGenerator {
    base_url: String,
}

impl SitemapGenerator {
    /// Create a new sitemap generator with the given base URL
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    /// Generate the complete sitemap XML including static pages and blog posts
    pub fn generate_sitemap(&self, blog_store: &BlogStore) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        info!("Generating sitemap for base URL: {}", self.base_url);
        
        let mut entries = Vec::new();
        
        // Add static pages
        self.add_static_pages(&mut entries);
        
        // Add blog posts
        self.add_blog_posts(&mut entries, blog_store);
        
        // Generate XML
        let xml = self.generate_xml(&entries)?;
        
        info!("Generated sitemap with {} entries", entries.len());
        Ok(xml)
    }

    /// Add static pages to the sitemap
    fn add_static_pages(&self, entries: &mut Vec<SitemapEntry>) {
        debug!("Adding static pages to sitemap");
        
        let now = Utc::now();
        
        // Homepage
        entries.push(SitemapEntry {
            url: format!("{}/", self.base_url),
            last_modified: now,
            change_frequency: ChangeFrequency::Daily,
            priority: 1.0,
        });
        
        // Blog list page
        entries.push(SitemapEntry {
            url: format!("{}/blog", self.base_url),
            last_modified: now,
            change_frequency: ChangeFrequency::Daily,
            priority: 0.8,
        });
    }

    /// Add blog posts to the sitemap
    fn add_blog_posts(&self, entries: &mut Vec<SitemapEntry>, blog_store: &BlogStore) {
        debug!("Adding blog posts to sitemap");
        
        let posts = blog_store.get_all_posts();
        let published_posts: Vec<_> = posts.iter().filter(|post| post.is_published()).collect();
        
        info!("Adding {} published blog posts to sitemap", published_posts.len());
        
        for post in published_posts {
            entries.push(SitemapEntry {
                url: format!("{}/blog/{}", self.base_url, post.slug),
                last_modified: post.meta.published_at,
                change_frequency: ChangeFrequency::Monthly,
                priority: 0.6,
            });
        }
    }

    /// Generate XML from sitemap entries
    fn generate_xml(&self, entries: &[SitemapEntry]) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut xml = String::new();
        
        // XML header
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
        
        // Add each entry
        for entry in entries {
            xml.push_str("  <url>\n");
            xml.push_str(&format!("    <loc>{}</loc>\n", entry.url));
            xml.push_str(&format!("    <lastmod>{}</lastmod>\n", entry.last_modified.format("%Y-%m-%d")));
            xml.push_str(&format!("    <changefreq>{}</changefreq>\n", entry.change_frequency.as_str()));
            xml.push_str(&format!("    <priority>{:.1}</priority>\n", entry.priority));
            xml.push_str("  </url>\n");
        }
        
        xml.push_str("</urlset>");
        
        Ok(xml)
    }
}

/// Generate sitemap XML for the website
pub fn generate_sitemap_xml(base_url: &str, blog_store: &BlogStore) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let generator = SitemapGenerator::new(base_url.to_string());
    generator.generate_sitemap(blog_store)
}
