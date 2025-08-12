use crate::blog::BlogStore;
use chrono::{DateTime, Utc};
use log::info;

/// RSS feed entry representing a blog post
#[derive(Debug, Clone)]
pub struct RSSEntry {
    pub title: String,
    pub link: String,
    pub description: String,
    pub author: String,
    pub pub_date: DateTime<Utc>,
    pub guid: String,
    pub categories: Vec<String>,
}

/// RSS feed generator for the blog
pub struct RSSGenerator {
    base_url: String,
    site_title: String,
    site_description: String,
    site_author: String,
}

impl RSSGenerator {
    /// Create a new RSS generator with the given configuration
    pub fn new(
        base_url: String,
        site_title: String,
        site_description: String,
        site_author: String,
    ) -> Self {
        Self {
            base_url,
            site_title,
            site_description,
            site_author,
        }
    }

    /// Generate RSS 2.0 feed XML for the blog
    pub fn generate_rss_feed(
        &self,
        blog_store: &BlogStore,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        info!("Generating RSS feed for: {}", self.base_url);

        let posts = blog_store.get_all_posts();
        let published_posts: Vec<_> = posts.iter().filter(|post| post.is_published()).collect();

        info!(
            "Adding {} published blog posts to RSS feed",
            published_posts.len()
        );

        // Convert blog posts to RSS entries
        let entries: Vec<RSSEntry> = published_posts
            .iter()
            .map(|post| RSSEntry {
                title: post.title().to_string(),
                link: format!("{}/blog/{}", self.base_url, post.slug),
                description: post.excerpt().to_string(),
                author: post.author().to_string(),
                pub_date: post.meta.published_at,
                guid: format!("{}/blog/{}", self.base_url, post.slug),
                categories: post.tags().to_vec(),
            })
            .collect();

        // Generate XML
        let xml = self.generate_xml(&entries)?;

        info!("Generated RSS feed with {} entries", entries.len());
        Ok(xml)
    }

    /// Generate RSS 2.0 XML from entries
    fn generate_xml(
        &self,
        entries: &[RSSEntry],
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut xml = String::new();

        // XML header
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");

        // RSS root element
        xml.push_str("<rss version=\"2.0\" xmlns:atom=\"http://www.w3.org/2005/Atom\">\n");
        xml.push_str("  <channel>\n");

        // Channel metadata
        xml.push_str(&format!(
            "    <title>{}</title>\n",
            self.escape_xml(&self.site_title)
        ));
        xml.push_str(&format!("    <link>{}</link>\n", self.base_url));
        xml.push_str(&format!(
            "    <description>{}</description>\n",
            self.escape_xml(&self.site_description)
        ));
        xml.push_str("    <language>en-us</language>\n");
        xml.push_str(&format!(
            "    <lastBuildDate>{}</lastBuildDate>\n",
            self.format_rfc822(Utc::now())
        ));
        xml.push_str(&format!(
            "    <atom:link href=\"{}/rss.xml\" rel=\"self\" type=\"application/rss+xml\" />\n",
            self.base_url
        ));

        // Add each entry
        for entry in entries {
            xml.push_str("    <item>\n");
            xml.push_str(&format!(
                "      <title>{}</title>\n",
                self.escape_xml(&entry.title)
            ));
            xml.push_str(&format!("      <link>{}</link>\n", entry.link));
            xml.push_str(&format!(
                "      <description>{}</description>\n",
                self.escape_xml(&entry.description)
            ));
            xml.push_str(&format!(
                "      <author>{}</author>\n",
                self.escape_xml(&entry.author)
            ));
            xml.push_str(&format!(
                "      <pubDate>{}</pubDate>\n",
                self.format_rfc822(entry.pub_date)
            ));
            xml.push_str(&format!("      <guid>{}</guid>\n", entry.guid));

            // Add categories (tags)
            for category in &entry.categories {
                xml.push_str(&format!(
                    "      <category>{}</category>\n",
                    self.escape_xml(category)
                ));
            }

            xml.push_str("    </item>\n");
        }

        xml.push_str("  </channel>\n");
        xml.push_str("</rss>");

        Ok(xml)
    }

    /// Escape XML special characters
    fn escape_xml(&self, text: &str) -> String {
        text.replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&#39;")
    }

    /// Format date in RFC 822 format (required for RSS)
    fn format_rfc822(&self, date: DateTime<Utc>) -> String {
        date.format("%a, %d %b %Y %H:%M:%S %z").to_string()
    }
}

/// Generate RSS feed XML for the blog
pub fn generate_rss_feed_xml(
    base_url: &str,
    site_title: &str,
    site_description: &str,
    site_author: &str,
    blog_store: &BlogStore,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let generator = RSSGenerator::new(
        base_url.to_string(),
        site_title.to_string(),
        site_description.to_string(),
        site_author.to_string(),
    );
    generator.generate_rss_feed(blog_store)
}
