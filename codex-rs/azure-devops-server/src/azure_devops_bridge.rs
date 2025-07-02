//! Bridge to connect to the existing Azure DevOps functionality in codex-core

use anyhow::Result;
use codex_core::azure_devops::tool_handler::handle_azure_devops_tool_call;
use codex_core::config::{Config, ConfigOverrides};
use codex_core::config_types::{AzureDevOpsConfig, AzureDevOpsAuthMethod};
use codex_core::mcp_tool_call::ToolCall;
use serde_json::Value;
use std::path::Path;
use std::sync::OnceLock;
use tracing::{info, error};

/// Global Azure DevOps configuration
static AZURE_DEVOPS_CONFIG: OnceLock<AzureDevOpsConfig> = OnceLock::new();

/// Initialize Azure DevOps configuration from a file
pub fn init_config(config_path: &Path) -> Result<()> {
    info!("Loading Azure DevOps configuration from: {}", config_path.display());
    
    let config_content = std::fs::read_to_string(config_path)?;
    let config: AzureDevOpsConfig = toml::from_str(&config_content)?;
    
    // Validate the configuration
    validate_config(&config)?;
    
    AZURE_DEVOPS_CONFIG.set(config).map_err(|_| {
        anyhow::anyhow!("Azure DevOps configuration already initialized")
    })?;
    
    info!("Azure DevOps configuration loaded successfully");
    Ok(())
}

/// Initialize Azure DevOps configuration from the main codex config
pub fn init_from_codex_config() -> Result<()> {
    info!("Loading Azure DevOps configuration from main codex config");
    
    // Load the main codex configuration
    let config = Config::load_with_cli_overrides(vec![], ConfigOverrides::default())
        .map_err(|e| anyhow::anyhow!("Failed to load codex config: {}", e))?;
    
    // Extract Azure DevOps configuration
    if let Some(azure_devops_config) = config.azure_devops {
        validate_config(&azure_devops_config)?;
        
        AZURE_DEVOPS_CONFIG.set(azure_devops_config).map_err(|_| {
            anyhow::anyhow!("Azure DevOps configuration already initialized")
        })?;
        
        info!("Azure DevOps configuration loaded from main codex config");
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "No Azure DevOps configuration found in main codex config.\n\
             Please add an [azure_devops] section to your ~/.codex/config.toml file.\n\
             Example:\n\
             [azure_devops]\n\
             organization_url = \"https://dev.azure.com/your-organization\"\n\
             auth_method = \"oauth\"  # or \"pat\" or \"auto\"\n\
             default_project = \"your-project\"  # optional"
        ))
    }
}

/// Initialize Azure DevOps configuration from default locations
pub fn init_default_config() -> Result<()> {
    // First try to load from main codex config
    if let Ok(()) = init_from_codex_config() {
        return Ok(());
    }
    
    info!("Main codex config not found or doesn't contain Azure DevOps config, trying standalone config files");
    
    // Try common configuration file locations
    let home_config = format!("{}/.config/codex/azure_devops.toml", std::env::var("HOME").unwrap_or_default());
    let possible_paths = [
        "azure_devops_config.toml",
        "config/azure_devops.toml",
        ".config/azure_devops.toml",
        &home_config,
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
        AZURE_DEVOPS_CONFIG.set(config).map_err(|_| {
            anyhow::anyhow!("Azure DevOps configuration already initialized")
        })?;
        info!("Azure DevOps configuration created from environment variables");
        return Ok(());
    }
    
    Err(anyhow::anyhow!(
        "No Azure DevOps configuration found. Please either:\n\
         1. Add an [azure_devops] section to your ~/.codex/config.toml file, or\n\
         2. Create a standalone config file in one of these locations: {:?}, or\n\
         3. Set environment variables: AZURE_DEVOPS_ORG, AZURE_DEVOPS_PAT",
        possible_paths
    ))
}

