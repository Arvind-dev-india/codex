//! Memory-optimized storage for code symbols using LRU cache + disk storage

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use lru::LruCache;
use std::num::NonZeroUsize;
use serde::{Serialize, Deserialize};
use tracing;

use super::context_extractor::CodeSymbol;

/// Memory-optimized symbol storage with LRU cache + disk persistence
pub struct MemoryOptimizedStorage {
    /// Hot cache: Recently accessed symbols in memory (configurable size)
    hot_cache: LruCache<String, CachedSymbol>,
    
    /// Cold storage index: Maps symbol FQNs to disk file paths
    cold_storage_index: HashMap<String, PathBuf>,
    
    /// File-to-symbols index for fast file-based lookups
    file_to_symbols: HashMap<String, Vec<String>>,
    
    /// Storage directory for persisted symbols
    storage_dir: PathBuf,
    
    /// Configuration
    config: StorageConfig,
    
    /// Statistics
    stats: StorageStats,
}

/// Configuration for memory-optimized storage
#[derive(Debug, Clone)]
pub struct StorageConfig {
    /// Maximum number of symbols to keep in memory
    pub cache_size: usize,
    /// Maximum memory usage in MB
    pub max_memory_mb: usize,
    /// Storage directory for disk cache
    pub storage_dir: PathBuf,
    /// Enable compression for disk storage
    pub use_compression: bool,
    /// Auto-cleanup threshold (0.0-1.0)
    pub cleanup_threshold: f32,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            cache_size: 10000,
            max_memory_mb: 2048, // 2GB default
            storage_dir: std::env::temp_dir().join("codex_symbol_cache"),
            use_compression: true,
            cleanup_threshold: 0.8,
        }
    }
}

impl StorageConfig {
    /// Configuration optimized for large projects (like yours with 24GB usage)
    pub fn for_large_projects() -> Self {
        Self {
            cache_size: 20000,  // Larger cache for better hit rate
            max_memory_mb: 4096, // 4GB limit (much less than your 24GB)
            storage_dir: Self::create_project_specific_cache_dir(),
            use_compression: true,
            cleanup_threshold: 0.75,
        }
    }
    
    /// Create project-specific cache directory to avoid cross-project contamination
    fn create_project_specific_cache_dir() -> PathBuf {
        let base_dir = std::env::temp_dir().join("codex_cache");
        
        // Use current working directory to create unique cache per project
        if let Ok(current_dir) = std::env::current_dir() {
            // Create hash of project path for unique directory name
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            
            let mut hasher = DefaultHasher::new();
            current_dir.hash(&mut hasher);
            let project_hash = hasher.finish();
            
            let project_name = current_dir
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            
            base_dir.join(format!("{}_{:x}", project_name, project_hash))
        } else {
            // Fallback to generic directory
            base_dir.join("default_project")
        }
    }
    
    /// Configuration for memory-constrained environments
    pub fn for_low_memory() -> Self {
        Self {
            cache_size: 5000,
            max_memory_mb: 512, // 512MB limit
            storage_dir: std::env::temp_dir().join("codex_low_memory_cache"),
            use_compression: true,
            cleanup_threshold: 0.6,
        }
    }
}

/// Optimized symbol representation for caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedSymbol {
    pub name: String,
    pub symbol_type: String,
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub parent: Option<String>,
    pub fqn: String,
    pub origin_project: Option<String>,
}

impl From<&CodeSymbol> for CachedSymbol {
    fn from(symbol: &CodeSymbol) -> Self {
        Self {
            name: symbol.name.clone(),
            symbol_type: symbol.symbol_type.as_str().to_string(),
            file_path: symbol.file_path.clone(),
            start_line: symbol.start_line,
            end_line: symbol.end_line,
            start_col: symbol.start_col,
            end_col: symbol.end_col,
            parent: symbol.parent.clone(),
            fqn: symbol.fqn.clone(),
            origin_project: symbol.origin_project.clone(),
        }
    }
}

