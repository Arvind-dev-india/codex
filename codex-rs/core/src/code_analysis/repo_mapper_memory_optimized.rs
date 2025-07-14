//! Memory-optimized extensions for RepoMapper

use super::repo_mapper::RepoMapper;
use super::memory_optimized_storage::ThreadSafeStorage;
use super::context_extractor::{CodeSymbol, create_context_extractor};
use super::parser_pool::SupportedLanguage;
use std::path::{Path, PathBuf};
use rayon::prelude::*;
use tracing;

impl RepoMapper {
    /// Map repository with memory-optimized storage
    pub fn map_repository_with_storage(&mut self, storage: Option<&ThreadSafeStorage>) -> Result<(), String> {
        if let Some(storage) = storage {
            self.map_repository_memory_optimized(storage)
        } else {
            // Fallback to original method
            self.map_repository()
        }
    }
    
    /// Memory-optimized repository mapping
    fn map_repository_memory_optimized(&mut self, storage: &ThreadSafeStorage) -> Result<(), String> {
        let root_path = self.get_root_path().to_path_buf();
        tracing::info!("Starting memory-optimized repository mapping for path: {}", root_path.display());
        
        // Collect files to process with timing
        let file_discovery_start = std::time::Instant::now();
        let mut files_to_process = Vec::<PathBuf>::new();
        self.collect_files_public(&root_path, &mut files_to_process)?;
        let file_discovery_time = file_discovery_start.elapsed();
        
        tracing::info!("File discovery completed in {:.2}s", file_discovery_time.as_secs_f64());
        
        tracing::info!("Found {} files to process", files_to_process.len());
        
        // Process files in batches to control memory usage
        let batch_size = 100; // Process 100 files at a time
        let total_batches = (files_to_process.len() + batch_size - 1) / batch_size;
        
        for (batch_idx, batch) in files_to_process.chunks(batch_size).enumerate() {
            tracing::info!("Processing batch {}/{} ({} files)", 
                          batch_idx + 1, total_batches, batch.len());
            
            // Process batch sequentially to allow mutable access for references
            let mut batch_results = Vec::new();
            for file_path in batch {
                let result = self.process_file_for_symbols(&file_path.to_string_lossy());
                batch_results.push(result);
            }
            
            // Store results in memory-optimized storage
            for (file_idx, result) in batch_results.into_iter().enumerate() {
                match result {
                    Ok(symbols) => {
                        let file_path = &batch[file_idx];
                        
                        // Store symbols in memory-optimized storage
                        for symbol in &symbols {
                            if let Err(e) = storage.store_symbol(symbol) {
                                tracing::warn!("Failed to store symbol {}: {}", symbol.fqn, e);
                            }
                        }
                        
                        self.increment_parsed_successfully();
                        tracing::debug!("Processed {} symbols from {}", symbols.len(), file_path.display());
                    }
                    Err(e) => {
                        let file_path = &batch[file_idx];
                        tracing::warn!("Failed to process file {}: {}", file_path.display(), e);
                        self.increment_failed_to_parse();
                        self.add_failed_file(file_path.to_string_lossy().to_string());
                    }
                }
            }
            
            // Log progress and memory stats
            if let Ok(stats) = storage.get_statistics() {
                tracing::info!("Batch {} complete - Memory: {}MB, Cache: {}/{}, Hit rate: {:.1}%", 
                              batch_idx + 1, stats.memory_usage_mb, stats.cache_size, 
                              stats.cache_capacity, stats.cache_hit_rate * 100.0);
            }
            
            // Force cleanup every 5 batches to prevent memory buildup
            if batch_idx % 5 == 4 {
                if let Err(e) = storage.cleanup_memory() {
                    tracing::warn!("Failed to cleanup memory: {}", e);
                }
            }
        }
        
        let (_total_files, files_processed, files_failed, _) = self.get_parsing_statistics();
        tracing::info!("Memory-optimized repository mapping completed: {} files processed, {} failed", 
                      files_processed, files_failed);
        
        // NOTE: Graph building is now handled separately via build_graph_from_storage()
        // This avoids redundant work and ensures proper symbol-reference integration
        
        Ok(())
    }
    
    /// Process a single file and extract symbols
    fn process_file_for_symbols(&mut self, file_path: &str) -> Result<Vec<CodeSymbol>, String> {
        // Check if file is supported
        let path = std::path::Path::new(file_path);
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| format!("No extension found for file: {}", file_path))?;
        
        if SupportedLanguage::from_extension(extension).is_none() {
            return Ok(Vec::new()); // Skip unsupported files
        }
        
        // Skip very large files to prevent memory issues
        if let Ok(metadata) = std::fs::metadata(file_path) {
            if metadata.len() > 10 * 1024 * 1024 { // Skip files > 10MB
                tracing::debug!("Skipping large file: {} ({} MB)", 
                               file_path, metadata.len() / (1024 * 1024));
                return Ok(Vec::new());
            }
        }
        
        // Create context extractor for this file
        let mut extractor = create_context_extractor();
        
        // Extract symbols
        extractor.extract_symbols_from_file_incremental(file_path)?;
        
        // Store references in the main context extractor for graph building
        let references = extractor.get_references();
        tracing::debug!("Adding {} references from {} to main context extractor", 
                       references.len(), file_path);
        for reference in references {
            self.add_reference(reference.clone());
        }
        
        // Return symbols as vector
        let symbols: Vec<CodeSymbol> = extractor.get_symbols().values().cloned().collect();
        
        Ok(symbols)
    }
    
    /// Get memory usage statistics from the repository mapper
    pub fn get_memory_stats(&self) -> String {
        let (total_files, files_processed, files_failed, _) = self.get_parsing_statistics();
        format!("RepoMapper stats: {} files processed, {} failed, {} total files attempted",
                files_processed, files_failed, total_files)
    }
    
    /// Public method to collect files (wrapper around private method)
    pub fn collect_files_public(&self, dir_path: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
        self.collect_files_recursive(dir_path, files)
    }
    
    /// Optimized recursive file collection with better performance
    fn collect_files_recursive(&self, dir_path: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
        // Use a stack-based approach instead of recursion for better performance
        let mut dirs_to_process = vec![dir_path.to_path_buf()];
        
        while let Some(current_dir) = dirs_to_process.pop() {
            let entries = match std::fs::read_dir(&current_dir) {
                Ok(entries) => entries,
                Err(e) => {
                    tracing::warn!("Failed to read directory {}: {}", current_dir.display(), e);
                    continue;
                }
            };

            for entry in entries {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(e) => {
                        tracing::debug!("Failed to read directory entry: {}", e);
                        continue;
                    }
                };
                
                let path = entry.path();
                
                // Get file type in one call for efficiency
                let file_type = match entry.file_type() {
                    Ok(ft) => ft,
                    Err(_) => continue,
                };

                if file_type.is_dir() {
                    // Skip hidden directories and common directories to ignore
                    let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if !dir_name.starts_with('.') && 
                       !["node_modules", "target", "dist", "build", "bin", "obj", ".git", ".vs", "packages", "Debug", "Release", ".vscode"].contains(&dir_name) {
                        dirs_to_process.push(path);
                    }
                } else if file_type.is_file() {
                    // Check if it's a supported file type
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        if SupportedLanguage::from_extension(ext).is_some() {
                            files.push(path);
                        }
                    }
                }
            }
        }

        Ok(())
    }
    
}