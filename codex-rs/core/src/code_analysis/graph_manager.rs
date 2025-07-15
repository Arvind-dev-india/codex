//! Global code graph manager for automatic graph maintenance.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use once_cell::sync::Lazy;

use super::memory_optimized_storage::{ThreadSafeStorage, StorageConfig};

use super::repo_mapper::RepoMapper;
use super::context_extractor::{CodeSymbol, SymbolReference};

/// Status of the code graph initialization
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphStatus {
    /// Not started yet
    NotStarted,
    /// Currently initializing
    Initializing { 
        files_processed: usize,
        total_files: usize,
        current_file: Option<String>,
    },
    /// Successfully initialized
    Ready {
        files_processed: usize,
        symbols_found: usize,
        initialization_time_ms: u64,
    },
    /// Failed to initialize
    Failed {
        error: String,
    },
}

/// Debug function to dump graph state
pub fn dump_graph_state() -> Result<(), String> {
    let manager = get_graph_manager();
    let manager = manager.read()
        .map_err(|e| format!("Failed to acquire read lock: {}", e))?;
    manager.dump_graph_state()
}

/// File metadata for change detection
#[derive(Debug, Clone)]
struct FileMetadata {
    _path: PathBuf,
    last_modified: SystemTime,
    size: u64,
    content_hash: u64,
}

/// Global code graph manager
pub struct CodeGraphManager {
    /// Memory-optimized storage for symbols
    symbol_storage: Option<ThreadSafeStorage>,
    /// Current repository mapper with parsed data
    repo_mapper: Option<RepoMapper>,
    /// Root path of the current workspace
    root_path: Option<PathBuf>,
    /// Supplementary projects from --supplementary arguments
    supplementary_projects: Vec<crate::config_types::SupplementaryProjectConfig>,
    /// File metadata for change detection
    file_metadata: HashMap<PathBuf, FileMetadata>,
    /// Whether the graph has been initialized
    initialized: bool,
    /// Current status of the graph initialization
    status: GraphStatus,
    /// Memory configuration
    memory_config: StorageConfig,
}

impl CodeGraphManager {
    fn new() -> Self {
        let memory_config = StorageConfig::for_large_projects();
        
        Self {
            symbol_storage: None,
            repo_mapper: None,
            root_path: None,
            supplementary_projects: Vec::new(),
            file_metadata: HashMap::new(),
            initialized: false,
            status: GraphStatus::NotStarted,
            memory_config,
        }
    }
    
    /// Initialize memory-optimized storage
    fn initialize_storage(&mut self) -> Result<(), String> {
        if self.symbol_storage.is_none() {
            tracing::info!("Initializing memory-optimized symbol storage with config: cache_size={}, max_memory={}MB", 
                          self.memory_config.cache_size, self.memory_config.max_memory_mb);
            
            let storage = ThreadSafeStorage::new(self.memory_config.clone())?;
            self.symbol_storage = Some(storage);
        }
        Ok(())
    }
    
    /// Get memory usage statistics
    pub fn get_memory_statistics(&self) -> Option<super::memory_optimized_storage::StorageStatistics> {
        if let Some(ref storage) = self.symbol_storage {
            storage.get_statistics().ok()
        } else {
            None
        }
    }
    
    /// Force memory cleanup
    pub fn cleanup_memory(&self) -> Result<(), String> {
        if let Some(ref storage) = self.symbol_storage {
            storage.cleanup_memory()?;
            tracing::info!("Memory cleanup completed");
        }
        Ok(())
    }
    
    /// Set memory limit (useful for different project sizes)
    pub fn set_memory_limit(&mut self, limit_mb: usize) {
        self.memory_config.max_memory_mb = limit_mb;
        tracing::info!("Updated memory limit to {} MB", limit_mb);
        
        // If storage is already initialized, we'd need to recreate it
        // For now, just log that it will take effect on next initialization
        if self.symbol_storage.is_some() {
            tracing::warn!("Memory limit change will take effect on next initialization");
        }
    }

