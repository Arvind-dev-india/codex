#!/bin/bash
# Build script for Recovery Services MCP Server

set -e

# Navigate to the workspace root
cd "$(dirname "$0")/.."

# Build the recovery-services-server
echo "Building Recovery Services MCP Server..."
cargo build --release --bin recovery-services-server

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "Build successful!"
    echo "Binary available at: target/release/recovery-services-server"
else
    echo "Build failed!"
    exit 1
fi

# Create a directory for the binary if it doesn't exist
mkdir -p bin

# Copy the binary to the bin directory
cp target/release/recovery-services-server bin/
echo "Binary copied to bin/recovery-services-server"

echo ""
echo "Usage:"
echo "  ./bin/recovery-services-server                         # Use default config"
echo "  ./bin/recovery-services-server --config config.toml    # Use specific config"
echo "  ./bin/recovery-services-server --verbose               # Enable verbose logging"
echo ""