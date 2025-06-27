//! Authentication handling for Kusto (Azure Data Explorer) API.

use crate::error::{CodexErr, Result};
use std::path::Path;

// Reuse the Azure DevOps OAuth handler since it uses the same Microsoft identity platform
use crate::azure_devops::auth::oauth_auth::{AzureDevOpsOAuthHandler, AzureDevOpsTokens};

/// Authentication methods for Kusto
#[derive(Debug, Clone)]
pub enum KustoAuth {
    /// OAuth authentication (device code flow)
    OAuth(String), // Contains the access token
    /// No authentication (for public resources)
    None,
}

/// Kusto authentication handler
#[derive(Debug, Clone)]
pub struct KustoAuthHandler {
    /// The cluster URL (e.g., "https://help.kusto.windows.net")
    pub cluster_url: String,
    /// Authentication method
    pub auth: KustoAuth,
}

impl KustoAuthHandler {
    /// Create a new authentication handler using OAuth (device code flow)
    pub async fn from_oauth(cluster_url: &str, codex_home: &Path) -> Result<Self> {
        // Reuse the Azure DevOps OAuth handler but with Kusto-specific scopes
        let oauth_handler = AzureDevOpsOAuthHandler::new(codex_home);
        
        // Get access token - this will use the same device code flow but with Kusto scopes
        let access_token = oauth_handler.get_access_token().await?;
        
        Ok(Self {
            cluster_url: cluster_url.to_string(),
            auth: KustoAuth::OAuth(access_token),
        })
    }

    /// Create a new authentication handler, trying OAuth first
    pub async fn from_config_with_oauth(
        cluster_url: &str,
        codex_home: &Path,
    ) -> Result<Self> {
        // Try OAuth authentication
        let oauth_handler = AzureDevOpsOAuthHandler::new(codex_home);
        if let Ok(access_token) = oauth_handler.get_access_token().await {
            return Ok(Self {
                cluster_url: cluster_url.to_string(),
                auth: KustoAuth::OAuth(access_token),
            });
        }

        // If OAuth fails, return error
        Err(CodexErr::Other("Failed to authenticate with Kusto".to_string()))
    }

    /// Create a new authentication handler with no authentication
    pub fn without_auth(cluster_url: &str) -> Self {
        Self {
            cluster_url: cluster_url.to_string(),
            auth: KustoAuth::None,
        }
    }

    /// Get the authorization header value for API requests
    pub fn get_auth_header(&self) -> Option<String> {
        match &self.auth {
            KustoAuth::OAuth(access_token) => {
                // OAuth uses Bearer token
                Some(format!("Bearer {}", access_token))
            }
            KustoAuth::None => None,
        }
    }
}