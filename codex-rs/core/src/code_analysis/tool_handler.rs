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
fn initialize_code_graph() {
    // Only initialize once
    INIT.call_once(|| {
        // Get the current working directory
        if let Ok(current_dir) = std::env::current_dir() {
            // Call update_code_graph with the current directory
            let args = json!({
                "root_path": current_dir.to_string_lossy().to_string()
            });
            
            if let Some(result) = handle_update_code_graph(args) {
                match result {
                    Ok(_) => {
                        // Set the flag to indicate that the code graph has been initialized
                        CODE_GRAPH_INITIALIZED.store(true, Ordering::SeqCst);
                        println!("Code graph initialized successfully");
                    },
                    Err(e) => {
                        eprintln!("Failed to initialize code graph: {}", e);
                    }
                }
            }
        }
    });
}

/// Handle Code Analysis tool calls
pub async fn handle_code_analysis_tool_call(
    tool_call: &ToolCall,
) -> Result<Value> {
    // Initialize the code graph if it hasn't been initialized yet
    if !CODE_GRAPH_INITIALIZED.load(Ordering::SeqCst) {
        initialize_code_graph();
    }
    
    // Extract tool name and arguments
    let name = &tool_call.name;
    let mut args = tool_call.arguments.clone();
    
    // Fix file paths in the arguments if needed
    if name.as_str() == "code_analysis.analyze_code" || name.as_str() == "code_analysis.get_code_graph" {
        if let Some(obj) = args.as_object_mut() {
            // Check file_path for analyze_code or root_path for get_code_graph
            let path_key = if name.as_str() == "code_analysis.analyze_code" { "file_path" } else { "root_path" };
            
            // Get the file path value
            let mut new_path_value = None;
            
            if let Some(file_path_value) = obj.get(path_key) {
                if let Some(file_path) = file_path_value.as_str() {
                    // Check if the file exists at the given path
                    let path = Path::new(file_path);
                    if !path.exists() {
                        // Try to find the file relative to the current working directory
                        if let Ok(current_dir) = std::env::current_dir() {
                            // First, check if the path starts with "codex-rs/"
                            if file_path.starts_with("codex-rs/") {
                                // Remove the "codex-rs/" prefix if we're already in the codex-rs directory
                                let dir_name = current_dir.file_name()
                                    .and_then(|name| name.to_str())
                                    .unwrap_or("");
                                
                                if dir_name == "codex-rs" {
                                    new_path_value = Some(file_path.strip_prefix("codex-rs/").unwrap_or(file_path).to_string());
                                }
                            }
                            
                            // Special case for "codex-rs/src/lib.rs" which doesn't exist
                            // Instead, we have multiple lib.rs files in different subdirectories
                            if file_path == "codex-rs/src/lib.rs" || file_path == "src/lib.rs" {
                                // Use the core/src/lib.rs as a fallback
                                new_path_value = Some("core/src/lib.rs".to_string());
                            }
                            
                            // Special case for "README.md" which might be at the root
                            if file_path == "README.md" {
                                // Check if README.md exists in the current directory
                                let readme_path = current_dir.join("README.md");
                                if readme_path.exists() {
                                    new_path_value = Some("README.md".to_string());
                                }
                            }
                        }
                    }
                }
            }
            
            // Apply the new path if we found one
            if let Some(new_path) = new_path_value {
                obj.insert(path_key.to_string(), json!(new_path));
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