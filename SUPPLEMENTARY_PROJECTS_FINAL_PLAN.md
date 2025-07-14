# Supplementary Projects - Implementation Plan

## MCP Server CLI Integration

See `CODE_ANALYSIS_MCP_SERVER_CLI_IMPLEMENTATION.md` for complete MCP server CLI arguments implementation.

Quick usage:
```bash
./code-analysis-server --project-dir ./main \
  --supplementary contracts:../contracts:100 \
  --supplementary utils:../utils:50 \
  --supplementary-languages csharp,typescript
```

## Core Principles

1. **No New Tools**: Existing tools work seamlessly with supplementary projects
2. **Smart Fallback**: Only reference supplementary projects when definitions are missing from main project
3. **Built-in Filtering**: Leverage existing `SupportedLanguage::from_extension()` filtering
4. **Minimal Configuration**: Simple, clean config without redundant patterns

## Why No Exclude Patterns Needed

The system already filters effectively:

```rust
// Built-in language filtering in parser_pool.rs
pub fn from_extension(ext: &str) -> Option<Self> {
    match ext.as_str() {
        "rs" => Some(SupportedLanguage::Rust),
        "js" | "jsx" | "mjs" => Some(SupportedLanguage::JavaScript),
        "ts" | "tsx" => Some(SupportedLanguage::TypeScript),
        "py" | "pyw" => Some(SupportedLanguage::Python),
        "go" => Some(SupportedLanguage::Go),
        "cpp" | "cc" | "cxx" | "c++" | "hpp" | "hh" | "hxx" | "h++" | "h" | "c" => Some(SupportedLanguage::Cpp),
        "cs" => Some(SupportedLanguage::CSharp),
        "java" => Some(SupportedLanguage::Java),
        _ => None,  // Everything else ignored automatically
    }
}

// In repo_mapper.rs - only supported files are processed
if SupportedLanguage::from_extension(ext).is_some() {
    // Process file
}
```

**Automatic exclusions already in place:**
- `node_modules/`, `target/`, `.git/`, `dist/` directories
- All unsupported file types (images, docs, configs, etc.)
- Binary files and build artifacts

## Simplified Configuration

### Core Configuration Types

```rust
// File: codex-rs/core/src/config_types.rs

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
pub struct CodeAnalysisConfig {
    /// Supplementary projects for definition fallback
    pub supplementary_projects: Option<Vec<SupplementaryProjectConfig>>,
    
    /// Enable supplementary project fallback (default: true)
    pub enable_supplementary_fallback: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SupplementaryProjectConfig {
    /// Unique name for the project
    pub name: String,
    
    /// Path to project root (absolute or relative to main project)
    pub path: String,
    
    /// Whether this project is enabled
    pub enabled: bool,
    
    /// Priority for symbol resolution (1-100, higher = preferred)
    pub priority: u32,
    
    /// Optional: limit to specific languages (defaults to all supported)
    /// Values: "rust", "javascript", "typescript", "python", "go", "cpp", "csharp", "java"
    pub languages: Option<Vec<String>>,
    
    /// Optional description
    pub description: Option<String>,
}
```

### Example Configuration

```toml
# ~/.codex/config.toml

[code_analysis]
enable_supplementary_fallback = true

# Shared contracts project
[[code_analysis.supplementary_projects]]
name = "shared-contracts"
path = "../shared-contracts"
enabled = true
priority = 100
languages = ["csharp", "typescript"]  # Only analyze these languages
description = "Shared contract definitions"

# Common utilities (all supported languages)
[[code_analysis.supplementary_projects]]
name = "common-utilities"
path = "/path/to/common-utilities"
enabled = true
priority = 50
# No languages specified = analyze all supported types

# Legacy API definitions
[[code_analysis.supplementary_projects]]
name = "legacy-api"
path = "~/projects/legacy-api"
enabled = false  # Disabled for now
priority = 25
languages = ["java", "csharp"]
```

## Implementation Strategy

### Phase 1: Core Data Model (Week 1)

#### 1.1 Minimal Symbol Extensions

```rust
// File: codex-rs/core/src/code_analysis/context_extractor.rs

#[derive(Debug, Clone, PartialEq)]
pub struct CodeSymbol {
    // ... existing fields unchanged ...
    
    /// Optional: Origin project name (None = main project)
    pub origin_project: Option<String>,
}

impl CodeSymbol {
    /// Check if this symbol is from a supplementary project
    pub fn is_supplementary(&self) -> bool {
        self.origin_project.is_some()
    }
    
    /// Get the project name (main or supplementary project name)
    pub fn project_name(&self) -> &str {
        self.origin_project.as_deref().unwrap_or("main")
    }
}
```

