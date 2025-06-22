# Code Analysis Tools

This module provides code analysis tools using Tree-sitter for parsing and generating code reference graphs.

## Overview

The code analysis tools allow the LLM to analyze code and understand the relationships between different parts of the codebase. This can be very helpful for tasks like code review, refactoring, and documentation.

The implementation is inspired by the Tree-sitter parsing functionality from the AzureCodeReviewExtension project.

## Components

### Parser Pool

The `parser_pool.rs` module manages Tree-sitter parsers for different languages. It provides:

- Incremental parsing to efficiently update the parse tree when files change
- Caching of parsed files to avoid unnecessary reparsing
- Support for multiple programming languages
- Loading and executing Tree-sitter queries
- Predefined queries for common code analysis tasks

#### Tree-sitter Queries

Tree-sitter queries are stored in the `queries` directory. Each language has its own query file:

- `rust.scm`: Queries for Rust code
- `python.scm`: Queries for Python code
- `javascript.scm`: Queries for JavaScript code
- `typescript.scm`: Queries for TypeScript code
- `csharp.scm`: Queries for C# code
- `cpp.scm`: Queries for C++ code
- `java.scm`: Queries for Java code
- `go.scm`: Queries for Go code

These queries are used to extract information about functions, classes, methods, and other symbols from the code.

### Context Extractor

The `context_extractor.rs` module extracts code context from parsed files. It provides:

- Extraction of code symbols (functions, classes, methods, etc.)
- Tracking of symbol references
- Incremental updates when files change

### Repository Mapper

The `repo_mapper.rs` module maps the repository structure and builds a code reference graph. It provides:

- Scanning of the repository to find all code files
- Building a graph of code references and dependencies
- BFS traversal to limit the size of the graph
- Incremental updates when files change

### Tools

The `tools.rs` module defines the OpenAI tools for code analysis. It provides the following tools:

#### analyze_code

Analyzes the code in a file and returns information about functions, classes, and other symbols.

**Parameters:**
- `file_path`: The path to the file to analyze

**Returns:**
- Information about the symbols in the file

#### find_symbol_references

Finds all references to a symbol (function, class, variable, etc.) in the codebase.

**Parameters:**
- `symbol_name`: The name of the symbol to find references for

**Returns:**
- A list of references to the symbol

#### find_symbol_definitions

Finds the definition of a symbol (function, class, variable, etc.) in the codebase.

**Parameters:**
- `symbol_name`: The name of the symbol to find the definition for

**Returns:**
- Information about the definition of the symbol

#### get_code_graph

Generates a graph of code references and dependencies.

**Parameters:**
- `root_path`: The root path of the repository
- `include_files` (optional): A list of files to include in the graph
- `exclude_patterns` (optional): A list of patterns to exclude from the graph

**Returns:**
- A graph of code references and dependencies

#### get_symbol_subgraph

Generates a subgraph of code references starting from a specific symbol, with a maximum traversal depth.

**Parameters:**
- `symbol_name`: The name of the symbol to start the subgraph from
- `max_depth`: The maximum depth to traverse in the graph

**Returns:**
- A subgraph of code references

#### update_code_graph

Updates the code graph by re-parsing any files that have changed since the last parse.

**Parameters:**
- None

**Returns:**
- Status of the update operation

## Usage

The code analysis tools are automatically registered with the OpenAI tools system when the codex-rs project starts. The LLM can then use these tools to analyze code and understand the relationships between different parts of the codebase.

### Registering with MCP Server

To make the code analysis tools available to the MCP server, you need to register them. This is done automatically when the codex-rs project starts, but if you're developing a custom MCP server, you'll need to register them manually.

Here's how to register the code analysis tools with your MCP server:

```rust
use codex_core::code_analysis::register_code_analysis_tools;

// Register the code analysis tools
let code_analysis_tools = register_code_analysis_tools();

// Add the tools to your MCP server's tool registry
for tool in code_analysis_tools {
    server.register_tool(tool);
}
```

### Testing with MCP Inspector

The code analysis tools are implemented as MCP (Model Context Protocol) tools, which means they can be tested using the MCP inspector. Here's how to test them:

1. **Start the MCP Server**

   First, you need to start the MCP server with the code analysis tools registered:

   ```bash
   cargo run --bin mcp-server
   ```

2. **Use the Claude MCP Inspector**

   The easiest way to test the tools is to use the Claude MCP Inspector:

   ```bash
   npx @modelcontextprotocol/inspector
   ```

   This will open a web interface where you can:
   - Connect to your MCP server
   - Browse available tools
   - Execute tool calls
   - View the results in a user-friendly format

3. **Configure the Inspector**

   In the MCP Inspector:
   - Set the MCP server URL (typically `http://localhost:8080`)
   - Click "Connect" to connect to your MCP server
   - You should see the code analysis tools listed in the available tools

4. **Alternative: Use the MCP Client**

   If you prefer a command-line interface, you can use the MCP client:

   ```bash
   cargo run --bin mcp-client
   ```

