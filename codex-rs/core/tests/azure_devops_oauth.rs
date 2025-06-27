//! Tests for Azure DevOps OAuth authentication

use codex_core::azure_devops::auth::{AzureDevOpsAuthHandler, AzureDevOpsOAuthHandler};
use tempfile::TempDir;

#[tokio::test]
async fn test_oauth_handler_creation() {
    let temp_dir = TempDir::new().unwrap();
    let codex_home = temp_dir.path();
    
    let _oauth_handler = AzureDevOpsOAuthHandler::new(codex_home);
    
    // Should be able to create the handler without errors
    assert!(true);
}

#[tokio::test]
async fn test_auth_handler_config_methods() {
    let temp_dir = TempDir::new().unwrap();
    let _codex_home = temp_dir.path();
    let org_url = "https://dev.azure.com/test-org";
    
    // Test PAT authentication
    unsafe {
        std::env::set_var("TEST_AZURE_PAT", "test-pat-token");
    }
    let auth_handler = AzureDevOpsAuthHandler::from_env("TEST_AZURE_PAT", org_url);
    assert!(auth_handler.is_ok());
    
    let handler = auth_handler.unwrap();
    let auth_header = handler.get_auth_header();
    assert!(auth_header.is_some());
    assert!(auth_header.unwrap().starts_with("Basic"));
    
    // Clean up
    unsafe {
        std::env::remove_var("TEST_AZURE_PAT");
    }
}

#[tokio::test]
async fn test_token_persistence_structure() {
    use codex_core::azure_devops::auth::AzureDevOpsTokens;
    use chrono::Utc;
    
    let tokens = AzureDevOpsTokens {
        access_token: "test-access-token".to_string(),
        refresh_token: "test-refresh-token".to_string(),
        expires_at: Utc::now() + chrono::Duration::hours(1),
        refresh_expires_at: Utc::now() + chrono::Duration::days(90),
    };
    
    // Test serialization
    let json = serde_json::to_string(&tokens).unwrap();
    assert!(json.contains("test-access-token"));
    assert!(json.contains("test-refresh-token"));
    
    // Test deserialization
    let parsed: AzureDevOpsTokens = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.access_token, tokens.access_token);
    assert_eq!(parsed.refresh_token, tokens.refresh_token);
}

// Note: We don't test the actual OAuth flow here as it requires user interaction
// and real network calls. These would be integration tests run manually.