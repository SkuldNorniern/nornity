#!/bin/bash

# CSS Minification Script
# This script minifies CSS files to improve performance

echo "Starting CSS minification..."

# Check if clean-css-cli is installed
if ! command -v npx &> /dev/null; then
    echo "Error: npx is not installed. Please install Node.js and npm first."
    exit 1
fi

# Minify style.css
echo "Minifying style.css..."
npx clean-css-cli -o static/css/style.min.css static/css/style.css

# Minify code-blocks.css
echo "Minifying code-blocks.css..."
npx clean-css-cli -o static/css/code-blocks.min.css static/css/code-blocks.css

# Show file size comparison
echo ""
echo "File size comparison:"
echo "===================="
ls -lh static/css/*.css

echo ""
echo "CSS minification completed successfully!"
echo "Remember to update version numbers in templates and handlers when deploying." 