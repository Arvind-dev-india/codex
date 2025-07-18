use std::collections::HashMap;
use lru::LruCache;
use std::num::NonZeroUsize;
use serde_json::{json, Value};
use tracing::{info, debug, error};
use crate::code_analysis::handle_analyze_code;
use crate::config_types::SupplementaryProjectConfig;

/// Enhanced registry for supplementary project symbols with AST-first approach (PHASE 2)
/// This replaces the heavy full-graph approach for supplementary projects
#[derive(Debug)]
pub struct SupplementarySymbolRegistry {
    /// Map: FQN → SupplementarySymbolInfo
    pub symbols: HashMap<String, SupplementarySymbolInfo>,
    /// Map: file_path → Vec<FQN> (for skeleton generation)
    pub file_to_symbols: HashMap<String, Vec<String>>,
    /// Map: project_name → Vec<FQN>
    pub project_to_symbols: HashMap<String, Vec<String>>,
    /// Total number of projects in registry
    pub project_count: usize,
    
    /// PHASE 2: LRU cache for AST relationship results (symbol_name -> relationships)
    ast_relationship_cache: LruCache<String, Vec<ASTRelationshipResult>>,
    
    /// PHASE 2: Cache for traditional FQN/name matching (fallback only)
    traditional_match_cache: LruCache<String, Vec<TraditionalMatchResult>>,
    
    /// PHASE 2: Performance metrics
    pub metrics: CrossProjectMetrics,
}

/// Lightweight symbol information for supplementary projects
#[derive(Debug, Clone)]
pub struct SupplementarySymbolInfo {
    pub fqn: String,
    pub name: String,
    pub file_path: String,
    pub symbol_type: String,
    pub project_name: String,
    pub start_line: u32,
    pub end_line: u32,
    pub parent: Option<String>,
}

impl SupplementarySymbolRegistry {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            file_to_symbols: HashMap::new(),
            project_to_symbols: HashMap::new(),
            project_count: 0,
            // PHASE 2: Initialize caches with reasonable sizes
            ast_relationship_cache: LruCache::new(NonZeroUsize::new(1000).unwrap()),
            traditional_match_cache: LruCache::new(NonZeroUsize::new(500).unwrap()),
            metrics: CrossProjectMetrics::default(),
        }
    }
    
    /// Add a symbol to the registry with all necessary mappings
    pub fn add_symbol(&mut self, symbol: SupplementarySymbolInfo) {
        let fqn = symbol.fqn.clone();
        let file_path = symbol.file_path.clone();
        let project_name = symbol.project_name.clone();
        
        // Add to main symbol map
        self.symbols.insert(fqn.clone(), symbol);
        
        // Add to file mapping
        self.file_to_symbols
            .entry(file_path)
            .or_insert_with(Vec::new)
            .push(fqn.clone());
        
        // Add to project mapping
        self.project_to_symbols
            .entry(project_name)
            .or_insert_with(Vec::new)
            .push(fqn);
    }
    
    /// Lookup symbol by FQN (for cross-project resolution)
    pub fn lookup_by_fqn(&self, fqn: &str) -> Option<&SupplementarySymbolInfo> {
        self.symbols.get(fqn)
    }
    
    /// Get all symbols in a specific file (for skeleton generation)
    pub fn get_symbols_in_file(&self, file_path: &str) -> Vec<&SupplementarySymbolInfo> {
        self.file_to_symbols
            .get(file_path)
            .map(|fqns| fqns.iter().filter_map(|fqn| self.symbols.get(fqn)).collect())
            .unwrap_or_default()
    }
    
    /// Get all symbols from a specific project
    pub fn get_symbols_in_project(&self, project_name: &str) -> Vec<&SupplementarySymbolInfo> {
        self.project_to_symbols
            .get(project_name)
            .map(|fqns| fqns.iter().filter_map(|fqn| self.symbols.get(fqn)).collect())
            .unwrap_or_default()
    }
    
    /// Check if a file belongs to any supplementary project
    pub fn contains_file(&self, file_path: &str) -> bool {
        self.file_to_symbols.contains_key(file_path)
    }
    
    /// Get project name for a file (if it exists in registry)
    pub fn get_project_for_file(&self, file_path: &str) -> Option<String> {
        self.get_symbols_in_file(file_path)
            .first()
            .map(|symbol| symbol.project_name.clone())
    }
    
    /// Get statistics about the registry
    pub fn get_stats(&self) -> SupplementaryRegistryStats {
        SupplementaryRegistryStats {
            total_symbols: self.symbols.len(),
            total_files: self.file_to_symbols.len(),
            total_projects: self.project_count,
            symbols_by_type: self.get_symbols_by_type(),
        }
    }
    
    fn get_symbols_by_type(&self) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for symbol in self.symbols.values() {
            *counts.entry(symbol.symbol_type.clone()).or_insert(0) += 1;
        }
        counts
    }
}

