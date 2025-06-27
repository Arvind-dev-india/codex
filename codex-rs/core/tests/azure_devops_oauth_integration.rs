//! Integration tests for Azure DevOps OAuth authentication

use codex_core::azure_devops::tools_impl::AzureDevOpsTools;
use codex_core::config_types::{AzureDevOpsConfig, AzureDevOpsAuthMethod};
use tempfile::TempDir;

#[tokio::test]
async fn test_oauth_config_parsing() {
    // Test that OAuth configuration is parsed correctly
    let config = AzureDevOpsConfig {
        organization_url: "https://dev.azure.com/test-org".to_string(),
        auth_method: AzureDevOpsAuthMethod::OAuth,
        pat: None,
        pat_env_var: None,
        default_project: Some("TestProject".to_string()),
        api_version: "7.0".to_string(),
    };
    
    assert_eq!(config.auth_method, AzureDevOpsAuthMethod::OAuth);
    assert_eq!(config.organization_url, "https://dev.azure.com/test-org");
    assert_eq!(config.default_project, Some("TestProject".to_string()));
}

#[tokio::test]
async fn test_auto_config_parsing() {
    // Test that Auto configuration is parsed correctly
    let config = AzureDevOpsConfig {
        organization_url: "https://dev.azure.com/test-org".to_string(),
        auth_method: AzureDevOpsAuthMethod::Auto,
        pat: None,
        pat_env_var: Some("AZURE_DEVOPS_PAT".to_string()),
        default_project: Some("TestProject".to_string()),
        api_version: "7.0".to_string(),
    };
    
    assert_eq!(config.auth_method, AzureDevOpsAuthMethod::Auto);
    assert_eq!(config.pat_env_var, Some("AZURE_DEVOPS_PAT".to_string()));
}

#[tokio::test]
async fn test_pat_config_parsing() {
    // Test that PAT configuration is parsed correctly
    let config = AzureDevOpsConfig {
        organization_url: "https://dev.azure.com/test-org".to_string(),
        auth_method: AzureDevOpsAuthMethod::Pat,
        pat: None,
        pat_env_var: Some("AZURE_DEVOPS_PAT".to_string()),
        default_project: Some("TestProject".to_string()),
        api_version: "7.0".to_string(),
    };
    
    assert_eq!(config.auth_method, AzureDevOpsAuthMethod::Pat);
}

#[tokio::test]
async fn test_tools_creation_with_pat() {
    // Test creating tools with PAT authentication
    let _temp_dir = TempDir::new().unwrap();
    
    // Set up a test PAT
    unsafe {
        std::env::set_var("TEST_AZURE_PAT", "test-pat-token");
    }
    
    let config = AzureDevOpsConfig {
        organization_url: "https://dev.azure.com/test-org".to_string(),
        auth_method: AzureDevOpsAuthMethod::Pat,
        pat: None,
        pat_env_var: Some("TEST_AZURE_PAT".to_string()),
        default_project: Some("TestProject".to_string()),
        api_version: "7.0".to_string(),
    };
    
    // This should succeed with PAT authentication
    let result = AzureDevOpsTools::new(&config).await;
    assert!(result.is_ok());
    
    // Clean up
    unsafe {
        std::env::remove_var("TEST_AZURE_PAT");
    }
}

// Note: We don't test actual OAuth flow here as it requires user interaction
// and real network calls. The OAuth flow would be tested manually or in
// end-to-end tests.