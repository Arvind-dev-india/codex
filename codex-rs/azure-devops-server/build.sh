#!/bin/bash

# Build script for Azure DevOps MCP Server

set -e

echo "Building Azure DevOps MCP Server..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "Error: Cargo.toml not found. Please run this script from the azure-devops-server directory."
    exit 1
fi

# Build the project
echo "Running cargo build --release..."
cargo build --release

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "Binary location: target/release/azure-devops-server"
    echo ""
    echo "To run the server:"
    echo "  ./target/release/azure-devops-server --config azure_devops_config.toml"
    echo ""
    echo "Or with environment variables:"
    echo "  export AZURE_DEVOPS_ORG='your-org'"
    echo "  export AZURE_DEVOPS_PAT='your-token'"
    echo "  ./target/release/azure-devops-server"
else
    echo "❌ Build failed!"
    exit 1
fi