impl Into<CodeSymbol> for CachedSymbol {
    fn into(self) -> CodeSymbol {
        use super::context_extractor::SymbolType;
        
        let symbol_type = match self.symbol_type.as_str() {
            "function" => SymbolType::Function,
            "method" => SymbolType::Method,
            "class" => SymbolType::Class,
            "struct" => SymbolType::Struct,
            "enum" => SymbolType::Enum,
            "interface" => SymbolType::Interface,
            "variable" => SymbolType::Variable,
            "constant" => SymbolType::Constant,
            "property" => SymbolType::Property,
            "import" => SymbolType::Import,
            "module" => SymbolType::Module,
            "package" => SymbolType::Package,
            _ => SymbolType::Function, // Default fallback
        };
        
        CodeSymbol {
            name: self.name,
            symbol_type,
            file_path: self.file_path,
            start_line: self.start_line,
            end_line: self.end_line,
            start_col: self.start_col,
            end_col: self.end_col,
            parent: self.parent,
            fqn: self.fqn,
            origin_project: self.origin_project,
        }
    }
}

/// Storage statistics
#[derive(Debug, Default)]
pub struct StorageStats {
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub disk_reads: u64,
    pub disk_writes: u64,
    pub memory_cleanups: u64,
}

impl StorageStats {
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total > 0 {
            self.cache_hits as f64 / total as f64
        } else {
            0.0
        }
    }
}

impl MemoryOptimizedStorage {
    /// Create new memory-optimized storage
    pub fn new(config: StorageConfig) -> Result<Self, String> {
        // Create storage directory
        std::fs::create_dir_all(&config.storage_dir)
            .map_err(|e| format!("Failed to create storage directory: {}", e))?;
        
        let cache_size = NonZeroUsize::new(config.cache_size)
            .ok_or("Cache size must be greater than 0")?;
        
        tracing::info!("Initializing memory-optimized storage: cache_size={}, max_memory={}MB, storage_dir={:?}", 
                      config.cache_size, config.max_memory_mb, config.storage_dir);
        
        Ok(Self {
            hot_cache: LruCache::new(cache_size),
            cold_storage_index: HashMap::new(),
            file_to_symbols: HashMap::new(),
            storage_dir: config.storage_dir.clone(),
            config,
            stats: StorageStats::default(),
        })
    }
    
    /// Store a symbol (automatically manages memory)
    pub fn store_symbol(&mut self, symbol: &CodeSymbol) -> Result<(), String> {
        let cached_symbol = CachedSymbol::from(symbol);
        let key = symbol.fqn.clone();
        
        // Check if we need to free up space
        if self.should_cleanup_memory() {
            self.cleanup_memory()?;
        }
        
        // If cache is at capacity, move LRU item to disk
        if self.hot_cache.len() >= self.hot_cache.cap().get() {
            if let Some((old_key, old_symbol)) = self.hot_cache.pop_lru() {
                self.move_to_cold_storage(old_key, &old_symbol)?;
            }
        }
        
        // Add to hot cache
        self.hot_cache.put(key.clone(), cached_symbol);
        
        // Update file-to-symbols index
        self.file_to_symbols
            .entry(symbol.file_path.clone())
            .or_insert_with(Vec::new)
            .push(key);
        
        Ok(())
    }
    
    /// Retrieve a symbol (checks cache first, then disk)
    pub fn get_symbol(&mut self, fqn: &str) -> Result<Option<CodeSymbol>, String> {
        // Check hot cache first
        if let Some(cached_symbol) = self.hot_cache.get(fqn) {
            self.stats.cache_hits += 1;
            return Ok(Some(cached_symbol.clone().into()));
        }
        
        // Cache miss - check cold storage
        self.stats.cache_misses += 1;
        
        if let Some(file_path) = self.cold_storage_index.get(fqn).cloned() {
            // Load from disk
            let cached_symbol = self.load_from_cold_storage(&file_path)?;
            let symbol: CodeSymbol = cached_symbol.clone().into();
            
            // Move back to hot cache (it's being accessed)
            self.hot_cache.put(fqn.to_string(), cached_symbol);
            self.cold_storage_index.remove(fqn);
            
            // Remove the cold storage file
            let _ = std::fs::remove_file(&file_path);
            
            return Ok(Some(symbol));
        }
        
        Ok(None)
    }
    
