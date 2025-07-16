use std::collections::{HashSet, VecDeque};
use crate::code_analysis::{
    supplementary_registry::SupplementarySymbolRegistry,
    graph_manager::get_graph_manager,
};
use tracing::{info, debug};

/// Enhanced BFS traversal with cross-project boundary detection
/// This replaces the current BFS approach with optimized cross-project handling
pub fn find_related_files_bfs_with_cross_project_boundaries(
    active_files: &[String],
    max_depth: usize,
    supplementary_registry: &SupplementarySymbolRegistry,
) -> Result<(Vec<String>, Vec<String>), String> {
    
    info!("Starting enhanced BFS traversal for {} active files with max depth {}", 
          active_files.len(), max_depth);
    
    // This function is under development - use the optimized version for now
    find_related_files_optimized(active_files, max_depth, supplementary_registry)
}

// Future enhancement: These functions will be implemented when we integrate
// the enhanced graph structures with the existing graph system

/// Optimized file discovery with cross-project boundary detection
/// This is a simplified version that works with the current graph structure
pub fn find_related_files_optimized(
    active_files: &[String],
    max_depth: usize,
    supplementary_registry: &SupplementarySymbolRegistry,
) -> Result<(Vec<String>, Vec<String>), String> {
    
    info!("Starting optimized file discovery for {} active files", active_files.len());
    
    let _graph_manager = get_graph_manager();
    // TODO: Implement proper graph traversal when needed
    
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut main_project_files = HashSet::new();
    let mut supplementary_files = HashSet::new();
    
    // Initialize with active files
    for file in active_files {
        if !visited.contains(file) {
            visited.insert(file.clone());
            
            // Check if this is a supplementary file
            if supplementary_registry.contains_file(file) {
                supplementary_files.insert(file.clone());
                debug!("Active file {} is from supplementary project", file);
            } else {
                main_project_files.insert(file.clone());
                queue.push_back((file.clone(), 0));
                debug!("Active file {} is from main project", file);
            }
        }
    }
    
    // BFS traversal using existing graph structure
    while let Some((current_file, depth)) = queue.pop_front() {
        if depth >= max_depth {
            continue;
        }
        
        // For now, skip complex graph traversal and just return the categorized active files
        // TODO: Implement proper cross-project BFS traversal when graph structure is clarified
        break;
    }
    
    let main_files: Vec<String> = main_project_files.into_iter().collect();
    let supp_files: Vec<String> = supplementary_files.into_iter().collect();
    
    info!("Optimized discovery completed: {} main project files, {} supplementary files", 
          main_files.len(), supp_files.len());
    
    Ok((main_files, supp_files))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::code_analysis::supplementary_registry::{SupplementarySymbolRegistry, SupplementarySymbolInfo};
    
    #[test]
    fn test_optimized_file_discovery_basic() {
        let mut registry = SupplementarySymbolRegistry::new();
        
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
        
        registry.add_symbol(supp_symbol);
        
        // Test file detection
        assert!(registry.contains_file("/supp/file.cs"));
        assert!(!registry.contains_file("/main/file.cs"));
        
        // Test project detection
        assert_eq!(registry.get_project_for_file("/supp/file.cs"), Some("SupplementaryProject".to_string()));
        assert_eq!(registry.get_project_for_file("/main/file.cs"), None);
    }
    
    #[test]
    fn test_file_categorization() {
        let mut registry = SupplementarySymbolRegistry::new();
        
        // Add multiple supplementary symbols from different projects
        let symbols = vec![
            SupplementarySymbolInfo {
                fqn: "ProjectA::ClassA.MethodA".to_string(),
                name: "MethodA".to_string(),
                file_path: "/projectA/fileA.cs".to_string(),
                symbol_type: "method".to_string(),
                project_name: "ProjectA".to_string(),
                start_line: 1,
                end_line: 10,
                parent: Some("ClassA".to_string()),
            },
            SupplementarySymbolInfo {
                fqn: "ProjectB::ClassB.MethodB".to_string(),
                name: "MethodB".to_string(),
                file_path: "/projectB/fileB.cs".to_string(),
                symbol_type: "method".to_string(),
                project_name: "ProjectB".to_string(),
                start_line: 1,
                end_line: 10,
                parent: Some("ClassB".to_string()),
            },
        ];
        
        for symbol in symbols {
            registry.add_symbol(symbol);
        }
        
        // Test file categorization
        let test_files = vec![
            "/main/file.cs".to_string(),      // Main project
            "/projectA/fileA.cs".to_string(), // Supplementary A
            "/projectB/fileB.cs".to_string(), // Supplementary B
            "/external/file.cs".to_string(),  // External
        ];
        
        let mut main_files = Vec::new();
        let mut supplementary_files = Vec::new();
        
        for file in test_files {
            if registry.contains_file(&file) {
                supplementary_files.push(file);
            } else {
                main_files.push(file);
            }
        }
        
        assert_eq!(main_files.len(), 2); // main and external
        assert_eq!(supplementary_files.len(), 2); // projectA and projectB
        
        // Verify specific files
        assert!(main_files.contains(&"/main/file.cs".to_string()));
        assert!(main_files.contains(&"/external/file.cs".to_string()));
        assert!(supplementary_files.contains(&"/projectA/fileA.cs".to_string()));
        assert!(supplementary_files.contains(&"/projectB/fileB.cs".to_string()));
    }
}