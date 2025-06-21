//! Authentication handling for Azure DevOps API.

use crate::error::{CodexErr, EnvVarError, Result};
use std::env;

/// Authentication methods for Azure DevOps
#[derive(Debug, Clone)]
pub enum AzureDevOpsAuth {
    /// Personal Access Token authentication
    PersonalAccessToken(String),
    /// No authentication (for public resources)
    None,
}

/// Azure DevOps authentication handler
#[derive(Debug, Clone)]
pub struct AzureDevOpsAuthHandler {
    /// The organization URL (e.g., "https://dev.azure.com/your-organization")
    pub organization_url: String,
    /// Authentication method
    pub auth: AzureDevOpsAuth,
}

impl AzureDevOpsAuthHandler {
    /// Create a new authentication handler from environment variables
    pub fn from_env(env_var_name: &str, organization_url: &str) -> Result<Self> {
        match env::var(env_var_name) {
            Ok(pat) if !pat.trim().is_empty() => Ok(Self {
                organization_url: organization_url.to_string(),
                auth: AzureDevOpsAuth::PersonalAccessToken(pat),
            }),
            _ => Err(CodexErr::EnvVar(EnvVarError {
                var: env_var_name.to_string(),
                instructions: Some(format!(
                    "Set the {} environment variable to your Azure DevOps Personal Access Token. \
                     You can create one at {}/settings/tokens",
                    env_var_name, organization_url
                )),
            })),
        }
    }

    /// Create a new authentication handler with an explicit PAT
    pub fn with_pat(organization_url: &str, pat: &str) -> Self {
        Self {
            organization_url: organization_url.to_string(),
            auth: AzureDevOpsAuth::PersonalAccessToken(pat.to_string()),
        }
    }

    /// Create a new authentication handler with no authentication
    pub fn without_auth(organization_url: &str) -> Self {
        Self {
            organization_url: organization_url.to_string(),
            auth: AzureDevOpsAuth::None,
        }
    }

    /// Get the authorization header value for API requests
    pub fn get_auth_header(&self) -> Option<String> {
        match &self.auth {
            AzureDevOpsAuth::PersonalAccessToken(pat) => {
                // Azure DevOps API uses Basic auth with empty username and PAT as password
                let auth_string = format!(":{}", pat);
                let encoded = base64::encode(auth_string);
                Some(format!("Basic {}", encoded))
            }
            AzureDevOpsAuth::None => None,
        }
    }
}