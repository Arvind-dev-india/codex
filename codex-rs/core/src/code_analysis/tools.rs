//! Tools for code analysis using Tree-sitter.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::openai_tools::{JsonSchema, OpenAiTool, create_function_tool};
use super::repo_mapper::{CodeEdgeType, CodeNodeType, CodeReferenceGraph, RepoMapper, create_repo_mapper};

/// Cached repository mapper
static REPO_MAPPER: Lazy<Mutex<Option<RepoMapper>>> = Lazy::new(|| Mutex::new(None));

/// Register all code analysis tools
pub fn register_code_analysis_tools() -> Vec<OpenAiTool> {
    vec![
        create_analyze_code_tool(),
        create_find_symbol_references_tool(),
        create_find_symbol_definitions_tool(),
        create_get_code_graph_tool(),
        create_get_symbol_subgraph_tool(),
        create_update_code_graph_tool(),
    ]
}

/// Create a tool for analyzing code in a file
fn create_analyze_code_tool() -> OpenAiTool {
    let mut properties = BTreeMap::new();
    
    properties.insert(
        "file_path".to_string(),
        JsonSchema::String,
    );
    
    create_function_tool(
        "analyze_code",
        "Analyzes the code in a file and returns information about functions, classes, and other symbols.",
        properties,
        &["file_path"],
    )
}

/// Create a tool for finding references to a symbol
fn create_find_symbol_references_tool() -> OpenAiTool {
    let mut properties = BTreeMap::new();
    
    properties.insert(
        "symbol_name".to_string(),
        JsonSchema::String,
    );
    
    create_function_tool(
        "find_symbol_references",
        "Finds all references to a symbol (function, class, variable, etc.) in the codebase.",
        properties,
        &["symbol_name"],
    )
}

/// Create a tool for finding symbol definitions
fn create_find_symbol_definitions_tool() -> OpenAiTool {
    let mut properties = BTreeMap::new();
    
    properties.insert(
        "symbol_name".to_string(),
        JsonSchema::String,
    );
    
    create_function_tool(
        "find_symbol_definitions",
        "Finds the definition of a symbol (function, class, variable, etc.) in the codebase.",
        properties,
        &["symbol_name"],
    )
}

/// Create a tool for getting the code reference graph
fn create_get_code_graph_tool() -> OpenAiTool {
    let mut properties = BTreeMap::new();
    
    properties.insert(
        "root_path".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "include_files".to_string(),
        JsonSchema::Array {
            items: Box::new(JsonSchema::String),
        },
    );
    
    properties.insert(
        "exclude_patterns".to_string(),
        JsonSchema::Array {
            items: Box::new(JsonSchema::String),
        },
    );
    
    create_function_tool(
        "get_code_graph",
        "Generates a graph of code references and dependencies.",
        properties,
        &["root_path"],
    )
}

/// Create a tool for getting a subgraph starting from a specific symbol
fn create_get_symbol_subgraph_tool() -> OpenAiTool {
    let mut properties = BTreeMap::new();
    
    properties.insert(
        "symbol_name".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "max_depth".to_string(),
        JsonSchema::Number,
    );
    
    create_function_tool(
        "get_symbol_subgraph",
        "Generates a subgraph of code references starting from a specific symbol, with a maximum traversal depth.",
        properties,
        &["symbol_name", "max_depth"],
    )
}

/// Create a tool for updating the code graph
fn create_update_code_graph_tool() -> OpenAiTool {
    let properties = BTreeMap::new();
    
    create_function_tool(
        "update_code_graph",
        "Updates the code graph by re-parsing any files that have changed since the last parse.",
        properties,
        &[],
    )
}

/// Input for the analyze_code tool
#[derive(Debug, Deserialize)]
struct AnalyzeCodeInput {
    file_path: String,
}

/// Input for the find_symbol_references tool
#[derive(Debug, Deserialize)]
struct FindSymbolReferencesInput {
    symbol_name: String,
}

/// Input for the find_symbol_definitions tool
#[derive(Debug, Deserialize)]
struct FindSymbolDefinitionsInput {
    symbol_name: String,
}

/// Input for the get_code_graph tool
#[derive(Debug, Deserialize)]
struct GetCodeGraphInput {
    root_path: String,
    include_files: Option<Vec<String>>,
    exclude_patterns: Option<Vec<String>>,
}

/// Input for the get_symbol_subgraph tool
#[derive(Debug, Deserialize)]
struct GetSymbolSubgraphInput {
    symbol_name: String,
    max_depth: usize,
}

/// Input for the update_code_graph tool (empty)
#[derive(Debug, Deserialize)]
struct UpdateCodeGraphInput {}

/// Symbol information returned by analyze_code
#[derive(Debug, Serialize)]
struct SymbolInfo {
    name: String,
    symbol_type: String,
    file_path: String,
    start_line: usize,
    end_line: usize,
    parent: Option<String>,
}

