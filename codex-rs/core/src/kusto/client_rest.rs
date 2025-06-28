//! Direct REST API client for Kusto (Azure Data Explorer).
//! This replaces the SDK to avoid API version limitations.

use crate::error::{CodexErr, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

/// REST API client for Kusto
pub struct KustoRestClient {
    /// Cluster URL (e.g., "https://help.kusto.windows.net")
    cluster_url: String,
    /// Database name
    database: String,
    /// Access token for authentication
    access_token: String,
    /// HTTP client
    client: reqwest::Client,
}

#[derive(Debug, Serialize)]
struct QueryRequest {
    db: String,
    csl: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<QueryProperties>,
}

#[derive(Debug, Serialize)]
struct QueryProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<HashMap<String, Value>>,
}

#[derive(Debug, Deserialize)]
struct QueryResponse {
    #[serde(rename = "Tables")]
    tables: Vec<Table>,
}

#[derive(Debug, Deserialize)]
struct Table {
    #[serde(rename = "TableName")]
    table_name: String,
    #[serde(rename = "Columns")]
    columns: Vec<Column>,
    #[serde(rename = "Rows")]
    rows: Vec<Vec<Value>>,
}

#[derive(Debug, Deserialize)]
struct Column {
    #[serde(rename = "ColumnName")]
    column_name: String,
    #[serde(rename = "DataType")]
    data_type: String,
    #[serde(rename = "ColumnType")]
    column_type: String,
}

impl KustoRestClient {
    /// Create a new Kusto REST client
    pub fn new(cluster_url: String, database: String, access_token: String) -> Self {
        let client = reqwest::Client::new();
        Self {
            cluster_url,
            database,
            access_token,
            client,
        }
    }

    /// Execute a Kusto query using direct REST API
    pub async fn execute_query(&self, query: &str) -> Result<Vec<HashMap<String, Value>>> {
        tracing::info!("=== Kusto REST API Query Execution ===");
        tracing::info!("  Cluster: '{}'", self.cluster_url);
        tracing::info!("  Database: '{}'", self.database);
        tracing::info!("  Query: '{}'", query);
        tracing::info!("  Query Length: {} characters", query.len());

        // Clean the query
        let cleaned_query = self.clean_query(query);
        if cleaned_query != query {
            tracing::info!("  Cleaned query from '{}' to '{}'", query, cleaned_query);
        }

        // Determine if this is a management command
        let is_management_command = cleaned_query.trim_start().starts_with('.');
        tracing::info!("  Query Type: {}", if is_management_command { "Management Command" } else { "Data Query" });

        // Choose the appropriate endpoint and API version
        let (endpoint, api_version) = if is_management_command {
            ("mgmt", "v1")  // Management commands use v1 API
        } else {
            ("query", "v2") // Data queries can use v2 API
        };

        let url = format!("{}/v1/rest/{}", self.cluster_url, endpoint);
        tracing::info!("  REST URL: {}", url);
        tracing::info!("  API Version: {}", api_version);

        // Prepare the request body
        let request_body = QueryRequest {
            db: self.database.clone(),
            csl: cleaned_query.clone(),
            properties: Some(QueryProperties {
                options: Some({
                    let mut options = HashMap::new();
                    options.insert("serializationformat".to_string(), json!("JSON"));
                    options.insert("results_progressive_enabled".to_string(), json!(false));
                    options
                }),
            }),
        };

        tracing::debug!("Request body: {}", serde_json::to_string_pretty(&request_body).unwrap_or_default());

        // Execute the request
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("x-ms-client-request-id", uuid::Uuid::new_v4().to_string())
            .json(&request_body)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to send request to Kusto: {}", e);
                CodexErr::Other(format!("Failed to send request: {}", e))
            })?;

        let status = response.status();
        tracing::info!("Response status: {}", status);

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            tracing::error!("Kusto REST API error: {}", error_text);
            
            return Err(CodexErr::Other(format!(
                "Kusto query failed with status {}: {}\n\nQuery details:\n- Database: {}\n- Query: {}\n- Cleaned Query: {}\n- Endpoint: {}\n- API Version: {}",
                status, error_text, self.database, query, cleaned_query, endpoint, api_version
            )));
        }

        // Parse the response
        let response_text = response.text().await.map_err(|e| {
            tracing::error!("Failed to read response text: {}", e);
            CodexErr::Other(format!("Failed to read response: {}", e))
        })?;

        tracing::debug!("Response body length: {} bytes", response_text.len());
        tracing::debug!("Response body: {}", response_text);

        let query_response: QueryResponse = serde_json::from_str(&response_text).map_err(|e| {
            tracing::error!("Failed to parse Kusto response: {}", e);
            tracing::debug!("Response text: {}", response_text);
            CodexErr::Other(format!("Failed to parse response: {}", e))
        })?;

        // Convert the response to our format
        let mut results = Vec::new();
        
        for table in query_response.tables {
            tracing::info!("Processing table '{}' with {} rows", table.table_name, table.rows.len());
            
            for row in table.rows {
                let mut row_map = HashMap::new();
                
                // Map column names to values
                for (i, value) in row.iter().enumerate() {
                    if i < table.columns.len() {
                        let column_name = &table.columns[i].column_name;
                        row_map.insert(column_name.clone(), value.clone());
                    }
                }
                
                results.push(row_map);
            }
        }

        tracing::info!("Successfully processed {} result rows", results.len());
        Ok(results)
    }

    /// Clean query by removing problematic characters
    fn clean_query(&self, query: &str) -> String {
        query.trim()
            .trim_start_matches('`')  // Remove leading backticks
            .trim_end_matches('`')    // Remove trailing backticks
            .trim()                   // Remove any remaining whitespace
            .to_string()
    }

    /// Test basic connectivity with a simple query
    pub async fn test_connectivity(&self) -> Result<()> {
        tracing::info!("Testing Kusto REST API connectivity...");
        
        let test_query = "print 'connectivity_test'";
        let _results = self.execute_query(test_query).await?;
        
        tracing::info!("Kusto REST API connectivity test successful");
        Ok(())
    }
}