/// Create configuration from environment variables
fn create_config_from_env() -> Result<AzureDevOpsConfig> {
    let organization = std::env::var("AZURE_DEVOPS_ORG")
        .map_err(|_| anyhow::anyhow!("AZURE_DEVOPS_ORG environment variable not set"))?;
    
    let personal_access_token = std::env::var("AZURE_DEVOPS_PAT")
        .map_err(|_| anyhow::anyhow!("AZURE_DEVOPS_PAT environment variable not set"))?;
    
    let default_project = std::env::var("AZURE_DEVOPS_PROJECT").ok();
    
    // Convert organization name to full URL if it's not already a URL
    let organization_url = if organization.starts_with("http") {
        organization
    } else {
        format!("https://dev.azure.com/{}", organization)
    };
    
    Ok(AzureDevOpsConfig {
        organization_url,
        auth_method: AzureDevOpsAuthMethod::Pat,
        pat: Some(personal_access_token),
        pat_env_var: None,
        default_project,
        api_version: "7.0".to_string(),
    })
}

/// Validate the Azure DevOps configuration
fn validate_config(config: &AzureDevOpsConfig) -> Result<()> {
    if config.organization_url.is_empty() {
        return Err(anyhow::anyhow!("Organization URL is required"));
    }
    
    // Check authentication configuration based on auth method
    match &config.auth_method {
        AzureDevOpsAuthMethod::Pat => {
            if config.pat.is_none() && config.pat_env_var.is_none() {
                return Err(anyhow::anyhow!(
                    "PAT authentication method selected but neither pat nor pat_env_var is configured"
                ));
            }
        }
        AzureDevOpsAuthMethod::OAuth => {
            // OAuth doesn't require PAT configuration
        }
        AzureDevOpsAuthMethod::Auto => {
            // Auto mode can work with OAuth only, PAT only, or both
        }
    }
    
    if let Some(pat) = &config.pat {
        if pat.is_empty() {
            return Err(anyhow::anyhow!("Personal access token cannot be empty"));
        }
    }
    
    Ok(())
}

/// Get the current Azure DevOps configuration
fn get_config() -> Result<&'static AzureDevOpsConfig> {
    AZURE_DEVOPS_CONFIG.get().ok_or_else(|| {
        anyhow::anyhow!("Azure DevOps configuration not initialized. Call init_config() first.")
    })
}

/// Call an Azure DevOps tool with the given name and arguments
pub async fn call_azure_devops_tool(tool_name: &str, arguments: Value) -> Result<Value> {
    let config = get_config()?;
    
    // Create a tool call structure
    let tool_call = ToolCall {
        name: tool_name.to_string(),
        arguments,
    };
    
    // Call the tool handler from codex-core
    match handle_azure_devops_tool_call(&tool_call, config).await {
        Ok(result) => {
            info!("Azure DevOps tool '{}' executed successfully", tool_name);
            Ok(result)
        },
        Err(e) => {
            error!("Error executing Azure DevOps tool '{}': {}", tool_name, e);
            Err(anyhow::anyhow!("Azure DevOps tool error: {}", e))
        }
    }
}

/// Get configuration status for debugging
pub fn get_config_status() -> String {
    match get_config() {
        Ok(config) => {
            format!(
                "Azure DevOps Configuration:\n\
                 - Organization URL: {}\n\
                 - Default Project: {}\n\
                 - Authentication Method: {:?}\n\
                 - Authentication: {}\n\
                 - API Version: {}\n\
                 - Status: Ready",
                config.organization_url,
                config.default_project.as_deref().unwrap_or("None"),
                config.auth_method,
                if config.pat.is_some() {
                    "Personal Access Token"
                } else if config.pat_env_var.is_some() {
                    "PAT Environment Variable"
                } else {
                    "None"
                },
                config.api_version
            )
        },
        Err(e) => format!("Configuration Error: {}", e),
    }
}