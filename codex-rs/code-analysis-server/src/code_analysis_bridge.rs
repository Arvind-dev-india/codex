//! Bridge to connect to the existing code analysis functionality in codex-core

use anyhow::Result;
use codex_core::code_analysis::{
    handle_analyze_code,
    handle_find_symbol_references,
    handle_find_symbol_definitions,
    handle_get_symbol_subgraph,
    handle_get_related_files_skeleton,
    handle_get_multiple_files_skeleton,
    graph_manager,
};
use codex_core::config_types::SupplementaryProjectConfig;
use serde_json::Value;
use tracing::{info, error};
use std::path::{Path, PathBuf};

/// Initialize the code graph for the current directory
pub fn init_code_graph() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    
    // Check if graph is already initialized for this path to avoid redundant initialization
    if graph_manager::is_graph_initialized() {
        info!("Code graph already initialized for: {}", current_dir.display());
        return Ok(());
    }
    
    info!("Initializing code graph for: {}", current_dir.display());
    
    // Use the graph manager to initialize and handle file changes
    match graph_manager::ensure_graph_for_path(&current_dir) {
        Ok(_) => {
            info!("Code graph initialized successfully for: {}", current_dir.display());
            Ok(())
        },
        Err(e) => {
            error!("Failed to initialize code graph: {}", e);
            Err(anyhow::anyhow!("Failed to initialize code graph: {}", e))
        }
    }
}

/// Initialize the code graph for a specific directory and wait for completion
pub async fn init_code_graph_and_wait(project_dir: Option<&Path>) -> Result<()> {
    let target_dir = if let Some(dir) = project_dir {
        dir.to_path_buf()
    } else {
        std::env::current_dir()?
    };
    
    let start_time = std::time::Instant::now();
    info!("Starting code graph initialization for: {}", target_dir.display());
    
    // Force synchronous initialization to ensure it completes
    match graph_manager::initialize_graph_async(&target_dir).await {
        Ok(_) => {
            let elapsed = start_time.elapsed();
            let elapsed_ms = elapsed.as_millis();
            let elapsed_secs = elapsed.as_secs_f64();
            
            // Get the detailed status from the graph manager
            let status = graph_manager::get_graph_status();
            match status {
                codex_core::code_analysis::graph_manager::GraphStatus::Ready { 
                    files_processed, 
                    symbols_found, 
                    initialization_time_ms 
                } => {
                    info!("Code graph initialization completed successfully!");
                    info!("Summary: {} files processed, {} symbols found", files_processed, symbols_found);
                    info!("Total time: {:.2}s ({}ms)", elapsed_secs, initialization_time_ms);
                },
                _ => {
                    info!("Code graph initialization completed in {:.2}s ({}ms)", elapsed_secs, elapsed_ms);
                }
            }
            
            // Log some statistics about what was parsed
            if let Some(symbols) = graph_manager::get_symbols() {
                // Count files by extension
                let mut file_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
                for symbol in symbols.values() {
                    if let Some(ext) = std::path::Path::new(&symbol.file_path).extension() {
                        *file_counts.entry(ext.to_string_lossy().to_string()).or_insert(0) += 1;
                    }
                }
                
                if !file_counts.is_empty() {
                    info!("Files with symbols by extension:");
                    for (ext, count) in file_counts {
                        info!("  .{}: {} files", ext, count);
                    }
                }
            }
            
            Ok(())
        },
        Err(e) => {
            let elapsed = start_time.elapsed();
            error!("Code graph initialization failed after {:.2}s: {}", elapsed.as_secs_f64(), e);
            Err(anyhow::anyhow!("Failed to initialize code graph: {}", e))
        }
    }
}

/// Initialize code graph with supplementary projects
pub async fn init_code_graph_with_supplementary(
    root_path: Option<PathBuf>,
    supplementary_projects: Vec<SupplementaryProjectConfig>
) -> Result<()> {
    let root = root_path.unwrap_or_else(|| std::env::current_dir().unwrap());
    
    // NOTE: Cleanup is now handled by main.rs before calling this function
    // Removing duplicate cleanup to prevent wiping out previous work
    
    // Log supplementary project configuration for debugging
    info!("Supplementary project configuration:");
    for (i, project) in supplementary_projects.iter().enumerate() {
        info!("  {}. {} -> {} (priority: {})", i + 1, project.name, project.path, project.priority);
    }
    
    let start_time = std::time::Instant::now();
    info!("Starting code graph initialization with {} supplementary projects for: {}", 
          supplementary_projects.len(), root.display());
    
    // Setup detailed logging for supplementary projects
    let log_file_path = setup_supplementary_logging(&root)?;
    
    // Start main project initialization and supplementary projects in parallel
    info!("Initializing main project graph for: {}", root.display());
    
    // Create futures for parallel processing
    let main_project_future = graph_manager::initialize_graph_async(&root);
    let supplementary_future = if !supplementary_projects.is_empty() {
        info!("Starting parallel processing of {} supplementary projects...", supplementary_projects.len());
        Some(process_supplementary_projects(&supplementary_projects, &log_file_path))
    } else {
        None
    };
    
    // Wait for main project to complete first (needed for cross-project analysis)
    match main_project_future.await {
        Ok(_) => {
            let elapsed = start_time.elapsed();
            let elapsed_secs = elapsed.as_secs_f64();
            
            // Get main project statistics
            let main_stats = get_main_project_stats();
            info!("Main project graph initialization completed in {:.2}s", elapsed_secs);
            info!("Main project stats: {} nodes, {} edges, {} symbols", 
                  main_stats.nodes, main_stats.edges, main_stats.symbols);
            
            // Wait for supplementary projects if they were started
            if let Some(supplementary_future) = supplementary_future {
                info!("Waiting for supplementary projects to complete...");
                info!("Supplementary project debug log: {}", log_file_path.display());
                
                let supplementary_stats = supplementary_future.await?;
                
                for project in &supplementary_projects {
                    info!("  - {} at {} (priority: {})", project.name, project.path, project.priority);
                }
                
                // Log combined statistics
                let total_nodes = main_stats.nodes + supplementary_stats.total_nodes;
                let total_edges = main_stats.edges + supplementary_stats.total_edges;
                let cross_project_edges = supplementary_stats.cross_project_edges;
                
                info!("Combined project stats: {} total nodes, {} total edges", total_nodes, total_edges);
                if cross_project_edges > 0 {
                    info!("Cross-project edges created: {} (logged to debug file)", cross_project_edges);
                } else {
                    info!("No cross-project edges found (main project is self-contained)");
                }
                
                log_final_statistics(&log_file_path, &main_stats, &supplementary_stats)?;
            }
            
            Ok(())
        },
        Err(e) => {
            let elapsed = start_time.elapsed();
            error!("Code graph initialization failed after {:.2}s: {}", elapsed.as_secs_f64(), e);
            Err(anyhow::anyhow!("Failed to initialize code graph: {}", e))
        }
    }
}

