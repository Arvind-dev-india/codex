//! OAuth-based authentication for Azure DevOps using device code flow.

use crate::error::{CodexErr, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;

/// Microsoft's public client ID for Azure DevOps
/// This is the same client ID used by Azure CLI and VS Code
const AZURE_DEVOPS_CLIENT_ID: &str = "04b07795-8ddb-461a-bbee-02f9e1bf7b46";

/// Azure DevOps OAuth scopes
const AZURE_DEVOPS_SCOPES: &str = "https://app.vssps.visualstudio.com/user_impersonation offline_access";

/// Microsoft OAuth endpoints
const DEVICE_CODE_ENDPOINT: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/devicecode";
const TOKEN_ENDPOINT: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/token";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureDevOpsTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub refresh_expires_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u64,
    interval: u64,
    _message: String,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: u64,
}

#[derive(Debug, Serialize)]
struct DeviceCodeRequest {
    client_id: &'static str,
    scope: &'static str,
}

#[derive(Debug, Serialize)]
struct TokenRequest {
    client_id: &'static str,
    grant_type: String,
    device_code: Option<String>,
    refresh_token: Option<String>,
}

/// Handles OAuth authentication for Azure DevOps
pub struct AzureDevOpsOAuthHandler {
    codex_home: std::path::PathBuf,
}

impl AzureDevOpsOAuthHandler {
    pub fn new(codex_home: &Path) -> Self {
        Self {
            codex_home: codex_home.to_path_buf(),
        }
    }

    /// Get a valid access token, refreshing if necessary or prompting for authentication
    pub async fn get_access_token(&self) -> Result<String> {
        // Try to load existing tokens
        if let Ok(tokens) = self.load_tokens().await {
            // Check if access token is still valid (with 5 minute buffer)
            if tokens.expires_at > Utc::now() + chrono::Duration::minutes(5) {
                return Ok(tokens.access_token);
            }

            // Try to refresh the token
            if tokens.refresh_expires_at > Utc::now() {
                if let Ok(new_tokens) = self.refresh_token(&tokens.refresh_token).await {
                    self.save_tokens(&new_tokens).await?;
                    return Ok(new_tokens.access_token);
                }
            }
        }

        // Need to authenticate from scratch
        let tokens = self.device_code_flow().await?;
        self.save_tokens(&tokens).await?;
        Ok(tokens.access_token)
    }

    /// Perform device code flow authentication
    async fn device_code_flow(&self) -> Result<AzureDevOpsTokens> {
        let client = reqwest::Client::new();

        // Step 1: Get device code
        let device_request = DeviceCodeRequest {
            client_id: AZURE_DEVOPS_CLIENT_ID,
            scope: AZURE_DEVOPS_SCOPES,
        };

        let device_response = client
            .post(DEVICE_CODE_ENDPOINT)
            .form(&device_request)
            .send()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to request device code: {}", e)))?;

        let device_code_resp: DeviceCodeResponse = device_response
            .json()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to parse device code response: {}", e)))?;

        // Step 2: Display instructions to user
        eprintln!("\n🔐 Azure DevOps Authentication Required");
        eprintln!("To sign in, use a web browser to open the page:");
        eprintln!("    {}", device_code_resp.verification_uri);
        eprintln!("And enter the code: {}", device_code_resp.user_code);
        eprintln!("\nWaiting for authentication...");

        // Step 3: Poll for token
        let poll_interval = Duration::from_secs(device_code_resp.interval);
        let expires_at = Utc::now() + chrono::Duration::seconds(device_code_resp.expires_in as i64);

        loop {
            if Utc::now() > expires_at {
                return Err(CodexErr::Other("Device code expired. Please try again.".to_string()));
            }

            sleep(poll_interval).await;

            let token_request = TokenRequest {
                client_id: AZURE_DEVOPS_CLIENT_ID,
                grant_type: "urn:ietf:params:oauth:grant-type:device_code".to_string(),
                device_code: Some(device_code_resp.device_code.clone()),
                refresh_token: None,
            };

            let token_response = client
                .post(TOKEN_ENDPOINT)
                .form(&token_request)
                .send()
                .await
                .map_err(|e| CodexErr::Other(format!("Failed to poll for token: {}", e)))?;

            if token_response.status().is_success() {
                let token_resp: TokenResponse = token_response
                    .json()
                    .await
                    .map_err(|e| CodexErr::Other(format!("Failed to parse token response: {}", e)))?;

                eprintln!("✅ Authentication successful!");

                return Ok(AzureDevOpsTokens {
                    access_token: token_resp.access_token,
                    refresh_token: token_resp.refresh_token.unwrap_or_default(),
                    expires_at: Utc::now() + chrono::Duration::seconds(token_resp.expires_in as i64),
                    refresh_expires_at: Utc::now() + chrono::Duration::days(90), // Typical refresh token lifetime
                });
            } else if token_response.status().as_u16() == 400 {
                // Still waiting for user to complete authentication
                continue;
            } else {
                return Err(CodexErr::Other(format!(
                    "Token request failed: {}",
                    token_response.status()
                )));
            }
        }
    }