#### 1.2 Enhanced Repository Mapper

```rust
// File: codex-rs/core/src/code_analysis/repo_mapper.rs

pub struct RepoMapper {
    // ... existing fields ...
    
    /// Supplementary project data
    supplementary_projects: HashMap<String, SupplementaryProjectData>,
    
    /// Configuration
    supplementary_configs: Vec<SupplementaryProjectConfig>,
}

struct SupplementaryProjectData {
    config: SupplementaryProjectConfig,
    context_extractor: ContextExtractor,
    root_path: PathBuf,
    last_loaded: SystemTime,
}

impl RepoMapper {
    /// Initialize with supplementary projects
    pub fn with_supplementary_projects(
        root_path: &Path,
        configs: Vec<SupplementaryProjectConfig>
    ) -> Self {
        let mut mapper = Self::new(root_path);
        mapper.supplementary_configs = configs;
        mapper
    }
    
    /// Load supplementary projects (called after main project is mapped)
    pub fn load_supplementary_projects(&mut self) -> Result<(), String> {
        for config in &self.supplementary_configs.clone() {
            if config.enabled {
                self.load_supplementary_project(config)?;
            }
        }
        Ok(())
    }
    
    /// Load a single supplementary project
    fn load_supplementary_project(&mut self, config: &SupplementaryProjectConfig) -> Result<(), String> {
        let project_path = PathBuf::from(&config.path);
        let project_path = if project_path.is_absolute() {
            project_path
        } else {
            self.root_path.join(&config.path)
        };
        
        if !project_path.exists() {
            return Err(format!("Supplementary project path does not exist: {}", config.path));
        }
        
        // Create a separate context extractor for this project
        let mut context_extractor = create_context_extractor();
        
        // Collect files with language filtering
        let files = self.collect_supplementary_files(&project_path, &config.languages)?;
        
        tracing::info!("Loading supplementary project '{}': {} files", config.name, files.len());
        
        // Parse files
        for file_path in files {
            if let Err(e) = context_extractor.extract_symbols_from_file_incremental(&file_path) {
                tracing::debug!("Failed to parse supplementary file {}: {}", file_path, e);
            }
        }
        
        // Mark all symbols as supplementary
        self.mark_symbols_as_supplementary(&mut context_extractor, &config.name);
        
        // Store the project data
        self.supplementary_projects.insert(
            config.name.clone(),
            SupplementaryProjectData {
                config: config.clone(),
                context_extractor,
                root_path: project_path,
                last_loaded: SystemTime::now(),
            }
        );
        
        Ok(())
    }
    
    /// Collect files from supplementary project with language filtering
    fn collect_supplementary_files(
        &self,
        project_path: &Path,
        language_filter: &Option<Vec<String>>
    ) -> Result<Vec<String>, String> {
        let mut files = Vec::new();
        self.collect_files_recursive(project_path, &mut files, language_filter)?;
        Ok(files)
    }
    
    /// Recursive file collection with language filtering
    fn collect_files_recursive(
        &self,
        dir_path: &Path,
        files: &mut Vec<String>,
        language_filter: &Option<Vec<String>>
    ) -> Result<(), String> {
        let entries = fs::read_dir(dir_path)
            .map_err(|e| format!("Failed to read directory {}: {}", dir_path.display(), e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                // Skip common excluded directories (already built-in)
                let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if !dir_name.starts_with('.') && !["node_modules", "target", "dist", "bin", "obj"].contains(&dir_name) {
                    self.collect_files_recursive(&path, files, language_filter)?;
                }
            } else if path.is_file() {
                // Check if file has supported extension
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if let Some(language) = SupportedLanguage::from_extension(ext) {
                        // Apply language filter if specified
                        if let Some(filter) = language_filter {
                            let lang_name = language.to_string().to_lowercase();
                            if !filter.iter().any(|f| f.to_lowercase() == lang_name) {
                                continue;
                            }
                        }
                        
                        files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Mark all symbols in context extractor as supplementary
    fn mark_symbols_as_supplementary(
        &self,
        context_extractor: &mut ContextExtractor,
        project_name: &str
    ) {
        // This would require extending ContextExtractor to support marking symbols
        // Implementation details depend on internal structure
    }
}
```

### Phase 2: Smart Fallback Logic (Week 2)

#### 2.1 Enhanced Symbol Resolution

