# Nornity - OS Designer & Developer

A modern, Apple-inspired personal website and blog built with Rust and Axum, featuring dynamic content loading and hot reload capabilities.

## Features

- **Modern Design**: Apple-inspired interface with custom OS designer color palette
- **Dynamic Blog**: Auto-loading blog posts from markdown files with YAML frontmatter
- **Hot Reload**: Automatic content updates when files change
- **Tag Filtering**: Filter blog posts by tags
- **Dark Mode**: Automatic dark mode support based on system preference
- **Responsive**: Mobile-friendly design
- **Performance**: Fast, lightweight, and efficient

## Quick Start

### Prerequisites

- Rust 1.70+ (2024 edition)
- Cargo

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd homepage
```

2. Run the development server:
```bash
# With hot reload (recommended for development)
cargo run --features hot-reload

# Without hot reload (production mode)
cargo run
```

3. Open your browser and visit `http://localhost:5000`

## Blog System

### Creating New Posts

The blog system automatically loads posts from the `content/` directory. Each post should be a markdown file with YAML frontmatter.

#### Using the Helper Script

Create new posts easily with the helper script:

```bash
# Create a new post with current date
./scripts/new_post.sh my-new-post "My New Post Title"

# Or just with a slug (uses default title)
./scripts/new_post.sh my-new-post
```

#### Manual Creation

Create a new markdown file in the `content/` directory with the following structure:

```markdown
---
title: "Your Post Title"
excerpt: "Brief description of the post content."
author: "Nornity"
tags: ["rust", "systems", "design"]
published_at: "2024-01-20 14:30:00"
draft: false
---

# Your Post Title

Your content goes here...
```

### Supported Date Formats

The `published_at` field supports multiple human-readable date formats:

- `"2024-01-20 14:30:00"` - Full datetime
- `"2024-01-20T14:30:00"` - ISO format
- `"2024-01-20T14:30:00Z"` - ISO format with timezone
- `"2024-01-20 14:30"` - Date and time (no seconds)
- `"2024-01-20"` - Date only
- `"January 20, 2024 14:30:00"` - Natural language
- `"January 20, 2024"` - Natural language date
- `"20 January 2024"` - Alternative natural language
- `1705758600` - Unix timestamp (still supported)

### Post Metadata

- **title**: The post title (required)
- **excerpt**: Brief description shown in post lists (required)
- **author**: Post author (required)
- **tags**: Array of tags for categorization (required)
- **published_at**: Publication date in any supported format (required)
- **draft**: Set to `true` to hide the post from public view (optional, defaults to `false`)

### Hot Reload

The blog system automatically detects changes to markdown files in the `content/` directory and reloads them without restarting the server. Simply save your changes and refresh the browser.

**Note:** Hot reload is an optional feature. Use `cargo run --features hot-reload` to enable it during development.

## Project Structure

```
homepage/
├── src/
│   ├── main.rs          # Application entry point
│   ├── routes.rs         # HTTP route handlers
│   ├── blog.rs          # Blog system implementation
│   └── logger.rs        # Custom logging implementation
├── static/
│   └── css/
│       └── style.css    # Main stylesheet
├── content/             # Blog posts (markdown files)
├── scripts/
│   └── new_post.sh     # Helper script for creating posts
└── Cargo.toml          # Rust dependencies
```

## Customization

### Colors

The color scheme is defined in `static/css/style.css` using CSS custom properties:

```css
:root {
    --primary-color: #6667ab;
    --primary-dark: #4b4c7a;
    --secondary-color: #8b8cc7;
    /* ... more colors */
}
```

### Dark Mode

Dark mode is automatically enabled based on system preference. Colors are defined in the `@media (prefers-color-scheme: dark)` section of the CSS.

### Adding New Routes

Add new routes in `src/routes.rs`:

```rust
pub async fn new_route() -> Html<String> {
    // Your route handler
    Html("Your content".to_string())
}
```

Then register the route in `src/main.rs`:

```rust
.route("/new-route", get(new_route))
```

## Development

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

## Deployment

### Local Development

```bash
# With hot reload
cargo run --features hot-reload

# Without hot reload
cargo run
```

### Production

1. Build the release version:
```bash
cargo build --release
```

2. Run the binary:
```bash
./target/release/nornity
```

### Docker (Optional)

Create a `Dockerfile`:

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/nornity /usr/local/bin/
COPY --from=builder /app/content /app/content
COPY --from=builder /app/static /app/static
EXPOSE 5000
CMD ["nornity"]
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [Axum](https://github.com/tokio-rs/axum) web framework
- Styled with modern CSS and Apple-inspired design principles
- Markdown processing with [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) 