//! Kusto (Azure Data Explorer) API client implementation.

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use std::time::Duration;

use super::auth::KustoAuthHandler;
use crate::error::{CodexErr, Result};

/// Client for interacting with the Kusto (Azure Data Explorer) REST API
pub struct KustoClient {
    /// HTTP client for making API requests
    client: reqwest::Client,
    /// Authentication handler
    auth: KustoAuthHandler,
    /// Database to query
    database: String,
}

impl KustoClient {
    /// Create a new Kusto client
    pub fn new(auth: KustoAuthHandler, database: &str) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_default();

        Self {
            client,
            auth,
            database: database.to_string(),
        }
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

    /// Execute a Kusto query
    pub async fn execute_query<T: DeserializeOwned>(&self, query: &str) -> Result<T> {
        let url = format!("{}/v2/rest/query", self.auth.cluster_url);
        let headers = self.create_headers()?;
        
        let query_request = serde_json::json!({
            "db": self.database,
            "csl": query
        });
        
        let response = self.client
            .post(&url)
            .headers(headers)
            .json(&query_request)
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
                "Kusto API error: {} - {}",
                status, text
            )));
        }
        
        response
            .json::<T>()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to parse response: {}", e)))
    }
}