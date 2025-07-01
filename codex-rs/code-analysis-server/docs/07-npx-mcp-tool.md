# Using with NPX MCP Tool

The Model Context Protocol (MCP) Inspector is a command-line tool that makes it easy to interact with MCP servers like the Code Analysis Server.

## About the MCP Inspector

The MCP Inspector (`@modelcontextprotocol/inspector`) is an official tool for interacting with MCP-compatible servers. It provides:

- Interactive command-line interface
- Scripting capabilities
- Server management
- Request/response visualization

## Installation

You don't need to install anything globally. Use `npx` to run the tool directly:

```bash
# Check if the tool is available
npx @modelcontextprotocol/inspector --help
```

## Starting the Inspector with Code Analysis Server

### Method 1: Direct Server Command

Start the MCP Inspector with the server command:

```bash
# Linux/macOS
npx @modelcontextprotocol/inspector --server-command="./code-analysis-server --project-dir /path/to/project"

# Windows
npx @modelcontextprotocol/inspector --server-command="code-analysis-server.exe --project-dir C:\path\to\project"
```

### Method 2: Using Configuration File

Create an MCP configuration file:

**mcp-config.json**:
```json
{
  "servers": {
    "code-analysis": {
      "command": "./code-analysis-server",
      "args": ["--project-dir", "/path/to/project"]
    }
  }
}
```

Then run:
```bash
npx @modelcontextprotocol/inspector --config mcp-config.json --server code-analysis
```

### Method 3: Connect to Already Running Server

If you've already started the Code Analysis Server in another terminal:

```bash
# First terminal: Start the server
./code-analysis-server --project-dir /path/to/project

# Second terminal: Connect to it using its process ID
npx @modelcontextprotocol/inspector --server-pid=$(pgrep code-analysis)
```

## Using the Inspector with Code Analysis Tools

Once the Inspector is running and connected to the server, you'll see a prompt. Here's how to use it with the Code Analysis Server:

### 1. Initialize the Server

The Inspector automatically initializes the server, but you can do it manually:

```
> initialize
```

### 2. List Available Tools

```
> tools/list
```

This will show the four available tools: `analyze_code`, `find_symbol_references`, `find_symbol_definitions`, and `get_symbol_subgraph`.

### 3. Analyze a File

```
> tools/call analyze_code {"file_path": "/path/to/file.py"}
```

The Inspector will format and display the results in a readable way.

### 4. Find References to a Symbol

```
> tools/call find_symbol_references {"symbol_name": "MyClass"}
```

You can also specify the symbol type:

```
> tools/call find_symbol_references {"symbol_name": "MyClass", "symbol_type": "class"}
```

### 5. Find Symbol Definitions

```
> tools/call find_symbol_definitions {"symbol_name": "process_data"}
```

### 6. Get Symbol Dependency Graph

```
> tools/call get_symbol_subgraph {"symbol_name": "UserService", "depth": 2}
```

## Advanced Inspector Features

### Scripted Usage

Create a script file with commands:

**code-analysis-commands.txt**:
```
tools/list
tools/call analyze_code {"file_path": "src/main.py"}
tools/call find_symbol_references {"symbol_name": "User"}
```

Run the script:
```bash
npx @modelcontextprotocol/inspector --server-command="./code-analysis-server --project-dir ." --script code-analysis-commands.txt
```

### Saving Results to Files

You can redirect output to files:

```bash
# Save analysis results to a file
npx @modelcontextprotocol/inspector --server-command="./code-analysis-server --project-dir ." \
  --script <(echo 'tools/call analyze_code {"file_path": "src/main.py"}') \
  --output analysis-results.json
```

### Batch Processing

Process multiple files:

```bash
# Create a script that analyzes multiple files
cat > analyze-files.txt << EOF
tools/call analyze_code {"file_path": "src/main.py"}
tools/call analyze_code {"file_path": "src/utils.py"}
tools/call analyze_code {"file_path": "src/models.py"}
EOF

# Run the batch script
npx @modelcontextprotocol/inspector --server-command="./code-analysis-server --project-dir ." --script analyze-files.txt
```

### Interactive Help

Inside the Inspector, you can get help:

```
> help
> help tools/call
```

**Next:** [Python Client Example](08-python-client.md)