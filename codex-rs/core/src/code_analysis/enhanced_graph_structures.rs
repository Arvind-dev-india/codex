use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::code_analysis::supplementary_registry::SupplementarySymbolRegistry;

/// Enhanced CodeNode with cross-project support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedCodeNode {
    pub id: String,
    pub symbol_type: String,
    pub file_path: String,
    pub start_line: u32,
    pub end_line: u32,
    pub fqn: String,
    
    // New cross-project fields
    pub is_cross_project: bool,
    pub project_name: Option<String>,
    pub boundary_type: Option<CrossProjectBoundaryType>,
}

/// Enhanced CodeEdge with cross-project support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedCodeEdge {
    pub source: String,
    pub target: String,
    pub edge_type: String,
    pub is_cross_project: bool,
}

/// Types of cross-project boundaries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrossProjectBoundaryType {
    Supplementary(String), // Project name
    External,
}

/// Unresolved reference from main project
#[derive(Debug, Clone)]
pub struct UnresolvedReference {
    pub source_fqn: String,
    pub target_fqn: String,
    pub reference_type: String,
    pub source_file: String,
    pub line_number: u32,
}

/// Enhanced CodeGraph with cross-project support
#[derive(Debug, Clone)]
pub struct EnhancedCodeGraph {
    pub nodes: HashMap<String, EnhancedCodeNode>,
    pub edges: Vec<EnhancedCodeEdge>,
    pub file_to_nodes: HashMap<String, Vec<String>>,
    pub unresolved_references: Vec<UnresolvedReference>,
    pub cross_project_nodes_count: usize,
}

