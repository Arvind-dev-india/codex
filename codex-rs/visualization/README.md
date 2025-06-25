# Code Symbol Graph Visualizer

A powerful tool for visualizing code structure and symbol relationships using Tree-sitter parsing and interactive D3.js graphs.

## Overview

This tool helps you understand your codebase by:
- Extracting symbols (functions, classes, structs, etc.) from source code
- Identifying relationships between symbols (calls, references, implementations)
- Visualizing the code structure as an interactive graph
- Allowing you to explore symbol connections and dependencies

## Features

- **Multi-language support**: Rust, Python, JavaScript/TypeScript, Java, C#, C++, Go
- **Interactive visualization**: Click, drag, zoom, and explore the graph
- **Symbol information**: View detailed information about each symbol
- **Relationship highlighting**: See how symbols are connected
- **Fully Qualified Names (FQN)**: Reduces naming collisions and provides precise identification
- **Tree-sitter powered**: Accurate parsing using Tree-sitter grammars

## Quick Start

### Option 1: Easy Script (Recommended)

```bash
cd codex-rs

# Analyze the code analysis module and open in browser
./scripts/visualize_code.sh ./core/src/code_analysis

# Analyze any directory
./scripts/visualize_code.sh /path/to/your/source/code

# Custom output file
./scripts/visualize_code.sh ./core -o my_graph.json

# Don't open browser automatically
./scripts/visualize_code.sh ./core --no-browser
```

### Option 2: Manual Steps

#### 1. Build the Tool

```bash
cd codex-rs
./scripts/build_graph_visualizer.sh
```

#### 2. Generate Graph Data

```bash
# Generate graph for the code analysis module
cargo run --bin generate_code_graph ./core/src/code_analysis visualization/code_graph.json

# Or analyze any other directory
cargo run --bin generate_code_graph /path/to/your/source/code visualization/code_graph.json
```

#### 3. View the Graph

The visualization now automatically loads `code_graph.json` when opened:

```bash
# Option A: Direct file opening (may have CORS restrictions)
open visualization/index.html

# Option B: Use the built-in HTTP server (recommended)
cd visualization
python3 serve.py
# Then open http://localhost:8000/index.html in your browser

# Option C: Use Python's built-in server
cd visualization  
python3 -m http.server 8000
# Then open http://localhost:8000/index.html in your browser
```

**Note**: If the graph doesn't appear when opening the file directly, use Option B or C to avoid CORS restrictions.

## Usage Guide

### Generating Graph Data

The graph generator accepts two arguments:

```bash
cargo run --bin generate_code_graph <source_directory> [output_file]
```

- `<source_directory>`: Path to the directory containing source code
- `[output_file]`: Optional output JSON file (defaults to `code_graph.json`)

**Examples:**

```bash
# Analyze the entire core module
cargo run --bin generate_code_graph ./core visualization/code_graph.json

# Analyze a specific subdirectory
cargo run --bin generate_code_graph ./core/src/code_analysis visualization/analysis_graph.json

# Analyze your own project
cargo run --bin generate_code_graph /home/user/my_project visualization/my_project_graph.json
```

### Using the Visualization

1. **Load Graph Data**: 
   - **Automatic**: The visualization automatically loads `code_graph.json` when opened
   - **Manual**: Click "Load Graph Data" to select a different JSON file
2. **Navigate**: 
   - **Zoom**: Mouse wheel or pinch gesture
   - **Pan**: Click and drag on empty space
   - **Move nodes**: Click and drag individual nodes
3. **Explore Symbols**:
   - **Select from dropdown**: Choose a symbol from the "Select Symbol" dropdown to focus on it
   - **Filter by type**: Use "Filter by Type" to show only specific symbol types
   - **Search**: Type in the search box to highlight matching symbols
   - **Click a node** to see detailed information and highlight connections
   - **View symbol info** in the left sidebar
   - **See relationships** highlighted in red
4. **Controls**:
   - **Reset View**: Return to original zoom and position and clear all filters
   - **Toggle Labels**: Show/hide symbol names
   - **Toggle Physics**: Enable/disable force simulation

### Understanding the Graph

#### Node Colors and Sizes

- **🟢 Function**: Green circles (medium size)
- **🔵 Method**: Blue circles (medium size)  
- **🟠 Class**: Orange circles (large size)
- **🟣 Struct**: Purple circles (large size)
- **🔵 Interface**: Cyan circles (medium size)
- **🟤 Enum**: Brown circles (medium size)
- **⚫ Variable**: Gray circles (small size)
- **🔴 Constant**: Pink circles (small size)
- **🟡 Module**: Yellow circles (extra large size)
- **🟢 Package**: Light green circles (largest size)

#### Edges (Connections)

- **Red lines**: Active connections (when a node is selected)
- **Gray lines**: All other relationships
- **Thickness**: Indicates relationship strength/importance

#### Symbol Information Panel

When you click a node or select from the dropdown, the left sidebar shows:
- **Name**: Symbol name
- **Type**: Symbol type (Function, Class, etc.)
- **File**: Source file path
- **Lines**: Start and end line numbers
- **FQN**: Fully Qualified Name (unique identifier)
- **Connections**: Number of relationships
- **Parent**: Parent symbol (if applicable)

#### Interactive Features

