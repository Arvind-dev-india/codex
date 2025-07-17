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
/// This version uses the actual graph with cross-project edges
pub fn find_related_files_optimized(
    active_files: &[String],
    max_depth: usize,
    supplementary_registry: &SupplementarySymbolRegistry,
) -> Result<(Vec<String>, Vec<String>), String> {
    
    println!("DEBUG: Enhanced BFS called with {} active files, max_depth {}", active_files.len(), max_depth);
    info!("Starting optimized file discovery for {} active files with max depth {}", active_files.len(), max_depth);
    
    let graph_manager = get_graph_manager();
    
    // Check if graph needs initialization
    {
        let manager = graph_manager.read().map_err(|e| format!("Failed to acquire read lock: {}", e))?;
        if !manager.has_symbols() {
            println!("DEBUG: Graph has no symbols - attempting initialization");
            drop(manager); // Release read lock
            
            // Force initialization if needed
            let mut write_manager = graph_manager.write().map_err(|e| format!("Failed to acquire write lock: {}", e))?;
            if !write_manager.has_symbols() {
                // Get the root path from active files
                if let Some(first_file) = active_files.first() {
                    if let Some(parent_dir) = std::path::Path::new(first_file).parent() {
                        println!("DEBUG: Force initializing graph for: {:?}", parent_dir);
                        if let Err(e) = write_manager.ensure_graph_for_path(parent_dir) {
                            println!("DEBUG: Graph initialization failed: {}", e);
                        }
                    }
                }
            }
        }
    }
    
    // Now get a fresh read lock and proceed
    let manager = graph_manager.read().map_err(|e| format!("Failed to acquire read lock: {}", e))?;
    
    if !manager.has_symbols() {
        println!("DEBUG: Still no symbols after initialization - returning empty results");
        return Ok((Vec::new(), Vec::new()));
    }
    
    // Get all symbols efficiently from graph manager
    let all_symbols = manager.get_all_symbols();
    println!("DEBUG: Graph contains {} total symbols", all_symbols.len());
    
    let repo_mapper = manager.get_repo_mapper()
        .ok_or("Repository mapper not available")?;
    
    if all_symbols.is_empty() {
        println!("DEBUG: No symbols available - returning empty results");
        return Ok((Vec::new(), Vec::new()));
    }
    
    // Now we have symbols - proceed with BFS traversal
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut main_project_files = HashSet::new();
    let mut supplementary_files = HashSet::new();
    
    // Initialize with active files
    for file in active_files {
        if !visited.contains(file) {
            visited.insert(file.clone());
            
            // Check if this is a supplementary file using the registry
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
    
    // Pre-compute expensive operations outside the loop to avoid performance issues
    let all_references = repo_mapper.get_all_references();
    
    // BFS traversal using actual graph structure with cross-project boundary detection
    // Add safety limits to prevent infinite loops and stack overflow
    const MAX_FILES: usize = 10;  // Very conservative limit to prevent stack overflow
    const MAX_ITERATIONS: usize = 25;   // Very conservative limit to prevent stack overflow
    let mut iteration_count = 0;
    
    while let Some((current_file, depth)) = queue.pop_front() {
        iteration_count += 1;
        
        // Safety checks - be very aggressive to prevent stack overflow
        if depth >= max_depth {
            debug!("BFS hit max depth ({}), skipping file at depth {}", max_depth, depth);
            continue;
        }
        if main_project_files.len() + supplementary_files.len() >= MAX_FILES {
            debug!("BFS hit file limit ({}), stopping traversal", MAX_FILES);
            break;
        }
        if iteration_count >= MAX_ITERATIONS {
            debug!("BFS hit iteration limit ({}), stopping traversal", MAX_ITERATIONS);
            break;
        }
        if queue.len() > 50 {
            debug!("BFS queue too large ({}), stopping traversal", queue.len());
            break;
        }
        
        debug!("Processing file {} at depth {}", current_file, depth);
        
        // Find symbols defined in current file
        let symbols_in_file: Vec<_> = all_symbols
            .iter()
            .filter(|(_, symbol)| symbol.file_path == current_file)
            .map(|(fqn, _)| fqn.clone())
            .collect();
        
        debug!("Found {} symbols in file {}", symbols_in_file.len(), current_file);
        
        // Find references both FROM and TO current file
        
        // 1. Find references FROM current file TO other files (outgoing references)
        // Look at all references that originate from this file
        let refs_from_file: Vec<_> = all_references.iter()
            .filter(|r| r.reference_file == current_file)
            .collect();
        
        debug!("Found {} outgoing references from file {}", refs_from_file.len(), current_file);
        
        for reference in refs_from_file {
            // Find the file where the referenced symbol is defined
            if !reference.symbol_fqn.is_empty() {
                if let Some(target_symbol) = all_symbols.values().find(|s| s.fqn == reference.symbol_fqn) {
                    let target_file = &target_symbol.file_path;
                    
                    if !visited.contains(target_file) && *target_file != current_file {
                        visited.insert(target_file.clone());
                        
                        // Check if target file is from supplementary project
                        if supplementary_registry.contains_file(target_file) {
                            if supplementary_files.insert(target_file.clone()) {
                                debug!("Found cross-project outgoing reference: {} -> {} (supplementary)", 
                                      current_file, target_file);
                            }
                        } else {
                            // Regular in-project file - add to queue for further traversal
                            main_project_files.insert(target_file.clone());
                            queue.push_back((target_file.clone(), depth + 1));
                            debug!("Found in-project outgoing reference: {} -> {} (depth {})", 
                                  current_file, target_file, depth + 1);
                        }
                    }
                }
            }
        }
        
        // 2. Count references FROM symbols defined in current file TO other files (symbol-based outgoing)
        for symbol_fqn in &symbols_in_file {
            let references = repo_mapper.find_symbol_references_by_fqn(symbol_fqn);
            debug!("Symbol {} has {} outgoing references", symbol_fqn, references.len());
            
            for reference in references {
                if !visited.contains(&reference.reference_file) && reference.reference_file != current_file {
                    visited.insert(reference.reference_file.clone());
                    
                    // Check if target file is from supplementary project using registry
                    if supplementary_registry.contains_file(&reference.reference_file) {
                        // Add to supplementary files but don't traverse further (cross-project boundary)
                        if supplementary_files.insert(reference.reference_file.clone()) {
                            debug!("Found cross-project boundary: {} -> {} (supplementary)", 
                                  current_file, reference.reference_file);
                        }
                    } else {
                        // Regular in-project file - add to queue for further traversal
                        main_project_files.insert(reference.reference_file.clone());
                        queue.push_back((reference.reference_file.clone(), depth + 1));
                        debug!("Found in-project reference: {} -> {} (depth {})", 
                              current_file, reference.reference_file, depth + 1);
                    }
                }
            }
        }
        
        // 2. Find references TO current file FROM other files
        // Look for files that reference symbols defined in the current file
        for symbol_fqn in &symbols_in_file {
            // Find all references to this symbol and extract unique file paths
            let references = repo_mapper.find_symbol_references_by_fqn(symbol_fqn);
            debug!("Symbol {} has {} incoming references", symbol_fqn, references.len());
            
            // Extract unique file paths from references
            let mut referencing_files = HashSet::new();
            for reference in references {
                referencing_files.insert(reference.reference_file.clone());
            }
            
            for referencing_file in referencing_files {
                if !visited.contains(&referencing_file) && referencing_file != current_file {
                    visited.insert(referencing_file.clone());
                    
                    // Check if referencing file is from supplementary project
                    if supplementary_registry.contains_file(&referencing_file) {
                        if supplementary_files.insert(referencing_file.clone()) {
                            debug!("Found cross-project incoming reference: {} <- {} (supplementary)", 
                                  current_file, referencing_file);
                        }
                    } else {
                        // Regular in-project file - add to queue for further traversal
                        main_project_files.insert(referencing_file.clone());
                        queue.push_back((referencing_file.clone(), depth + 1));
                        debug!("Found in-project incoming reference: {} <- {} (depth {})", 
                              current_file, referencing_file, depth + 1);
                    }
                }
            }
        }
    }
    
    let main_files: Vec<String> = main_project_files.into_iter().collect();
    let supp_files: Vec<String> = supplementary_files.into_iter().collect();
    
    info!("Enhanced BFS completed: {} main project files, {} supplementary files", 
          main_files.len(), supp_files.len());
    
    Ok((main_files, supp_files))
}


/// Find related files using memory-optimized storage when repo mapper is empty
fn find_related_files_using_storage(
    active_files: &[String],
    _max_depth: usize,
    supplementary_registry: &SupplementarySymbolRegistry,
    manager: &std::sync::RwLockReadGuard<crate::code_analysis::graph_manager::CodeGraphManager>,
) -> Result<(Vec<String>, Vec<String>), String> {
    
    println!("DEBUG: Implementing storage-based BFS traversal");
    
    let _storage = manager.get_symbol_storage()
        .ok_or("Storage not available")?;
    
    let mut main_project_files = HashSet::new();
    let mut supplementary_files = HashSet::new();
    let mut visited = HashSet::new();
    
    // For now, implement a simple version that finds related files based on file patterns
    // This is a temporary solution until we can properly access symbols from storage
    
    for active_file in active_files {
        visited.insert(active_file.clone());
        
        // Check if this is a supplementary file
        if supplementary_registry.contains_file(active_file) {
            supplementary_files.insert(active_file.clone());
        } else {
            // For Python files, look for related files in the same directory structure
            if active_file.ends_with(".py") {
                let file_path = std::path::Path::new(active_file);
                if let Some(parent_dir) = file_path.parent() {
                    // Find other Python files in the same directory and subdirectories
                    if let Ok(entries) = std::fs::read_dir(parent_dir) {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            if path.is_file() && path.extension().map_or(false, |ext| ext == "py") {
                                let path_str = path.to_string_lossy().to_string();
                                if !visited.contains(&path_str) && path_str != *active_file {
                                    main_project_files.insert(path_str);
                                    visited.insert(path.to_string_lossy().to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    let main_files: Vec<String> = main_project_files.into_iter().collect();
    let supp_files: Vec<String> = supplementary_files.into_iter().collect();
    
    println!("DEBUG: Storage-based BFS found {} main files, {} supplementary files", 
             main_files.len(), supp_files.len());
    
    Ok((main_files, supp_files))
}

/// Fallback BFS function when graph is not properly initialized
fn find_related_files_bfs_with_cross_project_detection_fallback(
    active_files: &[String], 
    max_depth: usize,
    detector: &super::repo_mapper::CrossProjectDetector,
    repo_mapper: &super::repo_mapper::RepoMapper,
) -> Result<(Vec<String>, Vec<String>), String> {
    use std::collections::{HashSet, BinaryHeap, HashMap};
    use std::cmp::Reverse;
    
    println!("DEBUG: Using fallback BFS with {} active files", active_files.len());
    
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    let mut in_project_files = HashSet::new();
    let mut cross_project_files = HashSet::new();
    
    // Categorize active files
    for file in active_files {
        if detector.is_cross_project_symbol(file) {
            cross_project_files.insert(file.clone());
        } else {
            in_project_files.insert(file.clone());
        }
        visited.insert(file.clone());
    }
    
    // Add in-project files to queue for BFS traversal
    for file in &in_project_files {
        queue.push((Reverse(0), 0, file.clone()));
    }
    
    // BFS traversal for in-project files only
    while let Some((Reverse(_), depth, current_file)) = queue.pop() {
        if depth >= max_depth {
            continue;
        }
        
        // Calculate edge counts between current file and potential next files
        let mut file_edge_counts: HashMap<String, usize> = HashMap::new();
        
        // Find symbols defined in current file
        let symbols_in_file: Vec<_> = repo_mapper.get_all_symbols()
            .iter()
            .filter(|(_, symbol)| symbol.file_path == current_file)
            .map(|(fqn, _)| fqn.clone())
            .collect();
        
        // Count references FROM current file TO other files
        for symbol_fqn in &symbols_in_file {
            let references = repo_mapper.find_symbol_references_by_fqn(symbol_fqn);
            for reference in references {
                if !visited.contains(&reference.reference_file) && reference.reference_file != current_file {
                    
                    // Check if target file is cross-project
                    if detector.is_cross_project_symbol(&reference.reference_file) {
                        // Add to cross-project files but dont traverse further
                        if cross_project_files.insert(reference.reference_file.clone()) {
                            debug!("Found cross-project boundary: {} -> {}", 
                                  current_file, reference.reference_file);
                        }
                        visited.insert(reference.reference_file.clone());
                    } else {
                        // Regular in-project file
                        *file_edge_counts.entry(reference.reference_file.clone()).or_insert(0) += 1;
                    }
                }
            }
        }
        
        // Add in-project files to queue prioritized by edge count
        for (file_path, edge_count) in file_edge_counts {
            if !visited.contains(&file_path) {
                visited.insert(file_path.clone());
                in_project_files.insert(file_path.clone());
                queue.push((Reverse(-(edge_count as i32)), depth + 1, file_path));
            }
        }
    }
    
    println!("DEBUG: Fallback BFS found {} in-project files, {} cross-project files", 
             in_project_files.len(), cross_project_files.len());
    
    Ok((in_project_files.into_iter().collect(), cross_project_files.into_iter().collect()))
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