impl EnhancedCodeGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            file_to_nodes: HashMap::new(),
            unresolved_references: Vec::new(),
            cross_project_nodes_count: 0,
        }
    }
    
    /// Add a regular main project node
    pub fn add_main_project_node(&mut self, node: EnhancedCodeNode) {
        let node_id = node.id.clone();
        let file_path = node.file_path.clone();
        
        // Add to nodes map
        self.nodes.insert(node_id.clone(), node);
        
        // Add to file mapping
        self.file_to_nodes
            .entry(file_path)
            .or_insert_with(Vec::new)
            .push(node_id);
    }
    
    /// Add a cross-project node (from supplementary project)
    pub fn add_cross_project_node(&mut self, symbol_info: &crate::code_analysis::supplementary_registry::SupplementarySymbolInfo) {
        let node = EnhancedCodeNode {
            id: symbol_info.fqn.clone(),
            symbol_type: symbol_info.symbol_type.clone(),
            file_path: symbol_info.file_path.clone(),
            start_line: symbol_info.start_line,
            end_line: symbol_info.end_line,
            fqn: symbol_info.fqn.clone(),
            is_cross_project: true,
            project_name: Some(symbol_info.project_name.clone()),
            boundary_type: Some(CrossProjectBoundaryType::Supplementary(
                symbol_info.project_name.clone()
            )),
        };
        
        let node_id = node.id.clone();
        let file_path = node.file_path.clone();
        
        // Add to nodes map
        self.nodes.insert(node_id.clone(), node);
        
        // Add to file mapping
        self.file_to_nodes
            .entry(file_path)
            .or_insert_with(Vec::new)
            .push(node_id);
        
        self.cross_project_nodes_count += 1;
        
        tracing::debug!("Added cross-project node: {} from project {}", 
                      symbol_info.fqn, symbol_info.project_name);
    }
    
    /// Add an edge (regular or cross-project)
    pub fn add_edge(&mut self, edge: EnhancedCodeEdge) {
        self.edges.push(edge);
    }
    
    /// Get nodes in a specific file
    pub fn get_nodes_in_file(&self, file_path: &str) -> Vec<&EnhancedCodeNode> {
        self.file_to_nodes
            .get(file_path)
            .map(|node_ids| {
                node_ids.iter()
                    .filter_map(|id| self.nodes.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Get a specific node by ID
    pub fn get_node(&self, node_id: &str) -> Option<&EnhancedCodeNode> {
        self.nodes.get(node_id)
    }
    
    /// Get outgoing edges from a node
    pub fn get_outgoing_edges(&self, node_id: &str) -> Vec<&EnhancedCodeEdge> {
        self.edges.iter()
            .filter(|edge| edge.source == node_id)
            .collect()
    }
    
    /// Get incoming edges to a node
    pub fn get_incoming_edges(&self, node_id: &str) -> Vec<&EnhancedCodeEdge> {
        self.edges.iter()
            .filter(|edge| edge.target == node_id)
            .collect()
    }
    
    /// Add an unresolved reference
    pub fn add_unresolved_reference(&mut self, reference: UnresolvedReference) {
        self.unresolved_references.push(reference);
    }
    
    /// Get statistics about the graph
    pub fn get_stats(&self) -> EnhancedGraphStats {
        let main_project_nodes = self.nodes.values()
            .filter(|node| !node.is_cross_project)
            .count();
        
        let cross_project_edges = self.edges.iter()
            .filter(|edge| edge.is_cross_project)
            .count();
        
        EnhancedGraphStats {
            total_nodes: self.nodes.len(),
            main_project_nodes,
            cross_project_nodes: self.cross_project_nodes_count,
            total_edges: self.edges.len(),
            cross_project_edges,
            unresolved_references: self.unresolved_references.len(),
            files_with_nodes: self.file_to_nodes.len(),
        }
    }
}

#[derive(Debug)]
pub struct EnhancedGraphStats {
    pub total_nodes: usize,
    pub main_project_nodes: usize,
    pub cross_project_nodes: usize,
    pub total_edges: usize,
    pub cross_project_edges: usize,
    pub unresolved_references: usize,
    pub files_with_nodes: usize,
}

/// Resolve cross-project references by adding supplementary symbols to main graph
pub fn resolve_cross_project_references(
    main_graph: &mut EnhancedCodeGraph,
    supplementary_registry: &SupplementarySymbolRegistry,
) -> Result<usize, String> {
    let mut cross_project_nodes_added = 0;
    
    tracing::info!("Resolving {} unresolved references against {} supplementary symbols", 
                  main_graph.unresolved_references.len(), 
                  supplementary_registry.symbols.len());
    
    // Process unresolved references
    let unresolved_refs = main_graph.unresolved_references.clone();
    
    for unresolved_ref in unresolved_refs {
        // Check if this FQN exists in supplementary projects
        if let Some(supp_symbol) = supplementary_registry.lookup_by_fqn(&unresolved_ref.target_fqn) {
            
            // Add cross-project node to main graph
            main_graph.add_cross_project_node(supp_symbol);
            
            // Create cross-project edge
            let edge = EnhancedCodeEdge {
                source: unresolved_ref.source_fqn.clone(),
                target: unresolved_ref.target_fqn.clone(),
                edge_type: unresolved_ref.reference_type.clone(),
                is_cross_project: true,
            };
            
            main_graph.add_edge(edge);
            cross_project_nodes_added += 1;
            
            tracing::debug!("Resolved cross-project reference: {} -> {} (project: {})", 
                          unresolved_ref.source_fqn, 
                          unresolved_ref.target_fqn,
                          supp_symbol.project_name);
        }
    }
    
    // Remove resolved references from unresolved list
    main_graph.unresolved_references.retain(|unresolved_ref| {
        !supplementary_registry.lookup_by_fqn(&unresolved_ref.target_fqn).is_some()
    });
    
    tracing::info!("Added {} cross-project nodes to main graph", cross_project_nodes_added);
    tracing::info!("Remaining unresolved references: {}", main_graph.unresolved_references.len());
    
    Ok(cross_project_nodes_added)
}

/// Smart cross-project reference resolution that tries multiple FQN patterns
pub fn resolve_cross_project_references_smart(
    main_symbols: &[crate::code_analysis::supplementary_registry::SupplementarySymbolInfo],
    supplementary_registry: &SupplementarySymbolRegistry,
) -> Result<Vec<CrossProjectReference>, String> {
    let mut cross_project_refs = Vec::new();
    
    tracing::info!("Smart resolving cross-project references for {} main symbols against {} supplementary symbols", 
                  main_symbols.len(), supplementary_registry.symbols.len());
    
    // For each main project symbol, look for potential references to supplementary symbols
    for main_symbol in main_symbols {
        // Try to find references by symbol name matching
        for (supp_fqn, supp_symbol) in &supplementary_registry.symbols {
            // Check if main symbol might reference supplementary symbol
            if could_reference_symbol(&main_symbol.name, &supp_symbol.name) {
                let cross_ref = CrossProjectReference {
                    source_fqn: main_symbol.fqn.clone(),
                    target_fqn: supp_fqn.clone(),
                    reference_type: "potential_usage".to_string(),
                    source_file: main_symbol.file_path.clone(),
                    target_file: supp_symbol.file_path.clone(),
                    source_project: "main".to_string(),
                    target_project: supp_symbol.project_name.clone(),
                };
                
                cross_project_refs.push(cross_ref);
                
                tracing::debug!("Found potential cross-project reference: {} -> {}", 
                              main_symbol.fqn, supp_fqn);
            }
        }
    }
    
    tracing::info!("Found {} potential cross-project references", cross_project_refs.len());
    Ok(cross_project_refs)
}

/// Check if a main symbol could reference a supplementary symbol
fn could_reference_symbol(main_symbol_name: &str, supp_symbol_name: &str) -> bool {
    // Simple heuristics for potential references
    
    // Exact name match
    if main_symbol_name.contains(supp_symbol_name) {
        return true;
    }
    
    // Common patterns: CreateUser -> User, GetUser -> User, etc.
    if main_symbol_name.contains("User") && supp_symbol_name == "User" {
        return true;
    }
    
    // Type usage patterns
    if main_symbol_name.ends_with("Service") && supp_symbol_name == "User" {
        return true;
    }
    
    false
}

/// Cross-project reference information
#[derive(Debug, Clone)]
pub struct CrossProjectReference {
    pub source_fqn: String,
    pub target_fqn: String,
    pub reference_type: String,
    pub source_file: String,
    pub target_file: String,
    pub source_project: String,
    pub target_project: String,
}

/// Create an unresolved reference
pub fn create_unresolved_reference(
    source_fqn: String,
    target_fqn: String,
    reference_type: String,
    source_file: String,
    line_number: u32,
) -> UnresolvedReference {
    UnresolvedReference {
        source_fqn,
        target_fqn,
        reference_type,
        source_file,
        line_number,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::code_analysis::supplementary_registry::SupplementarySymbolInfo;
    
    #[test]
    fn test_enhanced_graph_basic() {
        let mut graph = EnhancedCodeGraph::new();
        
        // Add main project node
        let main_node = EnhancedCodeNode {
            id: "main::Class.Method".to_string(),
            symbol_type: "method".to_string(),
            file_path: "/main/file.cs".to_string(),
            start_line: 10,
            end_line: 20,
            fqn: "main::Class.Method".to_string(),
            is_cross_project: false,
            project_name: None,
            boundary_type: None,
        };
        
        graph.add_main_project_node(main_node);
        
        // Add supplementary symbol
        let supp_symbol = SupplementarySymbolInfo {
            fqn: "SupplementaryProject::LibClass.LibMethod".to_string(),
            name: "LibMethod".to_string(),
            file_path: "/supp/file.cs".to_string(),
            symbol_type: "method".to_string(),
            project_name: "SupplementaryProject".to_string(),
            start_line: 5,
            end_line: 15,
            parent: Some("LibClass".to_string()),
        };
        
        graph.add_cross_project_node(&supp_symbol);
        
        // Test stats
        let stats = graph.get_stats();
        assert_eq!(stats.total_nodes, 2);
        assert_eq!(stats.main_project_nodes, 1);
        assert_eq!(stats.cross_project_nodes, 1);
        
        // Test node retrieval
        assert!(graph.get_node("main::Class.Method").is_some());
        assert!(graph.get_node("SupplementaryProject::LibClass.LibMethod").is_some());
        
        // Test cross-project node properties
        let cross_project_node = graph.get_node("SupplementaryProject::LibClass.LibMethod").unwrap();
        assert!(cross_project_node.is_cross_project);
        assert_eq!(cross_project_node.project_name, Some("SupplementaryProject".to_string()));
    }
    
    #[test]
    fn test_cross_project_resolution() {
        let mut graph = EnhancedCodeGraph::new();
        let mut registry = crate::code_analysis::supplementary_registry::SupplementarySymbolRegistry::new();
        
        // Add supplementary symbol to registry
        let supp_symbol = SupplementarySymbolInfo {
            fqn: "SupplementaryProject::LibClass.LibMethod".to_string(),
            name: "LibMethod".to_string(),
            file_path: "/supp/file.cs".to_string(),
            symbol_type: "method".to_string(),
            project_name: "SupplementaryProject".to_string(),
            start_line: 5,
            end_line: 15,
            parent: Some("LibClass".to_string()),
        };
        
        registry.add_symbol(supp_symbol);
        
        // Add unresolved reference to graph
        let unresolved_ref = UnresolvedReference {
            source_fqn: "main::Class.Method".to_string(),
            target_fqn: "SupplementaryProject::LibClass.LibMethod".to_string(),
            reference_type: "method_call".to_string(),
            source_file: "/main/file.cs".to_string(),
            line_number: 15,
        };
        
        graph.add_unresolved_reference(unresolved_ref);
        
        // Resolve cross-project references
        let resolved_count = resolve_cross_project_references(&mut graph, &registry).unwrap();
        
        assert_eq!(resolved_count, 1);
        assert_eq!(graph.get_stats().cross_project_nodes, 1);
        assert_eq!(graph.get_stats().cross_project_edges, 1);
        assert_eq!(graph.unresolved_references.len(), 0); // Should be resolved
    }
}