    /// Debug method to dump graph state
    pub fn dump_graph_state(&self) -> Result<(), String> {
        if let Some(ref mapper) = self.repo_mapper {
            tracing::info!("Graph state dump:");
            tracing::info!("  Root path: {:?}", self.root_path);
            tracing::info!("  Initialized: {}", self.initialized);
            tracing::info!("  Status: {:?}", self.status);

            let symbols = mapper.get_all_symbols();
            tracing::info!("  Total symbols: {}", symbols.len());

            // Get file count
            let mut unique_files = std::collections::HashSet::new();
            for symbol in symbols.values() {
                unique_files.insert(&symbol.file_path);
            }
            tracing::info!("  Unique files with symbols: {}", unique_files.len());

            // Show first few files
            for (i, file) in unique_files.iter().enumerate() {
                if i < 5 {
                    tracing::info!("    File {}: '{}'", i, file);
                }
            }
        } else {
            tracing::info!("Graph not initialized");
        }
        Ok(())
    }

    /// Initialize or update the code graph for a given root path
    pub fn ensure_graph_for_path(&mut self, root_path: &Path) -> Result<(), String> {
        // Check if we need to initialize or if the root path changed
        let needs_full_rebuild = !self.initialized || 
            self.root_path.as_ref().map(|p| p.as_path()) != Some(root_path);

        if needs_full_rebuild {
            tracing::info!("Initializing code graph for path: {}", root_path.display());
            self.full_rebuild(root_path)?;
        } else {
            // Check for file changes and update incrementally
            self.incremental_update()?;
        }

        Ok(())
    }

    /// Perform a full rebuild of the code graph
    fn full_rebuild(&mut self, root_path: &Path) -> Result<(), String> {
        // Force complete cleanup of any existing state to prevent contamination
        self.force_complete_cleanup()?;
        
        // Initialize memory-optimized storage first
        self.initialize_storage()?;
        
        // Initialize storage for this specific project (clears old data)
        if let Some(ref storage) = self.symbol_storage {
            storage.initialize_for_project(root_path)?;
        }
        
        // Create new repository mapper
        let mut repo_mapper = RepoMapper::new(root_path);
        
        // Map the repository with memory optimization
        repo_mapper.map_repository_with_storage(self.symbol_storage.as_ref())?;
        
        // Update file metadata
        self.update_file_metadata(root_path)?;
        
        // Store the new state
        self.repo_mapper = Some(repo_mapper);
        self.root_path = Some(root_path.to_path_buf());
        self.initialized = true;
        
        // Log memory statistics
        if let Some(stats) = self.get_memory_statistics() {
            tracing::info!("Code graph initialized successfully for path: {} - Memory: {}MB/{} MB, Cache: {}/{}", 
                          root_path.display(), stats.memory_usage_mb, stats.memory_limit_mb, 
                          stats.cache_size, stats.cache_capacity);
        }
        
        Ok(())
    }


    /// Perform incremental update if files have changed
    fn incremental_update(&mut self) -> Result<(), String> {
        let root_path = self.root_path.as_ref()
            .ok_or("No root path set")?;

        // Check for file changes
        let changed_files = self.detect_file_changes(root_path)?;
        
        if !changed_files.is_empty() {
            tracing::info!("Detected {} changed files, updating graph...", changed_files.len());
            
            if let Some(ref mut repo_mapper) = self.repo_mapper {
                // Update the repository mapper
                repo_mapper.update_repository()?;
                
                // Update file metadata
                let root_path_clone = root_path.clone();
                self.update_file_metadata(&root_path_clone)?;
                
                tracing::info!("Code graph updated successfully");
            }
        }

        Ok(())
    }