    /// Get all symbols for a file (optimized for file-based queries)
    pub fn get_symbols_for_file(&mut self, file_path: &str) -> Result<Vec<CodeSymbol>, String> {
        let mut symbols = Vec::new();
        
        // Get symbol keys for this file
        let symbol_keys = self.file_to_symbols.get(file_path).cloned().unwrap_or_default();
        
        for key in symbol_keys {
            if let Some(symbol) = self.get_symbol(&key)? {
                symbols.push(symbol);
            }
        }
        
        Ok(symbols)
    }
    
    /// Store multiple symbols efficiently (batch operation)
    pub fn store_symbols_batch(&mut self, symbols: Vec<&CodeSymbol>) -> Result<(), String> {
        tracing::debug!("Storing batch of {} symbols", symbols.len());
        
        for symbol in symbols {
            self.store_symbol(symbol)?;
        }
        
        // Log statistics periodically
        if self.stats.cache_hits + self.stats.cache_misses > 0 && 
           (self.stats.cache_hits + self.stats.cache_misses) % 1000 == 0 {
            self.log_statistics();
        }
        
        Ok(())
    }
    
    /// Check if memory cleanup is needed
    fn should_cleanup_memory(&self) -> bool {
        let current_usage = self.estimate_memory_usage();
        let limit = self.config.max_memory_mb * 1024 * 1024;
        let threshold = (limit as f32 * self.config.cleanup_threshold) as usize;
        
        current_usage > threshold
    }
    
    /// Estimate current memory usage
    fn estimate_memory_usage(&self) -> usize {
        // Rough estimation: each cached symbol ~200 bytes on average
        self.hot_cache.len() * 200 + 
        self.cold_storage_index.len() * 100 + // Index overhead
        self.file_to_symbols.len() * 50 // File index overhead
    }
    
    /// Perform memory cleanup
    fn cleanup_memory(&mut self) -> Result<(), String> {
        let initial_size = self.hot_cache.len();
        let target_size = self.hot_cache.cap().get() / 2; // Keep only half
        
        tracing::info!("Starting memory cleanup: {} -> {} symbols", initial_size, target_size);
        
        while self.hot_cache.len() > target_size {
            if let Some((key, symbol)) = self.hot_cache.pop_lru() {
                self.move_to_cold_storage(key, &symbol)?;
            }
        }
        
        self.stats.memory_cleanups += 1;
        tracing::info!("Memory cleanup completed: removed {} symbols", initial_size - self.hot_cache.len());
        
        Ok(())
    }
    
    /// Move symbol from hot cache to cold storage
    fn move_to_cold_storage(&mut self, key: String, symbol: &CachedSymbol) -> Result<(), String> {
        let filename = format!("symbol_{:x}.json", self.hash_key(&key));
        let file_path = self.storage_dir.join(filename);
        
        // Serialize symbol
        let json = serde_json::to_string(symbol)
            .map_err(|e| format!("Failed to serialize symbol: {}", e))?;
        
        // Apply compression if enabled
        let data = if self.config.use_compression {
            self.compress_data(json.as_bytes())
        } else {
            json.into_bytes()
        };
        
        // Write to disk
        std::fs::write(&file_path, data)
            .map_err(|e| format!("Failed to write to cold storage: {}", e))?;
        
        // Update index
        self.cold_storage_index.insert(key, file_path);
        self.stats.disk_writes += 1;
        
        Ok(())
    }
    
    /// Load symbol from cold storage
    fn load_from_cold_storage(&mut self, file_path: &Path) -> Result<CachedSymbol, String> {
        // Read from disk
        let data = std::fs::read(file_path)
            .map_err(|e| format!("Failed to read from cold storage: {}", e))?;
        
        // Decompress if needed
        let json = if self.config.use_compression {
            String::from_utf8(self.decompress_data(&data)?)
                .map_err(|e| format!("Invalid UTF-8 after decompression: {}", e))?
        } else {
            String::from_utf8(data)
                .map_err(|e| format!("Invalid UTF-8 in cold storage: {}", e))?
        };
        
        // Deserialize
        let symbol: CachedSymbol = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to deserialize symbol: {}", e))?;
        
        self.stats.disk_reads += 1;
        Ok(symbol)
    }
    
