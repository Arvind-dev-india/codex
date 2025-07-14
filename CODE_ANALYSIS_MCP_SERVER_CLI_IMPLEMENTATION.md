# Code Analysis MCP Server - CLI Arguments Implementation

## CLI Arguments Approach (Focused Implementation)

### Enhanced CLI Arguments

```rust
// File: codex-rs/code-analysis-server/src/main.rs

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Project directory to analyze
    #[arg(short, long, value_name = "DIR")]
    project_dir: Option<PathBuf>,

    /// Supplementary project (format: name:path or name:path:priority)
    /// Can be specified multiple times
    #[arg(long, value_name = "SPEC")]
    supplementary: Vec<String>,

    /// Default priority for supplementary projects (when not specified in --supplementary)
    #[arg(long, default_value = "50")]
    supplementary_priority: u32,

    /// Languages to analyze in supplementary projects (comma-separated)
    /// Example: --supplementary-languages csharp,typescript,python
    #[arg(long, value_name = "LANGS")]
    supplementary_languages: Option<String>,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
    
    /// Port to listen on for HTTP/SSE server (0 = stdio mode)
    #[arg(short, long, default_value = "0")]
    port: u16,
    
    /// Enable SSE (Server-Sent Events) mode for easier testing
    #[arg(long)]
    sse: bool,
}
```

### Implementation Details

```rust
impl Args {
    /// Parse supplementary project specifications from CLI
    fn parse_supplementary_projects(&self) -> Result<Vec<SupplementaryProjectConfig>, String> {
        let mut projects = Vec::new();
        
        // Parse global language filter
        let global_languages = self.parse_languages(&self.supplementary_languages)?;
        
        for (index, spec) in self.supplementary.iter().enumerate() {
            let project = self.parse_supplementary_spec(spec, index, &global_languages)?;
            projects.push(project);
        }
        
        // Sort by priority (highest first)
        projects.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        Ok(projects)
    }
    
    /// Parse a single supplementary project specification
    fn parse_supplementary_spec(
        &self, 
        spec: &str, 
        index: usize,
        global_languages: &Option<Vec<String>>
    ) -> Result<SupplementaryProjectConfig, String> {
        let parts: Vec<&str> = spec.split(':').collect();
        
        match parts.len() {
            2 => {
                // Format: name:path
                Ok(SupplementaryProjectConfig {
                    name: parts[0].to_string(),
                    path: parts[1].to_string(),
                    enabled: true,
                    priority: self.supplementary_priority,
                    languages: global_languages.clone(),
                    description: Some(format!("CLI supplementary project #{}", index + 1)),
                })
            },
            3 => {
                // Format: name:path:priority
                let priority = parts[2].parse::<u32>()
                    .map_err(|_| format!("Invalid priority '{}' in supplementary spec: {}", parts[2], spec))?;
                    
                Ok(SupplementaryProjectConfig {
                    name: parts[0].to_string(),
                    path: parts[1].to_string(),
                    enabled: true,
                    priority,
                    languages: global_languages.clone(),
                    description: Some(format!("CLI supplementary project #{}", index + 1)),
                })
            },
            _ => Err(format!(
                "Invalid supplementary project format: '{}'. Use 'name:path' or 'name:path:priority'", 
                spec
            )),
        }
    }
    
    /// Parse comma-separated language list
    fn parse_languages(&self, languages_str: &Option<String>) -> Result<Option<Vec<String>>, String> {
        match languages_str {
            Some(langs) => {
                let languages: Vec<String> = langs
                    .split(',')
                    .map(|s| s.trim().to_lowercase())
                    .filter(|s| !s.is_empty())
                    .collect();
                
                // Validate languages against supported ones
                let supported = ["rust", "javascript", "typescript", "python", "go", "cpp", "csharp", "java"];
                for lang in &languages {
                    if !supported.contains(&lang.as_str()) {
                        return Err(format!(
                            "Unsupported language: '{}'. Supported: {}", 
                            lang, 
                            supported.join(", ")
                        ));
                    }
                }
                
                Ok(Some(languages))
            },
            None => Ok(None),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Configure logging
    let log_level = if args.verbose { "debug" } else { "info" };
    std::env::set_var("RUST_LOG", format!("code_analysis_server={},codex_core={}", log_level, log_level));
    
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();
    
    // Set working directory
    if let Some(project_dir) = args.project_dir.as_ref() {
        std::env::set_current_dir(project_dir)?;
        tracing::info!("Set working directory to: {}", project_dir.display());
    }
    
    // Parse supplementary projects
    let supplementary_projects = args.parse_supplementary_projects()
        .map_err(|e| anyhow::anyhow!("Failed to parse supplementary projects: {}", e))?;
    
    // Log supplementary projects
    if !supplementary_projects.is_empty() {
        tracing::info!("Loaded {} supplementary projects:", supplementary_projects.len());
        for project in &supplementary_projects {
            let langs = project.languages.as_ref()
                .map(|l| l.join(", "))
                .unwrap_or_else(|| "all supported".to_string());
            tracing::info!("  - {} (priority: {}, path: {}, languages: {})", 
                project.name, project.priority, project.path, langs);
        }
    }
    
    // Get current directory for graph initialization
    let current_dir = std::env::current_dir()?;
    tracing::info!("Current working directory: {}", current_dir.display());
    
    // Initialize code graph with supplementary projects
    let graph_ready = std::sync::Arc::new(tokio::sync::Notify::new());
    let graph_ready_clone = graph_ready.clone();
    
    tokio::spawn(async move {
        tracing::info!("Initializing code graph with {} supplementary projects...", supplementary_projects.len());
        
        if let Err(e) = code_analysis_bridge::init_code_graph_with_supplementary(
            Some(current_dir), 
            supplementary_projects
        ).await {
            tracing::error!("Failed to initialize code graph: {}", e);
        } else {
            tracing::info!("Code graph is ready for use");
            graph_ready_clone.notify_waiters();
        }
    });
    
    // Run the server
    if args.sse || args.port > 0 {
        let port = if args.port > 0 { args.port } else { 3000 };
        server::run_http_server(port).await?;
    } else {
        server::run_server_with_graph_ready(graph_ready).await?;
    }
    
    Ok(())
}
```

