# Direct Communication

This guide shows how to communicate directly with the Code Analysis Server using stdin/stdout.

## MCP Protocol Basics

The server uses JSON-RPC 2.0 messages. Each message must:
- Be valid JSON
- End with a newline character
- Have a unique `id` for requests

## Step 1: Initialize the Server

First, send an initialization message:

```bash
echo '{"jsonrpc":"2.0","id":"1","method":"initialize","params":{"protocol_version":"2025-03-26"}}' | ./code-analysis-server --project-dir /path/to/project
```

**Expected Response**:
```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "result": {
    "capabilities": {
      "tools": {
        "list_changed": true
      }
    },
    "protocol_version": "2025-03-26",
    "server_info": {
      "name": "code-analysis-server",
      "version": "0.1.0"
    }
  }
}
```

## Step 2: List Available Tools

```bash
echo '{"jsonrpc":"2.0","id":"2","method":"tools/list","params":{}}' | ./code-analysis-server --project-dir /path/to/project
```

**Expected Response**:
```json
{
  "jsonrpc": "2.0",
  "id": "2",
  "result": {
    "tools": [
      {
        "name": "analyze_code",
        "description": "Analyze code structure and extract symbols from a file",
        "input_schema": {
          "type": "object",
          "properties": {
            "file_path": {
              "type": "string",
              "description": "Path to the file to analyze"
            }
          },
          "required": ["file_path"]
        }
      }
    ]
  }
}
```

## Step 3: Call a Tool

```bash
echo '{"jsonrpc":"2.0","id":"3","method":"tools/call","params":{"name":"analyze_code","arguments":{"file_path":"src/main.py"}}}' | ./code-analysis-server --project-dir /path/to/project
```

## Interactive Session

For multiple commands, you can use an interactive session:

```bash
# Start the server
./code-analysis-server --project-dir /path/to/project

# In the same terminal, type commands one by one:
{"jsonrpc":"2.0","id":"1","method":"initialize","params":{"protocol_version":"2025-03-26"}}
{"jsonrpc":"2.0","id":"2","method":"tools/list","params":{}}
{"jsonrpc":"2.0","id":"3","method":"tools/call","params":{"name":"analyze_code","arguments":{"file_path":"src/main.py"}}}
```

Press Ctrl+C to exit.

**Next:** [Using with NPX MCP Tool](07-npx-mcp-tool.md)