/// Setup logging for supplementary project debugging
fn setup_supplementary_logging(root_path: &Path) -> Result<PathBuf> {
    let log_dir = root_path.join(".codex");
    std::fs::create_dir_all(&log_dir)?;
    
    let log_file = log_dir.join("supplementary_projects_debug.log");
    
    // Always clear the log file to prevent contamination from previous runs
    info!("Clearing previous supplementary projects debug log to prevent contamination");
    std::fs::write(&log_file, format!(
        "=== Supplementary Projects Debug Log ===\n\
         Started at: {}\n\
         Main project: {}\n\
         Note: Previous log cleared to prevent cross-run contamination\n\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
        root_path.display()
    ))?;
    
    Ok(log_file)
}

/// Log detailed supplementary project information
fn log_supplementary_projects_details(
    projects: &[SupplementaryProjectConfig],
    log_file: &Path
) -> Result<()> {
    use std::io::Write;
    
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)?;
    
    writeln!(file, "=== Supplementary Projects Configuration ===")?;
    writeln!(file, "Total projects: {}", projects.len())?;
    writeln!(file)?;
    
    for (i, project) in projects.iter().enumerate() {
        writeln!(file, "Project #{}: {}", i + 1, project.name)?;
        writeln!(file, "  Path: {}", project.path)?;
        writeln!(file, "  Priority: {}", project.priority)?;
        writeln!(file, "  Enabled: {}", project.enabled)?;
        
        if let Some(languages) = &project.languages {
            writeln!(file, "  Languages: {}", languages.join(", "))?;
        } else {
            writeln!(file, "  Languages: all supported")?;
        }
        
        if let Some(desc) = &project.description {
            writeln!(file, "  Description: {}", desc)?;
        }
        
        // Check if path exists
        let path_exists = std::path::Path::new(&project.path).exists();
        writeln!(file, "  Path exists: {}", path_exists)?;
        
        if path_exists {
            // Count potential files
            if let Ok(file_count) = count_supported_files(&project.path, &project.languages) {
                writeln!(file, "  Supported files found: {}", file_count)?;
            }
        }
        
        writeln!(file)?;
    }
    
    writeln!(file, "=== Cross-Project Edges (will be populated when implemented) ===")?;
    writeln!(file, "Format: primary_project:file:symbol ---------> secondary_project:file:symbol")?;
    writeln!(file)?;
    
    file.flush()?;
    Ok(())
}

