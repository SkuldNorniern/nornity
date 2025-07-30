# Nornity - OS Designer & Developer

A modern, Apple-inspired personal website and blog built with Rust and Axum.

## Features

- **Dynamic Blog**: Auto-loading blog posts from markdown files
- **Syntax Highlighting**: Custom highlighting for Rust, JavaScript, Python, CSS, Bash, HTML
- **Hot Reload**: Automatic content updates during development
- **Server-side Search**: Full-text search and tag filtering
- **Dark Mode**: Automatic dark mode support
- **SEO Ready**: Built-in sitemap.xml, robots.txt, and RSS feed
- **Configuration**: Flexible config via files or environment variables

## Quick Start

### Prerequisites

- Rust 1.70+ (2024 edition)

### Installation

1. Clone and run:
```bash
git clone <repository-url>
cd nornity
cargo run --features hot-reload
```

2. Visit `http://localhost:5000`

## Blog System

### Creating Posts

Create markdown files in the `content/` directory:

```markdown
---
title: "Your Post Title"
excerpt: "Brief description"
author: "Nornity"
tags: ["rust", "systems"]
published_at: "2024-01-20 14:30:00"
draft: false
---

# Your Post Title

Your content here...

```rust
fn main() {
    println!("Hello, world!");
}
```
```

### Supported Date Formats

- `"2024-01-20 14:30:00"` - Full datetime
- `"2024-01-20"` - Date only
- `"January 20, 2024"` - Natural language
- `1705758600` - Unix timestamp

### Hot Reload

Changes to markdown files automatically reload. Use `cargo run --features hot-reload` for development.

## Configuration

### Files

Create `config.toml`:

```toml
host = "127.0.0.1"
port = 5000
static_dir = "static"
content_dir = "content"
base_url = "https://nornity.com"
```

### Environment Variables

```bash
export HOST="127.0.0.1"
export PORT="5000"
export BASE_URL="https://nornity.com"
```

## Project Structure

```
nornity/
├── src/
│   ├── main.rs              # Entry point
│   ├── handlers.rs          # HTTP handlers
│   ├── blog.rs              # Blog system
│   ├── config.rs            # Configuration
│   └── components/          # Syntax highlighting, search, sitemap
├── static/css/              # Stylesheets
├── templates/               # HTML templates
├── content/                 # Blog posts
└── Cargo.toml
```

## Customization

### Colors

Edit `static/css/style.css`:

```css
:root {
    --primary-color: #6667ab;
    --primary-dark: #4b4c7a;
    --secondary-color: #8b8cc7;
    --accent-color: #edbedc;
}
```

### Adding Routes

Add handlers in `src/handlers.rs` and register in `src/server.rs`.

## Development

```bash
cargo build
cargo test
cargo fmt
```

## Deployment

1. Build release:
```bash
cargo build --release
```

2. Run with environment:
```bash
export HOST="0.0.0.0"
export BASE_URL="https://yourdomain.com"
./target/release/nornity
```