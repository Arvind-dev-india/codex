//! Kusto client implementation using the official Azure Kusto Rust SDK.

use azure_kusto_data::prelude::*;
use azure_kusto_data::models::V2QueryResult;
use serde_json::Value;
use std::collections::HashMap;

use super::auth::KustoAuthHandler;
use crate::error::{CodexErr, Result};

/// Client for interacting with Kusto using the official Azure SDK
pub struct KustoSdkClient {
    /// Official Azure Kusto client
    client: KustoClient,
    /// Database to query
    database: String,
}

impl KustoSdkClient {
    /// Create a new Kusto client using the official SDK
    pub async fn new(auth: KustoAuthHandler, database: &str) -> Result<Self> {
        // Get the access token from our auth handler
        let access_token = match &auth.auth {
            super::auth::KustoAuth::OAuth(token) => token.clone(),
            super::auth::KustoAuth::None => {
                return Err(CodexErr::Other("No authentication provided".to_string()));
            }
        };

        // Create connection string with our token
        let connection_string = ConnectionString::with_token_auth(
            auth.cluster_url.clone(),
            access_token,
        );

        // Create the official Kusto client
        let client = KustoClient::try_from(connection_string)
            .map_err(|e| CodexErr::Other(format!("Failed to create Kusto client: {}", e)))?;

        Ok(Self {
            client,
            database: database.to_string(),
        })
    }

    /// Execute a Kusto query using the official SDK
    pub async fn execute_query(&self, query: &str) -> Result<Vec<HashMap<String, Value>>> {
        tracing::info!("Executing Kusto query with official SDK:");
        tracing::info!("  Database: {}", self.database);
        tracing::info!("  Query: {}", query);

        // Execute the query using the official SDK
        let response = self.client
            .execute_query(self.database.clone(), query, None)
            .await
            .map_err(|e| {
                tracing::error!("Kusto SDK query failed: {}", e);
                CodexErr::Other(format!("Query execution failed: {}", e))
            })?;

        tracing::info!("Kusto SDK query executed successfully");

        // Process the results
        let mut processed_results = Vec::new();

        for result in response.results {
            match result {
                V2QueryResult::DataTable(table) => {
                    tracing::info!("Processing DataTable with {} rows", table.rows.len());
                    
                    // Convert the table to our format
                    for row in table.rows {
                        let mut row_map = HashMap::new();
                        
                        // Each row should be an array of values
                        if let Value::Array(row_values) = row {
                            // Map column names to values
                            for (i, value) in row_values.iter().enumerate() {
                                if i < table.columns.len() {
                                    let column_name = &table.columns[i].column_name;
                                    row_map.insert(column_name.clone(), value.clone());
                                }
                            }
                            
                            processed_results.push(row_map);
                        } else {
                            tracing::warn!("Expected row to be an array, got: {:?}", row);
                        }
                    }
                }
                V2QueryResult::DataSetHeader(header) => {
                    tracing::debug!("Received DataSetHeader: {:?}", header);
                }
                V2QueryResult::DataSetCompletion(completion) => {
                    tracing::debug!("Received DataSetCompletion: {:?}", completion);
                }
                V2QueryResult::TableHeader(header) => {
                    tracing::debug!("Received TableHeader: {:?}", header);
                }
                V2QueryResult::TableFragment(fragment) => {
                    tracing::debug!("Received TableFragment with {} rows", fragment.rows.len());
                }
                V2QueryResult::TableProgress(progress) => {
                    tracing::debug!("Received TableProgress: {:?}", progress);
                }
                V2QueryResult::TableCompletion(completion) => {
                    tracing::debug!("Received TableCompletion: {:?}", completion);
                }
            }
        }

        tracing::info!("Processed {} result rows", processed_results.len());
        Ok(processed_results)
    }
}