//! Azure DevOps API client implementation.

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::Duration;

use super::auth::AzureDevOpsAuthHandler;
use crate::error::{CodexErr, Result};

/// Client for interacting with the Azure DevOps REST API
pub struct AzureDevOpsClient {
    /// HTTP client for making API requests
    client: reqwest::Client,
    /// Authentication handler
    auth: AzureDevOpsAuthHandler,
    /// API version to use
    api_version: String,
}

impl AzureDevOpsClient {
    /// Create a new Azure DevOps client
    pub fn new(auth: AzureDevOpsAuthHandler) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_default();

        Self {
            client,
            auth,
            api_version: "7.0".to_string(), // Default to API version 7.0
        }
    }

    /// Set the API version to use
    pub fn with_api_version(mut self, api_version: &str) -> Self {
        self.api_version = api_version.to_string();
        self
    }

    /// Create the common headers used for API requests
    fn create_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        
        // Add content type and accept headers
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        
        // Add authorization header if available
        if let Some(auth_header) = self.auth.get_auth_header() {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&auth_header).map_err(|_| {
                    CodexErr::Other("Failed to create authorization header".to_string())
                })?,
            );
        }
        
        Ok(headers)
    }

    /// Build the full URL for an API endpoint
    fn build_url(&self, project: Option<&str>, endpoint: &str) -> String {
        let base_url = &self.auth.organization_url;
        
        // Determine the correct separator (? or &) for the API version parameter
        let separator = if endpoint.contains('?') { "&" } else { "?" };
        
        if let Some(project) = project {
            format!("{}/{}/_apis/{}{}{}", base_url, project, endpoint, separator, format!("api-version={}", self.api_version))
        } else {
            format!("{}/_apis/{}{}{}", base_url, endpoint, separator, format!("api-version={}", self.api_version))
        }
    }

    /// Make a GET request to the Azure DevOps API
    pub async fn get<T: DeserializeOwned>(
        &self,
        project: Option<&str>,
        endpoint: &str,
    ) -> Result<T> {
        let url = self.build_url(project, endpoint);
        let headers = self.create_headers()?;
        
        let response = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| CodexErr::Other(format!("Request failed: {}", e)))?;
            
        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to get error response".to_string());
            return Err(CodexErr::Other(format!(
                "Azure DevOps API error: {} - {}",
                status, text
            )));
        }
        
        response
            .json::<T>()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to parse response: {}", e)))
    }

    /// Make a POST request to the Azure DevOps API
    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        project: Option<&str>,
        endpoint: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.build_url(project, endpoint);
        let headers = self.create_headers()?;
        
        let response = self.client
            .post(&url)
            .headers(headers)
            .json(body)
            .send()
            .await
            .map_err(|e| CodexErr::Other(format!("Request failed: {}", e)))?;
            
        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to get error response".to_string());
            return Err(CodexErr::Other(format!(
                "Azure DevOps API error: {} - {}",
                status, text
            )));
        }
        
        response
            .json::<T>()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to parse response: {}", e)))
    }

    /// Make a PATCH request to the Azure DevOps API
    pub async fn patch<T: DeserializeOwned, B: Serialize>(
        &self,
        project: Option<&str>,
        endpoint: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.build_url(project, endpoint);
        let headers = self.create_headers()?;
        
        let response = self.client
            .patch(&url)
            .headers(headers)
            .json(body)
            .send()
            .await
            .map_err(|e| CodexErr::Other(format!("Request failed: {}", e)))?;
            
        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to get error response".to_string());
            return Err(CodexErr::Other(format!(
                "Azure DevOps API error: {} - {}",
                status, text
            )));
        }
        
        response
            .json::<T>()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to parse response: {}", e)))
    }
    
    /// Make a PUT request to the Azure DevOps API
    pub async fn put<T: DeserializeOwned, B: Serialize>(
        &self,
        project: Option<&str>,
        endpoint: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.build_url(project, endpoint);
        let headers = self.create_headers()?;
        
        let response = self.client
            .put(&url)
            .headers(headers)
            .json(body)
            .send()
            .await
            .map_err(|e| CodexErr::Other(format!("Request failed: {}", e)))?;
            
        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to get error response".to_string());
            return Err(CodexErr::Other(format!(
                "Azure DevOps API error: {} - {}",
                status, text
            )));
        }
        
        response
            .json::<T>()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to parse response: {}", e)))
    }
}