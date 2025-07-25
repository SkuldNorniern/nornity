use chrono::{DateTime, NaiveDateTime, Utc};
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use tokio::fs;

/// Blog post metadata from YAML frontmatter
#[derive(Debug, Clone)]
pub struct BlogPostMeta {
    pub title: String,
    pub excerpt: String,
    pub author: String,
    pub tags: Vec<String>,
    pub published_at: DateTime<Utc>,
    pub draft: bool,
}

impl BlogPostMeta {
    /// Create a new blog post metadata with human-readable date parsing
    pub fn new(
        title: String,
        excerpt: String,
        author: String,
        tags: Vec<String>,
        published_at: &str,
        draft: bool,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let published_at = Self::parse_date(published_at)?;

        Ok(Self {
            title,
            excerpt,
            author,
            tags,
            published_at,
            draft,
        })
    }

    /// Parse various date formats into DateTime<Utc>
    fn parse_date(
        date_str: &str,
    ) -> Result<DateTime<Utc>, Box<dyn std::error::Error + Send + Sync>> {
        // Try common date formats
        let formats = [
            "%Y-%m-%d %H:%M:%S",  // 2024-01-15 10:30:00
            "%Y-%m-%dT%H:%M:%S",  // 2024-01-15T10:30:00
            "%Y-%m-%dT%H:%M:%SZ", // 2024-01-15T10:30:00Z
            "%Y-%m-%d %H:%M",     // 2024-01-15 10:30
            "%Y-%m-%d",           // 2024-01-15
            "%B %d, %Y %H:%M:%S", // January 15, 2024 10:30:00
            "%B %d, %Y",          // January 15, 2024
            "%d %B %Y %H:%M:%S",  // 15 January 2024 10:30:00
            "%d %B %Y",           // 15 January 2024
        ];

        for format in &formats {
            if let Ok(naive) = NaiveDateTime::parse_from_str(date_str, format) {
                return Ok(DateTime::from_naive_utc_and_offset(naive, Utc));
            }
        }

        // Try parsing as Unix timestamp
        if let Ok(timestamp) = date_str.parse::<i64>() {
            return Ok(DateTime::from_timestamp(timestamp, 0).ok_or("Invalid timestamp")?);
        }

        Err(format!("Unable to parse date: {date_str}").into())
    }
}

/// Blog post structure
#[derive(Debug, Clone)]
pub struct BlogPost {
    pub slug: String,
    pub meta: BlogPostMeta,
    pub content: String,
}

impl BlogPost {
    /// Create a new blog post from file
    pub async fn from_file(
        file_path: PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let content = fs::read_to_string(&file_path).await?;
        let slug = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("Invalid filename")?
            .to_string();

        // Parse frontmatter and content
        let (meta, content) = Self::parse_frontmatter(&content)?;

        Ok(Self {
            slug,
            meta,
            content,
        })
    }

    /// Parse YAML frontmatter from markdown content (manual parsing, no serde)
    fn parse_frontmatter(
        content: &str,
    ) -> Result<(BlogPostMeta, String), Box<dyn std::error::Error + Send + Sync>> {
        if !content.starts_with("---") {
            return Err("No frontmatter found".into());
        }

        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() < 3 {
            return Err("Invalid frontmatter format".into());
        }

        let frontmatter = parts[1].trim();
        let content = parts[2].trim();

        // Manual YAML parsing (very basic, expects each field on its own line)
        let mut title = None;
        let mut excerpt = None;
        let mut author = None;
        let mut tags = Vec::new();
        let mut published_at = None;
        let mut draft = false;

        for line in frontmatter.lines() {
            let line = line.trim();
            if let Some(stripped) = line.strip_prefix("title:") {
                title = Some(stripped.trim().trim_matches('"').to_string());
            } else if let Some(stripped) = line.strip_prefix("excerpt:") {
                excerpt = Some(stripped.trim().trim_matches('"').to_string());
            } else if let Some(stripped) = line.strip_prefix("author:") {
                author = Some(stripped.trim().trim_matches('"').to_string());
            } else if let Some(stripped) = line.strip_prefix("tags:") {
                // Parse tags: ["tag1", "tag2"] or [tag1, tag2]
                let tags_str = stripped.trim().trim_start_matches(':').trim();
                let tags_str = tags_str.trim_start_matches('[').trim_end_matches(']');
                tags = tags_str
                    .split(',')
                    .map(|t| t.trim().trim_matches('"').to_string())
                    .filter(|t| !t.is_empty())
                    .collect();
            } else if let Some(stripped) = line.strip_prefix("published_at:") {
                published_at = Some(stripped.trim().trim_matches('"').to_string());
            } else if let Some(stripped) = line.strip_prefix("draft:") {
                draft = stripped.trim().eq_ignore_ascii_case("true");
            }
        }

        let meta = BlogPostMeta::new(
            title.ok_or("Missing or invalid title")?,
            excerpt.ok_or("Missing or invalid excerpt")?,
            author.ok_or("Missing or invalid author")?,
            tags,
            &published_at.ok_or("Missing or invalid published_at")?,
            draft,
        )?;

        Ok((meta, content.to_string()))
    }

