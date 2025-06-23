//! Tests for Azure DevOps integration.

#[cfg(test)]
mod tests {
    use codex_core::azure_devops::auth::AzureDevOpsAuthHandler;
    use codex_core::azure_devops::client::AzureDevOpsClient;
    use std::env;

    /// Test creating an auth handler from environment variables
    #[test]
    fn test_auth_handler_from_env() {
        // This test will be skipped if the environment variable is not set
        if env::var("AZURE_DEVOPS_PAT").is_err() {
            println!("Skipping test_auth_handler_from_env as AZURE_DEVOPS_PAT is not set");
            return;
        }

        let auth_handler = AzureDevOpsAuthHandler::from_env(
            "AZURE_DEVOPS_PAT",
            "https://dev.azure.com/test-org",
        );
        assert!(auth_handler.is_ok());
        
        let auth_handler = auth_handler.unwrap();
        assert_eq!(auth_handler.organization_url, "https://dev.azure.com/test-org");
        assert!(auth_handler.get_auth_header().is_some());
    }

    /// Test creating an auth handler with an explicit PAT
    #[test]
    fn test_auth_handler_with_pat() {
        let auth_handler = AzureDevOpsAuthHandler::with_pat(
            "https://dev.azure.com/test-org",
            "test-pat",
        );
        
        assert_eq!(auth_handler.organization_url, "https://dev.azure.com/test-org");
        assert!(auth_handler.get_auth_header().is_some());
        
        let auth_header = auth_handler.get_auth_header().unwrap();
        assert!(auth_header.starts_with("Basic "));
    }

    /// Test creating an auth handler without authentication
    #[test]
    fn test_auth_handler_without_auth() {
        let auth_handler = AzureDevOpsAuthHandler::without_auth(
            "https://dev.azure.com/test-org",
        );
        
        assert_eq!(auth_handler.organization_url, "https://dev.azure.com/test-org");
        assert!(auth_handler.get_auth_header().is_none());
    }

    /// Test building URLs in the client
    #[test]
    fn test_client_url_building() {
        let auth_handler = AzureDevOpsAuthHandler::without_auth(
            "https://dev.azure.com/test-org",
        );
        
        let _client = AzureDevOpsClient::new(auth_handler);
        
        // This is testing an internal method, so we can't directly call it
        // Instead, we'll need to modify the client to expose this method for testing
        // or test it indirectly through public methods
    }
}