3. **Test Individual Tools**

   Here are some examples of how to test each tool:

   **Analyze Code**:
   ```json
   {
     "tool": "analyze_code",
     "arguments": {
       "file_path": "src/main.rs"
     }
   }
   ```

   **Find Symbol References**:
   ```json
   {
     "tool": "find_symbol_references",
     "arguments": {
       "symbol_name": "main"
     }
   }
   ```

   **Find Symbol Definitions**:
   ```json
   {
     "tool": "find_symbol_definitions",
     "arguments": {
       "symbol_name": "main"
     }
   }
   ```

   **Get Code Graph**:
   ```json
   {
     "tool": "get_code_graph",
     "arguments": {
       "root_path": "."
     }
   }
   ```

   **Get Symbol Subgraph**:
   ```json
   {
     "tool": "get_symbol_subgraph",
     "arguments": {
       "symbol_name": "main",
       "max_depth": 2
     }
   }
   ```

   **Update Code Graph**:
   ```json
   {
     "tool": "update_code_graph",
     "arguments": {}
   }
   ```

4. **Inspect the Results**

   The MCP inspector will show the results of each tool call, including the parsed code symbols, references, and graph structure.

### Using Claude MCP Inspector

The Claude MCP Inspector provides a user-friendly web interface for testing MCP tools. Here's a step-by-step guide for using it with our code analysis tools:

1. **Install and Start the Inspector**

   ```bash
   npx @modelcontextprotocol/inspector
   ```

   This will open a web browser with the MCP Inspector interface.

2. **Connect to Your MCP Server**

   - In the "Server URL" field, enter the URL of your MCP server (typically `http://localhost:8080`)
   - Click "Connect"
   - You should see a list of available tools, including the code analysis tools

3. **Execute a Tool Call**

   - Select a tool from the dropdown menu (e.g., "analyze_code")
   - Enter the arguments in the JSON editor (e.g., `{"file_path": "src/main.rs"}`)
   - Click "Execute"

4. **View the Results**

   - The results will be displayed in the "Response" section
   - For graph tools, you'll see a JSON representation of the graph
   - You can expand and collapse sections to explore the results

5. **Try Different Tools**

   - Try each of the code analysis tools with different arguments
   - For example, use `get_code_graph` to build a graph, then use `get_symbol_subgraph` to explore a specific part of the graph

6. **Test Incremental Updates**

   - Make changes to some code files
   - Use `update_code_graph` to update the graph
   - Use other tools to verify that the changes were detected

The Claude MCP Inspector makes it easy to test and debug the code analysis tools, and to explore the code structure of your project.

![Claude MCP Inspector](https://placeholder-for-mcp-inspector-screenshot.png)

*Note: Replace the placeholder image with an actual screenshot of the MCP Inspector showing the code analysis tools.*

### Integration with LLM

When using these tools with an LLM, the LLM can make function calls to analyze code and build a graph of code references. Here's an example of how the LLM might use these tools:

1. First, the LLM would call `get_code_graph` to build a graph of the entire codebase.
2. Then, it might call `get_symbol_subgraph` to focus on a specific part of the codebase.
3. It could call `find_symbol_references` to find all references to a specific function or class.
4. If files change, it could call `update_code_graph` to update the graph without rebuilding it from scratch.

## Performance Considerations

- The code graph is built incrementally, only reparsing files that have changed
- The `get_symbol_subgraph` tool allows limiting the size of the graph by specifying a maximum depth
- The `update_code_graph` tool allows updating the graph without rebuilding it from scratch

## Adding Support for Additional Languages

If you want to add support for additional programming languages, you'll need to:

1. **Add the Tree-sitter Dependency**

   Add the Tree-sitter parser for the language to `Cargo.toml`:

   ```toml
   tree-sitter-<language> = "<version>"
   ```

2. **Create a Query File**

   Create a query file for the language in the `queries` directory:

   ```
   codex-rs/core/src/code_analysis/queries/<language>.scm
   ```

   The query file should contain patterns for finding functions, methods, classes, and other symbols in the language.

3. **Update the SupportedLanguage Enum**

   Add the language to the `SupportedLanguage` enum in `parser_pool.rs`.

4. **Update the load_language and load_query Methods**

   Update the `load_language` and `load_query` methods in `parser_pool.rs` to support the new language.

5. **Implement Symbol Extraction**

   Add an `extract_<language>_symbols` method to `context_extractor.rs` for the new language.

6. **Update the extract_symbols_from_file Method**

   Update the `extract_symbols_from_file` and `extract_symbols_from_file_incremental` methods in `context_extractor.rs` to handle the new language.

## Future Enhancements

- Improve the accuracy of symbol extraction and reference tracking
- Add more tools for code analysis (e.g., finding unused code, detecting code smells)
- Add support for more complex queries (e.g., finding all callers of a function that match a certain pattern)
- Implement parent-child relationships between symbols (e.g., methods belonging to classes)