    /// Simple hash function for creating unique filenames
    fn hash_key(&self, key: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }
    
    /// Simple compression (can be replaced with proper compression library)
    fn compress_data(&self, data: &[u8]) -> Vec<u8> {
        // For now, just return as-is
        // In production, use flate2 or similar: flate2::write::GzEncoder
        data.to_vec()
    }
    
    /// Simple decompression
    fn decompress_data(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        // For now, just return as-is
        Ok(data.to_vec())
    }
    
    /// Get storage statistics
    pub fn get_statistics(&self) -> StorageStatistics {
        StorageStatistics {
            cache_size: self.hot_cache.len(),
            cache_capacity: self.hot_cache.cap().get(),
            cold_storage_items: self.cold_storage_index.len(),
            cache_hit_rate: self.stats.cache_hit_rate(),
            memory_usage_mb: self.estimate_memory_usage() / (1024 * 1024),
            memory_limit_mb: self.config.max_memory_mb,
            disk_reads: self.stats.disk_reads,
            disk_writes: self.stats.disk_writes,
            memory_cleanups: self.stats.memory_cleanups,
        }
    }
    
    /// Log current statistics
    pub fn log_statistics(&self) {
        let stats = self.get_statistics();
        tracing::info!(
            "Storage stats: {}% hit rate, {}/{} cache, {} on disk, {}MB/{} MB memory",
            (stats.cache_hit_rate * 100.0) as u32,
            stats.cache_size,
            stats.cache_capacity,
            stats.cold_storage_items,
            stats.memory_usage_mb,
            stats.memory_limit_mb
        );
    }
    
    /// Get all symbols (warning: this loads all symbols into memory)
    pub fn get_all_symbols(&mut self) -> Result<HashMap<String, CodeSymbol>, String> {
        let mut all_symbols = HashMap::new();
        
        // First, get all symbols from hot cache
        for (fqn, cached_symbol) in self.hot_cache.iter() {
            all_symbols.insert(fqn.clone(), cached_symbol.clone().into());
        }
        
        // Then, load all symbols from cold storage that aren't already in hot cache
        for (fqn, file_path) in &self.cold_storage_index.clone() {
            if !all_symbols.contains_key(fqn) {
                match self.load_from_cold_storage(file_path) {
                    Ok(cached_symbol) => {
                        all_symbols.insert(fqn.clone(), cached_symbol.into());
                    }
                    Err(e) => {
                        tracing::warn!("Failed to load symbol {} from disk: {}", fqn, e);
                    }
                }
            }
        }
        
        tracing::info!("Loaded {} symbols from storage (hot: {}, cold: {})", 
                      all_symbols.len(), self.hot_cache.len(), 
                      self.cold_storage_index.len() - self.hot_cache.len());
        
        Ok(all_symbols)
    }

    /// Force cleanup of old cold storage files
    pub fn cleanup_old_files(&mut self, max_age_hours: u64) -> Result<usize, String> {
        use std::time::{SystemTime, Duration};
        
        let cutoff_time = SystemTime::now() - Duration::from_secs(max_age_hours * 3600);
        let mut removed_count = 0;
        let mut keys_to_remove = Vec::new();
        
        for (key, file_path) in &self.cold_storage_index {
            if let Ok(metadata) = std::fs::metadata(file_path) {
                if let Ok(modified) = metadata.modified() {
                    if modified < cutoff_time {
                        let _ = std::fs::remove_file(file_path);
                        keys_to_remove.push(key.clone());
                        removed_count += 1;
                    }
                }
            }
        }
        
        for key in keys_to_remove {
            self.cold_storage_index.remove(&key);
        }
        
        tracing::info!("Cleaned up {} old cache files", removed_count);
        Ok(removed_count)
    }
    
