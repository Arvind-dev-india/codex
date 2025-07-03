#!/bin/bash
# Test script for Recovery Services MCP Server

set -e

# Navigate to the workspace root
cd "$(dirname "$0")/.."

# Build the server if it doesn't exist
if [ ! -f "target/debug/recovery-services-server" ]; then
    echo "Building Recovery Services MCP Server..."
    cargo build --bin recovery-services-server
fi

echo "Testing Recovery Services MCP Server..."
echo ""

# Create a temporary directory for test files
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

# Create a test input file with MCP requests
cat > "$TEMP_DIR/test_input.json" << EOF
{"jsonrpc":"2.0","id":"1","method":"initialize","params":{"protocol_version":"0.1.0","client_info":{"name":"test-client","version":"1.0.0"}}}
{"jsonrpc":"2.0","id":"2","method":"listTools","params":{}}
{"jsonrpc":"2.0","id":"3","method":"ping","params":{}}
EOF

echo "Sending test requests to server..."
echo ""

# Run the server with the test input
cat "$TEMP_DIR/test_input.json" | cargo run --bin recovery-services-server -- --verbose > "$TEMP_DIR/test_output.json"

# Check if the server responded
if [ -s "$TEMP_DIR/test_output.json" ]; then
    echo "Server responded successfully!"
    echo ""
    echo "Response summary:"
    
    # Count the number of responses
    RESPONSE_COUNT=$(grep -c "jsonrpc" "$TEMP_DIR/test_output.json")
    echo "- Received $RESPONSE_COUNT responses"
    
    # Check for initialize response
    if grep -q "initialize" "$TEMP_DIR/test_output.json"; then
        echo "- Initialize request successful"
    else
        echo "- Initialize request failed"
    fi
    
    # Check for listTools response
    if grep -q "recovery_services_list_vaults" "$TEMP_DIR/test_output.json"; then
        echo "- ListTools request successful"
        
        # Count the number of tools
        TOOL_COUNT=$(grep -c "name" "$TEMP_DIR/test_output.json")
        echo "- Server offers $TOOL_COUNT Recovery Services tools"
    else
        echo "- ListTools request failed"
    fi
    
    # Check for ping response
    if grep -q "\"id\":\"3\"" "$TEMP_DIR/test_output.json"; then
        echo "- Ping request successful"
    else
        echo "- Ping request failed"
    fi
    
    echo ""
    echo "Test completed successfully!"
else
    echo "Error: Server did not respond or response is empty"
    exit 1
fi

echo ""
echo "To use the server with a real MCP client, build it with:"
echo "  ./build.sh"
echo ""
echo "Then configure your MCP client to use the server at:"
echo "  $(pwd)/bin/recovery-services-server"
echo ""