/// Count supported files in a supplementary project
fn count_supported_files(project_path: &str, language_filter: &Option<Vec<String>>) -> Result<usize> {
    use std::fs;
    use codex_core::code_analysis::parser_pool::SupportedLanguage;
    
    let mut count = 0;
    let path = std::path::Path::new(project_path);
    
    if !path.exists() {
        return Ok(0);
    }
    
    fn count_recursive(
        dir: &std::path::Path, 
        language_filter: &Option<Vec<String>>, 
        count: &mut usize
    ) -> Result<()> {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                
                if path.is_dir() {
                    let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if !dir_name.starts_with('.') && !["node_modules", "target", "dist", "bin", "obj"].contains(&dir_name) {
                        count_recursive(&path, language_filter, count)?;
                    }
                } else if path.is_file() {
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        if let Some(language) = SupportedLanguage::from_extension(ext) {
                            // Apply language filter if specified
                            if let Some(filter) = language_filter {
                                let lang_name = format!("{:?}", language).to_lowercase();
                                if filter.iter().any(|f| f.to_lowercase() == lang_name) {
                                    *count += 1;
                                }
                            } else {
                                *count += 1;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
    
    count_recursive(path, language_filter, &mut count)?;
    Ok(count)
}

/// Statistics for project analysis
#[derive(Debug, Default)]
struct ProjectStats {
    nodes: usize,
    edges: usize,
    symbols: usize,
}

/// Statistics for supplementary projects
#[derive(Debug, Default)]
struct SupplementaryStats {
    total_nodes: usize,
    total_edges: usize,
    total_symbols: usize,
    cross_project_edges: usize,
    projects_processed: usize,
}

/// Get main project statistics
fn get_main_project_stats() -> ProjectStats {
    match graph_manager::get_graph_manager().read() {
        Ok(manager) => {
            if let Some(repo_mapper) = manager.get_repo_mapper() {
                info!("Debug: Getting main project stats from repo mapper");
                
                // Get symbol count from storage if available, otherwise from context extractor
                let symbols = if let Some(storage) = manager.get_symbol_storage() {
                    repo_mapper.get_all_symbols_from_storage(Some(storage))
                } else {
                    repo_mapper.get_all_symbols().clone()
                };
                let symbol_count = symbols.len();
                info!("Debug: Found {} symbols in main project", symbol_count);
                
                // Get actual graph statistics
                let graph = repo_mapper.get_graph();
                let node_count = graph.nodes.len();
                let edge_count = graph.edges.len();
                info!("Debug: Graph has {} nodes, {} edges", node_count, edge_count);
                
                ProjectStats {
                    nodes: node_count,
                    edges: edge_count,
                    symbols: symbol_count,
                }
            } else {
                info!("Debug: No repo mapper found in graph manager");
                ProjectStats {
                    nodes: 0,
                    edges: 0,
                    symbols: 0,
                }
            }
        },
        Err(e) => {
            info!("Debug: Failed to read graph manager: {:?}", e);
            ProjectStats {
                nodes: 0,
                edges: 0,
                symbols: 0,
            }
        }
    }
}

/// Process supplementary projects in parallel and return statistics
async fn process_supplementary_projects(
    projects: &[SupplementaryProjectConfig],
    log_file: &Path
) -> Result<SupplementaryStats> {
    let mut stats = SupplementaryStats::default();
    
    log_supplementary_projects_details(projects, log_file)?;
    
    // Get unresolved references from main project (only these need cross-project resolution)
    let main_unresolved_refs = get_main_project_unresolved_references();
    info!("Found {} unresolved references in main project (out of {} total)", 
          main_unresolved_refs.len(), get_main_project_total_references());
    
    if main_unresolved_refs.is_empty() {
        info!("No unresolved references in main project - skipping cross-project analysis");
        return Ok(stats);
    }
    
    // Parse all supplementary projects in TRUE parallel (definitions only, no references)
    info!("Parsing {} supplementary projects in parallel (definitions only)...", projects.len());
    let mut supp_handles = Vec::new();
    
    for project in projects {
        let project_clone = project.clone();
        let handle = tokio::spawn(async move {
            parse_supplementary_definitions_only(&project_clone).await
        });
        supp_handles.push(handle);
    }
    
    // Wait for all parallel tasks to complete
    let supp_results = futures::future::join_all(supp_handles).await;
    
    // Combine all supplementary symbols and collect statistics
    let mut all_supp_symbols = std::collections::HashMap::new();
    for (i, join_result) in supp_results.into_iter().enumerate() {
        let project = &projects[i];
        match join_result {
            Ok(parse_result) => {
                match parse_result {
                    Ok(symbols) => {
                        info!("  {} parsed: {} symbols", project.name, symbols.len());
                        stats.total_symbols += symbols.len();
                        stats.total_nodes += symbols.len();
                        stats.projects_processed += 1;
                        all_supp_symbols.extend(symbols);
                    }
                    Err(e) => {
                        error!("Failed to parse supplementary project {}: {}", project.name, e);
                    }
                }
            }
            Err(e) => {
                error!("Failed to spawn task for supplementary project {}: {}", project.name, e);
            }
        }
    }
    
    if all_supp_symbols.is_empty() {
        info!("No symbols found in supplementary projects");
        return Ok(stats);
    }
    
    // Efficient O(N) cross-project matching (only for unresolved references)
    info!("Creating cross-project edges: {} unresolved refs vs {} supplementary symbols", 
          main_unresolved_refs.len(), all_supp_symbols.len());
    
    let cross_edges = create_cross_project_edges_simple(&main_unresolved_refs, &all_supp_symbols, log_file)?;
    stats.cross_project_edges = cross_edges.len();
    stats.total_edges = cross_edges.len(); // Only count actual cross-project edges
    
    info!("Created {} actual cross-project edges", cross_edges.len());
    
    Ok(stats)
}

/// Analyze a single supplementary project and find actual cross-project references
async fn analyze_supplementary_project(
    project: &SupplementaryProjectConfig,
    log_file: &Path
) -> Result<ProjectStats> {
    let mut stats = ProjectStats::default();
    
    if !std::path::Path::new(&project.path).exists() {
        log_project_analysis(log_file, &project.name, "Path does not exist", &stats)?;
        return Ok(stats);
    }
    
    // Count files and estimate symbols
    let file_count = count_supported_files(&project.path, &project.languages)?;
    
    // Rough estimation based on file count and language
    let estimated_symbols_per_file = match project.languages.as_ref() {
        Some(langs) if langs.contains(&"rust".to_string()) => 15,
        Some(langs) if langs.contains(&"csharp".to_string()) => 12,
        Some(langs) if langs.contains(&"typescript".to_string()) => 10,
        Some(langs) if langs.contains(&"python".to_string()) => 8,
        _ => 10, // default
    };
    
    stats.symbols = file_count * estimated_symbols_per_file;
    stats.nodes = stats.symbols + file_count; // symbols + files
    stats.edges = stats.symbols * 2; // rough estimate of references
    
    // Find actual cross-project references
    let cross_project_edges = find_actual_cross_project_references(project, log_file).await?;
    stats.edges = cross_project_edges; // Use actual cross-project edges instead of estimate
    
    log_project_analysis(log_file, &project.name, "Analysis complete", &stats)?;
    
    Ok(stats)
}

/// Find actual cross-project references by analyzing symbol usage (using FQN-based approach like main project)
async fn find_actual_cross_project_references(
    supplementary_project: &SupplementaryProjectConfig,
    log_file: &Path
) -> Result<usize> {
    let mut cross_project_edges = 0;
    
    // Get main project symbols and references
    let main_symbols = get_main_project_symbols();
    let main_references = get_main_project_references();
    
    if main_symbols.is_empty() {
        info!("No main project symbols found for cross-project analysis");
        return Ok(0);
    }
    
    // Get supplementary project symbols by parsing actual files
    let supplementary_symbols = parse_supplementary_project_symbols(supplementary_project).await?;
    
    if supplementary_symbols.is_empty() {
        info!("No symbols found in supplementary project '{}'", supplementary_project.name);
        return Ok(0);
    }
    
    info!("Analyzing cross-project references: {} main symbols vs {} supplementary symbols", 
          main_symbols.len(), supplementary_symbols.len());
    
    // Create FQN lookup for supplementary symbols
    let mut supp_fqn_map = std::collections::HashMap::new();
    let mut supp_name_map = std::collections::HashMap::new();
    
    for supp_symbol in &supplementary_symbols {
        supp_fqn_map.insert(&supp_symbol.fqn, supp_symbol);
        supp_name_map.entry(&supp_symbol.name).or_insert_with(Vec::new).push(supp_symbol);
    }
    
    // Find actual cross-project references using the same logic as main project
    // Look for references in main project that point to supplementary project symbols
    for reference in &main_references {
        // Skip built-in types and internal references
        if should_exclude_symbol(&reference.symbol_name) {
            continue;
        }
        
        // Try to match by FQN first (most precise)
        if !reference.symbol_fqn.is_empty() {
            if let Some(supp_symbol) = supp_fqn_map.get(&reference.symbol_fqn) {
                cross_project_edges += 1;
                
                log_actual_cross_project_edge(
                    log_file,
                    "main",
                    &reference.reference_file,
                    &reference.symbol_name,
                    &supplementary_project.name,
                    &supp_symbol.file_path,
                    &supp_symbol.name,
                    &format!("{:?}", reference.reference_type)
                )?;
                continue;
            }
        }
        
        // Fall back to name matching (less precise, but sometimes necessary)
        if let Some(matching_symbols) = supp_name_map.get(&reference.symbol_name) {
            // Only create edge if there's exactly one match to avoid ambiguity
            if matching_symbols.len() == 1 {
                let supp_symbol = matching_symbols[0];
                
                // Additional filtering to ensure it's a meaningful cross-project reference
                if is_meaningful_cross_project_reference(&reference.symbol_name, &supp_symbol.symbol_type) {
                    cross_project_edges += 1;
                    
                    log_actual_cross_project_edge(
                        log_file,
                        "main",
                        &reference.reference_file,
                        &reference.symbol_name,
                        &supplementary_project.name,
                        &supp_symbol.file_path,
                        &supp_symbol.name,
                        &format!("{:?}", reference.reference_type)
                    )?;
                }
            }
        }
    }
    
    info!("Found {} actual cross-project references for '{}'", cross_project_edges, supplementary_project.name);
    Ok(cross_project_edges)
}

/// Check if a reference represents a meaningful cross-project dependency
fn is_meaningful_cross_project_reference(symbol_name: &str, symbol_type: &str) -> bool {
    // Only consider references to classes, interfaces, services, etc. as meaningful
    // Skip variables, constants, and other low-level symbols
    match symbol_type {
        "Class" | "Interface" | "Struct" | "Enum" | "Module" => true,
        "Function" | "Method" => {
            // Only include functions/methods that look like APIs (not internal helpers)
            !symbol_name.starts_with("_") && 
            !symbol_name.to_lowercase().contains("helper") &&
            !symbol_name.to_lowercase().contains("internal")
        },
        _ => false,
    }
}

/// Get unresolved references from main project (only these need cross-project resolution)
fn get_main_project_unresolved_references() -> Vec<SimpleReference> {
    match graph_manager::get_graph_manager().read() {
        Ok(manager) => {
            if let Some(repo_mapper) = manager.get_repo_mapper() {
                // Get unresolved references from the context extractor
                let context_extractor = repo_mapper.get_context_extractor();
                let unresolved_refs = context_extractor.find_unresolved_references();
                
                let simple_references: Vec<SimpleReference> = unresolved_refs.iter().map(|reference| SimpleReference {
                    symbol_name: reference.symbol_name.clone(),
                    symbol_fqn: reference.symbol_fqn.clone(),
                    reference_file: reference.reference_file.clone(),
                    reference_line: reference.reference_line,
                    reference_type: reference.reference_type.clone(),
                }).collect();
                
                info!("Debug: Found {} unresolved references for cross-project analysis", simple_references.len());
                simple_references
            } else {
                info!("Debug: No repo mapper available for unresolved reference retrieval");
                Vec::new()
            }
        },
        Err(e) => {
            info!("Debug: Error accessing graph manager for unresolved references: {:?}", e);
            Vec::new()
        }
    }
}

/// Get total number of references in main project (for statistics)
fn get_main_project_total_references() -> usize {
    match graph_manager::get_graph_manager().read() {
        Ok(manager) => {
            if let Some(repo_mapper) = manager.get_repo_mapper() {
                repo_mapper.get_all_references().len()
            } else {
                0
            }
        },
        Err(_) => 0
    }
}

/// Get main project symbols
fn get_main_project_symbols() -> Vec<SimpleSymbol> {
    match graph_manager::get_graph_manager().read() {
        Ok(manager) => {
            if let Some(repo_mapper) = manager.get_repo_mapper() {
                // Get symbols from storage if available, otherwise from context extractor
                let symbols = if let Some(storage) = manager.get_symbol_storage() {
                    repo_mapper.get_all_symbols_from_storage(Some(storage))
                } else {
                    repo_mapper.get_all_symbols().clone()
                };
                info!("Debug: Retrieved {} symbols for cross-project analysis", symbols.len());
                
                let simple_symbols: Vec<SimpleSymbol> = symbols.iter().map(|(fqn, symbol)| SimpleSymbol {
                    name: symbol.name.clone(),
                    file_path: symbol.file_path.clone(),
                    symbol_type: format!("{:?}", symbol.symbol_type),
                    fqn: fqn.clone(), // Include FQN for proper matching
                }).collect();
                
                if simple_symbols.len() > 0 {
                    info!("Debug: First few symbols: {:?}", 
                          simple_symbols.iter().take(3).map(|s| &s.name).collect::<Vec<_>>());
                }
                
                simple_symbols
            } else {
                info!("Debug: No repo mapper available for symbol retrieval");
                Vec::new()
            }
        },
        Err(e) => {
            info!("Debug: Error accessing graph manager for symbols: {:?}", e);
            Vec::new()
        }
    }
}

/// Simple symbol representation for cross-project analysis
#[derive(Debug, Clone)]
struct SimpleSymbol {
    name: String,
    file_path: String,
    symbol_type: String,
    fqn: String, // Add FQN for proper symbol matching
}

/// Parse supplementary project symbols (definitions only) for efficient cross-project analysis
async fn parse_supplementary_definitions_only(
    project: &SupplementaryProjectConfig
) -> Result<std::collections::HashMap<String, codex_core::code_analysis::context_extractor::CodeSymbol>> {
    use codex_core::code_analysis::context_extractor::create_context_extractor;
    use std::path::Path;
    
    info!("Parsing definitions only from supplementary project: {}", project.name);
    
    let mut context_extractor = create_context_extractor();
    let project_path = Path::new(&project.path);
    
    if !project_path.exists() {
        info!("Supplementary project path does not exist: {}", project.path);
        return Ok(std::collections::HashMap::new());
    }
    
    // Collect files to process (same logic as main project)
    let mut files_to_process = Vec::new();
    collect_supplementary_files(project_path, &mut files_to_process, &project.languages)
        .map_err(|e| anyhow::anyhow!("Failed to collect files: {}", e))?;
    
    info!("Found {} files to parse in supplementary project '{}'", files_to_process.len(), project.name);
    
    // Process files and extract symbols (definitions only - no references needed)
    let mut processed_count = 0;
    let mut failed_count = 0;
    
    for file_path in &files_to_process {
        match context_extractor.extract_symbols_from_file_incremental(&file_path.to_string_lossy()) {
            Ok(()) => {
                processed_count += 1;
                // NOTE: Only clear references for supplementary projects (not main project)
                // Main project needs references for edge creation
                context_extractor.clear_references();
            }
            Err(e) => {
                failed_count += 1;
                if failed_count <= 5 { // Log first few errors
                    tracing::debug!("Failed to parse supplementary file {}: {}", file_path.display(), e);
                }
            }
        }
    }
    
    info!("Supplementary project '{}': parsed {}/{} files successfully (definitions only)", 
          project.name, processed_count, files_to_process.len());
    
    // Return symbols as HashMap for efficient lookup
    let symbols = context_extractor.get_symbols().clone();
    info!("Extracted {} definitions from supplementary project '{}'", symbols.len(), project.name);
    
    Ok(symbols)
}

/// Parse supplementary project symbols using the same approach as main project
async fn parse_supplementary_project_symbols(
    project: &SupplementaryProjectConfig
) -> Result<Vec<SimpleSymbol>> {
    use codex_core::code_analysis::context_extractor::create_context_extractor;
    use codex_core::code_analysis::parser_pool::SupportedLanguage;
    use std::path::Path;
    
    info!("Parsing actual symbols from supplementary project: {}", project.name);
    
    let mut context_extractor = create_context_extractor();
    let project_path = Path::new(&project.path);
    
    if !project_path.exists() {
        info!("Supplementary project path does not exist: {}", project.path);
        return Ok(Vec::new());
    }
    
    // Collect files to process (same logic as main project)
    let mut files_to_process = Vec::new();
    collect_supplementary_files(project_path, &mut files_to_process, &project.languages)
        .map_err(|e| anyhow::anyhow!("Failed to collect files: {}", e))?;
    
    info!("Found {} files to parse in supplementary project '{}'", files_to_process.len(), project.name);
    
    // Process files and extract symbols (same as main project)
    let mut processed_count = 0;
    let mut failed_count = 0;
    
    for file_path in &files_to_process {
        match context_extractor.extract_symbols_from_file_incremental(&file_path.to_string_lossy()) {
            Ok(()) => {
                processed_count += 1;
            }
            Err(e) => {
                failed_count += 1;
                if failed_count <= 5 { // Log first few errors
                    tracing::debug!("Failed to parse supplementary file {}: {}", file_path.display(), e);
                }
            }
        }
    }
    
    info!("Supplementary project '{}': parsed {}/{} files successfully", 
          project.name, processed_count, files_to_process.len());
    
    // Convert extracted symbols to SimpleSymbol format
    let symbols: Vec<SimpleSymbol> = context_extractor.get_symbols()
        .iter()
        .map(|(fqn, symbol)| SimpleSymbol {
            name: symbol.name.clone(),
            file_path: symbol.file_path.clone(),
            symbol_type: format!("{:?}", symbol.symbol_type),
            fqn: fqn.clone(), // Include FQN for proper matching
        })
        .collect();
    
    info!("Extracted {} real symbols from supplementary project '{}'", symbols.len(), project.name);
    
    Ok(symbols)
}

/// Collect files from supplementary project (same logic as main project)
fn collect_supplementary_files(
    dir_path: &Path, 
    files: &mut Vec<PathBuf>, 
    language_filter: &Option<Vec<String>>
) -> Result<(), String> {
    use codex_core::code_analysis::parser_pool::SupportedLanguage;
    use std::fs;
    
    let entries = fs::read_dir(dir_path)
        .map_err(|e| format!("Failed to read directory {}: {}", dir_path.display(), e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        if path.is_dir() {
            // Skip hidden directories and common directories to ignore
            let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if !dir_name.starts_with('.') && !["node_modules", "target", "dist", "bin", "obj", ".git", ".vs", "packages"].contains(&dir_name) {
                collect_supplementary_files(&path, files, language_filter)?;
            }
        } else if path.is_file() {
            // Check if it's a supported file type
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if let Some(language) = SupportedLanguage::from_extension(ext) {
                    // Apply language filter if specified
                    if let Some(filter) = language_filter {
                        let lang_name = format!("{:?}", language).to_lowercase();
                        if filter.iter().any(|f| f.to_lowercase() == lang_name) {
                            files.push(path);
                        }
                    } else {
                        files.push(path);
                    }
                }
            }
        }
    }

    Ok(())
}

/// Get main project symbol references
fn get_main_project_references() -> Vec<SimpleReference> {
    match graph_manager::get_graph_manager().read() {
        Ok(manager) => {
            if let Some(repo_mapper) = manager.get_repo_mapper() {
                // Get all references from the context extractor
                let references = repo_mapper.get_all_references();
                
                let simple_references: Vec<SimpleReference> = references.iter().map(|reference| SimpleReference {
                    symbol_name: reference.symbol_name.clone(),
                    symbol_fqn: reference.symbol_fqn.clone(),
                    reference_file: reference.reference_file.clone(),
                    reference_line: reference.reference_line,
                    reference_type: reference.reference_type.clone(),
                }).collect();
                
                info!("Debug: Retrieved {} references for cross-project analysis", simple_references.len());
                simple_references
            } else {
                info!("Debug: No repo mapper available for reference retrieval");
                Vec::new()
            }
        },
        Err(e) => {
            info!("Debug: Error accessing graph manager for references: {:?}", e);
            Vec::new()
        }
    }
}

/// Simple reference representation for cross-project analysis
#[derive(Debug, Clone)]
struct SimpleReference {
    symbol_name: String,
    symbol_fqn: String,
    reference_file: String,
    reference_line: usize,
    reference_type: codex_core::code_analysis::context_extractor::ReferenceType,
}

/// Check if a symbol should be excluded from cross-project analysis
fn should_exclude_symbol(symbol_name: &str) -> bool {
    // Built-in types that should never create cross-project edges
    let builtin_types = [
        // C# built-in types
        "string", "String", "int", "Int32", "long", "Int64", "bool", "Boolean",
        "double", "Double", "float", "Single", "decimal", "Decimal", "char", "Char",
        "byte", "Byte", "sbyte", "SByte", "short", "Int16", "ushort", "UInt16",
        "uint", "UInt32", "ulong", "UInt64", "object", "Object", "void",
        
        // Common framework types
        "DateTime", "TimeSpan", "Guid", "Exception", "Task", "List", "Dictionary",
        "IEnumerable", "ICollection", "IList", "IDictionary", "Array", "Nullable",
        "Action", "Func", "Predicate", "EventHandler", "CancellationToken",
        
        // Common .NET namespaces/types
        "System", "Microsoft", "Windows", "Console", "File", "Directory", "Path",
        "HttpClient", "HttpRequest", "HttpResponse", "JsonSerializer", "Encoding",
        
        // Generic type parameters
        "T", "TKey", "TValue", "TResult", "TSource", "TDestination",
        
        // Very short or generic names that are likely false positives
        "Id", "ID", "Name", "Value", "Type", "Data", "Info", "Item", "Node",
        "Key", "Index", "Count", "Length", "Size", "Status", "State", "Mode",
    ];
    
    // Check if it's a built-in type
    if builtin_types.contains(&symbol_name) {
        return true;
    }
    
    // Check if it's a very short name (likely a variable or generic)
    if symbol_name.len() <= 2 {
        return true;
    }
    
    // Check if it's all uppercase (likely a constant)
    if symbol_name.chars().all(|c| c.is_uppercase() || c == '_') && symbol_name.len() > 3 {
        return true;
    }
    
    // Check if it starts with lowercase (likely a variable/field, not a type)
    if symbol_name.chars().next().map_or(false, |c| c.is_lowercase()) {
        return true;
    }
    
    false
}

/// Check if a symbol is already defined in the main project
fn is_symbol_in_main_project(symbol_name: &str, main_symbols: &[SimpleSymbol]) -> bool {
    main_symbols.iter().any(|main_sym| main_sym.name == symbol_name)
}

/// Check if two symbols are related (with proper filtering)
fn symbols_are_related(main_symbol: &str, supp_symbol: &str, main_symbols: &[SimpleSymbol]) -> bool {
    // First, exclude built-in types and symbols already in main project
    if should_exclude_symbol(supp_symbol) || should_exclude_symbol(main_symbol) {
        return false;
    }
    
    // Don't create edges to symbols that are already defined in the main project
    if is_symbol_in_main_project(supp_symbol, main_symbols) {
        return false;
    }
    
    // Only create edges for meaningful relationships
    // Look for specific patterns that indicate actual dependencies
    let main_lower = main_symbol.to_lowercase();
    let supp_lower = supp_symbol.to_lowercase();
    
    // Interface implementation pattern (e.g., UserService implements IUserService)
    if main_symbol.ends_with("Service") && supp_symbol.starts_with("I") && 
       &supp_symbol[1..] == main_symbol {
        return true;
    }
    
    // Controller-Service pattern (e.g., UserController uses UserService)
    if main_symbol.ends_with("Controller") && supp_symbol.ends_with("Service") {
        let main_base = &main_symbol[..main_symbol.len()-10]; // Remove "Controller"
        let supp_base = &supp_symbol[..supp_symbol.len()-7];  // Remove "Service"
        if main_base == supp_base {
            return true;
        }
    }
    
    // Repository pattern (e.g., UserService uses UserRepository)
    if main_symbol.ends_with("Service") && supp_symbol.ends_with("Repository") {
        let main_base = &main_symbol[..main_symbol.len()-7];  // Remove "Service"
        let supp_base = &supp_symbol[..supp_symbol.len()-10]; // Remove "Repository"
        if main_base == supp_base {
            return true;
        }
    }
    
    // Avoid generic substring matches that create noise
    false
}

/// Check if one symbol references another (with proper filtering)
fn symbol_references_other(referencing_symbol: &str, referenced_symbol: &str, main_symbols: &[SimpleSymbol]) -> bool {
    // Apply the same filtering as symbols_are_related
    if should_exclude_symbol(referenced_symbol) || should_exclude_symbol(referencing_symbol) {
        return false;
    }
    
    // Don't create edges to symbols already in main project
    if is_symbol_in_main_project(referenced_symbol, main_symbols) {
        return false;
    }
    
    // Only look for specific, meaningful reference patterns
    // This is more conservative to avoid false positives
    
    // Base class or interface usage
    if referencing_symbol.contains("Base") && referenced_symbol.starts_with("I") {
        return true;
    }
    
    // Dependency injection pattern
    if referencing_symbol.ends_with("Controller") && referenced_symbol.starts_with("I") && 
       referenced_symbol.ends_with("Service") {
        return true;
    }
    
    false
}

/// Log actual cross-project edge with enhanced formatting
fn log_actual_cross_project_edge(
    log_file: &Path,
    primary_project: &str,
    primary_file: &str,
    primary_symbol: &str,
    secondary_project: &str,
    secondary_file: &str,
    secondary_symbol: &str,
    edge_type: &str
) -> Result<()> {
    use std::io::Write;
    
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)?;
    
    // Create a separate section for cross-project edges
    writeln!(file, 
        "\n=== CROSS-PROJECT EDGE ===\n\
        Time: {}\n\
        Type: {}\n\
        From: {} ({})\n\
        To: {} ({})\n\
        Main Project: {}\n\
        Supplementary Project: {}\n\
        Status: VALID (passed filtering)\n\
        ===========================",
        chrono::Utc::now().format("%H:%M:%S"),
        edge_type,
        primary_symbol,
        primary_file.split('/').last().unwrap_or(primary_file),
        secondary_symbol,
        secondary_file.split('/').last().unwrap_or(secondary_file),
        primary_project,
        secondary_project
    )?;
    
    file.flush()?;
    Ok(())
}

/// Estimate cross-project edges based on supplementary stats
fn estimate_cross_project_edges(stats: &SupplementaryStats) -> usize {
    // Simple heuristic: assume 5% of supplementary symbols might be referenced by main project
    (stats.total_symbols as f64 * 0.05) as usize
}

/// Log project analysis results
fn log_project_analysis(
    log_file: &Path,
    project_name: &str,
    status: &str,
    stats: &ProjectStats
) -> Result<()> {
    use std::io::Write;
    
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)?;
    
    writeln!(file, 
        "[{}] Project '{}': {} - {} symbols, {} nodes, {} edges",
        chrono::Utc::now().format("%H:%M:%S"),
        project_name,
        status,
        stats.symbols,
        stats.nodes,
        stats.edges
    )?;
    
    file.flush()?;
    Ok(())
}

/// Log final combined statistics
fn log_final_statistics(
    log_file: &Path,
    main_stats: &ProjectStats,
    supplementary_stats: &SupplementaryStats
) -> Result<()> {
    use std::io::Write;
    
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)?;
    
    writeln!(file)?;
    writeln!(file, "=== Final Statistics ===")?;
    writeln!(file, "Main project: {} symbols, {} nodes, {} edges", 
             main_stats.symbols, main_stats.nodes, main_stats.edges)?;
    writeln!(file, "Supplementary projects: {} symbols, {} nodes, {} edges", 
             supplementary_stats.total_symbols, supplementary_stats.total_nodes, supplementary_stats.total_edges)?;
    writeln!(file, "Cross-project edges: {} (estimated)", supplementary_stats.cross_project_edges)?;
    writeln!(file, "Total combined: {} symbols, {} nodes, {} edges", 
             main_stats.symbols + supplementary_stats.total_symbols,
             main_stats.nodes + supplementary_stats.total_nodes,
             main_stats.edges + supplementary_stats.total_edges)?;
    writeln!(file)?;
    
    file.flush()?;
    Ok(())
}

/// Log cross-project edge creation
#[allow(dead_code)]
pub fn log_cross_project_edge(
    log_file: &Path,
    primary_project: &str,
    primary_file: &str,
    primary_symbol: &str,
    secondary_project: &str,
    secondary_file: &str,
    secondary_symbol: &str,
    edge_type: &str
) -> Result<()> {
    use std::io::Write;
    
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)?;
    
    writeln!(file, 
        "[{}] {}:{}:{} --{}-> {}:{}:{}",
        chrono::Utc::now().format("%H:%M:%S"),
        primary_project,
        primary_file,
        primary_symbol,
        edge_type,
        secondary_project,
        secondary_file,
        secondary_symbol
    )?;
    
    file.flush()?;
    Ok(())
}

