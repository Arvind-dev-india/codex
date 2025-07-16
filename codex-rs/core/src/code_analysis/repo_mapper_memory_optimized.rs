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
        
        // Collect files to process with timing (optimized)
        let file_discovery_start = std::time::Instant::now();
        let mut files_to_process = Vec::<PathBuf>::new();
        self.collect_files_public(&root_path, &mut files_to_process)?;
        let file_discovery_time = file_discovery_start.elapsed();
        
        tracing::info!("File discovery completed in {:.2}s", file_discovery_time.as_secs_f64());
        
        // Skip expensive sorting for better performance - random order is fine for parallel processing
        // files_to_process.sort_by_cached_key(|path| {
        //     std::fs::metadata(path).map(|m| m.len()).unwrap_or(0)
        // });
        
        tracing::info!("Found {} files to process", files_to_process.len());
        
        // Process files in batches to control memory usage - optimized for speed
        let cpu_count = rayon::current_num_threads();
        // Significantly increase batch size for better throughput - was too conservative
        let batch_size = std::cmp::min(500, std::cmp::max(cpu_count * 8, files_to_process.len() / 4)); // More aggressive batch size
        let total_batches = (files_to_process.len() + batch_size - 1) / batch_size;
        
        tracing::info!("Using batch size {} for {} CPU threads", batch_size, cpu_count);
        
        for (batch_idx, batch) in files_to_process.chunks(batch_size).enumerate() {
            let batch_start = std::time::Instant::now();
            tracing::info!("Processing batch {}/{} ({} files)", 
                          batch_idx + 1, total_batches, batch.len());
            
            // Process batch in parallel for better performance
            let batch_results: Vec<Result<(Vec<CodeSymbol>, Vec<super::context_extractor::SymbolReference>), String>> = batch
                .par_iter()
                .map(|file_path| {
                    // Create isolated context extractor for this file
                    let mut extractor = create_context_extractor();
                    
                    // Extract symbols
                    match extractor.extract_symbols_from_file_incremental(&file_path.to_string_lossy()) {
                        Ok(()) => {
                            // Collect symbols and references
                            let symbols: Vec<CodeSymbol> = extractor.get_symbols().values().cloned().collect();
                            let references: Vec<super::context_extractor::SymbolReference> = extractor.get_references().to_vec();
                            Ok((symbols, references))
                        }
                        Err(e) => Err(e)
                    }
                })
                .collect();
            
            let batch_time = batch_start.elapsed();
            tracing::debug!("Batch {} processed in {:.2}s ({:.1} files/sec)", 
                          batch_idx + 1, batch_time.as_secs_f64(), 
                          batch.len() as f64 / batch_time.as_secs_f64());
            
            // Store results in memory-optimized storage and collect references
            for (file_idx, result) in batch_results.into_iter().enumerate() {
                match result {
                    Ok((symbols, references)) => {
                        // Add references to main context extractor
                        for reference in references {
                            self.add_reference(reference);
                        }
                        
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
            
            // Reduce cleanup frequency - was too aggressive and slowing down processing
            if batch_idx % 10 == 9 {
                if let Err(e) = storage.cleanup_memory() {
                    tracing::warn!("Failed to cleanup memory: {}", e);
                }
            }
        }
        
        let (_total_files, files_processed, files_failed, _) = self.get_parsing_statistics();
        tracing::info!("Memory-optimized repository mapping completed: {} files processed, {} failed", 
                      files_processed, files_failed);
        
        // NOTE: Graph building is handled by graph_manager.rs after this function returns
        // This avoids duplicate graph building and ensures proper symbol-reference integration
        
        Ok(())
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
                    // Skip hidden directories and common directories to ignore (match feat/azure branch)
                    let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if !dir_name.starts_with('.') && !["node_modules", "target", "dist"].contains(&dir_name) {
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