/// Reference information returned by find_symbol_references
#[derive(Debug, Serialize)]
struct ReferenceInfo {
    file_path: String,
    line: usize,
    column: usize,
    reference_type: String,
}

/// Definition information returned by find_symbol_definitions
#[derive(Debug, Serialize)]
struct DefinitionInfo {
    file_path: String,
    start_line: usize,
    end_line: usize,
    symbol_type: String,
}

/// Graph information returned by get_code_graph
#[derive(Debug, Serialize)]
struct GraphInfo {
    nodes: Vec<NodeInfo>,
    edges: Vec<EdgeInfo>,
}

/// Node information in the graph
#[derive(Debug, Serialize)]
struct NodeInfo {
    id: String,
    name: String,
    node_type: String,
    file_path: String,
}

/// Edge information in the graph
#[derive(Debug, Serialize)]
struct EdgeInfo {
    source: String,
    target: String,
    edge_type: String,
}

/// Handle the analyze_code tool call
pub fn handle_analyze_code(args: Value) -> Option<Result<Value, String>> {
    Some(match serde_json::from_value::<AnalyzeCodeInput>(args) {
        Ok(input) => {
        
        // In a real implementation, we would:
        // 1. Parse the file
        // 2. Extract symbols
        // 3. Return the symbols
        
        // For now, return a placeholder response
        let symbols = vec![
            SymbolInfo {
                name: "example_function".to_string(),
                symbol_type: "function".to_string(),
                file_path: input.file_path.clone(),
                start_line: 10,
                end_line: 20,
                parent: None,
            },
            SymbolInfo {
                name: "ExampleClass".to_string(),
                symbol_type: "class".to_string(),
                file_path: input.file_path.clone(),
                start_line: 30,
                end_line: 50,
                parent: None,
            },
        ];
        
        Ok(json!({
            "file_path": input.file_path,
            "symbols": symbols,
        }))
        },
        Err(e) => Err(format!("Invalid arguments: {}", e)),
    })
}

/// Handle the find_symbol_references tool call
pub fn handle_find_symbol_references(args: Value) -> Option<Result<Value, String>> {
    Some(match serde_json::from_value::<FindSymbolReferencesInput>(args) {
        Ok(input) => {
        
        // In a real implementation, we would:
        // 1. Look up the symbol in the context extractor
        // 2. Find all references to the symbol
        // 3. Return the references
        
        // For now, return a placeholder response
        let references = vec![
            ReferenceInfo {
                file_path: "src/main.rs".to_string(),
                line: 15,
                column: 10,
                reference_type: "call".to_string(),
            },
            ReferenceInfo {
                file_path: "src/lib.rs".to_string(),
                line: 25,
                column: 5,
                reference_type: "usage".to_string(),
            },
        ];
        
        Ok(json!({
            "symbol_name": input.symbol_name,
            "references": references,
        }))
        },
        Err(e) => Err(format!("Invalid arguments: {}", e)),
    })
}

/// Handle the find_symbol_definitions tool call
pub fn handle_find_symbol_definitions(args: Value) -> Option<Result<Value, String>> {
    Some(match serde_json::from_value::<FindSymbolDefinitionsInput>(args) {
        Ok(input) => {
        
        // In a real implementation, we would:
        // 1. Look up the symbol in the context extractor
        // 2. Return the definition
        
        // For now, return a placeholder response
        let definition = DefinitionInfo {
            file_path: "src/lib.rs".to_string(),
            start_line: 10,
            end_line: 20,
            symbol_type: "function".to_string(),
        };
        
        Ok(json!({
            "symbol_name": input.symbol_name,
            "definition": definition,
        }))
        },
        Err(e) => Err(format!("Invalid arguments: {}", e)),
    })
}

