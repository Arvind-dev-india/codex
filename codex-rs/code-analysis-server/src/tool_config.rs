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
    ]
}

/// Create the analyze_code tool definition
fn create_analyze_code_tool() -> Tool {
    Tool {
        name: "analyze_code".to_string(),
        description: Some("Analyze code structure and extract symbols from a file".to_string()),
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
        description: Some("Find all references to a symbol in the codebase".to_string()),
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
        description: Some("Find the definition of a symbol in the codebase".to_string()),
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
        description: Some("Get a subgraph of symbols related to a specific symbol".to_string()),
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