#[derive(Debug)]
pub struct SupplementaryRegistryStats {
    pub total_symbols: usize,
    pub total_files: usize,
    pub total_projects: usize,
    pub symbols_by_type: HashMap<String, usize>,
}

/// Extract FQN-only symbols from supplementary projects (optimized approach)
pub async fn extract_supplementary_symbols_lightweight(
    projects: &[SupplementaryProjectConfig]
) -> Result<SupplementarySymbolRegistry, String> {
    let mut registry = SupplementarySymbolRegistry::new();
    registry.project_count = projects.len();
    
    info!("Extracting FQN-only symbols from {} supplementary projects", projects.len());
    
    // Process projects in parallel for efficiency
    let mut handles = Vec::new();
    
    for project in projects {
        let project_clone = project.clone();
        let handle = tokio::spawn(async move {
            extract_project_symbols_lightweight(&project_clone).await
        });
        handles.push(handle);
    }
    
    // Collect results from all projects
    let results = futures::future::join_all(handles).await;
    
    for (i, result) in results.into_iter().enumerate() {
        let project = &projects[i];
        match result {
            Ok(Ok(symbols)) => {
                info!("Extracted {} FQN symbols from supplementary project '{}'", 
                      symbols.len(), project.name);
                for symbol in symbols {
                    registry.add_symbol(symbol);
                }
            }
            Ok(Err(e)) => {
                error!("Failed to extract symbols from supplementary project '{}': {}", 
                       project.name, e);
            }
            Err(e) => {
                error!("Task failed for supplementary project '{}': {}", project.name, e);
            }
        }
    }
    
    let stats = registry.get_stats();
    info!("Supplementary registry created: {} symbols, {} files, {} projects", 
          stats.total_symbols, stats.total_files, stats.total_projects);
    
    debug!("Symbol types in registry: {:?}", stats.symbols_by_type);
    
    Ok(registry)
}

/// Extract symbols from a single supplementary project using direct analysis
async fn extract_project_symbols_lightweight(
    project: &SupplementaryProjectConfig
) -> Result<Vec<SupplementarySymbolInfo>, String> {
    let mut symbols = Vec::new();
    
    debug!("Processing supplementary project: {} at {}", project.name, project.path);
    
    // Collect files in the project
    let mut files = Vec::new();
    collect_supplementary_files(&project.path, &mut files, &project.languages)?;
    
    info!("Found {} files in supplementary project '{}'", files.len(), project.name);
    
    // Process files using direct analysis (no graph needed)
    for file_path in files {
        match analyze_supplementary_file(&file_path, &project.name).await {
            Ok(file_symbols) => {
                debug!("Extracted {} symbols from {}", file_symbols.len(), file_path.display());
                symbols.extend(file_symbols);
            }
            Err(e) => {
                debug!("Failed to analyze supplementary file {}: {}", file_path.display(), e);
            }
        }
    }
    
    Ok(symbols)
}

