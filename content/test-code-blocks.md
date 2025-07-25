---
title: "Enhanced Code Block Styling Demo"
excerpt: "A demonstration of the improved code block styling with syntax highlighting and language detection"
author: "Eira"
tags: ["demo", "styling", "code"]
published_at: "2025-01-16 10:30:00"
draft: false
---

# Enhanced Code Block Styling

This post demonstrates the improved code block styling with automatic language detection, syntax highlighting, and copy functionality.

## Rust Example

Here's a Rust code example:

```rust
use std::collections::HashMap;
use log::{debug, error, info, warn};

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
    /// Create a new blog post metadata
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
}
```

## JavaScript Example

Here's a JavaScript example:

```javascript
// Enhanced code block styling
document.addEventListener('DOMContentLoaded', function() {
    const codeBlocks = document.querySelectorAll('.post-content pre');
    
    codeBlocks.forEach(function(block) {
        // Detect language from class or content
        let language = 'text';
        const codeElement = block.querySelector('code');
        
        if (codeElement) {
            const classList = codeElement.className.split(' ');
            for (const className of classList) {
                if (className.startsWith('language-')) {
                    language = className.replace('language-', '');
                    break;
                }
            }
        }
        
        // Add language indicator
        block.setAttribute('data-language', language);
    });
});
```

## Python Example

Here's a Python example:

```python
import re
from typing import List, Optional

def enhance_code_blocks(html_content: str) -> str:
    """Enhance code blocks with better styling and language detection."""
    
    # Regex to match code blocks with optional language specification
    code_block_regex = re.compile(r'<pre><code(?: class="language-([^"]+)")?>(.*?)</code></pre>', re.DOTALL)
    
    def replace_code_block(match):
        language = match.group(1) or 'text'
        code_content = match.group(2) or ''
        
        # Escape HTML entities in code content
        escaped_content = (code_content
            .replace('&', '&amp;')
            .replace('<', '&lt;')
            .replace('>', '&gt;')
            .replace('"', '&quot;')
            .replace("'", '&#39;'))
        
        return f'<pre data-language="{language}"><code class="language-{language}">{escaped_content}</code></pre>'
    
    return code_block_regex.sub(replace_code_block, html_content)

# Example usage
if __name__ == "__main__":
    sample_html = """
    <pre><code class="language-rust">fn main() { println!("Hello, world!"); }</code></pre>
    """
    enhanced = enhance_code_blocks(sample_html)
    print(enhanced)
```

## CSS Example

Here's a CSS example:

```css
/* Enhanced Code Block Styling */
.post-content pre {
    background: linear-gradient(135deg, var(--bg-tertiary) 0%, var(--bg-secondary) 100%);
    color: var(--text-primary);
    padding: var(--spacing-xl);
    border-radius: var(--radius-xl);
    overflow-x: auto;
    margin: var(--spacing-xl) 0;
    font-family: 'JetBrains Mono', 'Fira Code', 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
    font-size: 0.95rem;
    line-height: 1.7;
    border: 1px solid var(--border-color);
    box-shadow: 
        0 4px 6px -1px rgba(0, 0, 0, 0.1),
        0 2px 4px -2px rgba(0, 0, 0, 0.1),
        inset 0 1px 0 rgba(255, 255, 255, 0.1);
    position: relative;
    backdrop-filter: blur(10px);
    transition: all var(--transition-normal);
}

.post-content pre:hover {
    box-shadow: 
        0 10px 15px -3px rgba(0, 0, 0, 0.1),
        0 4px 6px -4px rgba(0, 0, 0, 0.1),
        inset 0 1px 0 rgba(255, 255, 255, 0.15);
    transform: translateY(-2px);
}
```

## Bash Example

Here's a bash script example:

```bash
#!/bin/bash

# Enhanced code block processing script
echo "Processing code blocks..."

# Find all markdown files
find content/ -name "*.md" -type f | while read -r file; do
    echo "Processing: $file"
    
    # Extract code blocks
    grep -n "```" "$file" | while read -r line; do
        line_num=$(echo "$line" | cut -d: -f1)
        echo "Found code block at line: $line_num"
    done
done

# Function to enhance code blocks
enhance_code_block() {
    local language="$1"
    local content="$2"
    
    echo "Enhancing $language code block..."
    echo "Content length: ${#content} characters"
}

# Example usage
enhance_code_block "rust" "fn main() { println!(\"Hello, world!\"); }"
```

## HTML Example

Here's an HTML example:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Enhanced Code Blocks</title>
    <link rel="stylesheet" href="/css/style.css">
</head>
<body>
    <article class="blog-post">
        <div class="container">
            <header class="post-header">
                <h1>Enhanced Code Block Styling</h1>
                <div class="post-meta">
                    <time datetime="2025-01-16">January 16, 2025</time>
                    <span class="author">by Eira</span>
                </div>
            </header>
            <div class="post-content">
                <p>This demonstrates the improved code block styling.</p>
                <pre data-language="rust"><code class="language-rust">fn main() {
    println!("Hello, enhanced code blocks!");
}</code></pre>
            </div>
        </div>
    </article>
</body>
</html>
```

## Pure CSS Syntax Highlighting Demo

Below is an example of the pure CSS approach (like bou.io) using background gradients and text clipping. This approach doesn't require any HTML spans - the colors are drawn as CSS backgrounds:

```css
/* This code block uses pure CSS highlighting */
/* The colors are applied via CSS background gradients */
/* No HTML spans are needed - just plain text in <pre> */
```

## Features

The enhanced code block styling includes:

1. **Automatic Language Detection**: Detects programming languages from code content
2. **Syntax Highlighting**: Color-coded syntax for better readability (now working!)
3. **Copy Button**: One-click code copying functionality
4. **Language Indicator**: Shows the detected language in the top-right corner
5. **Hover Effects**: Subtle animations and visual feedback
6. **Dark Mode Support**: Optimized styling for both light and dark themes
7. **Responsive Design**: Adapts to different screen sizes
8. **Accessibility**: Proper focus states and keyboard navigation
9. **Pure CSS Option**: Alternative highlighting method using CSS only

## Inline Code

You can also use `inline code` with backticks for short code snippets, which will be styled with a subtle background and border.

The improvements make code blocks more visually appealing and functional without requiring any external JavaScript libraries! 

## How It Works

### Method 1: Rust + CSS Classes (Current Implementation)
- Rust code analyzes the source code and wraps keywords, strings, etc. in `<span class="keyword">` tags
- CSS applies colors to these classes
- Flexible and works with any dynamic content

### Method 2: Pure CSS Background Gradients (Bou.io Style)
- Uses CSS `background: linear-gradient()` with `ch` and `lh` units
- Applies `background-clip: text` and `color: transparent`
- Only practical for static, hand-written code samples
- No HTML manipulation needed 