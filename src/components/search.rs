use std::collections::HashMap;

/// Search and filter configuration
#[derive(Debug, Clone)]
pub struct SearchConfig {
    pub search_term: Option<String>,
    pub selected_tags: Vec<String>,
    pub sort_by: SortOption,
}

/// Sort options for search results
#[derive(Debug, Clone, PartialEq)]
pub enum SortOption {
    DateNewest,
    DateOldest,
    Title,
    Relevance,
}

impl Default for SortOption {
    fn default() -> Self {
        Self::DateNewest
    }
}

/// Search results with metadata
#[derive(Debug, Clone)]
pub struct SearchResults {
    pub posts: Vec<SearchResult>,
    pub total_count: usize,
    pub filtered_count: usize,
    pub search_term: Option<String>,
    pub selected_tags: Vec<String>,
}

/// Individual search result
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub slug: String,
    pub title: String,
    pub excerpt: String,
    pub date: String,
    pub iso_date: String,
    pub author: String,
    pub tags: Vec<String>,
    pub relevance_score: f32,
}

/// Search component for handling server-side search and filtering
pub struct SearchComponent;

impl SearchComponent {
    /// Parse search parameters from query string
    pub fn parse_params(params: &HashMap<String, String>) -> SearchConfig {
        let search_term = params
            .get("search")
            .cloned()
            .filter(|s| !s.trim().is_empty());

        // Parse multiple tags from query parameters
        let mut selected_tags = Vec::new();

        // Handle multiple tag parameters (tag=value1&tag=value2)
        // Since HashMap can only store one value per key, we need to parse the raw query string
        // For now, we'll use a different approach - we'll modify the handler to pass the raw query
        for (key, value) in params {
            if key == "tag" && !value.trim().is_empty() && !selected_tags.contains(value) {
                selected_tags.push(value.clone());
            }
        }

        let sort_by = params
            .get("sort")
            .map(|s| match s.as_str() {
                "date-oldest" => SortOption::DateOldest,
                "title" => SortOption::Title,
                "relevance" => SortOption::Relevance,
                _ => SortOption::DateNewest,
            })
            .unwrap_or_default();

        SearchConfig {
            search_term,
            selected_tags,
            sort_by,
        }
    }

