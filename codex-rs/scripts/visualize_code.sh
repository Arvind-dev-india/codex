#!/bin/bash

# Code Symbol Graph Visualizer - Easy Usage Script
# This script generates a graph and opens it in your browser

set -e

# Default values
SOURCE_DIR=""
OUTPUT_FILE="visualization/code_graph.json"
OPEN_BROWSER=true

# Function to show usage
show_usage() {
    echo "Usage: $0 <source_directory> [options]"
    echo ""
    echo "Options:"
    echo "  -o, --output FILE    Output JSON file (default: visualization/code_graph.json)"
    echo "  -n, --no-browser     Don't open browser automatically"
    echo "  -h, --help           Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 ./core/src/code_analysis"
    echo "  $0 ./core -o my_graph.json"
    echo "  $0 /path/to/project --no-browser"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -o|--output)
            OUTPUT_FILE="$2"
            shift 2
            ;;
        -n|--no-browser)
            OPEN_BROWSER=false
            shift
            ;;
        -h|--help)
            show_usage
            exit 0
            ;;
        -*)
            echo "Unknown option $1"
            show_usage
            exit 1
            ;;
        *)
            if [ -z "$SOURCE_DIR" ]; then
                SOURCE_DIR="$1"
            else
                echo "Error: Multiple source directories specified"
                show_usage
                exit 1
            fi
            shift
            ;;
    esac
done

# Check if source directory is provided
if [ -z "$SOURCE_DIR" ]; then
    echo "Error: Source directory is required"
    show_usage
    exit 1
fi

# Check if source directory exists
if [ ! -d "$SOURCE_DIR" ]; then
    echo "Error: Source directory '$SOURCE_DIR' does not exist"
    exit 1
fi

# Get the directory of this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Change to project directory
cd "$PROJECT_DIR"

echo "🔍 Analyzing code in: $SOURCE_DIR"
echo "📊 Output file: $OUTPUT_FILE"
echo ""

# Build the project if needed
echo "🔨 Building graph generator..."
cargo build --bin generate_code_graph --quiet

# Generate the graph
echo "📈 Generating symbol graph..."
cargo run --bin generate_code_graph --quiet -- "$SOURCE_DIR" "$OUTPUT_FILE"

# Check if generation was successful
if [ $? -eq 0 ]; then
    echo "✅ Graph generated successfully!"
    
    # Count nodes and edges
    NODES=$(grep -o '"id":' "$OUTPUT_FILE" | wc -l)
    EDGES=$(grep -o '"source":' "$OUTPUT_FILE" | wc -l)
    echo "📊 Found $NODES symbols and $EDGES relationships"
    
    # Open in browser if requested
    if [ "$OPEN_BROWSER" = true ]; then
        echo "🌐 Opening visualization in browser..."
        
        # Get the absolute path to the HTML file
        HTML_FILE="$(pwd)/visualization/index.html"
        
        # Try different commands to open the browser
        if command -v xdg-open > /dev/null; then
            xdg-open "$HTML_FILE"
        elif command -v open > /dev/null; then
            open "$HTML_FILE"
        elif command -v start > /dev/null; then
            start "$HTML_FILE"
        else
            echo "⚠️  Could not automatically open browser."
            echo "   Please open: $HTML_FILE"
        fi
    fi
    
    echo ""
    echo "🎉 Done! To view the graph:"
    echo "   1. Open: $(pwd)/visualization/index.html"
    echo "   2. Click 'Load Graph Data (JSON)'"
    echo "   3. Select: $(pwd)/$OUTPUT_FILE"
    echo ""
    echo "💡 Tips:"
    echo "   - Click nodes to see details and connections"
    echo "   - Drag nodes to rearrange the graph"
    echo "   - Use mouse wheel to zoom"
    echo "   - Use 'Reset View' button to return to original position"
    
else
    echo "❌ Failed to generate graph"
    exit 1
fi