//! Repository mapper for mapping code structure and relationships.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

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
        }
    }

    /// Map the repository structure
    pub fn map_repository(&mut self) -> Result<(), String> {
        let root_path = self.root_path.clone();
        self.scan_directory(&root_path)?;
        self.build_graph_from_context();
        Ok(())
    }

    /// Scan a directory recursively
    fn scan_directory(&mut self, dir_path: &Path) -> Result<(), String> {
        let entries = fs::read_dir(dir_path)
            .map_err(|e| format!("Failed to read directory {}: {}", dir_path.display(), e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                // Skip hidden directories and common directories to ignore
                let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if !dir_name.starts_with('.') && !["node_modules", "target", "dist"].contains(&dir_name) {
                    self.scan_directory(&path)?;
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
        // Extract symbols from the file using incremental parsing if possible
        self.context_extractor.extract_symbols_from_file_incremental(file_path)?;

        // Create a node for the file
        let relative_path = Path::new(file_path)
            .strip_prefix(&self.root_path)
            .map_err(|_| format!("Failed to get relative path for {}", file_path))?
            .to_str()
            .unwrap_or("")
            .to_string();

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
        
        for file_path in &self.processed_files {
            let parser_pool = super::parser_pool::get_parser_pool();
            if parser_pool.needs_reparse(file_path)? {
                modified_files.push(file_path.clone());
            }
        }
        
        // Process each modified file
        for file_path in modified_files {
            // Remove existing symbols and references for this file
            self.context_extractor.remove_symbols_for_file(&file_path);
            
            // Process the file again
            self.process_file(&file_path)?;
        }
        
        // Rebuild the graph
        self.build_graph_from_context();
        
        Ok(())
    }

    /// Build the graph from the extracted context
    fn build_graph_from_context(&mut self) {
        // Create nodes for all symbols
        for (symbol_name, symbol) in self.context_extractor.get_symbols() {
            let node_type = match symbol.symbol_type {
                super::context_extractor::SymbolType::Function => CodeNodeType::Function,
                super::context_extractor::SymbolType::Method => CodeNodeType::Method,
                super::context_extractor::SymbolType::Class => CodeNodeType::Class,
                super::context_extractor::SymbolType::Struct => CodeNodeType::Struct,
                super::context_extractor::SymbolType::Module => CodeNodeType::Module,
                _ => continue, // Skip other symbol types for now
            };

            let node = CodeNode {
                id: format!("symbol:{}", symbol_name),
                name: symbol_name.clone(),
                node_type,
                file_path: symbol.file_path.clone(),
                start_line: symbol.start_line,
                end_line: symbol.end_line,
            };

            self.symbol_nodes.insert(symbol_name.clone(), node);

            // Create a "contains" edge from the file to the symbol
            if let Some(file_node) = self.file_nodes.get(&symbol.file_path) {
                self.edges.push(CodeEdge {
                    source: file_node.id.clone(),
                    target: format!("symbol:{}", symbol_name),
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
            // This is a simplification - in a real implementation, we would need to find the
            // exact symbol containing this reference
            if let Some(file_node) = self.file_nodes.get(&reference.reference_file) {
                if let Some(target_node) = self.symbol_nodes.get(&reference.symbol_name) {
                    self.edges.push(CodeEdge {
                        source: file_node.id.clone(),
                        target: target_node.id.clone(),
                        edge_type,
                    });
                }
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
        
        // Find the starting node
        let start_node = if let Some(node) = self.symbol_nodes.get(start_symbol) {
            node
        } else {
            // If the symbol doesn't exist, return an empty graph
            return CodeReferenceGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
            };
        };
        
        // Add the starting node to the queue with depth 0
        queue.push_back((start_node.id.clone(), 0));
        visited_nodes.insert(start_node.id.clone());
        nodes.push(start_node.clone());
        
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
            let symbol_name = id.strip_prefix("symbol:")?;
            return self.symbol_nodes.get(symbol_name);
        }
        
        None
    }
}

/// Create a new repository mapper
pub fn create_repo_mapper(root_path: &Path) -> RepoMapper {
    RepoMapper::new(root_path)
}