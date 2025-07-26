use crate::blog::BlogStore;
use crate::components::code_block::process_markdown_content;
use crate::components::search::{SearchComponent, SearchResults, SearchResult, SortOption};
use crate::components::sitemap::generate_sitemap_xml;
use crate::template_helpers::{render_blog_preview, render_tags};
use crate::templates::TemplateEngine;
use axum::{
    extract::{Path, Query},
    http::{StatusCode, header},
    response::{Html, Response},
};
use log::{debug, error, info, warn};
use std::collections::HashMap;

/// Homepage handler
pub async fn homepage() -> Html<String> {
    info!("Serving homepage request");
    debug!("Homepage route accessed");

    let blog_store = get_blog_store();
    let recent_posts = blog_store.get_recent_posts(3);
    debug!("Retrieved {} recent posts for homepage", recent_posts.len());

    let posts_html = recent_posts
        .iter()
        .map(|post| {
            debug!("Processing post for homepage: {}", post.slug);
            render_blog_preview(
                &post.slug,
                post.title(),
                &post.formatted_date(),
                &post.iso_date(),
                post.author(),
                post.excerpt(),
                post.tags(),
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    debug!("Generated homepage HTML with {} posts", recent_posts.len());
    info!("Homepage served successfully");

    let template_engine = get_template_engine();
    let mut variables = HashMap::new();
    variables.insert("recent_posts".to_string(), posts_html);

    match template_engine.render("homepage.html", &variables) {
        Ok(content) => {
            match template_engine.render_base_with_meta(
                "Nornity - OS Designer & Developer",
                &content,
                "OS Designer & Developer specializing in systems programming, compiler design, and low-level development. Explore my projects, blog posts, and technical insights."
            ) {
                Ok(html) => Html(html),
                Err(e) => {
                    error!("Failed to render base template: {e}");
                    Html(format!("<h1>Error</h1><p>Failed to render page: {e}</p>"))
                }
            }
        }
        Err(e) => {
            error!("Failed to render homepage template: {e}");
            Html(format!("<h1>Error</h1><p>Failed to render page: {e}</p>"))
        }
    }
}

/// Blog list handler with server-side search and tag filtering
pub async fn blog_list(Query(params): Query<HashMap<String, String>>, req: axum::http::Request<axum::body::Body>) -> Html<String> {
    info!("Serving blog list page");
    debug!("Blog list query parameters: {params:?}");

    // Get the raw query string to handle multiple tag parameters
    let query_string = req.uri().query().unwrap_or("");
    debug!("Raw query string: {}", query_string);

    // Parse search configuration from raw query string to handle multiple tags
    let search_config = SearchComponent::parse_params_from_raw_query(query_string);
    debug!("Search config: {:?}", search_config);

    let blog_store = get_blog_store();
    let total_posts = blog_store.get_all_posts();
    let total_count = total_posts.len();

    // Apply filters
    let mut posts = if !search_config.selected_tags.is_empty() {
        info!("Filtering posts by tags: {:?}", search_config.selected_tags);
        
        // For multiple tags, we need to find posts that have ALL selected tags
        let mut filtered_posts = total_posts;
        for tag in &search_config.selected_tags {
            let tag_posts = blog_store.get_posts_by_tag(tag);
            // Keep only posts that exist in both current filtered and tag posts
            filtered_posts.retain(|post| tag_posts.iter().any(|tag_post| tag_post.slug == post.slug));
        }
        
        debug!("Found {} posts with all tags {:?}", filtered_posts.len(), search_config.selected_tags);
        filtered_posts
    } else {
        debug!("No tag filter applied, getting all posts");
        total_posts
    };

    // Apply search filter if provided
    if let Some(ref search_term) = search_config.search_term {
        info!("Filtering posts by search term: {search_term}");
        let search_lower = search_term.to_lowercase();
        posts.retain(|post| {
            post.title().to_lowercase().contains(&search_lower)
                || post.excerpt().to_lowercase().contains(&search_lower)
                || post
                    .tags()
                    .iter()
                    .any(|tag| tag.to_lowercase().contains(&search_lower))
        });
        debug!("Found {} posts matching search term", posts.len());
    }

    // Apply sorting
    posts.sort_by(|a, b| match search_config.sort_by {
        SortOption::DateNewest => b.meta.published_at.cmp(&a.meta.published_at),
        SortOption::DateOldest => a.meta.published_at.cmp(&b.meta.published_at),
        SortOption::Title => a.title().cmp(&b.title()),
        SortOption::Relevance => {
            // For relevance, we'll keep the current order (search results first)
            std::cmp::Ordering::Equal
        }
    });

    let filtered_count = posts.len();

    // Convert to search results
    let search_results = SearchResults {
        posts: posts
            .iter()
            .map(|post| SearchResult {
                slug: post.slug.clone(),
                title: post.title().to_string(),
                excerpt: post.excerpt().to_string(),
                date: post.formatted_date(),
                iso_date: post.iso_date(),
                author: post.author().to_string(),
                tags: post.tags().to_vec(),
                relevance_score: 1.0, // Simple relevance for now
            })
            .collect(),
        total_count,
        filtered_count,
        search_term: search_config.search_term.clone(),
        selected_tags: search_config.selected_tags.clone(),
    };

    // Generate HTML using search component
    let posts_html = search_results
        .posts
        .iter()
        .map(|result| {
            debug!("Processing search result: {}", result.slug);
            render_blog_preview(
                &result.slug,
                &result.title,
                &result.date,
                &result.iso_date,
                &result.author,
                &result.excerpt,
                &result.tags,
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Get all tags for the tag cloud
    let mut all_tags = blog_store.get_all_tags();
    all_tags.sort();

    // Generate search form and tag cloud using the search component
    let search_form = SearchComponent::render_search_form(&search_config);
    let tag_cloud = SearchComponent::render_tag_cloud(&all_tags, &search_config.selected_tags);
    let search_summary = SearchComponent::render_search_summary(&search_results);

    debug!("Generated blog list HTML with {} posts", search_results.filtered_count);
    info!("Blog list page served successfully");

    let template_engine = get_template_engine();
    let mut variables = HashMap::new();
    variables.insert("search_form".to_string(), search_form);
    variables.insert("tag_cloud".to_string(), tag_cloud);
    variables.insert("search_summary".to_string(), search_summary);
    variables.insert("blog_posts".to_string(), posts_html);

    match template_engine.render("blog_list.html", &variables) {
        Ok(content) => match template_engine.render_base_with_meta(
            "Blog",
            &content,
            "Read my latest thoughts on systems programming, compiler design, OS development, and technical insights. Browse articles by tags or search for specific topics."
        ) {
            Ok(html) => Html(html),
            Err(e) => {
                error!("Failed to render base template: {e}");
                Html(format!("<h1>Error</h1><p>Failed to render page: {e}</p>"))
            }
        },
        Err(e) => {
            error!("Failed to render blog list template: {e}");
            Html(format!("<h1>Error</h1><p>Failed to render page: {e}</p>"))
        }
    }
}

/// Individual blog post handler
pub async fn blog_post(Path(slug): Path<String>) -> Result<Html<String>, StatusCode> {
    info!("Serving blog post: {slug}");
    debug!("Blog post request for slug: {slug}");

    let blog_store = get_blog_store();
    let post = match blog_store.get_post_by_slug(&slug) {
        Some(p) => {
            debug!("Found post: {} (title: {})", slug, p.title());
            p
        }
        None => {
            warn!("Blog post not found: {slug}");
            return Err(StatusCode::NOT_FOUND);
        }
    };

    // Process markdown content with enhanced code blocks
    debug!("Rendering markdown content for post: {slug}");
    let html_content = process_markdown_content(&post.content);
    debug!("Enhanced HTML content length: {} chars", html_content.len());
    debug!(
        "Markdown rendered successfully, content length: {} chars",
        html_content.len()
    );

    let template_engine = get_template_engine();
    let mut variables = HashMap::new();
    variables.insert("post_title".to_string(), post.title().to_string());
    variables.insert("post_date".to_string(), post.formatted_date());
    variables.insert("post_iso_date".to_string(), post.iso_date());
    variables.insert("post_author".to_string(), post.author().to_string());
    variables.insert("post_content".to_string(), html_content);
    variables.insert("post_tags".to_string(), render_tags(post.tags()));

    match template_engine.render("blog_post.html", &variables) {
        Ok(content) => match template_engine.render_base_with_meta_and_css(
            post.title(),
            &content,
            post.excerpt(),
            Some("/static/css/code-blocks.min.css?v=3"),
        ) {
            Ok(html) => Ok(Html(html)),
            Err(e) => {
                error!("Failed to render base template: {e}");
                Ok(Html(format!(
                    "<h1>Error</h1><p>Failed to render page: {e}</p>"
                )))
            }
        },
        Err(e) => {
            error!("Failed to render blog post template: {e}");
            Ok(Html(format!(
                "<h1>Error</h1><p>Failed to render page: {e}</p>"
            )))
        }
    }
}

/// Sitemap XML handler
pub async fn sitemap() -> Result<Response<String>, StatusCode> {
    info!("Serving sitemap.xml request");
    debug!("Sitemap route accessed");

    let blog_store = get_blog_store();
    
    // Get base URL from config (loaded from file/env/defaults)
    let base_url = get_config().base_url.clone();
    
    match generate_sitemap_xml(&base_url, blog_store) {
        Ok(xml) => {
            info!("Sitemap generated successfully");
            debug!("Sitemap XML length: {} chars", xml.len());
            
            let response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
                .body(xml)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            Ok(response)
        }
        Err(e) => {
            error!("Failed to generate sitemap: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Robots.txt handler
pub async fn robots_txt() -> Result<Response<String>, StatusCode> {
    info!("Serving robots.txt request");
    debug!("Robots.txt route accessed");

    // Get base URL from config (loaded from file/env/defaults)
    let base_url = get_config().base_url.clone();
    
    let robots_content = format!(
        "User-agent: *\n\
         Allow: /\n\
         \n\
         Sitemap: {}/sitemap.xml\n",
        base_url
    );
    
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
        .body(robots_content)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(response)
}

// Helper functions to access global instances
fn get_blog_store() -> &'static BlogStore {
    crate::routes::get_blog_store()
}

fn get_template_engine() -> &'static TemplateEngine {
    crate::routes::get_template_engine()
}

fn get_config() -> &'static crate::config::Config {
    crate::routes::get_config()
}