### Bridge Integration

```rust
// File: codex-rs/code-analysis-server/src/code_analysis_bridge.rs

/// Initialize code graph with supplementary projects
pub async fn init_code_graph_with_supplementary(
    root_path: Option<PathBuf>,
    supplementary_projects: Vec<SupplementaryProjectConfig>
) -> Result<(), String> {
    let root = root_path.unwrap_or_else(|| std::env::current_dir().unwrap());
    
    // Initialize the graph manager with supplementary projects
    codex_core::code_analysis::graph_manager::initialize_with_supplementary_projects_async(
        &root, 
        supplementary_projects
    ).await
}
```

## Usage Examples

### Basic Usage

```bash
# Single supplementary project with default priority (50)
./code-analysis-server --project-dir ./main-project \
  --supplementary contracts:../shared-contracts

# Multiple supplementary projects
./code-analysis-server --project-dir ./main-project \
  --supplementary contracts:../shared-contracts \
  --supplementary utils:../common-utils
```

### Advanced Usage

```bash
# With custom priorities
./code-analysis-server --project-dir ./main-project \
  --supplementary contracts:../shared-contracts:100 \
  --supplementary utils:../common-utils:50 \
  --supplementary legacy:../legacy-code:25

# With language filtering
./code-analysis-server --project-dir ./main-project \
  --supplementary contracts:../shared-contracts:100 \
  --supplementary-languages csharp,typescript

# Change default priority
./code-analysis-server --project-dir ./main-project \
  --supplementary contracts:../shared-contracts \
  --supplementary utils:../common-utils \
  --supplementary-priority 75

# HTTP mode with supplementary projects
./code-analysis-server --project-dir ./main-project \
  --supplementary contracts:../shared-contracts:100 \
  --port 3000 --sse
```

### Real-world Examples

```bash
# .NET project with shared contracts
./code-analysis-server --project-dir ./MyWebApi \
  --supplementary contracts:../MyContracts:100 \
  --supplementary-languages csharp

# TypeScript project with shared utilities
./code-analysis-server --project-dir ./frontend \
  --supplementary shared:../shared-components:100 \
  --supplementary utils:../utility-functions:75 \
  --supplementary-languages typescript,javascript

# Multi-language project
./code-analysis-server --project-dir ./main-service \
  --supplementary contracts:../api-contracts:100 \
  --supplementary shared:../shared-libs:75 \
  --supplementary tools:../dev-tools:25

# Relative paths (common case)
./code-analysis-server \
  --supplementary contracts:../contracts \
  --supplementary shared:../shared
```

## Help Output

```bash
$ ./code-analysis-server --help

Standalone Code Analysis Server using the MCP protocol

Usage: code-analysis-server [OPTIONS]

Options:
  -p, --project-dir <DIR>
          Project directory to analyze

      --supplementary <SPEC>
          Supplementary project (format: name:path or name:path:priority)
          Can be specified multiple times

      --supplementary-priority <PRIORITY>
          Default priority for supplementary projects (when not specified in --supplementary)
          [default: 50]

      --supplementary-languages <LANGS>
          Languages to analyze in supplementary projects (comma-separated)
          Example: --supplementary-languages csharp,typescript,python

  -v, --verbose
          Enable verbose logging

  -p, --port <PORT>
          Port to listen on for HTTP/SSE server (0 = stdio mode)
          [default: 0]

      --sse
          Enable SSE (Server-Sent Events) mode for easier testing

  -h, --help
          Print help

  -V, --version
          Print version
```

## Error Handling

```rust
impl Args {
    fn validate(&self) -> Result<(), String> {
        // Validate project directory exists
        if let Some(project_dir) = &self.project_dir {
            if !project_dir.exists() {
                return Err(format!("Project directory does not exist: {}", project_dir.display()));
            }
        }
        
        // Validate supplementary project paths
        for spec in &self.supplementary {
            let parts: Vec<&str> = spec.split(':').collect();
            if parts.len() >= 2 {
                let path = PathBuf::from(parts[1]);
                if !path.exists() {
                    return Err(format!("Supplementary project path does not exist: {} (from spec: {})", 
                        path.display(), spec));
                }
            }
        }
        
        Ok(())
    }
}
```

## Benefits of CLI Approach

1. **Simple**: Easy to use and remember
2. **Scriptable**: Works well in CI/CD and scripts
3. **Flexible**: Can specify different priorities and languages
4. **Fast**: No config file parsing overhead
5. **Discoverable**: Built-in help shows all options
6. **Composable**: Easy to combine with other tools

This CLI approach provides a clean, intuitive interface for supplementary projects while keeping the implementation straightforward.