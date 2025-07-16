//! Authentication handling for Kusto (Azure Data Explorer) API.

use crate::error::{CodexErr, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;

/// Token status information for display
#[derive(Debug, Clone)]
pub struct TokenStatus {
    pub authenticated: bool,
    pub access_token_valid: bool,
    pub refresh_token_valid: bool,
    pub access_expires_at: Option<DateTime<Utc>>,
    pub refresh_expires_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub access_expires_in_minutes: i64,
    pub refresh_expires_in_days: i64,
    pub token_age_days: i64,
}

/// Microsoft's public client ID for Azure CLI (works for Kusto)
/// This is the same client ID used by Azure CLI and Azure PowerShell
const KUSTO_CLIENT_ID: &str = "04b07795-8ddb-461a-bbee-02f9e1bf7b46";

/// Kusto (Azure Data Explorer) OAuth scopes
/// This scope provides access to Azure Data Explorer clusters
const KUSTO_SCOPES: &str = "https://kusto.kusto.windows.net/user_impersonation offline_access";

/// Microsoft OAuth endpoints
const DEVICE_CODE_ENDPOINT: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/devicecode";
const TOKEN_ENDPOINT: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/token";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KustoTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub refresh_expires_at: DateTime<Utc>,
    #[serde(default = "default_created_at")]
    pub created_at: DateTime<Utc>,
}

