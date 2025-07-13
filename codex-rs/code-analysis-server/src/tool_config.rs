//! Configuration for code analysis tools

use mcp_types::{Tool, ToolInputSchema};
use serde_json::json;

/// Create the list of available code analysis tools
pub fn create_code_analysis_tools() -> Vec<Tool> {
    vec![
        create_analyze_code_tool(),
        create_find_symbol_references_tool(),
        create_find_symbol_definitions_tool(),
        create_get_symbol_subgraph_tool(),
        create_get_related_files_skeleton_tool(),
        create_get_multiple_files_skeleton_tool(),
    ]
}

/// Create the analyze_code tool definition
fn create_analyze_code_tool() -> Tool {
    Tool {
        name: "analyze_code".to_string(),
        description: Some("COMPREHENSIVE CODE ANALYSIS: Extracts ALL symbols (functions, classes, methods, structs, enums, interfaces) from a file with precise line numbers. Detects 20-50+ symbols per file across 8 languages (Rust, JS/TS, Python, Go, C++, C#, Java). Perfect for understanding file structure, finding entry points, or getting complete symbol inventory. Fast: 200ms-3s. Example: Finds 49 symbols in complex Rust files, 28 symbols in TypeScript with generics/interfaces.".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "file_path": {
                    "type": "string",
                    "description": "Path to the file to analyze"
                }
            })),
            required: Some(vec!["file_path".to_string()]),
        },
    }
}

/// Create the find_symbol_references tool definition
fn create_find_symbol_references_tool() -> Tool {
    Tool {
        name: "find_symbol_references".to_string(),
        description: Some("CROSS-LANGUAGE SYMBOL TRACKING: Finds ALL references to any symbol across the entire codebase, even across different programming languages! Tracks 50+ references for common symbols like 'User' across C#, Python, Rust, etc. Shows exact file paths, line numbers, and reference types (call, usage, declaration). Essential for impact analysis, refactoring, or understanding how code connects. Lightning fast: <1s for most queries.".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "symbol_name": {
                    "type": "string",
                    "description": "Name of the symbol to find references for"
                },
                "symbol_type": {
                    "type": "string",
                    "description": "Type of symbol (function, class, variable, etc.)",
                    "enum": ["function", "class", "variable", "method", "field", "interface", "enum"]
                }
            })),
            required: Some(vec!["symbol_name".to_string()]),
        },
    }
}

/// Create the find_symbol_definitions tool definition
fn create_find_symbol_definitions_tool() -> Tool {
    Tool {
        name: "find_symbol_definitions".to_string(),
        description: Some("PRECISE SYMBOL LOCATION: Instantly finds WHERE any symbol is defined with exact line numbers (start-end). Works across all languages and provides symbol type (function, class, method, etc.). Perfect for 'go to definition' functionality or understanding symbol origins. Example: Finds 'handle_analyze_code' at lines 269-2264 in tools.rs. Fast and accurate: <500ms.".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "symbol_name": {
                    "type": "string",
                    "description": "Name of the symbol to find definition for"
                },
                "symbol_type": {
                    "type": "string",
                    "description": "Type of symbol (function, class, variable, etc.)",
                    "enum": ["function", "class", "variable", "method", "field", "interface", "enum"]
                }
            })),
            required: Some(vec!["symbol_name".to_string()]),
        },
    }
}

/// Create the get_symbol_subgraph tool definition
fn create_get_symbol_subgraph_tool() -> Tool {
    Tool {
        name: "get_symbol_subgraph".to_string(),
        description: Some("INTELLIGENT DEPENDENCY MAPPING: Returns a subgraph of code references starting from a specific symbol, with configurable traversal depth. If multiple symbols have the same name (e.g., in different namespaces), includes all of them in the subgraph. Uses the pre-initialized code graph for fast lookups. Perfect for understanding code dependencies and relationships.".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "symbol_name": {
                    "type": "string",
                    "description": "Name of the symbol to get subgraph for"
                },
                "depth": {
                    "type": "integer",
                    "description": "Depth of the subgraph (default: 2)",
                    "default": 2,
                    "minimum": 1,
                    "maximum": 5
                }
            })),
            required: Some(vec!["symbol_name".to_string()]),
        },
    }
}

/// Create the get_related_files_skeleton tool definition
fn create_get_related_files_skeleton_tool() -> Tool {
    Tool {
        name: "get_related_files_skeleton".to_string(),
        description: Some("SMART FILE DISCOVERY: Uses intelligent BFS traversal to find and analyze related files through symbol references and dependencies. Provides collapsed code views with line numbers while respecting token limits. Perfect for exploring codebases, understanding file relationships, or getting context around specific functionality. Automatically prioritizes most relevant files.".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "active_files": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    },
                    "description": "List of active file paths to find related files for"
                },
                "max_tokens": {
                    "type": "integer",
                    "description": "Maximum number of tokens to include in the response (default: 4000)",
                    "default": 4000,
                    "minimum": 100,
                    "maximum": 20000
                },
                "max_depth": {
                    "type": "integer",
                    "description": "Maximum depth for BFS traversal (default: 3)",
                    "default": 3,
                    "minimum": 1,
                    "maximum": 10
                }
            })),
            required: Some(vec!["active_files".to_string()]),
        },
    }
}

/// Create the get_multiple_files_skeleton tool definition
fn create_get_multiple_files_skeleton_tool() -> Tool {
    Tool {
        name: "get_multiple_files_skeleton".to_string(),
        description: Some("MULTI-FILE CODE OVERVIEW: Generates collapsed views of multiple files simultaneously with function signatures, class definitions, and import statements. Includes precise line numbers for each symbol. Perfect for comparing files, understanding multi-file features, or getting quick overviews of related code. Handles mixed languages intelligently.".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "file_paths": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    },
                    "description": "List of file paths to generate skeletons for"
                },
                "max_tokens": {
                    "type": "integer",
                    "description": "Maximum number of tokens to include in the response (default: 4000)",
                    "default": 4000,
                    "minimum": 100,
                    "maximum": 20000
                }
            })),
            required: Some(vec!["file_paths".to_string()]),
        },
    }
}