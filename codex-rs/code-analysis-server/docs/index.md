# Code Analysis Server Documentation

Welcome to the Code Analysis Server documentation. This tool provides code analysis capabilities through the Message Control Protocol (MCP).

## Quick Start

1. [Prerequisites](01-prerequisites.md)
2. [Building for Linux](02-build-linux.md)
3. [Building for Windows](03-build-windows.md)
4. [Basic Usage](04-basic-usage.md)

## Features

- Analyze code structure and symbols
- Find references to symbols across a codebase
- Find symbol definitions
- Generate symbol dependency graphs

## Supported Languages

- Python
- JavaScript/TypeScript
- Rust
- C/C++
- C#
- Java
- Go

## Documentation Contents

1. [Prerequisites](01-prerequisites.md)
2. [Building for Linux](02-build-linux.md)
3. [Building for Windows](03-build-windows.md)
4. [Basic Usage](04-basic-usage.md)
5. [Available Tools](05-available-tools.md)
6. [Direct Communication](06-direct-communication.md)
7. [Using with NPX MCP Tool](07-npx-mcp-tool.md)
8. [Python Client Example](08-python-client.md)
9. [Node.js Client Example](09-nodejs-client.md)
10. [Troubleshooting](10-troubleshooting.md)
11. [Integration Examples](11-integration-examples.md)

## Quick Command Reference

```bash
# Build for Linux
./codex-rs/code-analysis-server/build.sh

# Build for Windows from Linux
./codex-rs/code-analysis-server/build-windows.sh

# Run the server
./code-analysis-server --project-dir /path/to/project

# Use with NPX MCP Tool
npx @modelcontextprotocol/inspector --server-command="./code-analysis-server --project-dir /path/to/project"
```