/// Analyze a single supplementary file using direct handle_analyze_code
async fn analyze_supplementary_file(
    file_path: &std::path::Path,
    project_name: &str,
) -> Result<Vec<SupplementarySymbolInfo>, String> {
    let mut symbols = Vec::new();
    
    // Use direct file analysis (same as handle_analyze_code)
    let input = json!({"file_path": file_path.to_string_lossy()});
    
    match handle_analyze_code(input) {
        Some(Ok(result)) => {
            if let Some(file_symbols) = result.get("symbols").and_then(|s| s.as_array()) {
                for symbol in file_symbols {
                    if let Some(symbol_info) = parse_symbol_to_supplementary_info(
                        symbol, file_path, project_name
                    ) {
                        symbols.push(symbol_info);
                    }
                }
            }
        }
        Some(Err(e)) => {
            return Err(format!("Analysis failed: {}", e));
        }
        None => {
            return Err("No result from analysis".to_string());
        }
    }
    
    Ok(symbols)
}

/// Parse a symbol from analyze_code result into SupplementarySymbolInfo
fn parse_symbol_to_supplementary_info(
    symbol: &Value,
    file_path: &std::path::Path,
    project_name: &str,
) -> Option<SupplementarySymbolInfo> {
    let name = symbol.get("name")?.as_str()?.to_string();
    let symbol_type = symbol.get("symbol_type")?.as_str()?.to_string();
    let start_line = symbol.get("start_line")?.as_u64()? as u32;
    let end_line = symbol.get("end_line")?.as_u64()? as u32;
    let parent = symbol.get("parent").and_then(|p| p.as_str()).map(|s| s.to_string());
    
    // Build FQN: project_name.namespace.class.method
    let fqn = build_supplementary_fqn(&name, &parent, project_name);
    
    Some(SupplementarySymbolInfo {
        fqn,
        name,
        file_path: file_path.to_string_lossy().to_string(),
        symbol_type,
        project_name: project_name.to_string(),
        start_line,
        end_line,
        parent,
    })
}

/// Build FQN for supplementary project symbol
fn build_supplementary_fqn(name: &str, parent: &Option<String>, project_name: &str) -> String {
    match parent {
        Some(parent_name) => format!("{}::{}.{}", project_name, parent_name, name),
        None => format!("{}::{}", project_name, name),
    }
}

/// Collect files in supplementary project (similar to existing logic)
fn collect_supplementary_files(
    project_path: &str,
    files: &mut Vec<std::path::PathBuf>,
    languages: &Option<Vec<String>>,
) -> Result<(), String> {
    use std::path::Path;
    
    let path = Path::new(project_path);
    if !path.exists() {
        return Err(format!("Supplementary project path does not exist: {}", project_path));
    }
    
    // Use existing file collection logic from the codebase
    collect_files_recursive(path, files, languages)?;
    
    Ok(())
}

/// Recursive file collection with language filtering
fn collect_files_recursive(
    dir: &std::path::Path,
    files: &mut Vec<std::path::PathBuf>,
    languages: &Option<Vec<String>>,
) -> Result<(), String> {
    use std::fs;
    
    if dir.is_dir() {
        let entries = fs::read_dir(dir)
            .map_err(|e| format!("Failed to read directory {}: {}", dir.display(), e))?;
        
        for entry in entries {
            let entry = entry
                .map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            
            if path.is_dir() {
                // Skip common non-source directories
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if should_skip_directory(dir_name) {
                        continue;
                    }
                }
                collect_files_recursive(&path, files, languages)?;
            } else if path.is_file() {
                if should_include_file(&path, languages) {
                    files.push(path);
                }
            }
        }
    }
    
    Ok(())
}

/// Check if directory should be skipped
fn should_skip_directory(dir_name: &str) -> bool {
    matches!(dir_name, 
        "node_modules" | "target" | "bin" | "obj" | ".git" | 
        ".vs" | ".vscode" | "packages" | "dist" | "build"
    )
}

