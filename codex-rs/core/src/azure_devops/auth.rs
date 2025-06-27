//! Authentication handling for Azure DevOps API.

use crate::error::{CodexErr, EnvVarError, Result};
use std::env;
use std::path::Path;

pub mod oauth_auth;
pub use oauth_auth::{AzureDevOpsOAuthHandler, AzureDevOpsTokens};

/// Authentication methods for Azure DevOps
#[derive(Debug, Clone)]
pub enum AzureDevOpsAuth {
    /// Personal Access Token authentication
    PersonalAccessToken(String),
    /// OAuth authentication (device code flow)
    OAuth(String), // Contains the access token
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

    /// Create a new authentication handler using OAuth (device code flow)
    pub async fn from_oauth(organization_url: &str, codex_home: &Path) -> Result<Self> {
        let oauth_handler = AzureDevOpsOAuthHandler::new(codex_home);
        let access_token = oauth_handler.get_access_token().await?;
        
        Ok(Self {
            organization_url: organization_url.to_string(),
            auth: AzureDevOpsAuth::OAuth(access_token),
        })
    }

    /// Create a new authentication handler, trying OAuth first, then falling back to PAT
    pub async fn from_config_with_oauth(
        organization_url: &str,
        pat_env_var: Option<&str>,
        codex_home: &Path,
    ) -> Result<Self> {
        // First try OAuth if we have saved tokens
        let oauth_handler = AzureDevOpsOAuthHandler::new(codex_home);
        if let Ok(access_token) = oauth_handler.get_access_token().await {
            return Ok(Self {
                organization_url: organization_url.to_string(),
                auth: AzureDevOpsAuth::OAuth(access_token),
            });
        }

        // Fall back to PAT if provided
        if let Some(env_var_name) = pat_env_var {
            if let Ok(pat) = env::var(env_var_name) {
                if !pat.trim().is_empty() {
                    return Ok(Self {
                        organization_url: organization_url.to_string(),
                        auth: AzureDevOpsAuth::PersonalAccessToken(pat),
                    });
                }
            }
        }

        // If no PAT, prompt for OAuth authentication
        let access_token = oauth_handler.get_access_token().await?;
        Ok(Self {
            organization_url: organization_url.to_string(),
            auth: AzureDevOpsAuth::OAuth(access_token),
        })
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
                let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, auth_string);
                Some(format!("Basic {}", encoded))
            }
            AzureDevOpsAuth::OAuth(access_token) => {
                // OAuth uses Bearer token
                Some(format!("Bearer {}", access_token))
            }
            AzureDevOpsAuth::None => None,
        }
    }
}