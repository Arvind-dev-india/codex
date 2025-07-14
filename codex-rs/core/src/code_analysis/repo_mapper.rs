//! Repository mapper for mapping code structure and relationships.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rayon::prelude::*;
use tracing;
use super::context_extractor::{CodeSymbol, ContextExtractor, SymbolReference, create_context_extractor};
use super::parser_pool::SupportedLanguage;

/// Node in the code reference graph
#[derive(Debug, Clone)]
pub struct CodeNode {
    pub id: String,
    pub name: String,
    pub node_type: CodeNodeType,
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
}

/// Type of code node
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeNodeType {
    File,
    Function,
    Method,
    Class,
    Struct,
    Module,
    // Add more node types as needed
}

/// Edge in the code reference graph
#[derive(Debug, Clone)]
pub struct CodeEdge {
    pub source: String,
    pub target: String,
    pub edge_type: CodeEdgeType,
}

/// Type of code edge
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeEdgeType {
    Calls,
    Imports,
    Inherits,
    Contains,
    References,
    // Add more edge types as needed
}

/// Code reference graph
#[derive(Debug, Clone)]
pub struct CodeReferenceGraph {
    pub nodes: Vec<CodeNode>,
    pub edges: Vec<CodeEdge>,
}

/// Repository mapper for mapping code structure
pub struct RepoMapper {
    context_extractor: ContextExtractor,
    root_path: PathBuf,
    processed_files: HashSet<String>,
    file_nodes: HashMap<String, CodeNode>,
    symbol_nodes: HashMap<String, CodeNode>,
    edges: Vec<CodeEdge>,
    // Statistics
    total_files_attempted: usize,
    files_parsed_successfully: usize,
    files_failed_to_parse: usize,
    failed_files: Vec<String>,
}

impl RepoMapper {
    /// Create a new repository mapper
    pub fn new(root_path: &Path) -> Self {
        Self {
            context_extractor: create_context_extractor(),
            root_path: root_path.to_path_buf(),
            processed_files: HashSet::new(),
            file_nodes: HashMap::new(),
            symbol_nodes: HashMap::new(),
            edges: Vec::new(),
            // Initialize statistics
            total_files_attempted: 0,
            files_parsed_successfully: 0,
            files_failed_to_parse: 0,
            failed_files: Vec::new(),
        }
    }

    /// Map the repository structure
    pub fn map_repository(&mut self) -> Result<(), String> {
        let root_path = self.root_path.clone();
        
        // First, collect all files to process
        let mut files_to_process = Vec::new();
        self.collect_files(&root_path, &mut files_to_process)?;

        // Log statistics about files found
        let mut file_extensions: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for file in &files_to_process {
            if let Some(ext) = std::path::Path::new(file).extension().and_then(|e| e.to_str()) {
                *file_extensions.entry(ext.to_string()).or_insert(0) += 1;
            }
        }

        tracing::info!("Found {} files to process:", files_to_process.len());
        for (ext, count) in &file_extensions {
            tracing::info!("  .{}: {} files", ext, count);
        }
        
        // Show timing estimate for large repositories
        if files_to_process.len() > 1000 {
            let estimated_seconds = (files_to_process.len() as f64 * 0.02).ceil() as u64; // Rough estimate: 20ms per file
            tracing::info!("Processing {} files (estimated time: {}s)...", files_to_process.len(), estimated_seconds);
        }

        // Log a few example file paths to verify correct paths
        if files_to_process.len() > 0 {
            tracing::info!("Example file paths:");
            for (i, file) in files_to_process.iter().enumerate() {
                if i < 5 || file.contains("test_files") {
                    tracing::info!("  {}", file);
                }
                if i > 20 && !file.contains("test_files") {
                    break;
                }
            }
        }
        
        // For very large repositories, we'll use a custom thread pool
        let max_threads = if files_to_process.len() > 1000 {
            let limited_threads = std::cmp::min(rayon::current_num_threads(), 8);
            limited_threads
        } else {
            rayon::current_num_threads()
        };
        
        // Process files in parallel
        self.process_files_parallel(files_to_process, max_threads)?;
        
        self.build_graph_from_context();
        self.print_parsing_statistics();
        Ok(())
    }

