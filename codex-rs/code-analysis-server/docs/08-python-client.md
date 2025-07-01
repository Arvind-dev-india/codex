# Python Client Example

This guide shows how to create a simple Python client for the Code Analysis Server.

## Basic Python Client

Save this as `code_analysis_client.py`:

```python
#!/usr/bin/env python3
import json
import subprocess
import sys
import argparse

class CodeAnalysisClient:
    def __init__(self, server_path, project_dir):
        """Initialize the client with server path and project directory."""
        self.server_process = subprocess.Popen(
            [server_path, '--project-dir', project_dir],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=1  # Line buffered
        )
        # Initialize the server
        self.initialize()
    
    def send_request(self, method, params):
        """Send a request to the server and get the response."""
        request_id = f"req-{id(params)}"
        request = {
            "jsonrpc": "2.0",
            "id": request_id,
            "method": method,
            "params": params
        }
        
        # Send the request
        self.server_process.stdin.write(json.dumps(request) + '\n')
        self.server_process.stdin.flush()
        
        # Read the response
        response = self.server_process.stdout.readline()
        return json.loads(response)
    
    def initialize(self):
        """Initialize the server."""
        return self.send_request("initialize", {
            "protocol_version": "2025-03-26"
        })
    
    def list_tools(self):
        """List available tools."""
        return self.send_request("tools/list", {})
    
    def analyze_code(self, file_path):
        """Analyze a specific file."""
        return self.send_request("tools/call", {
            "name": "analyze_code",
            "arguments": {
                "file_path": file_path
            }
        })
    
    def find_references(self, symbol_name, symbol_type=None):
        """Find references to a symbol."""
        arguments = {"symbol_name": symbol_name}
        if symbol_type:
            arguments["symbol_type"] = symbol_type
            
        return self.send_request("tools/call", {
            "name": "find_symbol_references",
            "arguments": arguments
        })
    
    def find_definitions(self, symbol_name, symbol_type=None):
        """Find definitions of a symbol."""
        arguments = {"symbol_name": symbol_name}
        if symbol_type:
            arguments["symbol_type"] = symbol_type
            
        return self.send_request("tools/call", {
            "name": "find_symbol_definitions",
            "arguments": arguments
        })
    
    def get_symbol_subgraph(self, symbol_name, depth=2):
        """Get a subgraph for a symbol."""
        return self.send_request("tools/call", {
            "name": "get_symbol_subgraph",
            "arguments": {
                "symbol_name": symbol_name,
                "depth": depth
            }
        })
    
    def close(self):
        """Close the server process."""
        self.server_process.terminate()
        self.server_process.wait()

def main():
    parser = argparse.ArgumentParser(description='Code Analysis Client')
    parser.add_argument('--server', required=True, help='Path to the code-analysis-server binary')
    parser.add_argument('--project', required=True, help='Project directory to analyze')
    parser.add_argument('--action', required=True, 
                        choices=['list-tools', 'analyze', 'find-refs', 'find-defs', 'subgraph'],
                        help='Action to perform')
    parser.add_argument('--file', help='File to analyze (for analyze action)')
    parser.add_argument('--symbol', help='Symbol name (for find-refs, find-defs, subgraph actions)')
    parser.add_argument('--type', help='Symbol type (optional)')
    parser.add_argument('--depth', type=int, default=2, help='Subgraph depth (default: 2)')
    
    args = parser.parse_args()
    
    client = CodeAnalysisClient(args.server, args.project)
    
    try:
        if args.action == 'list-tools':
            result = client.list_tools()
            print(json.dumps(result, indent=2))
        
        elif args.action == 'analyze':
            if not args.file:
                print("Error: --file is required for analyze action")
                return 1
            result = client.analyze_code(args.file)
            print(json.dumps(result, indent=2))
        
        elif args.action == 'find-refs':
            if not args.symbol:
                print("Error: --symbol is required for find-refs action")
                return 1
            result = client.find_references(args.symbol, args.type)
            print(json.dumps(result, indent=2))
        
        elif args.action == 'find-defs':
            if not args.symbol:
                print("Error: --symbol is required for find-defs action")
                return 1
            result = client.find_definitions(args.symbol, args.type)
            print(json.dumps(result, indent=2))
        
        elif args.action == 'subgraph':
            if not args.symbol:
                print("Error: --symbol is required for subgraph action")
                return 1
            result = client.get_symbol_subgraph(args.symbol, args.depth)
            print(json.dumps(result, indent=2))
    
    finally:
        client.close()
    
    return 0

if __name__ == "__main__":
    sys.exit(main())
```

## Usage Examples

```bash
# Make the script executable
chmod +x code_analysis_client.py

# List available tools
./code_analysis_client.py --server ./code-analysis-server --project /path/to/project --action list-tools

# Analyze a file
./code_analysis_client.py --server ./code-analysis-server --project /path/to/project --action analyze --file src/main.py

# Find references to a symbol
./code_analysis_client.py --server ./code-analysis-server --project /path/to/project --action find-refs --symbol UserService --type class

# Find definitions of a symbol
./code_analysis_client.py --server ./code-analysis-server --project /path/to/project --action find-defs --symbol process_data

# Get symbol subgraph
./code_analysis_client.py --server ./code-analysis-server --project /path/to/project --action subgraph --symbol DatabaseManager --depth 3
```

## Error Handling

The client handles basic communication but doesn't have extensive error handling. For production use, consider adding:

1. Timeout handling
2. Retry logic
3. Better error messages
4. Logging

**Next:** [Node.js Client Example](09-nodejs-client.md)