    /// Refresh an expired access token
    async fn refresh_token(&self, refresh_token: &str) -> Result<AzureDevOpsTokens> {
        let client = reqwest::Client::new();

        let token_request = TokenRequest {
            client_id: AZURE_DEVOPS_CLIENT_ID,
            grant_type: "refresh_token".to_string(),
            device_code: None,
            refresh_token: Some(refresh_token.to_string()),
        };

        let response = client
            .post(TOKEN_ENDPOINT)
            .form(&token_request)
            .send()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to refresh token: {}", e)))?;

        if response.status().is_success() {
            let token_resp: TokenResponse = response
                .json()
                .await
                .map_err(|e| CodexErr::Other(format!("Failed to parse refresh response: {}", e)))?;

            Ok(AzureDevOpsTokens {
                access_token: token_resp.access_token,
                refresh_token: token_resp.refresh_token.unwrap_or_else(|| refresh_token.to_string()),
                expires_at: Utc::now() + chrono::Duration::seconds(token_resp.expires_in as i64),
                refresh_expires_at: Utc::now() + chrono::Duration::days(90),
            })
        } else {
            Err(CodexErr::Other(format!(
                "Token refresh failed: {}",
                response.status()
            )))
        }
    }

    /// Load tokens from disk
    async fn load_tokens(&self) -> Result<AzureDevOpsTokens> {
        let auth_path = self.codex_home.join("azure_devops_auth.json");
        let mut file = std::fs::File::open(&auth_path)
            .map_err(|_| CodexErr::Other("No saved Azure DevOps tokens found".to_string()))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| CodexErr::Other(format!("Failed to read auth file: {}", e)))?;

        serde_json::from_str(&contents)
            .map_err(|e| CodexErr::Other(format!("Failed to parse auth file: {}", e)))
    }

    /// Save tokens to disk
    async fn save_tokens(&self, tokens: &AzureDevOpsTokens) -> Result<()> {
        let auth_path = self.codex_home.join("azure_devops_auth.json");

        // Ensure codex_home directory exists
        if let Some(parent) = auth_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| CodexErr::Other(format!("Failed to create auth directory: {}", e)))?;
        }

        let mut options = OpenOptions::new();
        options.truncate(true).write(true).create(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }

        let json_data = serde_json::to_string_pretty(tokens)
            .map_err(|e| CodexErr::Other(format!("Failed to serialize tokens: {}", e)))?;

        let mut file = options.open(&auth_path)
            .map_err(|e| CodexErr::Other(format!("Failed to open auth file: {}", e)))?;

        file.write_all(json_data.as_bytes())
            .map_err(|e| CodexErr::Other(format!("Failed to write auth file: {}", e)))?;

        file.flush()
            .map_err(|e| CodexErr::Other(format!("Failed to flush auth file: {}", e)))?;

        Ok(())
    }

    /// Clear saved tokens (for logout)
    pub async fn logout(&self) -> Result<()> {
        let auth_path = self.codex_home.join("azure_devops_auth.json");
        if auth_path.exists() {
            std::fs::remove_file(&auth_path)
                .map_err(|e| CodexErr::Other(format!("Failed to remove auth file: {}", e)))?;
        }
        Ok(())
    }
}