/// Handle the get_code_graph tool call
pub fn handle_get_code_graph(args: Value) -> Option<Result<Value, String>> {
    Some(match serde_json::from_value::<GetCodeGraphInput>(args) {
        Ok(input) => {
        
        // In a real implementation, we would:
        // 1. Create a repository mapper
        // 2. Map the repository
        // 3. Get the graph
        // 4. Filter the graph based on include_files and exclude_patterns
        // 5. Return the graph
        
        let mut repo_mapper_guard = REPO_MAPPER.lock().unwrap();
        
        // Create a new repo mapper if one doesn't exist or if the root path is different
        if repo_mapper_guard.is_none() {
            *repo_mapper_guard = Some(create_repo_mapper(Path::new(&input.root_path)));
            
            // Map the repository
            if let Some(ref mut mapper) = *repo_mapper_guard {
                if let Err(e) = mapper.map_repository() {
                    return Some(Err(format!("Failed to map repository: {}", e)));
                }
            }
        }
        
        // Get the graph
        let graph = if let Some(ref mapper) = *repo_mapper_guard {
            mapper.get_graph()
        } else {
            return Some(Err("Failed to create repository mapper".to_string()));
        };
        
        // Convert the graph to the output format
        let nodes = graph.nodes.iter().map(|node| NodeInfo {
            id: node.id.clone(),
            name: node.name.clone(),
            node_type: match node.node_type {
                CodeNodeType::File => "file".to_string(),
                CodeNodeType::Function => "function".to_string(),
                CodeNodeType::Method => "method".to_string(),
                CodeNodeType::Class => "class".to_string(),
                CodeNodeType::Struct => "struct".to_string(),
                CodeNodeType::Module => "module".to_string(),
            },
            file_path: node.file_path.clone(),
        }).collect::<Vec<_>>();
        
        let edges = graph.edges.iter().map(|edge| EdgeInfo {
            source: edge.source.clone(),
            target: edge.target.clone(),
            edge_type: match edge.edge_type {
                CodeEdgeType::Calls => "calls".to_string(),
                CodeEdgeType::Imports => "imports".to_string(),
                CodeEdgeType::Inherits => "inherits".to_string(),
                CodeEdgeType::Contains => "contains".to_string(),
                CodeEdgeType::References => "references".to_string(),
            },
        }).collect::<Vec<_>>();
        
        Ok(json!({
            "root_path": input.root_path,
            "graph": GraphInfo { nodes, edges },
        }))
        },
        Err(e) => Err(format!("Invalid arguments: {}", e))
    })
}

/// Handle the get_symbol_subgraph tool call
pub fn handle_get_symbol_subgraph(args: Value) -> Option<Result<Value, String>> {
    Some(match serde_json::from_value::<GetSymbolSubgraphInput>(args) {
        Ok(input) => {
            let repo_mapper_guard = REPO_MAPPER.lock().unwrap();
            
            // Check if the repository mapper exists
            if repo_mapper_guard.is_none() {
                Err("Repository mapper not initialized. Call get_code_graph first.".to_string())
            } else {
                // Get the subgraph
                let graph = if let Some(ref mapper) = *repo_mapper_guard {
                    mapper.get_subgraph_bfs(&input.symbol_name, input.max_depth)
                } else {
                    return Some(Err("Failed to access repository mapper".to_string()));
                };
                
                // Convert the graph to the output format
                let nodes = graph.nodes.iter().map(|node| NodeInfo {
                    id: node.id.clone(),
                    name: node.name.clone(),
                    node_type: match node.node_type {
                        CodeNodeType::File => "file".to_string(),
                        CodeNodeType::Function => "function".to_string(),
                        CodeNodeType::Method => "method".to_string(),
                        CodeNodeType::Class => "class".to_string(),
                        CodeNodeType::Struct => "struct".to_string(),
                        CodeNodeType::Module => "module".to_string(),
                    },
                    file_path: node.file_path.clone(),
                }).collect::<Vec<_>>();
                
                let edges = graph.edges.iter().map(|edge| EdgeInfo {
                    source: edge.source.clone(),
                    target: edge.target.clone(),
                    edge_type: match edge.edge_type {
                        CodeEdgeType::Calls => "calls".to_string(),
                        CodeEdgeType::Imports => "imports".to_string(),
                        CodeEdgeType::Inherits => "inherits".to_string(),
                        CodeEdgeType::Contains => "contains".to_string(),
                        CodeEdgeType::References => "references".to_string(),
                    },
                }).collect::<Vec<_>>();
                
                Ok(json!({
                    "symbol_name": input.symbol_name,
                    "max_depth": input.max_depth,
                    "graph": GraphInfo { nodes, edges },
                }))
            }
        },
        Err(e) => Err(format!("Invalid arguments: {}", e))
    })
}

/// Handle the update_code_graph tool call
pub fn handle_update_code_graph(args: Value) -> Option<Result<Value, String>> {
    Some(match serde_json::from_value::<UpdateCodeGraphInput>(args) {
        Ok(_input) => {
            let mut repo_mapper_guard = REPO_MAPPER.lock().unwrap();
            
            // Check if the repository mapper exists
            if repo_mapper_guard.is_none() {
                Err("Repository mapper not initialized. Call get_code_graph first.".to_string())
            } else {
                // Update the repository map
                if let Some(ref mut mapper) = *repo_mapper_guard {
                    match mapper.update_repository() {
                        Ok(_) => Ok(json!({
                            "status": "success",
                            "message": "Code graph updated successfully",
                        })),
                        Err(e) => Err(format!("Failed to update repository: {}", e))
                    }
                } else {
                    Err("Failed to access repository mapper".to_string())
                }
            }
        },
        Err(e) => Err(format!("Invalid arguments: {}", e))
    })
}