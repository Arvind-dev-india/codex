# Code Analysis Server

A standalone MCP (Message Control Protocol) server for code analysis that can be used independently or integrated with Codex.

## Features

- Analyze code structure and extract symbols from files
- Find references to symbols across a codebase
- Find definitions of symbols
- Generate symbol dependency graphs

## Usage

### As a standalone binary

Build and run the server:

```bash
# Build the server
cd codex-rs
cargo build --release -p code-analysis-server

# Run the server on a specific project
./target/release/code-analysis-server --project-dir /path/to/your/project
```

### Integration with other tools

The server uses the MCP protocol over stdin/stdout, making it easy to integrate with other tools:

```bash
# Example of piping a request to the server
echo '{"jsonrpc":"2.0","id":"1","method":"initialize","params":{"protocol_version":"2025-03-26"}}' | ./target/release/code-analysis-server
```

### Available Tools

The server provides the following code analysis tools:

1. `analyze_code` - Analyze code structure and extract symbols from a file
2. `find_symbol_references` - Find all references to a symbol in the codebase
3. `find_symbol_definitions` - Find the definition of a symbol in the codebase
4. `get_symbol_subgraph` - Get a subgraph of symbols related to a specific symbol

## Cross-Platform Support

The code analysis server is designed to be cross-platform and can be built and run on:

- Linux
- macOS
- Windows

## Integration with Codex

The code analysis server uses the same underlying code analysis functionality as Codex, ensuring consistent results across both tools.