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

        let sdk_client = Self {
            client,
            database: database.to_string(),
        };

        // Test basic connectivity with a simple query
        tracing::info!("Testing Kusto SDK connectivity...");
        match sdk_client.test_connectivity().await {
            Ok(_) => tracing::info!("Kusto SDK connectivity test passed"),
            Err(e) => tracing::warn!("Kusto SDK connectivity test failed: {}", e),
        }

        Ok(sdk_client)
    }

    /// Execute a Kusto query using the official SDK
    pub async fn execute_query(&self, query: &str) -> Result<Vec<HashMap<String, Value>>> {
        tracing::info!("=== Kusto SDK Query Execution ===");
        tracing::info!("  Database: '{}'", self.database);
        tracing::info!("  Query: '{}'", query);
        tracing::info!("  Query Length: {} characters", query.len());
        tracing::info!("  Query Type: {}", if query.starts_with('.') { "Management Command" } else { "Data Query" });
        
        // Clean and validate query before sending to server
        let cleaned_query = query.trim()
            .trim_start_matches('`')  // Remove leading backticks
            .trim_end_matches('`')    // Remove trailing backticks
            .trim();                  // Remove any remaining whitespace
            
        if cleaned_query.is_empty() {
            return Err(CodexErr::Other("Query cannot be empty".to_string()));
        }
        
        // Log if we cleaned the query
        if cleaned_query != query.trim() {
            tracing::info!("Cleaned query from '{}' to '{}'", query, cleaned_query);
        }
        
        // Check if this looks like a bare table name (common mistake)
        if !cleaned_query.contains("|") && !cleaned_query.starts_with(".") && !cleaned_query.contains(" ") {
            tracing::warn!("Query '{}' looks like a bare table name. This might cause a 400 error.", cleaned_query);
            tracing::warn!("Consider using: '{} | take 10' to get sample data", cleaned_query);
        }

        // Execute the query using the official SDK
        let response = self.client
            .execute_query(self.database.clone(), cleaned_query, None)
            .await
            .map_err(|e| {
                tracing::error!("Kusto SDK query failed with detailed error:");
                tracing::error!("  Error: {}", e);
                tracing::error!("  Error Debug: {:?}", e);
                tracing::error!("  Database: {}", self.database);
                tracing::error!("  Original Query: {}", query);
                tracing::error!("  Cleaned Query: {}", cleaned_query);
                
                // Try to extract more details from the error
                let error_string = format!("{:?}", e);
                let error_display = format!("{}", e);
                
                if error_string.contains("400") || error_display.contains("400") {
                    tracing::error!("  This is a 400 Bad Request error - likely query syntax or database issue");
                    tracing::error!("  Common causes:");
                    tracing::error!("    1. Invalid query syntax - ensure query follows KQL syntax");
                    tracing::error!("    2. Table/database doesn't exist");
                    tracing::error!("    3. Missing permissions to access the database/table");
                    tracing::error!("    4. Query is a bare table name - try adding '| take 10' or '| limit 10'");
                    
                    // Provide specific suggestions based on the query
                    if !cleaned_query.contains("|") && !cleaned_query.starts_with(".") && !cleaned_query.contains(" ") {
                        tracing::error!("  SUGGESTION: Query '{}' looks like a table name. Try: '{} | take 10'", cleaned_query, cleaned_query);
                    }
                }
                
                // Create a more detailed error message
                let detailed_error = if error_display.contains("400") {
                    format!("Error in azure-core: server returned error status which will not be retried: 400\n\nDetailed Analysis:\n- Database: {}\n- Original Query: {}\n- Cleaned Query: {}\n- Issue: 400 Bad Request indicates query syntax or permission problem\n\nSuggestions:\n1. If '{}' is a table name, try: '{} | take 10'\n2. Verify the database '{}' exists and you have access\n3. Check if the table exists with: '.show tables | where TableName == \"{}\"'\n4. Ensure your query follows KQL (Kusto Query Language) syntax", 
                        self.database, query, cleaned_query, cleaned_query, cleaned_query, self.database, cleaned_query)
                } else {
                    format!("Query execution failed: {} (Database: {}, Query: {})", e, self.database, query)
                };
                
                CodexErr::Other(detailed_error)
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

    /// Test basic connectivity with a simple query
    async fn test_connectivity(&self) -> Result<()> {
        // Use a very simple query that should work on any database
        let test_query = "print 'connectivity_test'";
        
        tracing::info!("Testing connectivity with query: {}", test_query);
        
        let response = self.client
            .execute_query(self.database.clone(), test_query, None)
            .await
            .map_err(|e| {
                tracing::error!("Connectivity test failed: {}", e);
                CodexErr::Other(format!("Connectivity test failed: {}", e))
            })?;

        tracing::info!("Connectivity test successful - received {} results", response.results.len());
        Ok(())
    }
}