/// Check if file should be included based on language filters
fn should_include_file(
    file_path: &std::path::Path,
    languages: &Option<Vec<String>>,
) -> bool {
    let extension = file_path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    // If no language filter, include common source files
    let languages = match languages {
        Some(langs) => langs,
        None => return is_source_file_extension(extension),
    };
    
    // Check if file extension matches any of the specified languages
    for language in languages {
        if language_matches_extension(language, extension) {
            return true;
        }
    }
    
    false
}

/// Check if extension is a common source file
fn is_source_file_extension(extension: &str) -> bool {
    matches!(extension.to_lowercase().as_str(),
        "cs" | "ts" | "js" | "py" | "rs" | "cpp" | "c" | "h" | 
        "hpp" | "java" | "go" | "php" | "rb" | "swift" | "kt"
    )
}

/// Check if language matches file extension
fn language_matches_extension(language: &str, extension: &str) -> bool {
    match language.to_lowercase().as_str() {
        "csharp" | "c#" => extension.eq_ignore_ascii_case("cs"),
        "typescript" => extension.eq_ignore_ascii_case("ts"),
        "javascript" => extension.eq_ignore_ascii_case("js"),
        "python" => extension.eq_ignore_ascii_case("py"),
        "rust" => extension.eq_ignore_ascii_case("rs"),
        "cpp" | "c++" => matches!(extension.to_lowercase().as_str(), "cpp" | "cxx" | "cc"),
        "c" => extension.eq_ignore_ascii_case("c"),
        "java" => extension.eq_ignore_ascii_case("java"),
        "go" => extension.eq_ignore_ascii_case("go"),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;
    
    #[test]
    fn test_supplementary_registry_basic() {
        let mut registry = SupplementarySymbolRegistry::new();
        
        let symbol = SupplementarySymbolInfo {
            fqn: "TestProject::TestClass.TestMethod".to_string(),
            name: "TestMethod".to_string(),
            file_path: "/test/file.cs".to_string(),
            symbol_type: "method".to_string(),
            project_name: "TestProject".to_string(),
            start_line: 10,
            end_line: 20,
            parent: Some("TestClass".to_string()),
        };
        
        registry.add_symbol(symbol);
        
        // Test lookup
        assert!(registry.lookup_by_fqn("TestProject::TestClass.TestMethod").is_some());
        assert!(registry.lookup_by_fqn("NonExistent").is_none());
        
        // Test file mapping
        let symbols_in_file = registry.get_symbols_in_file("/test/file.cs");
        assert_eq!(symbols_in_file.len(), 1);
        
        // Test project mapping
        let symbols_in_project = registry.get_symbols_in_project("TestProject");
        assert_eq!(symbols_in_project.len(), 1);
    }
    
    #[test]
    fn test_fqn_building() {
        assert_eq!(
            build_supplementary_fqn("Method", &Some("Class".to_string()), "Project"),
            "Project::Class.Method"
        );
        
        assert_eq!(
            build_supplementary_fqn("Class", &None, "Project"),
            "Project::Class"
        );
    }
    
    #[test]
    fn test_language_matching() {
        assert!(language_matches_extension("csharp", "cs"));
        assert!(language_matches_extension("typescript", "ts"));
        assert!(language_matches_extension("python", "py"));
        assert!(!language_matches_extension("csharp", "ts"));
    }
    
    #[tokio::test]
    async fn test_file_collection() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();
        
        // Create test files
        fs::write(project_path.join("test.cs"), "// C# file").unwrap();
        fs::write(project_path.join("test.ts"), "// TypeScript file").unwrap();
        fs::write(project_path.join("test.txt"), "// Text file").unwrap();
        
        let mut files = Vec::new();
        let languages = Some(vec!["csharp".to_string(), "typescript".to_string()]);
        
        collect_supplementary_files(
            &project_path.to_string_lossy(),
            &mut files,
            &languages,
        ).unwrap();
        
        // Should find 2 files (cs and ts, but not txt)
        assert_eq!(files.len(), 2);
        
        let extensions: Vec<_> = files.iter()
            .filter_map(|f| f.extension().and_then(|e| e.to_str()))
            .collect();
        
        assert!(extensions.contains(&"cs"));
        assert!(extensions.contains(&"ts"));
    }
}
/// PHASE 2: AST-based relationship result with high confidence
#[derive(Debug, Clone)]
pub struct ASTRelationshipResult {
    pub main_symbol_fqn: String,
    pub cross_project_symbol_fqn: String,
    pub relationship_type: String, // "wrapper", "implementation", "inheritance"
    pub confidence: f32, // AST analysis typically gives higher confidence
    pub ast_patterns_detected: Vec<String>, // Debug info about detected patterns
    pub file_path: String,
    pub project_name: String,
}