/// Call the analyze_code function from codex-core
pub fn call_analyze_code(args: Value) -> Result<Value> {
    // Check if graph is initialized, but don't force updates since graph manager handles changes
    if !graph_manager::is_graph_initialized() {
        return Err(anyhow::anyhow!("Code graph not initialized. Please wait for initialization to complete."));
    }
    
    match handle_analyze_code(args) {
        Some(Ok(result)) => Ok(result),
        Some(Err(e)) => Err(anyhow::anyhow!("Error in analyze_code: {}", e)),
        None => Err(anyhow::anyhow!("Failed to handle analyze_code")),
    }
}

/// Call the find_symbol_references function from codex-core
pub fn call_find_symbol_references(args: Value) -> Result<Value> {
    // Check if graph is initialized, but don't force updates since graph manager handles changes
    if !graph_manager::is_graph_initialized() {
        return Err(anyhow::anyhow!("Code graph not initialized. Please wait for initialization to complete."));
    }
    
    match handle_find_symbol_references(args) {
        Some(Ok(result)) => Ok(result),
        Some(Err(e)) => Err(anyhow::anyhow!("Error in find_symbol_references: {}", e)),
        None => Err(anyhow::anyhow!("Failed to handle find_symbol_references")),
    }
}

/// Call the find_symbol_definitions function from codex-core
pub fn call_find_symbol_definitions(args: Value) -> Result<Value> {
    // Check if graph is initialized, but don't force updates since graph manager handles changes
    if !graph_manager::is_graph_initialized() {
        return Err(anyhow::anyhow!("Code graph not initialized. Please wait for initialization to complete."));
    }
    
    match handle_find_symbol_definitions(args) {
        Some(Ok(result)) => Ok(result),
        Some(Err(e)) => Err(anyhow::anyhow!("Error in find_symbol_definitions: {}", e)),
        None => Err(anyhow::anyhow!("Failed to handle find_symbol_definitions")),
    }
}