    /// Collect all files to process (non-recursive collection)
    fn collect_files(&self, dir_path: &Path, files: &mut Vec<String>) -> Result<(), String> {
        let entries = fs::read_dir(dir_path)
            .map_err(|e| format!("Failed to read directory {}: {}", dir_path.display(), e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                // Skip hidden directories and common directories to ignore
                let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if !dir_name.starts_with('.') && !["node_modules", "target", "dist"].contains(&dir_name) {
                    self.collect_files(&path, files)?;
                }
            } else if path.is_file() {
                // Add file if it has a supported extension
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if SupportedLanguage::from_extension(ext).is_some() {
                        let file_path = path.to_str().unwrap_or("").to_string();
                        // Normalize to relative path immediately during collection
                        if let Ok(relative_path) = Self::normalize_file_path(&file_path, &self.root_path) {
                            files.push(relative_path);
                        } else {
                            // Fallback: use original path if normalization fails
                            files.push(file_path);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Process files in parallel with batching for better performance
    fn process_files_parallel(&mut self, files: Vec<String>, _max_threads: usize) -> Result<(), String> {
        let total_files = files.len();
        self.total_files_attempted = total_files;
        
        // Use Arc<Mutex<>> to share statistics across threads
        let successful_count = Arc::new(Mutex::new(0usize));
        let failed_count = Arc::new(Mutex::new(0usize));
        let failed_files = Arc::new(Mutex::new(Vec::<String>::new()));
        
        // Adaptive batch sizing based on file count and estimated file sizes
        let batch_size = self.calculate_adaptive_batch_size(&files);
        let batches: Vec<_> = files.chunks(batch_size).collect();
        
        for (batch_idx, batch) in batches.iter().enumerate() {
            
            // Process this batch in parallel
            let batch_results: Vec<Result<(String, super::context_extractor::ContextExtractor), String>> = batch
                .par_iter()
                .map(|file_path| {
                    // Create a new context extractor for this file
                    let mut context_extractor = super::context_extractor::create_context_extractor();
                    
                    // Ensure we're working with relative paths for consistent caching
                    let normalized_file_path = if std::path::Path::new(file_path).is_absolute() {
                        Self::normalize_file_path(file_path, &self.root_path).unwrap_or_else(|_| file_path.clone())
                    } else {
                        file_path.clone()
                    };
                    
                    match context_extractor.extract_symbols_from_file_incremental(&normalized_file_path) {
                        Ok(()) => {
                            let mut count = successful_count.lock().unwrap();
                            *count += 1;
                            Ok((file_path.clone(), context_extractor))
                        }
                        Err(e) => {
                            let mut count = failed_count.lock().unwrap();
                            *count += 1;
                            let mut failed_list = failed_files.lock().unwrap();
                            failed_list.push(file_path.clone());
                            // Log errors but don't spam the UI
                            if failed_list.len() <= 10 {
                                tracing::warn!("Failed to process file {}: {}", file_path, e);
                            } else if failed_list.len() == 11 {
                                tracing::warn!("Suppressing further file processing errors to avoid spam. Check logs for details.");
                            }
                            Err(e)
                        }
                    }
                })
                .collect();
            
            // Merge results from this batch
            for result in batch_results {
                if let Ok((file_path, file_context_extractor)) = result {
                    // Merge the symbols and references from this file's context extractor
                    self.merge_context_extractor(file_context_extractor, &file_path)?;
                }
            }
            
            // For large repositories, periodically suggest garbage collection
            if total_files > 1000 && (batch_idx + 1) % 10 == 0 {
                // Hint to the runtime that this might be a good time for GC
                std::hint::black_box(&self.context_extractor);
            }
            
        }
        
        // Update statistics
        self.files_parsed_successfully = *successful_count.lock().unwrap();
        self.files_failed_to_parse = *failed_count.lock().unwrap();
        self.failed_files = failed_files.lock().unwrap().clone();
        
        Ok(())
    }
    
    /// Merge symbols and references from a file's context extractor into the main one
    fn merge_context_extractor(&mut self, file_extractor: super::context_extractor::ContextExtractor, file_path: &str) -> Result<(), String> {
        // Get symbols and references from the file extractor
        let symbols = file_extractor.get_symbols();
        let references = file_extractor.get_references();

        // Normalize the file path for consistent storage
        let normalized_path = Self::normalize_file_path(file_path, &self.root_path)?;

        // Add symbols to our main context extractor
        for (fqn, symbol) in symbols {
            self.context_extractor.add_symbol(fqn.clone(), symbol.clone());
        }

        // Add references to our main context extractor  
        for reference in references {
            self.context_extractor.add_reference(reference.clone());
        }

        // Create a file node with normalized path
        let file_node = CodeNode {
            id: format!("file:{}", normalized_path),
            name: normalized_path.clone(),
            node_type: CodeNodeType::File,
            file_path: normalized_path.clone(),
            start_line: 0,
            end_line: 0,
        };

        self.file_nodes.insert(normalized_path.clone(), file_node);
        self.processed_files.insert(normalized_path);

        Ok(())
    }

    /// Scan a directory recursively (kept for compatibility)
    fn _scan_directory(&mut self, dir_path: &Path) -> Result<(), String> {
        let entries = fs::read_dir(dir_path)
            .map_err(|e| format!("Failed to read directory {}: {}", dir_path.display(), e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                // Skip hidden directories and common directories to ignore
                let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if !dir_name.starts_with('.') && !["node_modules", "target", "dist"].contains(&dir_name) {
                    self._scan_directory(&path)?;
                }
            } else if path.is_file() {
                // Process file if it has a supported extension
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if SupportedLanguage::from_extension(ext).is_some() {
                        let file_path = path.to_str().unwrap_or("").to_string();
                        if !self.processed_files.contains(&file_path) {
                            self.process_file(&file_path)?;
                            self.processed_files.insert(file_path);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Process a single file
    fn process_file(&mut self, file_path: &str) -> Result<(), String> {
        
        self.total_files_attempted += 1;
        
        // Extract symbols from the file using incremental parsing if possible
        match self.context_extractor.extract_symbols_from_file_incremental(file_path) {
            Ok(()) => {
                self.files_parsed_successfully += 1;
            }
            Err(e) => {
                tracing::warn!("Failed to process file {}: {}", file_path, e);
                self.files_failed_to_parse += 1;
                self.failed_files.push(file_path.to_string());
                // Don't return error - continue processing other files
            }
        }

        // Create a node for the file
        let relative_path = Self::normalize_file_path(file_path, &self.root_path)?;

        let file_node = CodeNode {
            id: format!("file:{}", relative_path),
            name: relative_path.clone(),
            node_type: CodeNodeType::File,
            file_path: relative_path,
            start_line: 0,
            end_line: 0,
        };

        self.file_nodes.insert(file_path.to_string(), file_node);

        Ok(())
    }
    
    /// Update the repository map for changed files
    pub fn update_repository(&mut self) -> Result<(), String> {
        // Get a list of all files that have been modified since the last parse
        let mut modified_files = Vec::new();
        let mut deleted_files = Vec::new();
        
        // Check each processed file for changes or deletion
        for file_path in &self.processed_files.clone() {
            // Check if file still exists
            if !std::path::Path::new(file_path).exists() {
                deleted_files.push(file_path.clone());
                continue;
            }
            
            let parser_pool = super::parser_pool::get_parser_pool();
            if parser_pool.needs_reparse(file_path)? {
                modified_files.push(file_path.clone());
            }
        }
        
        // Clean up deleted files
        for deleted_file in &deleted_files {
            tracing::info!("Cleaning up deleted file: {}", deleted_file);
            self.context_extractor.remove_symbols_for_file(deleted_file);
            self.processed_files.remove(deleted_file);
            self.file_nodes.remove(deleted_file);
            // Remove from failed files list if present
            self.failed_files.retain(|f| f != deleted_file);
        }
        
        if modified_files.is_empty() && deleted_files.is_empty() {
            return Ok(());
        }
        
        // Find all files that reference symbols from the modified files
        let mut files_to_reparse = HashSet::new();
        files_to_reparse.extend(modified_files.iter().cloned());
        
        for modified_file in &modified_files {
            // Find all files that have references to symbols defined in this modified file
            let referencing_files = self.find_files_referencing_symbols_from_file(modified_file);
            files_to_reparse.extend(referencing_files);
        }
        
        tracing::debug!("Files to reparse due to changes: {:?}", files_to_reparse);
        
        // Process each file that needs reparsing
        for file_path in files_to_reparse {
            // Remove existing symbols and references for this file
            self.context_extractor.remove_symbols_for_file(&file_path);
            
            // Process the file again
            self.process_file(&file_path)?;
        }
        
        // Rebuild the graph
        self.build_graph_from_context();
        
        Ok(())
    }
    
    /// Find all files that reference symbols defined in the given file
    fn find_files_referencing_symbols_from_file(&self, target_file: &str) -> HashSet<String> {
        let mut referencing_files = HashSet::new();
        
        // Get all symbols defined in the target file
        let symbols_in_file: Vec<String> = self.context_extractor
            .get_symbols()
            .iter()
            .filter(|(_, symbol)| symbol.file_path == target_file)
            .map(|(fqn, _)| fqn.clone())
            .collect();
        
        // Find all references to these symbols
        for symbol_fqn in symbols_in_file {
            let references = self.context_extractor.find_references_by_fqn(&symbol_fqn);
            for reference in references {
                if reference.reference_file != target_file {
                    referencing_files.insert(reference.reference_file.clone());
                }
            }
        }
        
        // Also check by symbol name (for cases where FQN resolution might not work perfectly)
        let symbol_names: Vec<String> = self.context_extractor
            .get_symbols()
            .iter()
            .filter(|(_, symbol)| symbol.file_path == target_file)
            .map(|(_, symbol)| symbol.name.clone())
            .collect();
        
        for symbol_name in symbol_names {
            let references = self.context_extractor.find_references(&symbol_name);
            for reference in references {
                if reference.reference_file != target_file {
                    referencing_files.insert(reference.reference_file.clone());
                }
            }
        }
        
        referencing_files
    }

    /// Build the graph from the extracted context
    fn build_graph_from_context(&mut self) {
        // Create nodes for all symbols
        for (fqn, symbol) in self.context_extractor.get_symbols() {
            let node_type = match symbol.symbol_type {
                super::context_extractor::SymbolType::Function => CodeNodeType::Function,
                super::context_extractor::SymbolType::Method => CodeNodeType::Method,
                super::context_extractor::SymbolType::Class => CodeNodeType::Class,
                super::context_extractor::SymbolType::Struct => CodeNodeType::Struct,
                super::context_extractor::SymbolType::Module => CodeNodeType::Module,
                _ => continue, // Skip other symbol types for now
            };

            let node = CodeNode {
                id: format!("symbol:{}", fqn),
                name: symbol.name.clone(),
                node_type,
                file_path: symbol.file_path.clone(),
                start_line: symbol.start_line,
                end_line: symbol.end_line,
            };

            self.symbol_nodes.insert(fqn.clone(), node);

            // Create a "contains" edge from the file to the symbol
            if let Some(file_node) = self.file_nodes.get(&symbol.file_path) {
                self.edges.push(CodeEdge {
                    source: file_node.id.clone(),
                    target: format!("symbol:{}", fqn),
                    edge_type: CodeEdgeType::Contains,
                });
            }
        }

        // Create edges for all references
        for reference in self.context_extractor.get_references() {
            let edge_type = match reference.reference_type {
                super::context_extractor::ReferenceType::Call => CodeEdgeType::Calls,
                super::context_extractor::ReferenceType::Import => CodeEdgeType::Imports,
                super::context_extractor::ReferenceType::Inheritance => CodeEdgeType::Inherits,
                _ => CodeEdgeType::References,
            };

            // Find the source symbol node (the one containing the reference)
            // Use the most specific symbol that contains the reference line
            let source_node_id = if let Some(containing_symbol) = self.context_extractor
                .find_most_specific_containing_symbol(&reference.reference_file, reference.reference_line) {
                // Use the containing symbol as the source
                format!("symbol:{}", containing_symbol.fqn)
            } else {
                // Fall back to file node if no containing symbol found
                if let Some(file_node) = self.file_nodes.get(&reference.reference_file) {
                    file_node.id.clone()
                } else {
                    continue;
                }
            };

            // Find the target symbol node
            let target_key = if !reference.symbol_fqn.is_empty() {
                &reference.symbol_fqn
            } else {
                // Try to find the FQN from the name
                if let Some(fqns) = self.context_extractor.get_name_to_fqns().get(&reference.symbol_name) {
                    if !fqns.is_empty() {
                        &fqns[0]
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            };
            
            if let Some(target_node) = self.symbol_nodes.get(target_key) {
                self.edges.push(CodeEdge {
                    source: source_node_id,
                    target: target_node.id.clone(),
                    edge_type,
                });
            }
        }
    }

    /// Get the code reference graph
    pub fn get_graph(&self) -> CodeReferenceGraph {
        let mut nodes = Vec::new();
        
        // Add file nodes
        nodes.extend(self.file_nodes.values().cloned());
        
        // Add symbol nodes
        nodes.extend(self.symbol_nodes.values().cloned());

        CodeReferenceGraph {
            nodes,
            edges: self.edges.clone(),
        }
    }
    
    /// Get a subgraph starting from a specific symbol with a maximum depth
    pub fn get_subgraph_bfs(&self, start_symbol: &str, max_depth: usize) -> CodeReferenceGraph {
        let mut visited_nodes = HashSet::new();
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        
        // Find all starting nodes - first try exact FQN match, then try by name
        let start_nodes = if let Some(node) = self.symbol_nodes.get(start_symbol) {
            // Exact FQN match found
            vec![node]
        } else {
            // Use the name-to-FQN mapping for efficient lookup
            if let Some(fqns) = self.context_extractor.get_name_to_fqns().get(start_symbol) {
                // Get all nodes for symbols with this name
                let mut found_nodes = Vec::new();
                for fqn in fqns {
                    if let Some(node) = self.symbol_nodes.get(fqn) {
                        found_nodes.push(node);
                    }
                }
                found_nodes
            } else {
                // No symbols found with this name
                Vec::new()
            }
        };
        
        // If no symbols found, return empty graph
        if start_nodes.is_empty() {
            return CodeReferenceGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
            };
        }
        
        // Add all starting nodes to the queue with depth 0
        for start_node in start_nodes {
            queue.push_back((start_node.id.clone(), 0));
            visited_nodes.insert(start_node.id.clone());
            nodes.push(start_node.clone());
        }
        
        // BFS traversal
        while let Some((node_id, depth)) = queue.pop_front() {
            // Stop if we've reached the maximum depth
            if depth >= max_depth {
                continue;
            }
            
            // Find all edges where this node is the source
            for edge in &self.edges {
                if edge.source == node_id && !visited_nodes.contains(&edge.target) {
                    // Add the edge
                    edges.push(edge.clone());
                    
                    // Find the target node
                    let target_node = self.find_node_by_id(&edge.target);
                    if let Some(target_node) = target_node {
                        // Add the target node
                        nodes.push(target_node.clone());
                        visited_nodes.insert(edge.target.clone());
                        
                        // Add the target to the queue with increased depth
                        queue.push_back((edge.target.clone(), depth + 1));
                    }
                }
            }
            
            // Also find all edges where this node is the target (for reverse traversal)
            for edge in &self.edges {
                if edge.target == node_id && !visited_nodes.contains(&edge.source) {
                    // Add the edge
                    edges.push(edge.clone());
                    
                    // Find the source node
                    let source_node = self.find_node_by_id(&edge.source);
                    if let Some(source_node) = source_node {
                        // Add the source node
                        nodes.push(source_node.clone());
                        visited_nodes.insert(edge.source.clone());
                        
                        // Add the source to the queue with increased depth
                        queue.push_back((edge.source.clone(), depth + 1));
                    }
                }
            }
        }
        
        CodeReferenceGraph { nodes, edges }
    }

    /// Find related files by traversing the graph from a set of starting symbols.
    pub fn find_related_files_from_symbols(
        &self,
        start_symbols: Vec<&super::context_extractor::CodeSymbol>,
        max_depth: usize,
        exclude_files: &std::collections::HashSet<String>,
    ) -> Vec<String> {
        use std::collections::{HashMap, HashSet, VecDeque};
        let mut visited_nodes = HashSet::new();
        let mut queue = VecDeque::new();
        let mut related_file_counts: HashMap<String, usize> = HashMap::new();

        // Add all starting symbols to the queue with depth 0
        for symbol in start_symbols {
            let node_id = format!("symbol:{}", symbol.fqn);
            if visited_nodes.insert(node_id.clone()) {
                queue.push_back((node_id, 0));
            }
        }

        // BFS traversal
        while let Some((node_id, depth)) = queue.pop_front() {
            if depth >= max_depth {
                continue;
            }

            // Find all edges connected to this node (both outgoing and incoming)
            for edge in &self.edges {
                let neighbor_id = if edge.source == node_id {
                    Some(&edge.target)
                } else if edge.target == node_id {
                    Some(&edge.source)
                } else {
                    None
                };

                if let Some(neighbor_id) = neighbor_id {
                    if visited_nodes.insert(neighbor_id.clone()) {
                        // Find the node details for the neighbor
                        if let Some(neighbor_node) = self.find_node_by_id(neighbor_id) {
                            // Add to queue for further traversal
                            queue.push_back((neighbor_id.clone(), depth + 1));

                            // If it's a symbol, count its file
                            if neighbor_id.starts_with("symbol:") {
                                if !exclude_files.contains(&neighbor_node.file_path) {
                                    *related_file_counts.entry(neighbor_node.file_path.clone()).or_insert(0) += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Sort files by relevance (number of connections found)
        let mut sorted_files: Vec<(String, usize)> = related_file_counts.into_iter().collect();
        sorted_files.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        // Return just the file paths
        sorted_files.into_iter().map(|(path, _)| path).collect()
    }
    
    /// Find a node by its ID
    fn find_node_by_id(&self, id: &str) -> Option<&CodeNode> {
        // Check if it's a file node
        if id.starts_with("file:") {
            for node in self.file_nodes.values() {
                if node.id == id {
                    return Some(node);
                }
            }
        } 
        // Check if it's a symbol node
        else if id.starts_with("symbol:") {
            let fqn = id.strip_prefix("symbol:")?;
            return self.symbol_nodes.get(fqn);
        }
        
        None
    }
    
    /// Find references to a symbol by name
    pub fn find_symbol_references(&self, symbol_name: &str) -> Vec<&SymbolReference> {
        self.context_extractor.find_references(symbol_name)
    }
    
    /// Find references to a symbol by FQN
    pub fn find_symbol_references_by_fqn(&self, fqn: &str) -> Vec<&SymbolReference> {
        self.context_extractor.find_references_by_fqn(fqn)
    }
    
    /// Find symbol definitions by name
    pub fn find_symbol_definitions(&self, symbol_name: &str) -> Vec<&CodeSymbol> {
        self.context_extractor.find_symbols_by_name(symbol_name)
    }
    
    /// Find symbol definition by FQN
    pub fn find_symbol_definition_by_fqn(&self, fqn: &str) -> Option<&CodeSymbol> {
        self.context_extractor.find_symbol_by_fqn(fqn)
    }
    
    /// Get all symbols with their FQNs
    pub fn get_all_symbols(&self) -> &std::collections::HashMap<String, CodeSymbol> {
        self.context_extractor.get_symbols()
    }
    
    /// Get all symbols with their FQNs from memory-optimized storage if available
    pub fn get_all_symbols_from_storage(&self, storage: Option<&super::memory_optimized_storage::ThreadSafeStorage>) -> std::collections::HashMap<String, CodeSymbol> {
        if let Some(storage) = storage {
            // Get all symbols from memory-optimized storage
            match storage.get_all_symbols() {
                Ok(symbols) => symbols,
                Err(e) => {
                    tracing::warn!("Failed to get symbols from storage: {}", e);
                    // Fallback to context extractor
                    self.context_extractor.get_symbols().clone()
                }
            }
        } else {
            // No storage provided, use context extractor
            self.context_extractor.get_symbols().clone()
        }
    }
    
    /// Get symbols for a specific file - O(1) lookup using cached index
    pub fn get_symbols_for_file(&self, file_path: &str) -> Vec<&CodeSymbol> {
        self.context_extractor.get_symbols_for_file_with_root(file_path, Some(&self.root_path))
    }
    
    /// Get mapping from symbol names to FQNs
    pub fn get_name_to_fqns(&self) -> &std::collections::HashMap<String, Vec<String>> {
        self.context_extractor.get_name_to_fqns()
    }
    
    /// Get all symbol references
    pub fn get_all_references(&self) -> &[SymbolReference] {
        self.context_extractor.get_references()
    }
    
    /// Add a reference to the context extractor (for memory-optimized processing)
    pub fn add_reference(&mut self, reference: SymbolReference) {
        self.context_extractor.add_reference(reference);
    }
    
    /// Print parsing statistics
    fn print_parsing_statistics(&self) {
        // Calculate nodes and edges for summary
        let total_nodes = self.file_nodes.len() + self.symbol_nodes.len();
        let total_edges = self.edges.len();
        
        if self.total_files_attempted > 0 {
            let success_rate = (self.files_parsed_successfully as f64 / self.total_files_attempted as f64) * 100.0;
            tracing::info!("Code analysis complete: {} nodes, {} edges, {:.0}% parsed ({}/{} files)", 
                     total_nodes, total_edges, success_rate, 
                     self.files_parsed_successfully, self.total_files_attempted);
        } else {
            tracing::info!("Code analysis complete: {} nodes, {} edges", total_nodes, total_edges);
        }
    }
    
    /// Get parsing statistics
    pub fn get_parsing_statistics(&self) -> (usize, usize, usize, &Vec<String>) {
        (
            self.total_files_attempted,
            self.files_parsed_successfully,
            self.files_failed_to_parse,
            &self.failed_files,
        )
    }
    
    /// Get root path (public accessor)
    pub fn get_root_path(&self) -> &std::path::Path {
        &self.root_path
    }
    
    /// Increment parsed successfully counter (public method)
    pub fn increment_parsed_successfully(&mut self) {
        self.files_parsed_successfully += 1;
    }
    
    /// Increment failed to parse counter (public method)
    pub fn increment_failed_to_parse(&mut self) {
        self.files_failed_to_parse += 1;
    }
    
    /// Add failed file (public method)
    pub fn add_failed_file(&mut self, file_path: String) {
        self.failed_files.push(file_path);
    }
    
    /// Calculate adaptive batch size based on file characteristics
    fn calculate_adaptive_batch_size(&self, files: &[String]) -> usize {
        let total_files = files.len();
        
        // Sample a few files to estimate average size
        let sample_size = std::cmp::min(10, total_files);
        let mut total_sample_size = 0u64;
        let mut valid_samples = 0;
        
        for file_path in files.iter().take(sample_size) {
            if let Ok(metadata) = std::fs::metadata(file_path) {
                total_sample_size += metadata.len();
                valid_samples += 1;
            }
        }
        
        let avg_file_size = if valid_samples > 0 {
            total_sample_size / valid_samples as u64
        } else {
            50_000 // Default assumption: 50KB average
        };
        
        // Adaptive batch sizing logic
        match (total_files, avg_file_size) {
            // Very large repositories with small files
            (n, size) if n > 10_000 && size < 10_000 => 50,
            // Large repositories with medium files  
            (n, size) if n > 5_000 && size < 50_000 => 30,
            // Large repositories with large files
            (n, size) if n > 1_000 && size > 100_000 => 10,
            // Medium repositories with very large files
            (n, size) if n > 500 && size > 500_000 => 5,
            // Small repositories or default case
            (n, _) if n > 1_000 => 20,
            _ => 50, // Default for small repositories
        }
    }

    /// Normalize file path to always use relative paths with forward slashes
    fn normalize_file_path(file_path: &str, root_path: &Path) -> Result<String, String> {
        let path = Path::new(file_path);
        
        // If already relative, normalize separators
        if path.is_relative() {
            return Ok(file_path.replace('\\', "/"));
        }
        
        // For absolute paths, try to make relative to root
        let relative_path = path
            .strip_prefix(root_path)
            .map_err(|_| format!("Failed to get relative path for {}", file_path))?
            .to_str()
            .unwrap_or("")
            .replace('\\', "/");
            
        Ok(relative_path)
    }
}

/// Create a new repository mapper
pub fn create_repo_mapper(root_path: &Path) -> RepoMapper {
    RepoMapper::new(root_path)
}