    /// Parse search parameters from raw query string to handle multiple values
    pub fn parse_params_from_raw_query(query: &str) -> SearchConfig {
        let mut search_term = None;
        let mut selected_tags = Vec::new();
        let mut sort_by = SortOption::DateNewest;

        // Parse the raw query string
        for param in query.split('&') {
            if let Some((key, value)) = param.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "search" => {
                        if !value.is_empty() {
                            search_term = Some(value.to_string());
                        }
                    }
                    "tag" => {
                        if !value.is_empty() && !selected_tags.contains(&value.to_string()) {
                            selected_tags.push(value.to_string());
                        }
                    }
                    "sort" => {
                        sort_by = match value {
                            "date-oldest" => SortOption::DateOldest,
                            "title" => SortOption::Title,
                            "relevance" => SortOption::Relevance,
                            _ => SortOption::DateNewest,
                        };
                    }
                    _ => {}
                }
            }
        }

        SearchConfig {
            search_term,
            selected_tags,
            sort_by,
        }
    }

    /// Generate search form HTML
    pub fn render_search_form(config: &SearchConfig) -> String {
        let search_value = config.search_term.as_deref().unwrap_or("");

        format!(
            r#"
            <form method="GET" action="/blog" class="search-form">
                <div class="search-container">
                    <button type="submit" class="search-submit" aria-label="Submit search">
                        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path d="M21 21L16.514 16.506L21 21ZM19 10.5C19 15.194 15.194 19 10.5 19C5.806 19 2 15.194 2 10.5C2 5.806 5.806 2 10.5 2C15.194 2 19 5.806 19 10.5Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                        </svg>
                    </button>
                    <input 
                        type="text" 
                        name="search" 
                        class="search-input" 
                        placeholder="Search posts..." 
                        value="{}"
                        aria-label="Search blog posts"
                    >
                </div>
                
                {}
            </form>
            "#,
            html_escape(search_value),
            {
                let mut hidden_inputs = String::new();
                for tag in &config.selected_tags {
                    hidden_inputs.push_str(&format!(
                        r#"<input type="hidden" name="tag" value="{}">"#,
                        html_escape(tag)
                    ));
                }
                hidden_inputs
            }
        )
    }

    /// Generate tag cloud HTML with active state
    pub fn render_tag_cloud(all_tags: &[String], selected_tags: &[String]) -> String {
        let mut sorted_tags = all_tags.to_vec();
        sorted_tags.sort();

        let tag_links = sorted_tags
            .iter()
            .map(|tag_name| {
                let is_active = selected_tags.contains(tag_name);
                let active_class = if is_active { " active" } else { "" };
                if is_active {
                    // If this tag is active, remove it from URL
                    format!(
                        r##"<a href="{}" class="tag-link{active_class}" aria-label="Remove tag filter: {}">{}<span class="remove-icon">✕</span></a>"##,
                        Self::build_url_without_tag(selected_tags, tag_name),
                        tag_name, tag_name
                    )
                } else {
                    // Add this tag to URL - ensure we don't add duplicates
                    format!(
                        r##"<a href="{}" class="tag-link{active_class}" aria-label="Filter by tag: {}">{}</a>"##,
                        Self::build_url_with_tag(selected_tags, tag_name),
                        tag_name, tag_name
                    )
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        format!(
            r#"
            <div class="tag-cloud" role="region" aria-label="Blog post tags">
                {}
            </div>
            "#,
            tag_links
        )
    }

    /// Generate search results summary
    pub fn render_search_summary(results: &SearchResults) -> String {
        let mut summary_parts = Vec::new();

        if let Some(search_term) = &results.search_term {
            summary_parts.push(format!("search for '{}'", search_term));
        }

        if !results.selected_tags.is_empty() {
            if results.selected_tags.len() == 1 {
                summary_parts.push(format!("tagged '{}'", results.selected_tags[0]));
            } else {
                let tags_text = results.selected_tags.join("', '");
                summary_parts.push(format!("tagged '{}'", tags_text));
            }
        }

        let filter_text = if summary_parts.is_empty() {
            "all posts".to_string()
        } else {
            summary_parts.join(" and ")
        };

        format!(
            r#"
            <div class="search-summary" role="status" aria-live="polite">
                <p>Showing {} of {} posts ({})</p>
            </div>
            "#,
            results.filtered_count, results.total_count, filter_text
        )
    }

    /// Generate pagination controls
    pub fn render_pagination(
        current_page: usize,
        total_pages: usize,
        config: &SearchConfig,
    ) -> String {
        if total_pages <= 1 {
            return String::new();
        }

        let mut pagination_links = Vec::new();

        // Previous page
        if current_page > 1 {
            let prev_url = Self::build_pagination_url(current_page - 1, config);
            pagination_links.push(format!(
                r#"<a href="{}" class="pagination-link prev" aria-label="Previous page">← Previous</a>"#,
                prev_url
            ));
        }

        // Page numbers
        let start_page = (current_page.saturating_sub(2)).max(1);
        let end_page = (current_page + 2).min(total_pages);

        for page in start_page..=end_page {
            if page == current_page {
                pagination_links.push(format!(
                    r#"<span class="pagination-current" aria-current="page">{}</span>"#,
                    page
                ));
            } else {
                let page_url = Self::build_pagination_url(page, config);
                pagination_links.push(format!(
                    r#"<a href="{}" class="pagination-link" aria-label="Page {}">{}</a>"#,
                    page_url, page, page
                ));
            }
        }

        // Next page
        if current_page < total_pages {
            let next_url = Self::build_pagination_url(current_page + 1, config);
            pagination_links.push(format!(
                r#"<a href="{}" class="pagination-link next" aria-label="Next page">Next →</a>"#,
                next_url
            ));
        }

        format!(
            r#"
            <nav class="pagination" role="navigation" aria-label="Search results pagination">
                {}
            </nav>
            "#,
            pagination_links.join(" ")
        )
    }

    /// Build URL for pagination with current search parameters
    fn build_pagination_url(page: usize, config: &SearchConfig) -> String {
        let mut params = Vec::new();

        if let Some(search_term) = &config.search_term {
            params.push(format!("search={}", url_encode(search_term)));
        }

        for tag in &config.selected_tags {
            params.push(format!("tag={}", url_encode(tag)));
        }

        if config.sort_by != SortOption::DateNewest {
            let sort_value = match config.sort_by {
                SortOption::DateOldest => "date-oldest",
                SortOption::Title => "title",
                SortOption::Relevance => "relevance",
                _ => "date-newest",
            };
            params.push(format!("sort={}", sort_value));
        }

        if page > 1 {
            params.push(format!("page={}", page));
        }

        if params.is_empty() {
            "/blog".to_string()
        } else {
            format!("/blog?{}", params.join("&"))
        }
    }

    /// Build URL with a new tag added
    fn build_url_with_tag(selected_tags: &[String], new_tag: &str) -> String {
        let mut params = Vec::new();

        // Add existing tags
        for tag in selected_tags {
            params.push(format!("tag={}", url_encode(tag)));
        }

        // Always add the new tag
        params.push(format!("tag={}", url_encode(new_tag)));

        if params.is_empty() {
            "/blog".to_string()
        } else {
            format!("/blog?{}", params.join("&"))
        }
    }

    /// Build URL with a tag removed
    fn build_url_without_tag(selected_tags: &[String], tag_to_remove: &str) -> String {
        let mut params = Vec::new();

        // Add all tags except the one to remove
        for tag in selected_tags {
            if tag != tag_to_remove {
                params.push(format!("tag={}", url_encode(tag)));
            }
        }

        if params.is_empty() {
            "/blog".to_string()
        } else {
            format!("/blog?{}", params.join("&"))
        }
    }
}

/// Simple HTML escaping function
fn html_escape(input: &str) -> String {
    input
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")
}

/// Simple URL encoding function
fn url_encode(input: &str) -> String {
    input
        .replace(" ", "%20")
        .replace("&", "%26")
        .replace("=", "%3D")
        .replace("?", "%3F")
        .replace("#", "%23")
        .replace("[", "%5B")
        .replace("]", "%5D")
        .replace("|", "%7C")
        .replace("\\", "%5C")
        .replace("^", "%5E")
        .replace("`", "%60")
        .replace("{", "%7B")
        .replace("}", "%7D")
        .replace("~", "%7E")
}