- **Symbol Dropdown**: All symbols are listed alphabetically with their type and file path
- **Type Filter**: Filter to show only Functions, Classes, Structs, etc.
- **Search Box**: Real-time search that highlights matching symbols
- **Click Navigation**: Click any node to focus on it and see its connections
- **Synchronized Selection**: Dropdown and graph selection stay in sync

## Supported Languages and File Extensions

| Language | Extensions | Symbol Types |
|----------|------------|--------------|
| Rust | `.rs` | Functions, Methods, Structs, Enums, Traits, Modules |
| Python | `.py`, `.pyw` | Functions, Methods, Classes |
| JavaScript | `.js`, `.jsx`, `.mjs` | Functions, Classes, Methods |
| TypeScript | `.ts`, `.tsx` | Functions, Classes, Methods, Interfaces, Types |
| Java | `.java` | Methods, Classes, Interfaces |
| C# | `.cs` | Methods, Classes, Interfaces, Namespaces |
| C++ | `.cpp`, `.cc`, `.cxx`, `.h`, `.hpp` | Functions, Classes, Structs |
| Go | `.go` | Functions, Methods, Structs, Interfaces, Types |

## Examples

### Example 1: Analyzing the Code Analysis Module

```bash
# Generate graph
cargo run --bin generate_code_graph ./core/src/code_analysis visualization/analysis_graph.json

# Open visualization/index.html and load analysis_graph.json
```

This will show you:
- How `ContextExtractor` relates to `ParsedFile`
- Which functions call each other
- The structure of the Tree-sitter integration

### Example 2: Analyzing a Python Project

```bash
# Generate graph for a Python project
cargo run --bin generate_code_graph /path/to/python/project visualization/python_graph.json
```

You'll see:
- Class hierarchies
- Function call relationships
- Module dependencies

### Example 3: Large Codebase Analysis

```bash
# Analyze the entire codex-rs project
cargo run --bin generate_code_graph ./core visualization/full_project.json
```

**Note**: Large codebases may generate complex graphs. Use the zoom and filter features to navigate effectively.

## Troubleshooting

### Common Issues

1. **"No symbols found"**
   - Check that the source directory contains supported file types
   - Verify file permissions
   - Look for parsing errors in the console output

2. **"Failed to parse file"**
   - Some files may have syntax errors
   - Tree-sitter parsers may not support all language features
   - Check the console for specific error messages

3. **Graph is too cluttered**
   - Use zoom to focus on specific areas
   - Toggle labels off for cleaner view
   - Consider analyzing smaller subdirectories

4. **Browser performance issues**
   - Large graphs (>1000 nodes) may be slow
   - Try analyzing smaller code sections
   - Use a modern browser with good JavaScript performance

### Debug Information

The graph generator outputs progress information:

```bash
cargo run --bin generate_code_graph ./core/src/code_analysis visualization/debug.json
```

Look for messages like:
- `Processed <file>` - Successfully parsed files
- `Error processing <file>: <error>` - Parsing failures
- `Loaded X nodes and Y edges` - Final graph statistics

## Advanced Usage

### Custom Analysis

You can modify the graph generator to:
- Filter specific symbol types
- Add custom relationship types
- Include additional metadata
- Export different formats

### Integration with IDEs

The JSON output can be consumed by:
- VS Code extensions
- IntelliJ plugins
- Custom analysis tools
- CI/CD pipelines for code quality metrics

## Contributing

To add support for new languages:

1. Add the Tree-sitter parser dependency to `core/Cargo.toml`
2. Update `SupportedLanguage` enum in `parser_pool.rs`
3. Add language detection in `from_extension()`
4. Create a new `.scm` query file in `queries/`
5. Test with sample code

## Technical Details

### Architecture

```
Source Code → Tree-sitter Parser → Symbol Extractor → Graph Generator → JSON → D3.js Visualization
```

### Data Format

The generated JSON contains:
```json
{
  "nodes": [
    {
      "id": "file.rs::function::my_function",
      "name": "my_function", 
      "symbol_type": "Function",
      "file_path": "src/file.rs",
      "start_line": 10,
      "end_line": 20,
      "parent": null
    }
  ],
  "edges": [
    {
      "source": "caller_fqn",
      "target": "callee_fqn", 
      "edge_type": "Call"
    }
  ]
}
```

### Performance

- **Parsing**: O(n) where n is total lines of code
- **Graph generation**: O(m) where m is number of symbols
- **Visualization**: O(n²) for force simulation with n nodes

## Summary

The Code Symbol Graph Visualizer is a powerful tool that helps you understand your codebase by:

1. **Extracting symbols** using Tree-sitter parsers for accurate parsing
2. **Building relationships** between symbols (calls, references, implementations)  
3. **Visualizing the structure** as an interactive D3.js graph
4. **Exploring connections** through click-to-navigate functionality

### Key Benefits

- **Multi-language support** for 8+ programming languages
- **Accurate parsing** using Tree-sitter grammars
- **FQN-based identification** to avoid naming collisions
- **Interactive exploration** with zoom, pan, and click navigation
- **Easy to use** with simple command-line interface

### Getting Started

```bash
cd codex-rs
./scripts/visualize_code.sh ./core/src/code_analysis
```

That's it! The tool will generate the graph and open it in your browser automatically.

## License

This tool is part of the codex-rs project and follows the same license terms.