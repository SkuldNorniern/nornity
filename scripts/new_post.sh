#!/bin/bash

# Script to create a new blog post with current date

if [ $# -eq 0 ]; then
    echo "Usage: $0 <post-slug> [title]"
    echo "Example: $0 my-new-post \"My New Post Title\""
    exit 1
fi

SLUG=$1
TITLE=${2:-"New Blog Post"}

# Get current date in various formats
CURRENT_DATE=$(date '+%Y-%m-%d %H:%M:%S')
CURRENT_DATE_SIMPLE=$(date '+%Y-%m-%d')

# Create the content directory if it doesn't exist
mkdir -p content

# Create the new post file
POST_FILE="content/${SLUG}.md"

if [ -f "$POST_FILE" ]; then
    echo "Error: Post file $POST_FILE already exists!"
    exit 1
fi

# Generate the frontmatter
cat > "$POST_FILE" << EOF
---
title: "$TITLE"
excerpt: "Brief description of the post content."
author: "Nornity"
tags: ["rust", "systems", "design"]
published_at: "$CURRENT_DATE"
draft: false
---

# $TITLE

Your content goes here...

## Introduction

Start your blog post here...

## Main Content

Add your main content sections...

## Conclusion

Wrap up your thoughts...

EOF

echo "Created new blog post: $POST_FILE"
echo "Current date: $CURRENT_DATE"
echo ""
echo "You can now edit the file and add your content!"
echo "The blog will automatically reload when you save changes." 