#!/bin/bash

# Build the project first
cd "$(dirname "$0")/.."
echo "Building codex-core..."
cargo build

# Create the visualization directory if it doesn't exist
mkdir -p visualization

# Copy the HTML and JS files
cp scripts/graph_visualizer.html visualization/index.html
cp scripts/graph_visualizer.js visualization/graph_visualizer.js

echo "Build complete!"
echo ""
echo "To generate a graph:"
echo "1. Run: cargo run --bin generate_code_graph <source_directory> visualization/code_graph.json"
echo "2. Open visualization/index.html in a web browser to view the graph"
echo ""
echo "Example:"
echo "cargo run --bin generate_code_graph ./core/src/code_analysis visualization/code_graph.json"