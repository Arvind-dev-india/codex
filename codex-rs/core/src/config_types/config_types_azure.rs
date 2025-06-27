//! Azure DevOps configuration types.

use serde::{Deserialize, Serialize};

/// Authentication method preference for Azure DevOps
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AzureDevOpsAuthMethod {
    /// Use OAuth device code flow (recommended)
    OAuth,
    /// Use Personal Access Token
    Pat,
    /// Try OAuth first, fall back to PAT
    Auto,
}

impl Default for AzureDevOpsAuthMethod {
    fn default() -> Self {
        Self::Auto
    }
}

/// Azure DevOps configuration
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AzureDevOpsConfig {
    /// Azure DevOps organization URL (e.g., "https://dev.azure.com/your-organization")
    pub organization_url: String,
    
    /// Authentication method preference
    #[serde(default)]
    pub auth_method: AzureDevOpsAuthMethod,
    
    /// Personal Access Token for authentication (used when auth_method is "pat" or as fallback)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pat: Option<String>,
    
    /// Environment variable name that contains the PAT
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pat_env_var: Option<String>,
    
    /// Default project to use when not specified
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_project: Option<String>,
    
    /// API version to use (defaults to "7.0")
    #[serde(default = "default_api_version", skip_serializing_if = "is_default_api_version")]
    pub api_version: String,
}

fn default_api_version() -> String {
    "7.0".to_string()
}

fn is_default_api_version(version: &str) -> bool {
    version == "7.0"
}

impl Default for AzureDevOpsConfig {
    fn default() -> Self {
        Self {
            organization_url: String::new(),
            auth_method: AzureDevOpsAuthMethod::default(),
            pat: None,
            pat_env_var: None,
            default_project: None,
            api_version: default_api_version(),
        }
    }
}