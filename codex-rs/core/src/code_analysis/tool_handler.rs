//! Handler for Code Analysis tool calls.

use serde_json::{Value, json};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Once;

use crate::code_analysis::tools::{
    handle_analyze_code,
    handle_find_symbol_references,
    handle_find_symbol_definitions,
    handle_get_code_graph,
    handle_get_symbol_subgraph,
    handle_update_code_graph,
};
use crate::error::{CodexErr, Result};
use crate::mcp_tool_call::ToolCall;

// Static flag to track whether the code graph has been initialized
static CODE_GRAPH_INITIALIZED: AtomicBool = AtomicBool::new(false);
static INIT: Once = Once::new();

/// Initialize the code graph by scanning the repository
fn initialize_code_graph_with_path(root_path: &str) {
    // Only initialize once
    INIT.call_once(|| {
        // Call update_code_graph with the specified root path
        let args = json!({
            "root_path": root_path
        });
        
        if let Some(result) = handle_update_code_graph(args) {
            match result {
                Ok(_) => {
                    // Set the flag to indicate that the code graph has been initialized
                    CODE_GRAPH_INITIALIZED.store(true, Ordering::SeqCst);
                    println!("Code graph initialized successfully for path: {}", root_path);
                },
                Err(e) => {
                    eprintln!("Failed to initialize code graph for path {}: {}", root_path, e);
                }
            }
        }
    });
}

/// Initialize the code graph by scanning the current working directory (fallback)
fn initialize_code_graph() {
    // Get the current working directory as fallback
    if let Ok(current_dir) = std::env::current_dir() {
        initialize_code_graph_with_path(&current_dir.to_string_lossy().to_string());
    }
}

/// Handle Code Analysis tool calls
pub async fn handle_code_analysis_tool_call(
    tool_call: &ToolCall,
) -> Result<Value> {
    // Extract tool name and arguments
    let name = &tool_call.name;
    let mut args = tool_call.arguments.clone();
    
    // Initialize the code graph if it hasn't been initialized yet
    // Try to get the root path from the arguments first
    if !CODE_GRAPH_INITIALIZED.load(Ordering::SeqCst) {
        if let Some(obj) = args.as_object() {
            if let Some(root_path_value) = obj.get("root_path") {
                if let Some(root_path) = root_path_value.as_str() {
                    initialize_code_graph_with_path(root_path);
                } else {
                    initialize_code_graph();
                }
            } else {
                initialize_code_graph();
            }
        } else {
            initialize_code_graph();
        }
    }
    
    // Fix file paths in the arguments if needed
    if name.as_str() == "code_analysis.analyze_code" {
        if let Some(obj) = args.as_object_mut() {
            if let Some(file_path_value) = obj.get("file_path") {
                if let Some(file_path) = file_path_value.as_str() {
                    // Check if the file exists at the given path
                    let path = Path::new(file_path);
                    if !path.exists() {
                        // Try to resolve the path relative to the current working directory
                        if let Ok(current_dir) = std::env::current_dir() {
                            let absolute_path = current_dir.join(file_path);
                            if absolute_path.exists() {
                                obj.insert("file_path".to_string(), json!(absolute_path.to_string_lossy().to_string()));
                            } else {
                                // Try without any prefixes (in case the path has unnecessary prefixes)
                                let file_name = Path::new(file_path).file_name()
                                    .and_then(|name| name.to_str())
                                    .unwrap_or(file_path);
                                
                                // Search for the file in the current directory tree
                                if let Some(found_path) = find_file_in_directory(&current_dir, file_name) {
                                    obj.insert("file_path".to_string(), json!(found_path));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Dispatch to appropriate tool function
    match name.as_str() {
        "code_analysis.analyze_code" => {
            Ok(handle_analyze_code(args)
                .ok_or_else(|| CodexErr::Other("Failed to handle analyze_code".to_string()))?
                .map_err(|e| CodexErr::Other(e.to_string()))?)
        },
        "code_analysis.find_symbol_references" => {
            Ok(handle_find_symbol_references(args)
                .ok_or_else(|| CodexErr::Other("Failed to handle find_symbol_references".to_string()))?
                .map_err(|e| CodexErr::Other(e.to_string()))?)
        },
        "code_analysis.find_symbol_definitions" => {
            Ok(handle_find_symbol_definitions(args)
                .ok_or_else(|| CodexErr::Other("Failed to handle find_symbol_definitions".to_string()))?
                .map_err(|e| CodexErr::Other(e.to_string()))?)
        },
        "code_analysis.get_code_graph" => {
            Ok(handle_get_code_graph(args)
                .ok_or_else(|| CodexErr::Other("Failed to handle get_code_graph".to_string()))?
                .map_err(|e| CodexErr::Other(e.to_string()))?)
        },
        "code_analysis.get_symbol_subgraph" => {
            Ok(handle_get_symbol_subgraph(args)
                .ok_or_else(|| CodexErr::Other("Failed to handle get_symbol_subgraph".to_string()))?
                .map_err(|e| CodexErr::Other(e.to_string()))?)
        },
        "code_analysis.update_code_graph" => {
            Ok(handle_update_code_graph(args)
                .ok_or_else(|| CodexErr::Other("Failed to handle update_code_graph".to_string()))?
                .map_err(|e| CodexErr::Other(e.to_string()))?)
        },
        _ => {
            Err(CodexErr::Other(format!("Unknown Code Analysis tool: {}", name)))
        }
    }
}

/// Helper function to find a file by name in a directory tree
fn find_file_in_directory(dir: &Path, file_name: &str) -> Option<String> {
    use std::fs;
    
    // Check if the file exists directly in this directory
    let direct_path = dir.join(file_name);
    if direct_path.exists() {
        return Some(direct_path.to_string_lossy().to_string());
    }
    
    // Search recursively in subdirectories (with depth limit to avoid infinite loops)
    fn search_recursive(dir: &Path, file_name: &str, depth: usize) -> Option<String> {
        if depth > 10 {  // Limit recursion depth
            return None;
        }
        
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.file_name().and_then(|n| n.to_str()) == Some(file_name) {
                    return Some(path.to_string_lossy().to_string());
                } else if path.is_dir() {
                    // Skip hidden directories and common directories to ignore
                    let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if !dir_name.starts_with('.') && !["node_modules", "target", "dist"].contains(&dir_name) {
                        if let Some(found) = search_recursive(&path, file_name, depth + 1) {
                            return Some(found);
                        }
                    }
                }
            }
        }
        None
    }
    
    search_recursive(dir, file_name, 0)
}