    /// Check if post has a specific tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.meta.tags.iter().any(|t| t == tag)
    }

    /// Get formatted publication date
    pub fn formatted_date(&self) -> String {
        self.meta.published_at.format("%B %d, %Y").to_string()
    }

    /// Get ISO date for datetime attribute
    pub fn iso_date(&self) -> String {
        self.meta.published_at.format("%Y-%m-%d").to_string()
    }

    /// Get title
    pub fn title(&self) -> &str {
        &self.meta.title
    }

    /// Get excerpt
    pub fn excerpt(&self) -> &str {
        &self.meta.excerpt
    }

    /// Get author
    pub fn author(&self) -> &str {
        &self.meta.author
    }

    /// Get tags
    pub fn tags(&self) -> &[String] {
        &self.meta.tags
    }

    /// Check if post is published
    pub fn is_published(&self) -> bool {
        !self.meta.draft
    }
}

/// Blog storage and management with hot reload
pub struct BlogStore {
    posts: Arc<RwLock<HashMap<String, BlogPost>>>,
    content_dir: PathBuf,
}

impl BlogStore {
    /// Create a new blog store
    pub fn new(content_dir: PathBuf) -> Self {
        Self {
            posts: Arc::new(RwLock::new(HashMap::new())),
            content_dir,
        }
    }

    /// Load all blog posts from the content directory
    pub async fn load_posts(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Loading blog posts from {:?}", self.content_dir);

        let mut posts = HashMap::new();

        if !self.content_dir.exists() {
            warn!("Content directory does not exist: {:?}", self.content_dir);
            return Ok(());
        }

        debug!("Reading content directory: {:?}", self.content_dir);
        let mut entries = fs::read_dir(&self.content_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            debug!("Checking file: {path:?}");

            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                debug!("Processing markdown file: {path:?}");
                match BlogPost::from_file(path.clone()).await {
                    Ok(post) => {
                        if post.is_published() {
                            let slug = post.slug.clone();
                            posts.insert(slug.clone(), post);
                            info!("Loaded blog post: {slug}");
                        } else {
                            debug!("Skipped draft post: {}", post.slug);
                        }
                    }
                    Err(e) => {
                        error!("Failed to load blog post {path:?}: {e}");
                    }
                }
            } else {
                debug!("Skipping non-markdown file: {path:?}");
            }
        }

        {
            let mut posts_guard = self.posts.write().unwrap();
            *posts_guard = posts;
        }

        let post_count = self.posts.read().unwrap().len();
        info!("Successfully loaded {post_count} blog posts");
        debug!("Blog store updated with {post_count} posts");
        Ok(())
    }

    /// Get all blog posts, sorted by publication date (newest first)
    pub fn get_all_posts(&self) -> Vec<BlogPost> {
        let posts_guard = self.posts.read().unwrap();
        let mut posts: Vec<BlogPost> = posts_guard.values().cloned().collect();
        posts.sort_by(|a, b| b.meta.published_at.cmp(&a.meta.published_at));
        posts
    }

    /// Get a specific blog post by slug
    pub fn get_post_by_slug(&self, slug: &str) -> Option<BlogPost> {
        let posts_guard = self.posts.read().unwrap();
        posts_guard.get(slug).cloned()
    }

    /// Get recent blog posts (limit to specified count)
    pub fn get_recent_posts(&self, limit: usize) -> Vec<BlogPost> {
        let mut posts = self.get_all_posts();
        posts.truncate(limit);
        posts
    }

    /// Get posts by tag
    pub fn get_posts_by_tag(&self, tag: &str) -> Vec<BlogPost> {
        self.get_all_posts()
            .into_iter()
            .filter(|post| post.has_tag(tag))
            .collect()
    }

    /// Get all unique tags
    pub fn get_all_tags(&self) -> Vec<String> {
        let posts_guard = self.posts.read().unwrap();
        let mut tags: Vec<String> = posts_guard
            .values()
            .flat_map(|post| &post.meta.tags)
            .cloned()
            .collect();
        tags.sort();
        tags.dedup();
        tags
    }
}

impl Default for BlogStore {
    fn default() -> Self {
        Self::new(PathBuf::from("content"))
    }
}