    /// Detect which files have changed since last scan
    fn detect_file_changes(&self, root_path: &Path) -> Result<Vec<PathBuf>, String> {
        let mut changed_files = Vec::new();
        let mut current_files = std::collections::HashSet::new();
        
        // Scan for supported files
        self.scan_for_files(root_path, &mut |file_path| {
            current_files.insert(file_path.clone());
            
            let metadata = std::fs::metadata(&file_path)
                .map_err(|e| format!("Failed to get metadata for {}: {}", file_path.display(), e))?;
            
            // Calculate content hash for more accurate change detection
            let content_hash = match std::fs::read(&file_path) {
                Ok(bytes) => match String::from_utf8(bytes.clone()) {
                    Ok(content) => {
                        use std::collections::hash_map::DefaultHasher;
                        use std::hash::{Hash, Hasher};
                        let mut hasher = DefaultHasher::new();
                        content.hash(&mut hasher);
                        hasher.finish()
                    }
                    Err(_) => {
                        // For non-UTF-8 files, hash the raw bytes directly
                        use std::collections::hash_map::DefaultHasher;
                        use std::hash::{Hash, Hasher};
                        let mut hasher = DefaultHasher::new();
                        bytes.hash(&mut hasher);
                        hasher.finish()
                    }
                }
                Err(_) => 0, // Fallback to 0 if can't read content
            };
            
            let current_metadata = FileMetadata {
                _path: file_path.clone(),
                last_modified: metadata.modified()
                    .map_err(|e| format!("Failed to get modified time for {}: {}", file_path.display(), e))?,
                size: metadata.len(),
                content_hash,
            };
            
            // Check if file is new or changed
            if let Some(old_metadata) = self.file_metadata.get(&file_path) {
                if old_metadata.last_modified != current_metadata.last_modified ||
                   old_metadata.size != current_metadata.size ||
                   old_metadata.content_hash != current_metadata.content_hash {
                    changed_files.push(file_path);
                }
            } else {
                // New file
                changed_files.push(file_path);
            }
            
            Ok(())
        })?;

        // Find deleted files (files in metadata but not in current scan)
        let deleted_files: Vec<PathBuf> = self.file_metadata.keys()
            .filter(|path| !current_files.contains(*path))
            .cloned()
            .collect();
        
        // Add deleted files to changed list for cleanup
        changed_files.extend(deleted_files);

        Ok(changed_files)
    }

    /// Update file metadata for all supported files
    fn update_file_metadata(&mut self, root_path: &Path) -> Result<(), String> {
        let mut new_metadata = HashMap::new();
        
        self.scan_for_files(root_path, &mut |file_path| {
            let metadata = std::fs::metadata(&file_path)
                .map_err(|e| format!("Failed to get metadata for {}: {}", file_path.display(), e))?;
            
            // Calculate content hash for accurate change detection
            let content_hash = match std::fs::read(&file_path) {
                Ok(bytes) => match String::from_utf8(bytes.clone()) {
                    Ok(content) => {
                        use std::collections::hash_map::DefaultHasher;
                        use std::hash::{Hash, Hasher};
                        let mut hasher = DefaultHasher::new();
                        content.hash(&mut hasher);
                        hasher.finish()
                    }
                    Err(_) => {
                        // For non-UTF-8 files, hash the raw bytes directly
                        use std::collections::hash_map::DefaultHasher;
                        use std::hash::{Hash, Hasher};
                        let mut hasher = DefaultHasher::new();
                        bytes.hash(&mut hasher);
                        hasher.finish()
                    }
                }
                Err(_) => 0, // Fallback to 0 if can't read content
            };
            
            let file_metadata = FileMetadata {
                _path: file_path.clone(),
                last_modified: metadata.modified()
                    .map_err(|e| format!("Failed to get modified time for {}: {}", file_path.display(), e))?,
                size: metadata.len(),
                content_hash,
            };
            
            new_metadata.insert(file_path, file_metadata);
            Ok(())
        })?;

        self.file_metadata = new_metadata;
        Ok(())
    }

