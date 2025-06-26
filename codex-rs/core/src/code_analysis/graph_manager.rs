//! Global code graph manager for automatic graph maintenance.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use once_cell::sync::Lazy;

use super::repo_mapper::RepoMapper;
use super::context_extractor::{CodeSymbol, SymbolReference};

/// File metadata for change detection
#[derive(Debug, Clone)]
struct FileMetadata {
    path: PathBuf,
    last_modified: SystemTime,
    size: u64,
}

/// Global code graph manager
pub struct CodeGraphManager {
    /// Current repository mapper with parsed data
    repo_mapper: Option<RepoMapper>,
    /// Root path of the current workspace
    root_path: Option<PathBuf>,
    /// File metadata for change detection
    file_metadata: HashMap<PathBuf, FileMetadata>,
    /// Whether the graph has been initialized
    initialized: bool,
}

impl CodeGraphManager {
    fn new() -> Self {
        Self {
            repo_mapper: None,
            root_path: None,
            file_metadata: HashMap::new(),
            initialized: false,
        }
    }

    /// Initialize or update the code graph for a given root path
    pub fn ensure_graph_for_path(&mut self, root_path: &Path) -> Result<(), String> {
        // Check if we need to initialize or if the root path changed
        let needs_full_rebuild = !self.initialized || 
            self.root_path.as_ref().map(|p| p.as_path()) != Some(root_path);

        if needs_full_rebuild {
            eprintln!("Initializing code graph for path: {}", root_path.display());
            self.full_rebuild(root_path)?;
        } else {
            // Check for file changes and update incrementally
            self.incremental_update()?;
        }

        Ok(())
    }

    /// Perform a full rebuild of the code graph
    fn full_rebuild(&mut self, root_path: &Path) -> Result<(), String> {
        // Create new repository mapper
        let mut repo_mapper = RepoMapper::new(root_path);
        
        // Map the repository
        repo_mapper.map_repository()?;
        
        // Update file metadata
        self.update_file_metadata(root_path)?;
        
        // Store the new state
        self.repo_mapper = Some(repo_mapper);
        self.root_path = Some(root_path.to_path_buf());
        self.initialized = true;
        
        eprintln!("Code graph initialized successfully");
        Ok(())
    }

    /// Perform incremental update if files have changed
    fn incremental_update(&mut self) -> Result<(), String> {
        let root_path = self.root_path.as_ref()
            .ok_or("No root path set")?;

        // Check for file changes
        let changed_files = self.detect_file_changes(root_path)?;
        
        if !changed_files.is_empty() {
            eprintln!("Detected {} changed files, updating graph...", changed_files.len());
            
            if let Some(ref mut repo_mapper) = self.repo_mapper {
                // Update the repository mapper
                repo_mapper.update_repository()?;
                
                // Update file metadata
                let root_path_clone = root_path.clone();
                self.update_file_metadata(&root_path_clone)?;
                
                eprintln!("Code graph updated successfully");
            }
        }

        Ok(())
    }

    /// Detect which files have changed since last scan
    fn detect_file_changes(&self, root_path: &Path) -> Result<Vec<PathBuf>, String> {
        let mut changed_files = Vec::new();
        
        // Scan for supported files
        self.scan_for_files(root_path, &mut |file_path| {
            let metadata = std::fs::metadata(&file_path)
                .map_err(|e| format!("Failed to get metadata for {}: {}", file_path.display(), e))?;
            
            let current_metadata = FileMetadata {
                path: file_path.clone(),
                last_modified: metadata.modified()
                    .map_err(|e| format!("Failed to get modified time for {}: {}", file_path.display(), e))?,
                size: metadata.len(),
            };
            
            // Check if file is new or changed
            if let Some(old_metadata) = self.file_metadata.get(&file_path) {
                if old_metadata.last_modified != current_metadata.last_modified ||
                   old_metadata.size != current_metadata.size {
                    changed_files.push(file_path);
                }
            } else {
                // New file
                changed_files.push(file_path);
            }
            
            Ok(())
        })?;

        Ok(changed_files)
    }

    /// Update file metadata for all supported files
    fn update_file_metadata(&mut self, root_path: &Path) -> Result<(), String> {
        let mut new_metadata = HashMap::new();
        
        self.scan_for_files(root_path, &mut |file_path| {
            let metadata = std::fs::metadata(&file_path)
                .map_err(|e| format!("Failed to get metadata for {}: {}", file_path.display(), e))?;
            
            let file_metadata = FileMetadata {
                path: file_path.clone(),
                last_modified: metadata.modified()
                    .map_err(|e| format!("Failed to get modified time for {}: {}", file_path.display(), e))?,
                size: metadata.len(),
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

    /// Get all symbols from the current graph
    pub fn get_symbols(&self) -> Option<&HashMap<String, CodeSymbol>> {
        self.repo_mapper.as_ref().map(|rm| rm.get_all_symbols())
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

/// Get symbols from the global graph
pub fn get_symbols() -> Option<HashMap<String, CodeSymbol>> {
    let manager = get_graph_manager();
    let manager = manager.read().ok()?;
    manager.get_symbols().cloned()
}

/// Find symbol references using the global graph
pub fn find_symbol_references(symbol_name: &str) -> Vec<SymbolReference> {
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