    /// Clear all cached data (both memory and disk) - use when switching projects
    pub fn clear_all_data(&mut self) -> Result<(), String> {
        tracing::info!("Clearing all cached data for project isolation");
        
        // Clear memory cache
        self.hot_cache.clear();
        
        // Remove all disk files
        let mut removed_files = 0;
        for (_, file_path) in &self.cold_storage_index {
            if std::fs::remove_file(file_path).is_ok() {
                removed_files += 1;
            }
        }
        
        // Clear indices
        self.cold_storage_index.clear();
        self.file_to_symbols.clear();
        
        // Reset statistics
        self.stats = StorageStats::default();
        
        tracing::info!("Cleared all data: {} disk files removed, memory cache cleared", removed_files);
        Ok(())
    }
    
    /// Initialize for a new project (clears old data to prevent contamination)
    pub fn initialize_for_project(&mut self, project_path: &Path) -> Result<(), String> {
        tracing::info!("Initializing storage for new project: {}", project_path.display());
        
        // Always clear existing data to prevent cross-project contamination
        // This is especially important when supplementary projects change between runs
        self.clear_all_data()?;
        
        // Update storage directory to be project-specific
        let project_hash = {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            project_path.hash(&mut hasher);
            hasher.finish()
        };
        
        let project_name = project_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        
        let new_storage_dir = std::env::temp_dir()
            .join("codex_cache")
            .join(format!("{}_{:x}", project_name, project_hash));
        
        // Create new project-specific directory
        std::fs::create_dir_all(&new_storage_dir)
            .map_err(|e| format!("Failed to create project storage directory: {}", e))?;
        
        self.storage_dir = new_storage_dir;
        
        tracing::info!("Project storage initialized: {:?}", self.storage_dir);
        Ok(())
    }
}

/// Public statistics structure
#[derive(Debug)]
pub struct StorageStatistics {
    pub cache_size: usize,
    pub cache_capacity: usize,
    pub cold_storage_items: usize,
    pub cache_hit_rate: f64,
    pub memory_usage_mb: usize,
    pub memory_limit_mb: usize,
    pub disk_reads: u64,
    pub disk_writes: u64,
    pub memory_cleanups: u64,
}

/// Thread-safe wrapper for the storage
pub struct ThreadSafeStorage {
    storage: Arc<Mutex<MemoryOptimizedStorage>>,
}

impl ThreadSafeStorage {
    pub fn new(config: StorageConfig) -> Result<Self, String> {
        let storage = MemoryOptimizedStorage::new(config)?;
        Ok(Self {
            storage: Arc::new(Mutex::new(storage)),
        })
    }
    
    pub fn store_symbol(&self, symbol: &CodeSymbol) -> Result<(), String> {
        let mut storage = self.storage.lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;
        storage.store_symbol(symbol)
    }
    
    pub fn get_symbol(&self, fqn: &str) -> Result<Option<CodeSymbol>, String> {
        let mut storage = self.storage.lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;
        storage.get_symbol(fqn)
    }
    
    pub fn get_symbols_for_file(&self, file_path: &str) -> Result<Vec<CodeSymbol>, String> {
        let mut storage = self.storage.lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;
        storage.get_symbols_for_file(file_path)
    }
    
    pub fn get_statistics(&self) -> Result<StorageStatistics, String> {
        let storage = self.storage.lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;
        Ok(storage.get_statistics())
    }
    
    pub fn cleanup_memory(&self) -> Result<(), String> {
        let mut storage = self.storage.lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;
        storage.cleanup_memory()
    }
    
    /// Clear all data for project isolation
    pub fn clear_all_data(&self) -> Result<(), String> {
        let mut storage = self.storage.lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;
        storage.clear_all_data()
    }
    
    /// Initialize for a new project
    pub fn initialize_for_project(&self, project_path: &Path) -> Result<(), String> {
        let mut storage = self.storage.lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;
        storage.initialize_for_project(project_path)
    }
    
    /// Get all symbols (warning: this loads all symbols into memory)
    pub fn get_all_symbols(&self) -> Result<HashMap<String, CodeSymbol>, String> {
        let mut storage = self.storage.lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;
        storage.get_all_symbols()
    }
}