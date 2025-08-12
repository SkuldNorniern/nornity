use std::collections::HashMap;

use crate::components::ui::render_component;

/// Preprocess markdown content to replace custom component shortcode syntax with HTML
///
/// Supported syntax examples (non-HTML, markdown-friendly):
/// - [[image src="/static/path.jpg" alt="Alt text" caption="Caption text"]]
/// - [[img src="..." alt="..."]]
/// - [[component image src="..." alt="..." caption="..."]]
pub fn preprocess_markdown_with_components(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'[' && i + 1 < bytes.len() && bytes[i + 1] == b'[' {
            if let Some((consumed, replacement)) = try_parse_bracket_component(&input[i..]) {
                output.push_str(&replacement);
                i += consumed;
                continue;
            }
        }
        output.push(bytes[i] as char);
        i += 1;
    }

    output
}

fn try_parse_bracket_component(s: &str) -> Option<(usize, String)> {
    // Expect prefix "[["
    let rest = s.strip_prefix("[[")?;

    // Parse component name
    let (name_consumed, name) = parse_identifier(rest)?;
    let rest_after_name = &rest[name_consumed..];

    // After name, we expect either whitespace, attributes, then closing "]]"
    let (attrs_consumed, attrs) = parse_attributes_until_closing(rest_after_name, "]] ")
        .or_else(|| parse_attributes_until_closing(rest_after_name, "]]"))?; // tolerate no trailing space before closing

    let html = if name == "component" {
        // Expect a required attribute "name" or first positional word as real component name
        // If attrs has "name", use it; else fallback to empty
        let mut attrs_clone = attrs.clone();
        let real_name = attrs_clone.remove("name").unwrap_or_else(String::new);
        if real_name.is_empty() {
            // If name is missing, we cannot render
            return None;
        }
        render_component(&real_name, &attrs_clone)?
    } else {
        // Direct component name like [[image ...]] or [[img ...]]
        let comp = if name == "img" {
            "image"
        } else {
            name.as_str()
        };
        render_component(comp, &attrs)?
    };

    // Total consumed: "[[" + name + attrs + closing "]]"
    let total_consumed = 2 + name_consumed + attrs_consumed;
    Some((total_consumed, html))
}

fn parse_identifier(s: &str) -> Option<(usize, String)> {
    let mut end = 0;
    for ch in s.chars() {
        if ch.is_alphanumeric() || ch == '-' || ch == '_' {
            end += ch.len_utf8();
        } else {
            break;
        }
    }
    if end == 0 {
        return None;
    }
    Some((end, s[..end].to_string()))
}

fn parse_attributes_until_closing(
    s: &str,
    _closing: &str,
) -> Option<(usize, HashMap<String, String>)> {
    let mut i = 0;
    let bytes = s.as_bytes();
    let mut attrs = HashMap::new();

    // Skip leading whitespace
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }

    loop {
        // Check for closing ]]
        if s[i..].starts_with("]] ") || s[i..].starts_with("]]") {
            // consume closing ]]
            if s[i..].starts_with("]] ") {
                i += 3; // "]] "
            } else {
                i += 2; // "]]"
            }
            break;
        }

        // Parse key
        let start_key = i;
        while i < bytes.len() {
            let c = bytes[i] as char;
            if c.is_alphanumeric() || c == '-' || c == '_' {
                i += 1;
            } else {
                break;
            }
        }
        if i == start_key {
            return None;
        }
        let key = &s[start_key..i];

        // Skip whitespace
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }

        // Expect '='
        if i >= bytes.len() || bytes[i] != b'=' {
            return None;
        }
        i += 1; // consume '='

        // Skip whitespace
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }

        // Parse value (quoted or unquoted)
        let value = if i < bytes.len() && (bytes[i] == b'"' || bytes[i] == b'\'') {
            let quote = bytes[i];
            i += 1; // consume opening quote
            let start_val = i;
            while i < bytes.len() && bytes[i] != quote {
                i += 1;
            }
            let val = &s[start_val..i];
            if i < bytes.len() && bytes[i] == quote {
                i += 1; // consume closing quote
            }
            val.to_string()
        } else {
            // unquoted until whitespace or closing
            let start_val = i;
            while i < bytes.len()
                && !bytes[i].is_ascii_whitespace()
                && !s[i..].starts_with("]] ")
                && !s[i..].starts_with("]]")
            {
                i += 1;
            }
            s[start_val..i].to_string()
        };

        attrs.insert(key.to_string(), value);

        // Skip whitespace and optional commas
        while i < bytes.len() && (bytes[i].is_ascii_whitespace() || bytes[i] == b',') {
            i += 1;
        }
    }

    Some((i, attrs))
}