```rust
impl RepoMapper {
    /// Find symbol definition with supplementary fallback
    pub fn find_symbol_definition_with_fallback(&self, symbol_name: &str) -> Vec<&CodeSymbol> {
        // 1. First check main project
        let main_symbols = self.find_symbol_definitions(symbol_name);
        if !main_symbols.is_empty() {
            return main_symbols;
        }
        
        // 2. Check supplementary projects by priority
        let mut supplementary_symbols = Vec::new();
        let mut projects_by_priority: Vec<_> = self.supplementary_projects.values().collect();
        projects_by_priority.sort_by(|a, b| b.config.priority.cmp(&a.config.priority));
        
        for project_data in projects_by_priority {
            let symbols = project_data.context_extractor.find_symbols_by_name(symbol_name);
            supplementary_symbols.extend(symbols);
        }
        
        supplementary_symbols
    }
    
    /// Check if symbol is defined in main project
    pub fn is_symbol_defined_in_main(&self, symbol_name: &str) -> bool {
        !self.find_symbol_definitions(symbol_name).is_empty()
    }
    
    /// Build graph with smart supplementary integration
    pub fn build_graph_with_supplementary_fallback(&mut self) {
        // 1. Build main project graph
        self.build_graph_from_context();
        
        // 2. Find unresolved references
        let unresolved_refs = self.find_unresolved_references();
        
        // 3. For each unresolved reference, check supplementary projects
        for unresolved_ref in unresolved_refs {
            if let Some((symbol, project_name)) = self.find_in_supplementary_projects(&unresolved_ref.symbol_name) {
                self.create_supplementary_edge(unresolved_ref, symbol, &project_name);
            }
        }
    }
    
    /// Find references that don't have definitions in main project
    fn find_unresolved_references(&self) -> Vec<&SymbolReference> {
        self.context_extractor
            .get_references()
            .iter()
            .filter(|reference| !self.is_symbol_defined_in_main(&reference.symbol_name))
            .collect()
    }
    
    /// Find symbol in supplementary projects
    fn find_in_supplementary_projects(&self, symbol_name: &str) -> Option<(&CodeSymbol, String)> {
        // Search by priority
        let mut projects_by_priority: Vec<_> = self.supplementary_projects.iter().collect();
        projects_by_priority.sort_by(|(_, a), (_, b)| b.config.priority.cmp(&a.config.priority));
        
        for (project_name, project_data) in projects_by_priority {
            let symbols = project_data.context_extractor.find_symbols_by_name(symbol_name);
            if let Some(symbol) = symbols.first() {
                return Some((symbol, project_name.clone()));
            }
        }
        
        None
    }
}
```

### Phase 3: Tool Integration (Week 3)

#### 3.1 Enhanced Existing Tools (No New Tools)

```rust
// File: codex-rs/core/src/code_analysis/tools.rs

/// Enhanced find_symbol_definitions with supplementary fallback
pub fn handle_find_symbol_definitions(args: Value) -> Option<Result<Value, String>> {
    let input: FindSymbolDefinitionsInput = serde_json::from_value(args).ok()?;
    
    // Get the graph manager
    let manager = get_graph_manager();
    let manager = manager.read().ok()?;
    let repo_mapper = manager.get_repo_mapper()?;
    
    // Use fallback resolution
    let definitions = repo_mapper.find_symbol_definition_with_fallback(&input.symbol_name);
    
    let result: Vec<_> = definitions.iter().map(|symbol| {
        json!({
            "name": symbol.name,
            "symbol_type": symbol.symbol_type.as_str(),
            "file_path": symbol.file_path,
            "start_line": symbol.start_line,
            "end_line": symbol.end_line,
            "parent": symbol.parent,
            // Optional fields for supplementary symbols
            "source_project": symbol.origin_project,
            "is_supplementary": symbol.is_supplementary()
        })
    }).collect();
    
    Some(Ok(json!({
        "definitions": result,
        "total_found": result.len()
    })))
}
```

## Benefits of This Approach

1. **Zero Configuration Overhead**: No complex pattern matching needed
2. **Automatic Filtering**: Leverages existing robust file type detection
3. **Performance Optimized**: Only processes supported file types
4. **Clean and Simple**: Minimal configuration surface area
5. **Language-Aware**: Optional language filtering for fine control

## Example Usage

```toml
# Simple, clean configuration
[code_analysis]
enable_supplementary_fallback = true

[[code_analysis.supplementary_projects]]
name = "contracts"
path = "../contracts"
priority = 100
enabled = true
languages = ["csharp"]  # Only C# files
```

```bash
# Same tools, enhanced behavior
./code-analysis-server --project-dir ./main-project

# Existing tools automatically include supplementary context when needed
echo '{"method": "tools/call", "params": {"name": "find_symbol_definitions", "arguments": {"symbol_name": "UserContract"}}}' | ./code-analysis-server
```

This approach is much cleaner and leverages the existing robust filtering system effectively.