fn default_created_at() -> DateTime<Utc> {
    // For backward compatibility, use a reasonable default for old tokens
    Utc::now() - chrono::Duration::days(30)
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

/// Handles OAuth authentication for Kusto
pub struct KustoOAuthHandler {
    codex_home: std::path::PathBuf,
}

impl KustoOAuthHandler {
    pub fn new(codex_home: &Path) -> Self {
        Self {
            codex_home: codex_home.to_path_buf(),
        }
    }

    /// Get a valid access token, refreshing if necessary or prompting for authentication
    pub async fn get_access_token(&self) -> Result<String> {
        tracing::info!("=== Kusto Authentication Flow ===");
        
        // Try to load existing tokens
        match self.load_tokens().await {
            Ok(tokens) => {
                let now = Utc::now();
                let expires_in = tokens.expires_at.signed_duration_since(now);
                let refresh_expires_in = tokens.refresh_expires_at.signed_duration_since(now);
                
                tracing::info!("Found existing tokens:");
                tracing::info!("  Access token expires in: {} minutes", expires_in.num_minutes());
                tracing::info!("  Refresh token expires in: {} days", refresh_expires_in.num_days());
                
                // Check if access token is still valid (with 5 minute buffer)
                if tokens.expires_at > now + chrono::Duration::minutes(5) {
                    tracing::info!("Access token is still valid, using existing token");
                    return Ok(tokens.access_token);
                }

                tracing::info!("Access token expired, attempting refresh...");
                
                // Try to refresh the token
                if tokens.refresh_expires_at > now {
                    match self.refresh_token(&tokens.refresh_token).await {
                        Ok(new_tokens) => {
                            tracing::info!("Token refresh successful");
                            self.save_tokens(&new_tokens).await?;
                            return Ok(new_tokens.access_token);
                        }
                        Err(e) => {
                            tracing::warn!("Token refresh failed: {}", e);
                        }
                    }
                } else {
                    tracing::info!("Refresh token also expired");
                }
            }
            Err(e) => {
                tracing::info!("No existing tokens found: {}", e);
            }
        }

        // Need to authenticate from scratch
        tracing::info!("Starting new authentication flow...");
        let tokens = self.device_code_flow().await?;
        self.save_tokens(&tokens).await?;
        Ok(tokens.access_token)
    }

    /// Perform device code flow authentication
    async fn device_code_flow(&self) -> Result<KustoTokens> {
        let client = reqwest::Client::new();

        // Step 1: Get device code
        let device_request = DeviceCodeRequest {
            client_id: KUSTO_CLIENT_ID,
            scope: KUSTO_SCOPES,
        };

        let device_response = client
            .post(DEVICE_CODE_ENDPOINT)
            .form(&device_request)
            .send()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to request device code: {}", e)))?;

        // Log response details for debugging
        let status = device_response.status();
        tracing::info!("Device code response status: {}", status);
        
        if !status.is_success() {
            let error_text = device_response.text().await
                .unwrap_or_else(|_| "Failed to read error response".to_string());
            tracing::error!("Device code request failed with status {}: {}", status, error_text);
            return Err(CodexErr::Other(format!("Device code request failed: {} - {}", status, error_text)));
        }

        // Get response text first for debugging
        let response_text = device_response.text().await
            .map_err(|e| CodexErr::Other(format!("Failed to read device code response: {}", e)))?;
        
        tracing::debug!("Device code response body: {}", response_text);

        let device_code_resp: DeviceCodeResponse = serde_json::from_str(&response_text)
            .map_err(|e| CodexErr::Other(format!("Failed to parse device code response: {} - Response was: {}", e, response_text)))?;

        // Step 2: Display instructions to user
        eprintln!("\nKusto (Azure Data Explorer) Authentication Required");
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
                client_id: KUSTO_CLIENT_ID,
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

                eprintln!("Kusto authentication successful!");

                let now = Utc::now();
                return Ok(KustoTokens {
                    access_token: token_resp.access_token,
                    refresh_token: token_resp.refresh_token.unwrap_or_default(),
                    expires_at: now + chrono::Duration::seconds(token_resp.expires_in as i64),
                    refresh_expires_at: now + chrono::Duration::days(90), // Typical refresh token lifetime
                    created_at: now,
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
    async fn refresh_token(&self, refresh_token: &str) -> Result<KustoTokens> {
        tracing::info!("Attempting to refresh Kusto access token...");
        let client = reqwest::Client::new();

        let token_request = TokenRequest {
            client_id: KUSTO_CLIENT_ID,
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

            tracing::info!("Token refresh successful");
            let now = Utc::now();
            Ok(KustoTokens {
                access_token: token_resp.access_token,
                refresh_token: token_resp.refresh_token.unwrap_or_else(|| refresh_token.to_string()),
                expires_at: now + chrono::Duration::seconds(token_resp.expires_in as i64),
                refresh_expires_at: now + chrono::Duration::days(90),
                created_at: now, // New token created during refresh
            })
        } else {
            // Get the status before consuming the response
            let status = response.status();
            // Get the error details from the response
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            tracing::error!("Token refresh failed with status: {}", status);
            tracing::error!("Error response: {}", error_text);
            
            // Common refresh token errors
            if error_text.contains("invalid_grant") {
                tracing::error!("Refresh token is invalid or expired. User needs to re-authenticate.");
            } else if error_text.contains("invalid_client") {
                tracing::error!("Client ID is invalid or not authorized for refresh tokens.");
            }
            
            Err(CodexErr::Other(format!(
                "Token refresh failed: {} - {}",
                status,
                error_text
            )))
        }
    }

    /// Load tokens from disk
    async fn load_tokens(&self) -> Result<KustoTokens> {
        let auth_path = self.codex_home.join("kusto_auth.json");
        tracing::debug!("Looking for Kusto auth file at: {}", auth_path.display());
        
        if !auth_path.exists() {
            tracing::debug!("Kusto auth file does not exist");
            return Err(CodexErr::Other("No saved Kusto tokens found".to_string()));
        }
        
        let mut file = std::fs::File::open(&auth_path)
            .map_err(|e| {
                tracing::error!("Failed to open Kusto auth file: {}", e);
                CodexErr::Other(format!("Failed to open auth file: {}", e))
            })?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| {
                tracing::error!("Failed to read Kusto auth file: {}", e);
                CodexErr::Other(format!("Failed to read auth file: {}", e))
            })?;

        tracing::debug!("Kusto auth file contents length: {} bytes", contents.len());
        
        let tokens: KustoTokens = serde_json::from_str(&contents)
            .map_err(|e| {
                tracing::error!("Failed to parse Kusto auth file: {}", e);
                tracing::debug!("Auth file contents: {}", contents);
                CodexErr::Other(format!("Failed to parse auth file: {}", e))
            })?;
            
        tracing::debug!("Successfully loaded Kusto tokens");
        Ok(tokens)
    }

    /// Save tokens to disk
    async fn save_tokens(&self, tokens: &KustoTokens) -> Result<()> {
        let auth_path = self.codex_home.join("kusto_auth.json");

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
        let auth_path = self.codex_home.join("kusto_auth.json");
        if auth_path.exists() {
            std::fs::remove_file(&auth_path)
                .map_err(|e| CodexErr::Other(format!("Failed to remove auth file: {}", e)))?;
        }
        Ok(())
    }

    /// Get token status information for display
    pub async fn get_token_status(&self) -> Result<TokenStatus> {
        match self.load_tokens().await {
            Ok(tokens) => {
                let now = Utc::now();
                let access_expires_in = tokens.expires_at.signed_duration_since(now);
                let refresh_expires_in = tokens.refresh_expires_at.signed_duration_since(now);
                let token_age = now.signed_duration_since(tokens.created_at);
                
                Ok(TokenStatus {
                    authenticated: true,
                    access_token_valid: tokens.expires_at > now,
                    refresh_token_valid: tokens.refresh_expires_at > now,
                    access_expires_at: Some(tokens.expires_at),
                    refresh_expires_at: Some(tokens.refresh_expires_at),
                    created_at: Some(tokens.created_at),
                    access_expires_in_minutes: access_expires_in.num_minutes(),
                    refresh_expires_in_days: refresh_expires_in.num_days(),
                    token_age_days: token_age.num_days(),
                })
            }
            Err(_) => {
                Ok(TokenStatus {
                    authenticated: false,
                    access_token_valid: false,
                    refresh_token_valid: false,
                    access_expires_at: None,
                    refresh_expires_at: None,
                    created_at: None,
                    access_expires_in_minutes: 0,
                    refresh_expires_in_days: 0,
                    token_age_days: 0,
                })
            }
        }
    }
}

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
        // Use the dedicated Kusto OAuth handler with correct scopes
        let oauth_handler = KustoOAuthHandler::new(codex_home);
        
        // Get access token with Kusto-specific scopes
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
        let oauth_handler = KustoOAuthHandler::new(codex_home);
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