    /// Scan directory for supported files
    fn scan_for_files<F>(&self, dir_path: &Path, callback: &mut F) -> Result<(), String>
    where
        F: FnMut(PathBuf) -> Result<(), String>,
    {
        use super::parser_pool::SupportedLanguage;
        
        let entries = std::fs::read_dir(dir_path)
            .map_err(|e| format!("Failed to read directory {}: {}", dir_path.display(), e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                // Skip hidden directories and common directories to ignore
                let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if !dir_name.starts_with('.') && !["node_modules", "target", "dist", "build"].contains(&dir_name) {
                    self.scan_for_files(&path, callback)?;
                }
            } else if path.is_file() {
                // Check if it's a supported file type
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if SupportedLanguage::from_extension(ext).is_some() {
                        callback(path)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Get the current repository mapper (if initialized)
    pub fn get_repo_mapper(&self) -> Option<&RepoMapper> {
        self.repo_mapper.as_ref()
    }

    pub fn get_supplementary_projects(&self) -> &[crate::config_types::SupplementaryProjectConfig] {
        &self.supplementary_projects
    }

    /// Get the symbol storage (if initialized)
    pub fn get_symbol_storage(&self) -> Option<&ThreadSafeStorage> {
        self.symbol_storage.as_ref()
    }

    /// Get all symbols from the current graph (now uses memory-optimized storage)
    pub fn get_symbols(&self) -> Option<HashMap<String, CodeSymbol>> {
        // Note: This method now returns owned HashMap instead of reference
        // to work with the memory-optimized storage
        if let Some(ref _storage) = self.symbol_storage {
            // This is a simplified implementation - in practice, you'd want to
            // implement a more efficient way to get all symbols
            tracing::warn!("get_symbols() called - this loads all symbols into memory. Consider using get_symbols_for_file() instead.");
            None // Return None to encourage using more memory-efficient methods
        } else {
            self.repo_mapper.as_ref().map(|rm| rm.get_all_symbols().clone())
        }
    }
    
    /// Get symbols for a specific file (memory-optimized)
    pub fn get_symbols_for_file(&self, file_path: &str) -> Vec<CodeSymbol> {
        if let Some(ref storage) = self.symbol_storage {
            storage.get_symbols_for_file(file_path).unwrap_or_default()
        } else if let Some(ref repo_mapper) = self.repo_mapper {
            repo_mapper.get_symbols_for_file(file_path).into_iter().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Find symbol references by name
    pub fn find_symbol_references(&self, symbol_name: &str) -> Vec<&SymbolReference> {
        self.repo_mapper.as_ref()
            .map(|rm| rm.find_symbol_references(symbol_name))
            .unwrap_or_default()
    }

    /// Find symbol definitions by name
    pub fn find_symbol_definitions(&self, symbol_name: &str) -> Vec<&CodeSymbol> {
        self.repo_mapper.as_ref()
            .map(|rm| rm.find_symbol_definitions(symbol_name))
            .unwrap_or_default()
    }

    /// Check if the graph is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get the current root path
    pub fn get_root_path(&self) -> Option<&Path> {
        self.root_path.as_deref()
    }

    /// Get the current status of the graph initialization
    pub fn get_status(&self) -> &GraphStatus {
        &self.status
    }

    /// Update the status of the graph initialization
    pub fn set_status(&mut self, status: GraphStatus) {
        self.status = status;
    }
    
    /// Force complete cleanup of all state to prevent contamination between runs
    pub fn force_complete_cleanup(&mut self) -> Result<(), String> {
        tracing::info!("Performing complete cleanup to prevent cross-run contamination");
        
        // Clear all storage
        if let Some(ref storage) = self.symbol_storage {
            storage.clear_all_data()?;
        }
        
        // Clean up .codex directory files that might cause contamination
        if let Some(ref root_path) = self.root_path {
            let codex_dir = root_path.join(".codex");
            if codex_dir.exists() {
                // Clean up supplementary projects debug log
                let debug_log_path = codex_dir.join("supplementary_projects_debug.log");
                if debug_log_path.exists() {
                    if let Err(e) = std::fs::remove_file(&debug_log_path) {
                        tracing::warn!("Failed to remove old supplementary projects debug log: {}", e);
                    } else {
                        tracing::info!("Removed old supplementary projects debug log: {}", debug_log_path.display());
                    }
                }
                
                // Clean up any other potential contamination files
                // Note: We don't remove the entire .codex directory as it might contain user data
                let contamination_files = [
                    "cross_project_edges.log",
                    "symbol_cache.json", 
                    "project_state.json"
                ];
                
                for file_name in &contamination_files {
                    let file_path = codex_dir.join(file_name);
                    if file_path.exists() {
                        if let Err(e) = std::fs::remove_file(&file_path) {
                            tracing::warn!("Failed to remove contamination file {}: {}", file_path.display(), e);
                        } else {
                            tracing::info!("Removed contamination file: {}", file_path.display());
                        }
                    }
                }
            }
        }
        
        // Reset all state
        self.symbol_storage = None;
        self.repo_mapper = None;
        self.root_path = None;
        self.file_metadata.clear();
        self.initialized = false;
        self.status = GraphStatus::NotStarted;
        
        tracing::info!("Complete cleanup finished - all state cleared");
        Ok(())
    }
}

/// Global instance of the code graph manager
static GRAPH_MANAGER: Lazy<Arc<RwLock<CodeGraphManager>>> = Lazy::new(|| {
    Arc::new(RwLock::new(CodeGraphManager::new()))
});

/// Get a reference to the global code graph manager
pub fn get_graph_manager() -> Arc<RwLock<CodeGraphManager>> {
    GRAPH_MANAGER.clone()
}

/// Ensure the code graph is initialized for the given path
pub fn ensure_graph_for_path(root_path: &Path) -> Result<(), String> {
    let manager = get_graph_manager();
    let mut manager = manager.write()
        .map_err(|e| format!("Failed to acquire write lock: {}", e))?;
    manager.ensure_graph_for_path(root_path)
}

/// Initialize the code graph asynchronously for the given path
pub async fn initialize_graph_async(root_path: &Path) -> Result<(), String> {
    let start_time = std::time::Instant::now();
    tracing::info!("Starting background code graph initialization for path: {}", root_path.display());
    
    // Check if already initialized or currently initializing for this path
    {
        let manager = get_graph_manager();
        let manager = manager.read()
            .map_err(|e| format!("Failed to acquire read lock: {}", e))?;
        
        if manager.initialized && manager.root_path.as_ref().map(|p| p.as_path()) == Some(root_path) {
            tracing::info!("Code graph already initialized for path: {}", root_path.display());
            return Ok(());
        }
        
        // Also check if already initializing
        if matches!(manager.status, GraphStatus::Initializing { .. }) {
            tracing::info!("Code graph initialization already in progress for path: {}", root_path.display());
            return Ok(());
        }
    }
    
    // Set status to initializing
    {
        let manager = get_graph_manager();
        let mut manager = manager.write()
            .map_err(|e| format!("Failed to acquire write lock: {}", e))?;
        manager.set_status(GraphStatus::Initializing {
            files_processed: 0,
            total_files: 0,
            current_file: None,
        });
    }
    
    // Create and initialize the repository mapper in a blocking task
    let root_path_clone = root_path.to_path_buf();
    let result = tokio::task::spawn_blocking(move || {
        let task_start = std::time::Instant::now();
        tracing::info!("Starting spawn_blocking task for graph initialization");
        
        // Initialize memory-optimized storage for this task
        let storage_start = std::time::Instant::now();
        let storage_config = StorageConfig::for_large_projects();
        let storage = ThreadSafeStorage::new(storage_config)?;
        storage.initialize_for_project(&root_path_clone)?;
        tracing::info!("Storage initialization completed in {:.2}s", storage_start.elapsed().as_secs_f64());
        
        let mut repo_mapper = RepoMapper::new(&root_path_clone);
        
        // Use memory-optimized mapping
        let mapping_start = std::time::Instant::now();
        repo_mapper.map_repository_with_storage(Some(&storage))?;
        tracing::info!("Repository mapping completed in {:.2}s", mapping_start.elapsed().as_secs_f64());
        
        // CRITICAL: Build the graph after parsing symbols
        // But we need to build it from the storage, not the empty context extractor
        let graph_start = std::time::Instant::now();
        repo_mapper.build_graph_from_storage(&storage)?;
        tracing::info!("Graph building completed in {:.2}s", graph_start.elapsed().as_secs_f64());
        
        let task_total = task_start.elapsed();
        tracing::info!("spawn_blocking task completed in {:.2}s - about to return", task_total.as_secs_f64());
        
        Ok::<(RepoMapper, ThreadSafeStorage), String>((repo_mapper, storage))
    }).await.map_err(|e| format!("Task join error: {}", e))?;
    
    tracing::info!("spawn_blocking task returned, processing results...");
    
    match result {
        Ok((repo_mapper, storage)) => {
            tracing::info!("spawn_blocking result received successfully");
            
            // Now acquire the write lock and update the manager
            let lock_start = std::time::Instant::now();
            let manager = get_graph_manager();
            let mut manager = manager.write()
                .map_err(|e| format!("Failed to acquire write lock: {}", e))?;
            tracing::info!("Write lock acquired in {:.2}s", lock_start.elapsed().as_secs_f64());
            
            // Start file metadata update in background for incremental updates
            let metadata_start = std::time::Instant::now();
            tracing::info!("Starting background file metadata update for incremental change detection");
            
            // Clone data for background task
            let root_path_bg = root_path.to_path_buf();
            let manager_clone = Arc::clone(&get_graph_manager());
            
            // Start background metadata update (truly non-blocking and deferred)
            let manager_weak = Arc::downgrade(&get_graph_manager());
            tokio::spawn(async move {
                // Defer metadata update to avoid blocking initialization
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                
                let bg_start = std::time::Instant::now();
                tracing::info!("Background metadata update started (deferred)");
                
                // Use weak reference to avoid keeping manager alive
                if let Some(manager_arc) = manager_weak.upgrade() {
                    // Run metadata update in background thread with timeout
                    let result = tokio::time::timeout(
                        tokio::time::Duration::from_secs(30),
                        tokio::task::spawn_blocking(move || {
                            if let Ok(mut manager) = manager_arc.write() {
                                manager.update_file_metadata(&root_path_bg)
                            } else {
                                Err("Failed to acquire write lock for metadata".to_string())
                            }
                        })
                    ).await;
                    
                    match result {
                        Ok(Ok(Ok(()))) => {
                            let bg_time = bg_start.elapsed();
                            tracing::info!("Background file metadata update completed in {:.2}s", bg_time.as_secs_f64());
                        }
                        Ok(Ok(Err(e))) => {
                            tracing::warn!("Background metadata update failed: {}", e);
                        }
                        Ok(Err(e)) => {
                            tracing::warn!("Background metadata task failed: {}", e);
                        }
                        Err(_) => {
                            tracing::warn!("Background metadata update timed out after 30s");
                        }
                    }
                } else {
                    tracing::debug!("Manager dropped, skipping background metadata update");
                }
            });
            
            tracing::info!("Background metadata update started in {:.2}s (non-blocking)", metadata_start.elapsed().as_secs_f64());
            
            // Get statistics from storage (not from repo_mapper which might be empty)
            let symbols = storage.get_all_symbols()
                .map_err(|e| format!("Failed to get symbols for stats: {}", e))?;
            let symbols_count = symbols.len();
            let (_total_files, files_processed, _, _) = repo_mapper.get_parsing_statistics();
            let initialization_time_ms = start_time.elapsed().as_millis() as u64;
            
            // Store the new state
            manager.repo_mapper = Some(repo_mapper);
            manager.symbol_storage = Some(storage); // Store the storage for later access
            manager.root_path = Some(root_path.to_path_buf());
            manager.initialized = true;
            manager.set_status(GraphStatus::Ready {
                files_processed,
                symbols_found: symbols_count,
                initialization_time_ms,
            });
            
            tracing::info!("Code graph initialized successfully for path: {} ({} files, {} symbols, {}ms)", 
                          root_path.display(), files_processed, symbols_count, initialization_time_ms);
            Ok(())
        }
        Err(error) => {
            // Set status to failed
            let manager = get_graph_manager();
            let mut manager = manager.write()
                .map_err(|e| format!("Failed to acquire write lock: {}", e))?;
            manager.set_status(GraphStatus::Failed {
                error: error.clone(),
            });
            
            tracing::error!("Code graph initialization failed for path {}: {}", root_path.display(), error);
            Err(error)
        }
    }
}

/// Check if the code graph is initialized
pub fn is_graph_initialized() -> bool {
    let manager = get_graph_manager();
    if let Ok(manager) = manager.read() {
        manager.is_initialized()
    } else {
        false
    }
}

/// Get the current status of the code graph
pub fn get_graph_status() -> GraphStatus {
    let manager = get_graph_manager();
    if let Ok(manager) = manager.read() {
        manager.get_status().clone()
    } else {
        GraphStatus::Failed {
            error: "Failed to acquire read lock".to_string(),
        }
    }
}

/// Get the current root path from the global graph manager
pub fn get_root_path() -> Option<PathBuf> {
    let manager = get_graph_manager();
    if let Ok(manager) = manager.read() {
        manager.get_root_path().map(|p| p.to_path_buf())
    } else {
        None
    }
}

/// Get symbols from the global graph (memory-optimized)
pub fn get_symbols() -> Option<HashMap<String, CodeSymbol>> {
    // First, ensure the graph is up-to-date by checking for file changes
    if let Some(root_path) = get_root_path() {
        if let Err(e) = ensure_graph_for_path(&root_path) {
            tracing::warn!("Failed to update graph before getting symbols: {}", e);
        }
    }
    
    let manager = get_graph_manager();
    let manager = manager.read().ok()?;
    
    // Log warning about memory usage
    tracing::warn!("get_symbols() called - this may load many symbols into memory. Consider using get_symbols_for_file() instead.");
    
    manager.get_symbols()
}

/// Get symbols for a specific file (memory-optimized)
pub fn get_symbols_for_file(file_path: &str) -> Vec<CodeSymbol> {
    // First, ensure the graph is up-to-date by checking for file changes
    if let Some(root_path) = get_root_path() {
        if let Err(e) = ensure_graph_for_path(&root_path) {
            tracing::warn!("Failed to update graph before getting symbols: {}", e);
        }
    }
    
    let manager = get_graph_manager();
    if let Ok(manager) = manager.read() {
        manager.get_symbols_for_file(file_path)
    } else {
        Vec::new()
    }
}

/// Find symbol references using the global graph
pub fn find_symbol_references(symbol_name: &str) -> Vec<SymbolReference> {
    // First, ensure the graph is up-to-date by checking for file changes
    if let Some(root_path) = get_root_path() {
        if let Err(e) = ensure_graph_for_path(&root_path) {
            tracing::warn!("Failed to update graph before finding symbol references: {}", e);
        }
    }
    
    let manager = get_graph_manager();
    if let Ok(manager) = manager.read() {
        manager.find_symbol_references(symbol_name)
            .into_iter()
            .cloned()
            .collect()
    } else {
        Vec::new()
    }
}

/// Find symbol definitions using the global graph
pub fn find_symbol_definitions(symbol_name: &str) -> Vec<CodeSymbol> {
    // First, ensure the graph is up-to-date by checking for file changes
    if let Some(root_path) = get_root_path() {
        if let Err(e) = ensure_graph_for_path(&root_path) {
            tracing::warn!("Failed to update graph before finding symbol definitions: {}", e);
        }
    }
    
    let manager = get_graph_manager();
    if let Ok(manager) = manager.read() {
        manager.find_symbol_definitions(symbol_name)
            .into_iter()
            .cloned()
            .collect()
    } else {
        Vec::new()
    }
}

/// Get the code graph from the global manager
pub fn get_code_graph() -> Option<super::repo_mapper::CodeReferenceGraph> {
    let manager = get_graph_manager();
    if let Ok(manager) = manager.read() {
        manager.get_repo_mapper().map(|rm| rm.get_graph())
    } else {
        None
    }
}

/// Get a symbol subgraph from the global manager
pub fn get_symbol_subgraph(symbol_name: &str, max_depth: usize) -> Option<super::repo_mapper::CodeReferenceGraph> {
    let manager = get_graph_manager();
    if let Ok(manager) = manager.read() {
        manager.get_repo_mapper().map(|rm| rm.get_subgraph_bfs(symbol_name, max_depth))
    } else {
        None
    }
}

/// Force complete cleanup of all state to prevent contamination between runs
pub fn force_complete_cleanup() -> Result<(), String> {
    let manager = get_graph_manager();
    let mut manager = manager.write()
        .map_err(|e| format!("Failed to acquire write lock: {}", e))?;
    manager.force_complete_cleanup()
}