/// Call the get_symbol_subgraph function from codex-core
pub fn call_get_symbol_subgraph(args: Value) -> Result<Value> {
    // Check if graph is initialized, but don't force updates since graph manager handles changes
    if !graph_manager::is_graph_initialized() {
        return Err(anyhow::anyhow!("Code graph not initialized. Please wait for initialization to complete."));
    }
    
    match handle_get_symbol_subgraph(args) {
        Some(Ok(result)) => Ok(result),
        Some(Err(e)) => Err(anyhow::anyhow!("Error in get_symbol_subgraph: {}", e)),
        None => Err(anyhow::anyhow!("Failed to handle get_symbol_subgraph")),
    }
}

/// Call the get_related_files_skeleton function from codex-core
pub fn call_get_related_files_skeleton(args: Value) -> Result<Value> {
    // Skip graph update for skeleton operations since they use cached data
    // and the graph is already initialized during server startup
    match handle_get_related_files_skeleton(args) {
        Some(Ok(result)) => Ok(result),
        Some(Err(e)) => Err(anyhow::anyhow!("Error in get_related_files_skeleton: {}", e)),
        None => Err(anyhow::anyhow!("Failed to handle get_related_files_skeleton")),
    }
}

/// Call the get_multiple_files_skeleton function from codex-core
pub fn call_get_multiple_files_skeleton(args: Value) -> Result<Value> {
    // Skip graph update for skeleton operations since they use cached data
    // and the graph is already initialized during server startup
    match handle_get_multiple_files_skeleton(args) {
        Some(Ok(result)) => Ok(result),
        Some(Err(e)) => Err(anyhow::anyhow!("Error in get_multiple_files_skeleton: {}", e)),
        None => Err(anyhow::anyhow!("Failed to handle get_multiple_files_skeleton")),
    }
}

