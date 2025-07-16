//! Example usage of memory-optimized code analysis

use std::path::Path;
use super::graph_manager;
use super::memory_optimized_storage::StorageConfig;

/// Example: Initialize code analysis with memory optimization for large projects
pub async fn initialize_for_large_project(project_path: &Path) -> Result<(), String> {
    tracing::info!("Initializing memory-optimized code analysis for: {}", project_path.display());
    
    // Set memory limit to 4GB (much less than the original 24GB usage)
    {
        let manager = graph_manager::get_graph_manager();
        let mut manager = manager.write()
            .map_err(|e| format!("Failed to acquire write lock: {}", e))?;
        manager.set_memory_limit(4096); // 4GB limit
    }
    
    // Initialize the graph
    graph_manager::initialize_graph_async(project_path).await?;
    
    // Log memory statistics
    {
        let manager = graph_manager::get_graph_manager();
        let manager = manager.read()
            .map_err(|e| format!("Failed to acquire read lock: {}", e))?;
        
        if let Some(stats) = manager.get_memory_statistics() {
            tracing::info!("Initialization complete!");
            tracing::info!("Memory usage: {}MB / {} MB ({:.1}%)", 
                          stats.memory_usage_mb, 
                          stats.memory_limit_mb,
                          (stats.memory_usage_mb as f64 / stats.memory_limit_mb as f64) * 100.0);
            tracing::info!("Cache: {} / {} symbols ({:.1}% hit rate)", 
                          stats.cache_size, 
                          stats.cache_capacity,
                          stats.cache_hit_rate * 100.0);
            tracing::info!("Disk storage: {} symbols", stats.cold_storage_items);
        }
    }
    
    Ok(())
}

/// Example: Get symbols for a file (memory-efficient)
pub fn get_file_symbols_example(file_path: &str) -> Vec<super::context_extractor::CodeSymbol> {
    // This uses the memory-optimized storage automatically
    graph_manager::get_symbols_for_file(file_path)
}

/// Example: Monitor memory usage
pub fn monitor_memory_usage() -> Result<(), String> {
    let manager = graph_manager::get_graph_manager();
    let manager = manager.read()
        .map_err(|e| format!("Failed to acquire read lock: {}", e))?;
    
    if let Some(stats) = manager.get_memory_statistics() {
        println!("=== Memory Usage Statistics ===");
        println!("Memory: {}MB / {} MB", stats.memory_usage_mb, stats.memory_limit_mb);
        println!("Cache: {} / {} symbols", stats.cache_size, stats.cache_capacity);
        println!("Cache hit rate: {:.1}%", stats.cache_hit_rate * 100.0);
        println!("Cold storage: {} symbols", stats.cold_storage_items);
        println!("Disk reads: {}", stats.disk_reads);
        println!("Disk writes: {}", stats.disk_writes);
        println!("Memory cleanups: {}", stats.memory_cleanups);
        
        // Warn if memory usage is high
        let usage_percent = (stats.memory_usage_mb as f64 / stats.memory_limit_mb as f64) * 100.0;
        if usage_percent > 80.0 {
            println!("⚠️  WARNING: Memory usage is high ({:.1}%)", usage_percent);
            println!("💡 Consider calling cleanup_memory() or reducing cache size");
        }
        
        // Warn if cache hit rate is low
        if stats.cache_hit_rate < 0.8 {
            println!("⚠️  WARNING: Cache hit rate is low ({:.1}%)", stats.cache_hit_rate * 100.0);
            println!("💡 Consider increasing cache size for better performance");
        }
    } else {
        println!("Memory-optimized storage not initialized");
    }
    
    Ok(())
}

/// Example: Force memory cleanup
pub fn cleanup_memory_example() -> Result<(), String> {
    let manager = graph_manager::get_graph_manager();
    let manager = manager.read()
        .map_err(|e| format!("Failed to acquire read lock: {}", e))?;
    
    manager.cleanup_memory()?;
    println!("✅ Memory cleanup completed");
    
    Ok(())
}

/// Example: Configuration for different project sizes
pub fn get_config_for_project_size(estimated_files: usize) -> StorageConfig {
    if estimated_files < 1000 {
        // Small project
        StorageConfig {
            cache_size: 5000,
            max_memory_mb: 512,
            ..StorageConfig::default()
        }
    } else if estimated_files < 10000 {
        // Medium project
        StorageConfig {
            cache_size: 10000,
            max_memory_mb: 2048,
            ..StorageConfig::default()
        }
    } else {
        // Large project (like yours)
        StorageConfig::for_large_projects()
    }
}

/// Example: Complete workflow for large project analysis
pub async fn analyze_large_project_workflow(project_path: &Path) -> Result<(), String> {
    println!("🚀 Starting memory-optimized analysis of large project...");
    
    // Step 1: Initialize with memory optimization
    initialize_for_large_project(project_path).await?;
    
    // Step 2: Analyze a few sample files
    let sample_files = [
        "src/main.rs",
        "src/lib.rs", 
        "Program.cs",
        "UserService.cs"
    ];
    
    for file_path in &sample_files {
        let symbols = get_file_symbols_example(file_path);
        if !symbols.is_empty() {
            println!("📁 {}: {} symbols found", file_path, symbols.len());
            
            // Show first few symbols
            for (i, symbol) in symbols.iter().take(3).enumerate() {
                println!("  {}. {} ({}) at line {}", 
                        i + 1, symbol.name, symbol.symbol_type.as_str(), symbol.start_line);
            }
            if symbols.len() > 3 {
                println!("  ... and {} more", symbols.len() - 3);
            }
        }
    }
    
    // Step 3: Monitor memory usage
    monitor_memory_usage()?;
    
    // Step 4: Cleanup if needed
    cleanup_memory_example()?;
    
    println!("✅ Analysis complete!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    
    #[test]
    fn test_config_selection() {
        let small_config = get_config_for_project_size(500);
        assert_eq!(small_config.max_memory_mb, 512);
        
        let medium_config = get_config_for_project_size(5000);
        assert_eq!(medium_config.max_memory_mb, 2048);
        
        let large_config = get_config_for_project_size(15000);
        assert_eq!(large_config.max_memory_mb, 4096);
    }
}