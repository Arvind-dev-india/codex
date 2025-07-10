//! Handler for Code Analysis tool calls.

use serde_json::{Value, json};
use std::path::Path;

use crate::code_analysis::tools::{
    handle_analyze_code,
    handle_find_symbol_references,
    handle_find_symbol_definitions,
    handle_get_symbol_subgraph,
    handle_get_related_files_skeleton,
};
use crate::error::{CodexErr, Result};
use crate::mcp_tool_call::ToolCall;
use super::graph_manager::is_graph_initialized;

/// Handle Code Analysis tool calls
pub async fn handle_code_analysis_tool_call(
    tool_call: &ToolCall,
) -> Result<Value> {
    // Extract tool name and arguments
    let name = &tool_call.name;
    let mut args = tool_call.arguments.clone();
    
    // Check if the code graph is initialized
    if !is_graph_initialized() {
        return Err(CodexErr::Other(
            "Code graph not initialized. Please wait for initialization to complete.".to_string()
        ));
    }
    
    // Fix file paths in the arguments if needed
    if name.as_str() == "code_analysis_analyze_code" {
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
        "code_analysis_analyze_code" => {
            Ok(handle_analyze_code(args)
                .ok_or_else(|| CodexErr::Other("Failed to handle analyze_code".to_string()))?
                .map_err(|e| CodexErr::Other(e.to_string()))?)
        },
        "code_analysis_find_symbol_references" => {
            Ok(handle_find_symbol_references(args)
                .ok_or_else(|| CodexErr::Other("Failed to handle find_symbol_references".to_string()))?
                .map_err(|e| CodexErr::Other(e.to_string()))?)
        },
        "code_analysis_find_symbol_definitions" => {
            Ok(handle_find_symbol_definitions(args)
                .ok_or_else(|| CodexErr::Other("Failed to handle find_symbol_definitions".to_string()))?
                .map_err(|e| CodexErr::Other(e.to_string()))?)
        },
        "code_analysis_get_symbol_subgraph" => {
            Ok(handle_get_symbol_subgraph(args)
                .ok_or_else(|| CodexErr::Other("Failed to handle get_symbol_subgraph".to_string()))?
                .map_err(|e| CodexErr::Other(e.to_string()))?)
        },
        "code_analysis_get_related_files_skeleton" => {
            Ok(handle_get_related_files_skeleton(args)
                .ok_or_else(|| CodexErr::Other("Failed to handle get_related_files_skeleton".to_string()))?
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