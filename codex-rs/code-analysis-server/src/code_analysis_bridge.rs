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
use std::path::Path;

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
            info!("Code graph initialized successfully for: {}", current_dir.display());
            Ok(())
        },
        Err(e) => {
            error!("Failed to initialize code graph: {}", e);
            Err(anyhow::anyhow!("Failed to initialize code graph: {}", e))
        }
    }
}

/// Initialize the code graph for a specific directory and wait for completion
pub async fn init_code_graph_and_wait(project_dir: Option<&Path>) -> Result<()> {
    let target_dir = if let Some(dir) = project_dir {
        dir.to_path_buf()
    } else {
        std::env::current_dir()?
    };
    
    let start_time = std::time::Instant::now();
    info!("Starting code graph initialization for: {}", target_dir.display());
    
    // Force synchronous initialization to ensure it completes
    match graph_manager::initialize_graph_async(&target_dir).await {
        Ok(_) => {
            let elapsed = start_time.elapsed();
            let elapsed_ms = elapsed.as_millis();
            let elapsed_secs = elapsed.as_secs_f64();
            
            // Get the detailed status from the graph manager
            let status = graph_manager::get_graph_status();
            match status {
                codex_core::code_analysis::graph_manager::GraphStatus::Ready { 
                    files_processed, 
                    symbols_found, 
                    initialization_time_ms 
                } => {
                    info!("Code graph initialization completed successfully!");
                    info!("Summary: {} files processed, {} symbols found", files_processed, symbols_found);
                    info!("Total time: {:.2}s ({}ms)", elapsed_secs, initialization_time_ms);
                },
                _ => {
                    info!("Code graph initialization completed in {:.2}s ({}ms)", elapsed_secs, elapsed_ms);
                }
            }
            
            // Log some statistics about what was parsed
            if let Some(symbols) = graph_manager::get_symbols() {
                // Count files by extension
                let mut file_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
                for symbol in symbols.values() {
                    if let Some(ext) = std::path::Path::new(&symbol.file_path).extension() {
                        *file_counts.entry(ext.to_string_lossy().to_string()).or_insert(0) += 1;
                    }
                }
                
                if !file_counts.is_empty() {
                    info!("Files with symbols by extension:");
                    for (ext, count) in file_counts {
                        info!("  .{}: {} files", ext, count);
                    }
                }
            }
            
            Ok(())
        },
        Err(e) => {
            let elapsed = start_time.elapsed();
            error!("Code graph initialization failed after {:.2}s: {}", elapsed.as_secs_f64(), e);
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
