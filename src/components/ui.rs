use std::collections::HashMap;

/// Render a responsive image with optional caption
pub fn render_responsive_image(attributes: &HashMap<String, String>) -> String {
    let src = attributes.get("src").cloned().unwrap_or_default();
    let alt = attributes.get("alt").cloned().unwrap_or_default();
    let caption = attributes.get("caption").cloned();

    let loading = attributes.get("loading").cloned().unwrap_or_else(|| "lazy".to_string());
    let decoding = attributes.get("decoding").cloned().unwrap_or_else(|| "async".to_string());

    let img = format!(
        r#"<img src="{}" alt="{}" loading="{}" decoding="{}" />"#,
        src, alt, loading, decoding
    );

    match caption {
        Some(c) if !c.trim().is_empty() => format!(
            r#"<figure class="image-figure">{}<figcaption>{}</figcaption></figure>"#,
            img, c
        ),
        _ => format!(r#"<figure class="image-figure">{}</figure>"#, img),
    }
}

/// Render a component by name with attributes
pub fn render_component(name: &str, attributes: &HashMap<String, String>) -> Option<String> {
    match name {
        "image" | "img" => Some(render_responsive_image(attributes)),
        _ => None,
    }
} 