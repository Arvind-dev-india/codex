//! Tests for Azure DevOps configuration parsing from TOML

use codex_core::config_types::{AzureDevOpsConfig, AzureDevOpsAuthMethod};

#[test]
fn test_oauth_config_from_toml() {
    let toml_content = r#"
        organization_url = "https://dev.azure.com/test-org"
        auth_method = "oauth"
        default_project = "TestProject"
    "#;
    
    let config: AzureDevOpsConfig = toml::from_str(toml_content).unwrap();
    
    assert_eq!(config.organization_url, "https://dev.azure.com/test-org");
    assert_eq!(config.auth_method, AzureDevOpsAuthMethod::OAuth);
    assert_eq!(config.default_project, Some("TestProject".to_string()));
    assert_eq!(config.api_version, "7.0");
}

#[test]
fn test_auto_config_from_toml() {
    let toml_content = r#"
        organization_url = "https://dev.azure.com/test-org"
        auth_method = "auto"
        pat_env_var = "AZURE_DEVOPS_PAT"
        default_project = "TestProject"
    "#;
    
    let config: AzureDevOpsConfig = toml::from_str(toml_content).unwrap();
    
    assert_eq!(config.organization_url, "https://dev.azure.com/test-org");
    assert_eq!(config.auth_method, AzureDevOpsAuthMethod::Auto);
    assert_eq!(config.pat_env_var, Some("AZURE_DEVOPS_PAT".to_string()));
    assert_eq!(config.default_project, Some("TestProject".to_string()));
}

#[test]
fn test_pat_config_from_toml() {
    let toml_content = r#"
        organization_url = "https://dev.azure.com/test-org"
        auth_method = "pat"
        pat_env_var = "AZURE_DEVOPS_PAT"
        default_project = "TestProject"
    "#;
    
    let config: AzureDevOpsConfig = toml::from_str(toml_content).unwrap();
    
    assert_eq!(config.organization_url, "https://dev.azure.com/test-org");
    assert_eq!(config.auth_method, AzureDevOpsAuthMethod::Pat);
    assert_eq!(config.pat_env_var, Some("AZURE_DEVOPS_PAT".to_string()));
}

#[test]
fn test_default_auth_method() {
    let toml_content = r#"
        organization_url = "https://dev.azure.com/test-org"
        default_project = "TestProject"
    "#;
    
    let config: AzureDevOpsConfig = toml::from_str(toml_content).unwrap();
    
    // Should default to Auto
    assert_eq!(config.auth_method, AzureDevOpsAuthMethod::Auto);
}

#[test]
fn test_minimal_oauth_config() {
    let toml_content = r#"
        organization_url = "https://dev.azure.com/test-org"
        auth_method = "oauth"
    "#;
    
    let config: AzureDevOpsConfig = toml::from_str(toml_content).unwrap();
    
    assert_eq!(config.organization_url, "https://dev.azure.com/test-org");
    assert_eq!(config.auth_method, AzureDevOpsAuthMethod::OAuth);
    assert_eq!(config.default_project, None);
    assert_eq!(config.pat, None);
    assert_eq!(config.pat_env_var, None);
}