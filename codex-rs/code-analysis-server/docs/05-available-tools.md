# Available Tools

The Code Analysis Server provides four main tools for code analysis.

## 1. analyze_code

**Purpose**: Analyze a specific file and extract its symbols

**Parameters**:
- `file_path` (string, required): Path to the file to analyze

**Example**:
```json
{
  "name": "analyze_code",
  "arguments": {
    "file_path": "/path/to/file.py"
  }
}
```

**Returns**: Detailed information about functions, classes, and other symbols in the file.

## 2. find_symbol_references

**Purpose**: Find all places where a symbol is used

**Parameters**:
- `symbol_name` (string, required): Name of the symbol to search for
- `symbol_type` (string, optional): Type of symbol (function, class, variable, etc.)

**Example**:
```json
{
  "name": "find_symbol_references",
  "arguments": {
    "symbol_name": "UserService",
    "symbol_type": "class"
  }
}
```

**Returns**: List of all files and line numbers where the symbol is referenced.

## 3. find_symbol_definitions

**Purpose**: Find where a symbol is defined

**Parameters**:
- `symbol_name` (string, required): Name of the symbol to find
- `symbol_type` (string, optional): Type of symbol

**Example**:
```json
{
  "name": "find_symbol_definitions",
  "arguments": {
    "symbol_name": "process_data",
    "symbol_type": "function"
  }
}
```

**Returns**: Location where the symbol is defined (file, line number, etc.).

## 4. get_symbol_subgraph

**Purpose**: Get related symbols and their relationships

**Parameters**:
- `symbol_name` (string, required): Starting symbol
- `depth` (integer, optional): How many levels deep to search (default: 2, max: 5)

**Example**:
```json
{
  "name": "get_symbol_subgraph",
  "arguments": {
    "symbol_name": "DatabaseManager",
    "depth": 3
  }
}
```

**Returns**: A graph showing the symbol and all related symbols within the specified depth.

## Symbol Types

When specifying `symbol_type`, you can use:
- `function`
- `class`
- `variable`
- `method`
- `field`
- `interface`
- `enum`

**Next:** [Direct Communication](06-direct-communication.md)