/// PHASE 2: Traditional matching result (fallback when AST fails)
#[derive(Debug, Clone)]
pub struct TraditionalMatchResult {
    pub cross_project_symbol_fqn: String,
    pub match_type: String, // "fqn_exact", "name_exact", "name_partial"
    pub confidence: f32, // Traditional matching gives lower confidence
    pub file_path: String,
    pub project_name: String,
}

/// PHASE 2: Performance metrics for cross-project analysis
#[derive(Debug, Default)]
pub struct CrossProjectMetrics {
    pub ast_cache_hits: u64,
    pub ast_cache_misses: u64,
    pub ast_analysis_time_ms: u64,
    pub traditional_fallbacks: u64,
    pub total_queries: u64,
}

/// PHASE 2: Unified cross-project relationship result
#[derive(Debug, Clone)]
pub struct CrossProjectRelationship {
    pub main_symbol_fqn: Option<String>, // Some for AST, None for traditional
    pub cross_project_symbol_fqn: String,
    pub relationship_type: String,
    pub confidence: f32,
    pub detection_method: String, // "ast_analysis", "fqn_exact", "name_exact", etc.
    pub file_path: String,
    pub project_name: String,
    pub ast_patterns: Option<Vec<String>>, // Only for AST-detected relationships
}

impl SupplementarySymbolRegistry {
    /// PHASE 2: AST-FIRST query for relationships using AST analysis as primary method
    pub fn query_relationships_ast_first(
        &mut self, 
        symbol_name: &str, 
        main_symbols: &std::collections::HashMap<String, super::context_extractor::CodeSymbol>
    ) -> Vec<CrossProjectRelationship> {
        self.metrics.total_queries += 1;
        let start_time = std::time::Instant::now();

        tracing::debug!("AST-first query for symbol: {}", symbol_name);

        // Step 1: Try AST-based analysis first (PRIMARY)
        let ast_relationships = self.get_ast_relationships_cached(symbol_name, main_symbols);
        
        // Step 2: Traditional matching as fallback (SECONDARY)
        let traditional_relationships = if ast_relationships.is_empty() {
            self.metrics.traditional_fallbacks += 1;
            tracing::debug!("AST analysis found no relationships for '{}', falling back to traditional matching", symbol_name);
            self.get_traditional_relationships_cached(symbol_name)
        } else {
            tracing::debug!("AST analysis found {} relationships for '{}', skipping traditional fallback", 
                           ast_relationships.len(), symbol_name);
            Vec::new()
        };

        // Step 3: Combine results with AST taking priority
        let mut combined_relationships = Vec::new();

        // Add AST relationships (higher priority, higher confidence)
        for ast_rel in ast_relationships {
            combined_relationships.push(CrossProjectRelationship {
                main_symbol_fqn: Some(ast_rel.main_symbol_fqn),
                cross_project_symbol_fqn: ast_rel.cross_project_symbol_fqn,
                relationship_type: ast_rel.relationship_type,
                confidence: ast_rel.confidence,
                detection_method: "ast_analysis".to_string(),
                file_path: ast_rel.file_path,
                project_name: ast_rel.project_name,
                ast_patterns: Some(ast_rel.ast_patterns_detected),
            });
        }

        // Add traditional relationships only if no AST relationships found
        for trad_rel in traditional_relationships {
            combined_relationships.push(CrossProjectRelationship {
                main_symbol_fqn: None, // Traditional matching doesn't provide main symbol context
                cross_project_symbol_fqn: trad_rel.cross_project_symbol_fqn,
                relationship_type: "definition".to_string(), // Traditional matching only finds definitions
                confidence: trad_rel.confidence,
                detection_method: trad_rel.match_type,
                file_path: trad_rel.file_path,
                project_name: trad_rel.project_name,
                ast_patterns: None,
            });
        }

        self.metrics.ast_analysis_time_ms += start_time.elapsed().as_millis() as u64;

        tracing::info!("AST-first query for '{}': {} AST rels, {} traditional rels, {} total", 
                      symbol_name, 
                      ast_relationships.len(), 
                      traditional_relationships.len(),
                      combined_relationships.len());

        combined_relationships
    }

