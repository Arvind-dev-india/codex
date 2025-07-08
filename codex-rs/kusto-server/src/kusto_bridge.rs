//! Bridge to connect to the existing Kusto functionality in codex-core

use anyhow::Result;
use codex_core::kusto::tool_handler::handle_kusto_tool_call;
use codex_core::kusto::auth::KustoOAuthHandler;
use codex_core::config_types::KustoConfig;
use codex_core::mcp_tool_call::ToolCall;
use serde_json::{json, Value};
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
    // Handle authentication tools separately
    match tool_name {
        "kusto_auth_login" => handle_auth_login().await,
        "kusto_auth_logout" => handle_auth_logout().await,
        "kusto_auth_status" => handle_auth_status().await,
        _ => {
            // Handle regular Kusto tools
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
    }
}

/// Get the current Kusto configuration (for debugging/info purposes)
pub fn get_config() -> Option<&'static KustoConfig> {
    KUSTO_CONFIG.get()
}

/// Handle authentication login
async fn handle_auth_login() -> Result<Value> {
    let codex_home = get_codex_home()?;
    let oauth_handler = KustoOAuthHandler::new(&codex_home);
    
    match oauth_handler.get_access_token().await {
        Ok(_) => {
            info!("Kusto authentication successful");
            Ok(json!({
                "status": "success",
                "message": "Successfully authenticated with Kusto (Azure Data Explorer). Tokens have been stored securely.",
                "token_location": format!("{}/.codex/kusto_auth.json", codex_home.display())
            }))
        },
        Err(e) => {
            error!("Kusto authentication failed: {}", e);
            Err(anyhow::anyhow!("Authentication failed: {}", e))
        }
    }
}

/// Handle authentication logout
async fn handle_auth_logout() -> Result<Value> {
    let codex_home = get_codex_home()?;
    let oauth_handler = KustoOAuthHandler::new(&codex_home);
    
    match oauth_handler.logout().await {
        Ok(_) => {
            info!("Kusto logout successful");
            Ok(json!({
                "status": "success",
                "message": "Successfully logged out from Kusto. Authentication tokens have been cleared."
            }))
        },
        Err(e) => {
            error!("Kusto logout failed: {}", e);
            Err(anyhow::anyhow!("Logout failed: {}", e))
        }
    }
}

/// Handle authentication status check
async fn handle_auth_status() -> Result<Value> {
    let codex_home = get_codex_home()?;
    let oauth_handler = KustoOAuthHandler::new(&codex_home);
    
    // Try to get a valid access token to test authentication
    match oauth_handler.get_access_token().await {
        Ok(token) => {
            // Make a simple API call to verify the token works
            let config = get_config();
            let cluster_url = config
                .map(|c| c.cluster_url.clone())
                .unwrap_or_else(|| "https://unknown.kusto.windows.net".to_string());
            
            let client = reqwest::Client::new();
            let test_url = format!("{}/v1/rest/mgmt", cluster_url);
            
            let test_query = json!({
                "csl": ".show version",
                "db": config.map(|c| c.database.clone()).unwrap_or_else(|| "unknown".to_string())
            });
            
            let response = client
                .post(&test_url)
                .bearer_auth(&token)
                .json(&test_query)
                .send()
                .await;
            
            match response {
                Ok(resp) if resp.status().is_success() => {
                    info!("Kusto authentication status: valid");
                    Ok(json!({
                        "status": "authenticated",
                        "message": "Authentication is valid and working",
                        "token_location": format!("{}/.codex/kusto_auth.json", codex_home.display()),
                        "cluster_url": cluster_url,
                        "test_result": "API call successful"
                    }))
                },
                Ok(resp) => {
                    error!("Kusto API test failed with status: {}", resp.status());
                    Ok(json!({
                        "status": "invalid",
                        "message": format!("Authentication token exists but API test failed with status: {}", resp.status()),
                        "token_location": format!("{}/.codex/kusto_auth.json", codex_home.display()),
                        "recommendation": "Try logging out and logging in again"
                    }))
                },
                Err(e) => {
                    error!("Kusto API test failed: {}", e);
                    Ok(json!({
                        "status": "error",
                        "message": format!("Authentication token exists but API test failed: {}", e),
                        "token_location": format!("{}/.codex/kusto_auth.json", codex_home.display()),
                        "recommendation": "Check network connectivity and try logging out and logging in again"
                    }))
                }
            }
        },
        Err(e) => {
            info!("Kusto authentication status: not authenticated");
            Ok(json!({
                "status": "not_authenticated",
                "message": format!("Not authenticated: {}", e),
                "token_location": format!("{}/.codex/kusto_auth.json", codex_home.display()),
                "recommendation": "Run the kusto_auth_login tool to authenticate"
            }))
        }
    }
}

/// Get the codex home directory
fn get_codex_home() -> Result<std::path::PathBuf> {
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| anyhow::anyhow!("Could not determine home directory"))?;
    
    Ok(std::path::PathBuf::from(home_dir).join(".codex"))
}