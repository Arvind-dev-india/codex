#!/bin/bash
# Build script for Kusto MCP Server

set -e

# Navigate to the workspace root
cd "$(dirname "$0")/.."

# Build the kusto-server
echo "Building Kusto MCP Server..."
cargo build --release --bin kusto-server

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "Build successful!"
    echo "Binary available at: target/release/kusto-server"
else
    echo "Build failed!"
    exit 1
fi

# Create a directory for the binary if it doesn't exist
mkdir -p bin

# Copy the binary to the bin directory
cp target/release/kusto-server bin/
echo "Binary copied to bin/kusto-server"

echo ""
echo "Usage:"
echo "  ./bin/kusto-server                         # Use default config"
echo "  ./bin/kusto-server --config config.toml    # Use specific config"
echo "  ./bin/kusto-server --verbose               # Enable verbose logging"
echo ""