    /// Get AST relationships with caching
    fn get_ast_relationships_cached(
        &mut self, 
        symbol_name: &str, 
        main_symbols: &std::collections::HashMap<String, super::context_extractor::CodeSymbol>
    ) -> Vec<ASTRelationshipResult> {
        // Check cache first
        if let Some(cached_results) = self.ast_relationship_cache.get(symbol_name) {
            self.metrics.ast_cache_hits += 1;
            tracing::debug!("AST cache HIT for '{}': {} relationships", symbol_name, cached_results.len());
            return cached_results.clone();
        }

        self.metrics.ast_cache_misses += 1;
        tracing::debug!("AST cache MISS for '{}', computing relationships", symbol_name);

        // Compute AST relationships
        let ast_relationships = self.compute_ast_relationships(symbol_name, main_symbols);

        // Cache the results
        self.ast_relationship_cache.put(symbol_name.to_string(), ast_relationships.clone());

        tracing::debug!("Cached {} AST relationships for '{}'", ast_relationships.len(), symbol_name);
        ast_relationships
    }

    /// Compute AST relationships using pure Tree-sitter analysis
    fn compute_ast_relationships(
        &self, 
        symbol_name: &str, 
        main_symbols: &std::collections::HashMap<String, super::context_extractor::CodeSymbol>
    ) -> Vec<ASTRelationshipResult> {
        let mut ast_relationships = Vec::new();

        // Get main project symbols with this name
        let main_symbols_with_name: Vec<_> = main_symbols
            .values()
            .filter(|s| s.name == symbol_name)
            .collect();

        // Get cross-project symbols with this name
        let cross_symbols_with_name: Vec<_> = self.symbols
            .iter()
            .filter(|(_, s)| s.name == symbol_name)
            .collect();

        tracing::debug!("AST analysis: {} main symbols, {} cross-project symbols with name '{}'", 
                       main_symbols_with_name.len(), cross_symbols_with_name.len(), symbol_name);

        // Analyze relationships between same-named symbols using AST
        for main_symbol in &main_symbols_with_name {
            for (cross_fqn, cross_symbol) in &cross_symbols_with_name {
                // Use the existing pure AST-based relationship detection
                let relationship_type = super::tools::detect_symbol_relationship(main_symbol, cross_symbol);
                
                if relationship_type != "unrelated" {
                    let confidence = self.calculate_ast_confidence(&relationship_type);
                    
                    ast_relationships.push(ASTRelationshipResult {
                        main_symbol_fqn: main_symbol.fqn.clone(),
                        cross_project_symbol_fqn: cross_fqn.clone(),
                        relationship_type: relationship_type.clone(),
                        confidence,
                        ast_patterns_detected: vec!["tree_sitter_analysis".to_string()], // TODO: Add actual patterns
                        file_path: cross_symbol.file_path.clone(),
                        project_name: cross_symbol.project_name.clone(),
                    });

                    tracing::info!("AST detected {} relationship: {} -> {} (confidence: {})", 
                                  relationship_type,
                                  main_symbol.fqn, 
                                  cross_fqn, 
                                  confidence);
                }
            }
        }

        ast_relationships
    }