/// Simple cross-project edge representation
#[derive(Debug, Clone)]
struct CrossProjectEdge {
    main_file: String,
    main_symbol: String,
    main_line: usize,
    supp_file: String,
    supp_symbol: String,
    edge_type: String,
}

/// Create cross-project edges efficiently using O(N) lookup
fn create_cross_project_edges_simple(
    unresolved_refs: &[SimpleReference],
    supp_symbols: &std::collections::HashMap<String, codex_core::code_analysis::context_extractor::CodeSymbol>,
    log_file: &Path
) -> Result<Vec<CrossProjectEdge>> {
    use std::collections::HashMap;
    
    if unresolved_refs.is_empty() || supp_symbols.is_empty() {
        return Ok(Vec::new());
    }
    
    // Build O(1) lookup index for supplementary symbols
    let mut name_index: HashMap<&str, Vec<&codex_core::code_analysis::context_extractor::CodeSymbol>> = HashMap::new();
    
    for (_fqn, symbol) in supp_symbols {
        name_index.entry(symbol.name.as_str()).or_default().push(symbol);
    }
    
    info!("Built lookup index: {} name entries", name_index.len());
    
    // O(N) matching - only process unresolved references
    let mut cross_edges = Vec::new();
    let mut matches = 0;
    let mut ambiguous_skipped = 0;
    
    for reference in unresolved_refs {
        // Try name match (single match only to avoid ambiguity)
        if let Some(candidates) = name_index.get(reference.symbol_name.as_str()) {
            if candidates.len() == 1 {
                let symbol = candidates[0];
                
                // Additional filtering to ensure meaningful cross-project reference
                if is_meaningful_cross_project_reference(&reference.symbol_name, &format!("{:?}", symbol.symbol_type)) {
                    cross_edges.push(CrossProjectEdge {
                        main_file: reference.reference_file.clone(),
                        main_symbol: reference.symbol_name.clone(),
                        main_line: reference.reference_line,
                        supp_file: symbol.file_path.clone(),
                        supp_symbol: symbol.name.clone(),
                        edge_type: format!("{:?}", reference.reference_type),
                    });
                    matches += 1;
                    
                    // Log to debug file
                    let _ = log_actual_cross_project_edge(
                        log_file,
                        "main",
                        &reference.reference_file,
                        &reference.symbol_name,
                        &symbol.project_name(),
                        &symbol.file_path,
                        &symbol.name,
                        "NAME_MATCH"
                    );
                }
            } else if candidates.len() > 1 {
                ambiguous_skipped += 1;
                tracing::debug!("Skipped ambiguous reference '{}' with {} candidates", 
                              reference.symbol_name, candidates.len());
            }
        }
    }
    
    info!("Cross-project matching results: {} matches, {} ambiguous skipped, {} total edges", 
          matches, ambiguous_skipped, cross_edges.len());
    
    Ok(cross_edges)
}
