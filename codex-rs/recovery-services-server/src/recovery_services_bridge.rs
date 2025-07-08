//! Bridge to connect to the existing Recovery Services functionality in codex-core

use anyhow::Result;
use codex_core::recovery_services::tool_handler::handle_recovery_services_tool_call;
use codex_core::recovery_services::auth::RecoveryServicesOAuthHandler;
use codex_core::config_types::RecoveryServicesConfig;
use codex_core::mcp_tool_call::ToolCall;
use serde_json::{json, Value};
use serde::Deserialize;
use std::path::Path;
use std::sync::OnceLock;
use tracing::{info, error};

/// Global Recovery Services configuration
static RECOVERY_SERVICES_CONFIG: OnceLock<RecoveryServicesConfig> = OnceLock::new();

/// Initialize Recovery Services configuration from a file
pub fn init_config(config_path: &Path) -> Result<()> {
    info!("Loading Recovery Services configuration from: {}", config_path.display());
    
    let config_content = std::fs::read_to_string(config_path)?;
    
    // Try to parse as a complete config first
    #[derive(Deserialize)]
    struct CompleteConfig {
        recovery_services: Option<RecoveryServicesConfig>,
    }
    
    if let Ok(config) = toml::from_str::<CompleteConfig>(&config_content) {
        if let Some(recovery_services_config) = config.recovery_services {
            RECOVERY_SERVICES_CONFIG.set(recovery_services_config).map_err(|_| {
                anyhow::anyhow!("Recovery Services configuration already initialized")
            })?;
            info!("Recovery Services configuration loaded successfully from complete config");
            return Ok(());
        }
    }
    
    // Try to parse as standalone Recovery Services config
    if let Ok(recovery_services_config) = toml::from_str::<RecoveryServicesConfig>(&config_content) {
        RECOVERY_SERVICES_CONFIG.set(recovery_services_config).map_err(|_| {
            anyhow::anyhow!("Recovery Services configuration already initialized")
        })?;
        info!("Recovery Services configuration loaded successfully from standalone config");
        return Ok(());
    }
    
    // Try to parse as a config with [recovery_services] section
    #[derive(Deserialize)]
    struct ConfigWithRecoveryServices {
        recovery_services: Option<RecoveryServicesConfig>,
    }
    
    if let Ok(config) = toml::from_str::<ConfigWithRecoveryServices>(&config_content) {
        if let Some(recovery_services_config) = config.recovery_services {
            RECOVERY_SERVICES_CONFIG.set(recovery_services_config).map_err(|_| {
                anyhow::anyhow!("Recovery Services configuration already initialized")
            })?;
            info!("Recovery Services configuration loaded successfully from [recovery_services] section");
            return Ok(());
        }
    }
    
    Err(anyhow::anyhow!("No valid Recovery Services configuration found in file"))
}

/// Initialize Recovery Services configuration from default locations
pub fn init_default_config() -> Result<()> {
    // First try to load from main codex config
    if let Ok(()) = init_from_codex_config() {
        return Ok(());
    }
    
    info!("Main codex config not found or doesn't contain Recovery Services config, trying standalone config files");
    
    // Try common configuration file locations
    let home_config = format!("{}/.config/codex/recovery_services.toml", std::env::var("HOME").unwrap_or_default());
    let possible_paths = [
        "recovery_services_config.toml",
        "config/recovery_services.toml",
        ".config/recovery_services.toml",
        &home_config,
        "recovery_services_config_example.toml", // For development/testing
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
        RECOVERY_SERVICES_CONFIG.set(config).map_err(|_| {
            anyhow::anyhow!("Recovery Services configuration already initialized")
        })?;
        info!("Recovery Services configuration created from environment variables");
        return Ok(());
    }
    
    Err(anyhow::anyhow!(
        "No Recovery Services configuration found. Please either:\n\
         1. Add a [recovery_services] section to your ~/.codex/config.toml file, or\n\
         2. Create a standalone config file in one of these locations: {:?}, or\n\
         3. Set environment variables: AZURE_SUBSCRIPTION_ID, AZURE_RESOURCE_GROUP",
        possible_paths
    ))
}

/// Create configuration from environment variables
fn create_config_from_env() -> Result<RecoveryServicesConfig> {
    let subscription_id = std::env::var("AZURE_SUBSCRIPTION_ID")
        .map_err(|_| anyhow::anyhow!("AZURE_SUBSCRIPTION_ID environment variable not set"))?;
    
    let resource_group = std::env::var("AZURE_RESOURCE_GROUP")
        .map_err(|_| anyhow::anyhow!("AZURE_RESOURCE_GROUP environment variable not set"))?;
    
    use std::collections::HashMap;
    use codex_core::config_types::RecoveryServicesVaultConfig;
    
    let vault_name = std::env::var("AZURE_VAULT_NAME").unwrap_or_else(|_| "default-vault".to_string());
    
    let mut vaults = HashMap::new();
    vaults.insert(vault_name.clone(), RecoveryServicesVaultConfig {
        name: vault_name.clone(),
        subscription_id: Some(subscription_id.clone()),
        resource_group: Some(resource_group.clone()),
        description: Some("Default vault from environment variables".to_string()),
        is_default: true,
    });
    
    Ok(RecoveryServicesConfig {
        enabled: Some(true),
        subscription_id,
        resource_group,
        vault_name,
        vaults,
    })
}