    /// Calculate confidence score for AST-detected relationships
    fn calculate_ast_confidence(&self, relationship_type: &str) -> f32 {
        match relationship_type {
            "wrapper" => 0.85,           // High confidence for wrapper detection
            "implementation" => 0.90,    // Very high confidence for interface implementation
            "inheritance" => 0.90,       // Very high confidence for class inheritance
            "possible_wrapper" => 0.60,  // Medium confidence for uncertain patterns
            _ => 0.30,                   // Low confidence for other relationships
        }
    }

    /// Get traditional relationships with caching (fallback only)
    fn get_traditional_relationships_cached(&mut self, symbol_name: &str) -> Vec<TraditionalMatchResult> {
        // Check cache first
        if let Some(cached_results) = self.traditional_match_cache.get(symbol_name) {
            tracing::debug!("Traditional cache HIT for '{}': {} relationships", symbol_name, cached_results.len());
            return cached_results.clone();
        }

        tracing::debug!("Traditional cache MISS for '{}', computing fallback relationships", symbol_name);

        // Compute traditional relationships (simple name/FQN matching)
        let traditional_relationships = self.compute_traditional_relationships(symbol_name);

        // Cache the results
        self.traditional_match_cache.put(symbol_name.to_string(), traditional_relationships.clone());

        traditional_relationships
    }

    /// Compute traditional relationships (fallback when AST fails)
    fn compute_traditional_relationships(&self, symbol_name: &str) -> Vec<TraditionalMatchResult> {
        self.symbols
            .iter()
            .filter_map(|(fqn, symbol)| {
                let (match_type, confidence) = if symbol.name == symbol_name {
                    ("name_exact".to_string(), 0.70) // Lower confidence than AST
                } else if symbol.name.contains(symbol_name) {
                    ("name_partial".to_string(), 0.50)
                } else if fqn.contains(symbol_name) {
                    ("fqn_partial".to_string(), 0.40)
                } else {
                    return None;
                };

                Some(TraditionalMatchResult {
                    cross_project_symbol_fqn: fqn.clone(),
                    match_type,
                    confidence,
                    file_path: symbol.file_path.clone(),
                    project_name: symbol.project_name.clone(),
                })
            })
            .collect()
    }

    /// Get cache statistics for monitoring
    pub fn get_cache_stats(&self) -> Value {
        json!({
            "ast_cache": {
                "capacity": self.ast_relationship_cache.cap().get(),
                "current_size": self.ast_relationship_cache.len(),
                "hit_rate": if self.metrics.total_queries > 0 {
                    self.metrics.ast_cache_hits as f64 / self.metrics.total_queries as f64
                } else { 0.0 }
            },
            "traditional_cache": {
                "capacity": self.traditional_match_cache.cap().get(),
                "current_size": self.traditional_match_cache.len()
            },
            "metrics": {
                "total_queries": self.metrics.total_queries,
                "ast_cache_hits": self.metrics.ast_cache_hits,
                "ast_cache_misses": self.metrics.ast_cache_misses,
                "traditional_fallbacks": self.metrics.traditional_fallbacks,
                "avg_analysis_time_ms": if self.metrics.total_queries > 0 {
                    self.metrics.ast_analysis_time_ms / self.metrics.total_queries
                } else { 0 }
            }
        })
    }

    /// Clear caches (useful for testing or memory management)
    pub fn clear_caches(&mut self) {
        self.ast_relationship_cache.clear();
        self.traditional_match_cache.clear();
        tracing::info!("Cleared cross-project relationship caches");
    }
}

impl Clone for SupplementarySymbolRegistry {
    fn clone(&self) -> Self {
        Self {
            symbols: self.symbols.clone(),
            file_to_symbols: self.file_to_symbols.clone(),
            project_to_symbols: self.project_to_symbols.clone(),
            project_count: self.project_count,
            // Create new empty caches for the clone (don't copy cache contents)
            ast_relationship_cache: LruCache::new(NonZeroUsize::new(1000).unwrap()),
            traditional_match_cache: LruCache::new(NonZeroUsize::new(500).unwrap()),
            metrics: CrossProjectMetrics::default(),
        }
    }
}
