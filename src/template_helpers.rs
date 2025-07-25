/// Helper function to create a tag HTML string
pub fn render_tags(tags: &[String]) -> String {
    let mut sorted_tags = tags.to_vec();
    sorted_tags.sort();
    sorted_tags
        .iter()
        .map(|tag| format!("<span class=\"tag\"><span class=\"tag-inner\">{tag}</span></span>"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Helper function to render blog post preview
pub fn render_blog_preview(
    slug: &str,
    title: &str,
    date: &str,
    iso_date: &str,
    author: &str,
    excerpt: &str,
    tags: &[String],
) -> String {
    format!(
        r##"
        <article class="blog-post-preview">
            <h2><a href="/blog/{}">{}</a></h2>
            <p class="post-meta">
                <time datetime="{}">{}</time>
                <span class="author">by {}</span>
            </p>
            <p class="excerpt">{}</p>
            <div class="tags">
                {}
            </div>
        </article>
        "##,
        slug,
        title,
        iso_date,
        date,
        author,
        excerpt,
        render_tags(tags)
    )
}