/// Try to initialize from the main codex configuration
fn init_from_codex_config() -> Result<()> {
    // Define the config structure
    #[derive(Deserialize)]
    struct CompleteConfig {
        recovery_services: Option<RecoveryServicesConfig>,
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
                if let Some(recovery_services_config) = config.recovery_services {
                    RECOVERY_SERVICES_CONFIG.set(recovery_services_config).map_err(|_| {
                        anyhow::anyhow!("Recovery Services configuration already initialized")
                    })?;
                    info!("Recovery Services configuration loaded from main codex config");
                    return Ok(());
                }
            }
        }
    }
    
    Err(anyhow::anyhow!("Main codex config not found or doesn't contain Recovery Services configuration"))
}

/// Call a Recovery Services tool with the given arguments
pub async fn call_recovery_services_tool(tool_name: &str, arguments: Value) -> Result<Value> {
    // Handle authentication tools separately
    match tool_name {
        "recovery_services_auth_login" => handle_auth_login().await,
        "recovery_services_auth_logout" => handle_auth_logout().await,
        "recovery_services_auth_status" => handle_auth_status().await,
        _ => {
            // Handle regular Recovery Services tools
            let config = RECOVERY_SERVICES_CONFIG.get()
                .ok_or_else(|| anyhow::anyhow!("Recovery Services configuration not initialized"))?;
            
            // Create a tool call structure
            let tool_call = ToolCall {
                name: tool_name.to_string(),
                arguments,
            };
            
            // Call the Recovery Services tool handler
            match handle_recovery_services_tool_call(&tool_call, config).await {
                Ok(result) => Ok(result),
                Err(e) => {
                    error!("Error calling Recovery Services tool '{}': {}", tool_name, e);
                    Err(anyhow::anyhow!("Recovery Services tool error: {}", e))
                }
            }
        }
    }
}

/// Get the current Recovery Services configuration (for debugging/info purposes)
pub fn get_config() -> Option<&'static RecoveryServicesConfig> {
    RECOVERY_SERVICES_CONFIG.get()
}

/// Handle authentication login
async fn handle_auth_login() -> Result<Value> {
    let codex_home = get_codex_home()?;
    let oauth_handler = RecoveryServicesOAuthHandler::new(&codex_home);
    
    match oauth_handler.get_access_token().await {
        Ok(_) => {
            info!("Recovery Services authentication successful");
            Ok(json!({
                "status": "success",
                "message": "Successfully authenticated with Azure Recovery Services. Tokens have been stored securely.",
                "token_location": format!("{}/.codex/recovery_services_auth.json", codex_home.display())
            }))
        },
        Err(e) => {
            error!("Recovery Services authentication failed: {}", e);
            Err(anyhow::anyhow!("Authentication failed: {}", e))
        }
    }
}

/// Handle authentication logout
async fn handle_auth_logout() -> Result<Value> {
    let codex_home = get_codex_home()?;
    let oauth_handler = RecoveryServicesOAuthHandler::new(&codex_home);
    
    match oauth_handler.logout().await {
        Ok(_) => {
            info!("Recovery Services logout successful");
            Ok(json!({
                "status": "success",
                "message": "Successfully logged out from Recovery Services. Authentication tokens have been cleared."
            }))
        },
        Err(e) => {
            error!("Recovery Services logout failed: {}", e);
            Err(anyhow::anyhow!("Logout failed: {}", e))
        }
    }
}

/// Handle authentication status check
async fn handle_auth_status() -> Result<Value> {
    let codex_home = get_codex_home()?;
    let oauth_handler = RecoveryServicesOAuthHandler::new(&codex_home);
    
    // Try to get a valid access token to test authentication
    match oauth_handler.get_access_token().await {
        Ok(token) => {
            // Make a simple API call to verify the token works
            let config = get_config();
            let subscription_id = config
                .map(|c| c.subscription_id.clone())
                .unwrap_or_else(|| "unknown".to_string());
            
            let client = reqwest::Client::new();
            let test_url = format!(
                "https://management.azure.com/subscriptions/{}/providers/Microsoft.RecoveryServices/vaults?api-version=2023-04-01",
                subscription_id
            );
            
            let response = client
                .get(&test_url)
                .bearer_auth(&token)
                .send()
                .await;
            
            match response {
                Ok(resp) if resp.status().is_success() => {
                    info!("Recovery Services authentication status: valid");
                    Ok(json!({
                        "status": "authenticated",
                        "message": "Authentication is valid and working",
                        "token_location": format!("{}/.codex/recovery_services_auth.json", codex_home.display()),
                        "subscription_id": subscription_id,
                        "test_result": "API call successful"
                    }))
                },
                Ok(resp) => {
                    error!("Recovery Services API test failed with status: {}", resp.status());
                    Ok(json!({
                        "status": "invalid",
                        "message": format!("Authentication token exists but API test failed with status: {}", resp.status()),
                        "token_location": format!("{}/.codex/recovery_services_auth.json", codex_home.display()),
                        "recommendation": "Try logging out and logging in again"
                    }))
                },
                Err(e) => {
                    error!("Recovery Services API test failed: {}", e);
                    Ok(json!({
                        "status": "error",
                        "message": format!("Authentication token exists but API test failed: {}", e),
                        "token_location": format!("{}/.codex/recovery_services_auth.json", codex_home.display()),
                        "recommendation": "Check network connectivity and try logging out and logging in again"
                    }))
                }
            }
        },
        Err(e) => {
            info!("Recovery Services authentication status: not authenticated");
            Ok(json!({
                "status": "not_authenticated",
                "message": format!("Not authenticated: {}", e),
                "token_location": format!("{}/.codex/recovery_services_auth.json", codex_home.display()),
                "recommendation": "Run the recovery_services_auth_login tool to authenticate"
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