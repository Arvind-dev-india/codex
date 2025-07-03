//! Bridge to connect to the existing Kusto functionality in codex-core

use anyhow::Result;
use codex_core::kusto::tool_handler::handle_kusto_tool_call;
use codex_core::config_types::KustoConfig;
use codex_core::mcp_tool_call::ToolCall;
use serde_json::Value;
use serde::Deserialize;
use std::path::Path;
use std::sync::OnceLock;
use tracing::{info, error};

/// Global Kusto configuration
static KUSTO_CONFIG: OnceLock<KustoConfig> = OnceLock::new();

/// Initialize Kusto configuration from a file
pub fn init_config(config_path: &Path) -> Result<()> {
    info!("Loading Kusto configuration from: {}", config_path.display());
    
    let config_content = std::fs::read_to_string(config_path)?;
    
    // Try to parse as a complete config first
    #[derive(Deserialize)]
    struct CompleteConfig {
        kusto: Option<KustoConfig>,
    }
    
    if let Ok(config) = toml::from_str::<CompleteConfig>(&config_content) {
        if let Some(kusto_config) = config.kusto {
            KUSTO_CONFIG.set(kusto_config).map_err(|_| {
                anyhow::anyhow!("Kusto configuration already initialized")
            })?;
            info!("Kusto configuration loaded successfully from complete config");
            return Ok(());
        }
    }
    
    // Try to parse as standalone Kusto config
    if let Ok(kusto_config) = toml::from_str::<KustoConfig>(&config_content) {
        KUSTO_CONFIG.set(kusto_config).map_err(|_| {
            anyhow::anyhow!("Kusto configuration already initialized")
        })?;
        info!("Kusto configuration loaded successfully from standalone config");
        return Ok(());
    }
    
    // Try to parse as a config with [kusto] section
    #[derive(serde::Deserialize)]
    struct ConfigWithKusto {
        kusto: Option<KustoConfig>,
    }
    
    if let Ok(config) = toml::from_str::<ConfigWithKusto>(&config_content) {
        if let Some(kusto_config) = config.kusto {
            KUSTO_CONFIG.set(kusto_config).map_err(|_| {
                anyhow::anyhow!("Kusto configuration already initialized")
            })?;
            info!("Kusto configuration loaded successfully from [kusto] section");
            return Ok(());
        }
    }
    
    Err(anyhow::anyhow!("No valid Kusto configuration found in file"))
}

/// Initialize Kusto configuration from default locations
pub fn init_default_config() -> Result<()> {
    // First try to load from main codex config
    if let Ok(()) = init_from_codex_config() {
        return Ok(());
    }
    
    info!("Main codex config not found or doesn't contain Kusto config, trying standalone config files");
    
    // Try common configuration file locations
    let home_config = format!("{}/.config/codex/kusto.toml", std::env::var("HOME").unwrap_or_default());
    let possible_paths = [
        "kusto_config.toml",
        "config/kusto.toml",
        ".config/kusto.toml",
        &home_config,
        "kusto_config_example.toml", // For development/testing
    ];
    
    for path_str in &possible_paths {
        let path = Path::new(path_str);
        if path.exists() {
            info!("Found configuration file at: {}", path.display());
            return init_config(path);
        }
    }
    
    // If no config file found, try to create a minimal config from environment variables
    if let Ok(config) = create_config_from_env() {
        KUSTO_CONFIG.set(config).map_err(|_| {
            anyhow::anyhow!("Kusto configuration already initialized")
        })?;
        info!("Kusto configuration created from environment variables");
        return Ok(());
    }
    
    Err(anyhow::anyhow!(
        "No Kusto configuration found. Please either:\n\
         1. Add a [kusto] section to your ~/.codex/config.toml file, or\n\
         2. Create a standalone config file in one of these locations: {:?}, or\n\
         3. Set environment variables: KUSTO_CLUSTER_URL, KUSTO_DATABASE",
        possible_paths
    ))
}

/// Create configuration from environment variables
fn create_config_from_env() -> Result<KustoConfig> {
    let cluster_url = std::env::var("KUSTO_CLUSTER_URL")
        .map_err(|_| anyhow::anyhow!("KUSTO_CLUSTER_URL environment variable not set"))?;
    
    let database = std::env::var("KUSTO_DATABASE")
        .map_err(|_| anyhow::anyhow!("KUSTO_DATABASE environment variable not set"))?;
    
    Ok(KustoConfig {
        cluster_url,
        database,
        ..Default::default()
    })
}

/// Try to initialize from the main codex configuration
fn init_from_codex_config() -> Result<()> {
    // Define the config structure
    #[derive(Deserialize)]
    struct CompleteConfig {
        kusto: Option<KustoConfig>,
    }

    // Try to find the main codex config
    let home_dir = std::env::var("HOME").unwrap_or_default();
    let codex_config_paths = [
        format!("{}/.codex/config.toml", home_dir),
        "config.toml".to_string(),
        ".codex/config.toml".to_string(),
    ];
    
    for config_path in &codex_config_paths {
        let path = Path::new(config_path);
        if path.exists() {
            info!("Found main codex config at: {}", path.display());
            
            let config_content = std::fs::read_to_string(path)?;
            if let Ok(config) = toml::from_str::<CompleteConfig>(&config_content) {
                if let Some(kusto_config) = config.kusto {
                    KUSTO_CONFIG.set(kusto_config).map_err(|_| {
                        anyhow::anyhow!("Kusto configuration already initialized")
                    })?;
                    info!("Kusto configuration loaded from main codex config");
                    return Ok(());
                }
            }
        }
    }
    
    Err(anyhow::anyhow!("Main codex config not found or doesn't contain Kusto configuration"))
}

/// Call a Kusto tool with the given arguments
pub async fn call_kusto_tool(tool_name: &str, arguments: Value) -> Result<Value> {
    let config = KUSTO_CONFIG.get()
        .ok_or_else(|| anyhow::anyhow!("Kusto configuration not initialized"))?;
    
    // Create a tool call structure
    let tool_call = ToolCall {
        name: tool_name.to_string(),
        arguments,
    };
    
    // Call the Kusto tool handler
    match handle_kusto_tool_call(&tool_call, config).await {
        Ok(result) => Ok(result),
        Err(e) => {
            error!("Error calling Kusto tool '{}': {}", tool_name, e);
            Err(anyhow::anyhow!("Kusto tool error: {}", e))
        }
    }
}

/// Get the current Kusto configuration (for debugging/info purposes)
pub fn get_config() -> Option<&'static KustoConfig> {
    KUSTO_CONFIG.get()
}