//! Bridge to connect to the existing code analysis functionality in codex-core

use anyhow::Result;
use codex_core::code_analysis::{
    handle_analyze_code,
    handle_find_symbol_references,
    handle_find_symbol_definitions,
    handle_get_symbol_subgraph,
    handle_get_related_files_skeleton,
    handle_get_multiple_files_skeleton,
    graph_manager,
};
use serde_json::Value;
use tracing::{info, error};

/// Initialize the code graph for the current directory
pub fn init_code_graph() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    
    // Check if graph is already initialized for this path to avoid redundant initialization
    if graph_manager::is_graph_initialized() {
        info!("Code graph already initialized for: {}", current_dir.display());
        return Ok(());
    }
    
    info!("Initializing code graph for: {}", current_dir.display());
    
    // Use the graph manager to initialize and handle file changes
    match graph_manager::ensure_graph_for_path(&current_dir) {
        Ok(_) => {
            info!("Code graph initialized successfully");
            Ok(())
        },
        Err(e) => {
            error!("Failed to initialize code graph: {}", e);
            Err(anyhow::anyhow!("Failed to initialize code graph: {}", e))
        }
    }
}


/// Call the analyze_code function from codex-core
pub fn call_analyze_code(args: Value) -> Result<Value> {
    // Check if graph is initialized, but don't force updates since graph manager handles changes
    if !graph_manager::is_graph_initialized() {
        return Err(anyhow::anyhow!("Code graph not initialized. Please wait for initialization to complete."));
    }
    
    match handle_analyze_code(args) {
        Some(Ok(result)) => Ok(result),
        Some(Err(e)) => Err(anyhow::anyhow!("Error in analyze_code: {}", e)),
        None => Err(anyhow::anyhow!("Failed to handle analyze_code")),
    }
}

/// Call the find_symbol_references function from codex-core
pub fn call_find_symbol_references(args: Value) -> Result<Value> {
    // Check if graph is initialized, but don't force updates since graph manager handles changes
    if !graph_manager::is_graph_initialized() {
        return Err(anyhow::anyhow!("Code graph not initialized. Please wait for initialization to complete."));
    }
    
    match handle_find_symbol_references(args) {
        Some(Ok(result)) => Ok(result),
        Some(Err(e)) => Err(anyhow::anyhow!("Error in find_symbol_references: {}", e)),
        None => Err(anyhow::anyhow!("Failed to handle find_symbol_references")),
    }
}

/// Call the find_symbol_definitions function from codex-core
pub fn call_find_symbol_definitions(args: Value) -> Result<Value> {
    // Check if graph is initialized, but don't force updates since graph manager handles changes
    if !graph_manager::is_graph_initialized() {
        return Err(anyhow::anyhow!("Code graph not initialized. Please wait for initialization to complete."));
    }
    
    match handle_find_symbol_definitions(args) {
        Some(Ok(result)) => Ok(result),
        Some(Err(e)) => Err(anyhow::anyhow!("Error in find_symbol_definitions: {}", e)),
        None => Err(anyhow::anyhow!("Failed to handle find_symbol_definitions")),
    }
}

/// Call the get_symbol_subgraph function from codex-core
pub fn call_get_symbol_subgraph(args: Value) -> Result<Value> {
    // Check if graph is initialized, but don't force updates since graph manager handles changes
    if !graph_manager::is_graph_initialized() {
        return Err(anyhow::anyhow!("Code graph not initialized. Please wait for initialization to complete."));
    }
    
    match handle_get_symbol_subgraph(args) {
        Some(Ok(result)) => Ok(result),
        Some(Err(e)) => Err(anyhow::anyhow!("Error in get_symbol_subgraph: {}", e)),
        None => Err(anyhow::anyhow!("Failed to handle get_symbol_subgraph")),
    }
}

/// Call the get_related_files_skeleton function from codex-core
pub fn call_get_related_files_skeleton(args: Value) -> Result<Value> {
    // Skip graph update for skeleton operations since they use cached data
    // and the graph is already initialized during server startup
    match handle_get_related_files_skeleton(args) {
        Some(Ok(result)) => Ok(result),
        Some(Err(e)) => Err(anyhow::anyhow!("Error in get_related_files_skeleton: {}", e)),
        None => Err(anyhow::anyhow!("Failed to handle get_related_files_skeleton")),
    }
}

/// Call the get_multiple_files_skeleton function from codex-core
pub fn call_get_multiple_files_skeleton(args: Value) -> Result<Value> {
    // Skip graph update for skeleton operations since they use cached data
    // and the graph is already initialized during server startup
    match handle_get_multiple_files_skeleton(args) {
        Some(Ok(result)) => Ok(result),
        Some(Err(e)) => Err(anyhow::anyhow!("Error in get_multiple_files_skeleton: {}", e)),
        None => Err(anyhow::anyhow!("Failed to handle get_multiple_files_skeleton")),
    }
}