//! Implementation of Kusto (Azure Data Explorer) tool functions.

use serde_json::{json, Value};
use std::sync::Arc;

use crate::kusto::client::KustoClient;
use crate::kusto::models::*;
use crate::config_types::KustoConfig;
use crate::error::{CodexErr, Result};

/// Implementation of Kusto tools
pub struct KustoTools {
    pub client: Arc<KustoClient>,
}

impl KustoTools {
    /// Create a new instance of Kusto tools
    pub async fn new(config: &KustoConfig) -> Result<Self> {
        use crate::kusto::auth::KustoAuthHandler;
        
        // Get codex home directory for OAuth token storage
        let codex_home = dirs::home_dir()
            .ok_or_else(|| CodexErr::Other("Could not determine home directory".to_string()))?
            .join(".codex");
        
        // Create auth handler using OAuth
        let auth = KustoAuthHandler::from_oauth(&config.cluster_url, &codex_home).await?;
        
        // Create client with auth
        let client = KustoClient::new(auth, &config.database);
            
        Ok(Self {
            client: Arc::new(client),
        })
    }

    /// Execute a Kusto query
    pub async fn execute_query(&self, args: Value) -> Result<Value> {
        let query = args["query"].as_str().ok_or_else(|| {
            CodexErr::Other("query parameter is required".to_string())
        })?;
        
        // Execute the query
        let result: KustoQueryResult = self.client
            .execute_query(query)
            .await?;
            
        // Process the results into a more usable format
        let processed_results = process_kusto_results(result);
        
        Ok(json!(processed_results))
    }
    
    /// Get schema information for a table
    pub async fn get_table_schema(&self, args: Value) -> Result<Value> {
        let table_name = args["table_name"].as_str().ok_or_else(|| {
            CodexErr::Other("table_name parameter is required".to_string())
        })?;
        
        // Use .schema command to get table information
        let query = format!(".show table {} schema as json", table_name);
        
        // Execute the query
        let result: KustoQueryResult = self.client
            .execute_query(&query)
            .await?;
            
        // Process the results
        let processed_results = process_kusto_results(result);
        
        Ok(json!(processed_results))
    }
    
    /// List available tables
    pub async fn list_tables(&self, _args: Value) -> Result<Value> {
        // Use .show tables command
        let query = ".show tables | project TableName, DatabaseName";
        
        // Execute the query
        let result: KustoQueryResult = self.client
            .execute_query(query)
            .await?;
            
        // Process the results
        let processed_results = process_kusto_results(result);
        
        Ok(json!(processed_results))
    }
}