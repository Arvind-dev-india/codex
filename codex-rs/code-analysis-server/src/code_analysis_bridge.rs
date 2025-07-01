//! Bridge to connect to the existing code analysis functionality in codex-core

use anyhow::Result;
use codex_core::code_analysis::{
    handle_analyze_code,
    handle_find_symbol_references,
    handle_find_symbol_definitions,
    handle_get_symbol_subgraph,
    graph_manager,
};
use serde_json::Value;
use tracing::{info, error};

/// Initialize the code graph for the current directory
pub fn init_code_graph() -> Result<()> {
    let current_dir = std::env::current_dir()?;
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

/// Ensure the graph is up-to-date before processing requests
fn ensure_graph_updated() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    graph_manager::ensure_graph_for_path(&current_dir)
        .map_err(|e| anyhow::anyhow!("Failed to update code graph: {}", e))
}

/// Call the analyze_code function from codex-core
pub fn call_analyze_code(args: Value) -> Result<Value> {
    // Ensure the graph is up-to-date before processing
    ensure_graph_updated()?;
    
    match handle_analyze_code(args) {
        Some(Ok(result)) => Ok(result),
        Some(Err(e)) => Err(anyhow::anyhow!("Error in analyze_code: {}", e)),
        None => Err(anyhow::anyhow!("Failed to handle analyze_code")),
    }
}

/// Call the find_symbol_references function from codex-core
pub fn call_find_symbol_references(args: Value) -> Result<Value> {
    // Ensure the graph is up-to-date before processing
    ensure_graph_updated()?;
    
    match handle_find_symbol_references(args) {
        Some(Ok(result)) => Ok(result),
        Some(Err(e)) => Err(anyhow::anyhow!("Error in find_symbol_references: {}", e)),
        None => Err(anyhow::anyhow!("Failed to handle find_symbol_references")),
    }
}

/// Call the find_symbol_definitions function from codex-core
pub fn call_find_symbol_definitions(args: Value) -> Result<Value> {
    // Ensure the graph is up-to-date before processing
    ensure_graph_updated()?;
    
    match handle_find_symbol_definitions(args) {
        Some(Ok(result)) => Ok(result),
        Some(Err(e)) => Err(anyhow::anyhow!("Error in find_symbol_definitions: {}", e)),
        None => Err(anyhow::anyhow!("Failed to handle find_symbol_definitions")),
    }
}

/// Call the get_symbol_subgraph function from codex-core
pub fn call_get_symbol_subgraph(args: Value) -> Result<Value> {
    // Ensure the graph is up-to-date before processing
    ensure_graph_updated()?;
    
    match handle_get_symbol_subgraph(args) {
        Some(Ok(result)) => Ok(result),
        Some(Err(e)) => Err(anyhow::anyhow!("Error in get_symbol_subgraph: {}", e)),
        None => Err(anyhow::anyhow!("Failed to handle get_symbol_subgraph")),
    }
}