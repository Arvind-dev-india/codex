#!/bin/bash

# Test script for Azure DevOps MCP Server
# This script tests basic MCP protocol communication

set -e

echo "Testing Azure DevOps MCP Server..."

# Check if binary exists
BINARY="../target/release/azure-devops-server"
if [ ! -f "$BINARY" ]; then
    echo "Error: Binary not found at $BINARY"
    echo "Please run 'cargo build --release' from the codex-rs directory first"
    exit 1
fi

# Check if config exists
CONFIG_FILE="azure_devops_config.toml"
if [ ! -f "$CONFIG_FILE" ]; then
    echo "Warning: Configuration file $CONFIG_FILE not found"
    echo "The server will try to use environment variables"
fi

echo "Starting server test..."

# Create a temporary file for communication
TEMP_INPUT=$(mktemp)
TEMP_OUTPUT=$(mktemp)

# Cleanup function
cleanup() {
    rm -f "$TEMP_INPUT" "$TEMP_OUTPUT"
    if [ ! -z "$SERVER_PID" ]; then
        kill "$SERVER_PID" 2>/dev/null || true
    fi
}
trap cleanup EXIT

# Test 1: Initialize request
echo "Test 1: Sending initialize request..."
cat > "$TEMP_INPUT" << 'EOF'
{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test-client", "version": "1.0.0"}}}
EOF

# Start server and send initialize request
timeout 10s "$BINARY" ${CONFIG_FILE:+--config "$CONFIG_FILE"} < "$TEMP_INPUT" > "$TEMP_OUTPUT" 2>&1 &
SERVER_PID=$!

# Wait a moment for the server to process
sleep 2

# Check if server is still running
if ! kill -0 "$SERVER_PID" 2>/dev/null; then
    echo "❌ Server exited unexpectedly"
    echo "Server output:"
    cat "$TEMP_OUTPUT"
    exit 1
fi

# Kill the server
kill "$SERVER_PID" 2>/dev/null || true
wait "$SERVER_PID" 2>/dev/null || true
SERVER_PID=""

# Check output
if grep -q "initialize" "$TEMP_OUTPUT"; then
    echo "✅ Server responded to initialize request"
else
    echo "❌ Server did not respond properly to initialize request"
    echo "Server output:"
    cat "$TEMP_OUTPUT"
    exit 1
fi

# Test 2: List tools request
echo "Test 2: Testing list tools request..."
cat > "$TEMP_INPUT" << 'EOF'
{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test-client", "version": "1.0.0"}}}
{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}
EOF

timeout 10s "$BINARY" ${CONFIG_FILE:+--config "$CONFIG_FILE"} < "$TEMP_INPUT" > "$TEMP_OUTPUT" 2>&1 &
SERVER_PID=$!

sleep 2

if ! kill -0 "$SERVER_PID" 2>/dev/null; then
    echo "❌ Server exited unexpectedly during tools/list test"
    echo "Server output:"
    cat "$TEMP_OUTPUT"
    exit 1
fi

kill "$SERVER_PID" 2>/dev/null || true
wait "$SERVER_PID" 2>/dev/null || true
SERVER_PID=""

if grep -q "azure_devops_" "$TEMP_OUTPUT"; then
    echo "✅ Server returned Azure DevOps tools"
else
    echo "❌ Server did not return expected tools"
    echo "Server output:"
    cat "$TEMP_OUTPUT"
    exit 1
fi

echo ""
echo "🎉 Basic server tests passed!"
echo ""
echo "The Azure DevOps MCP Server appears to be working correctly."
echo "You can now use it with MCP-compatible clients."
echo ""
echo "Available tools include:"
echo "  - azure_devops_query_work_items"
echo "  - azure_devops_get_work_item"
echo "  - azure_devops_create_work_item"
echo "  - azure_devops_update_work_item"
echo "  - azure_devops_add_work_item_comment"
echo "  - azure_devops_query_pull_requests"
echo "  - azure_devops_get_pull_request"
echo "  - azure_devops_comment_on_pull_request"
echo "  - azure_devops_get_wiki_page"
echo "  - azure_devops_update_wiki_page"
echo "  - azure_devops_run_pipeline"
echo "  - azure_devops_get_pipeline_status"
echo ""
echo "For full functionality testing, configure your Azure DevOps credentials"
echo "and test with a real MCP client like Claude Desktop."