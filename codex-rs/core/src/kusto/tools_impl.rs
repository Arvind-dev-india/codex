//! Implementation of Kusto (Azure Data Explorer) tool functions.

use serde_json::{json, Value};
use std::sync::Arc;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::kusto::client::KustoClient;
use crate::kusto::models::*;
use crate::kusto::knowledge_base::*;
use crate::config_types::KustoConfig;
use crate::error::{CodexErr, Result};

/// Implementation of Kusto tools
pub struct KustoTools {
    pub clients: HashMap<String, Arc<KustoClient>>,
    pub config: KustoConfig,
    pub knowledge_base_path: PathBuf,
    pub knowledge_base: Arc<tokio::sync::RwLock<KustoKnowledgeBase>>,
}

impl KustoTools {
    /// Create a new instance of Kusto tools
    pub async fn new(config: &KustoConfig) -> Result<Self> {
        use crate::kusto::auth::KustoAuthHandler;
        
        // Get codex home directory for OAuth token storage and knowledge base
        let codex_home = dirs::home_dir()
            .ok_or_else(|| CodexErr::Other("Could not determine home directory".to_string()))?
            .join(".codex");
        
        // Determine knowledge base path
        let knowledge_base_path = if config.knowledge_base_path.starts_with('/') {
            PathBuf::from(&config.knowledge_base_path)
        } else {
            codex_home.join(&config.knowledge_base_path)
        };
        
        // Load knowledge base
        let knowledge_base = KustoKnowledgeBase::load_from_file(&knowledge_base_path).await?;
        let knowledge_base = Arc::new(tokio::sync::RwLock::new(knowledge_base));
        
        // Create clients for all configured databases
        let mut clients = HashMap::new();
        
        // Create auth handler using OAuth
        let auth = KustoAuthHandler::from_oauth(&config.cluster_url, &codex_home).await?;
        
        // Add default database client
        if !config.database.is_empty() {
            let client = KustoClient::new(auth.clone(), &config.database);
            clients.insert("default".to_string(), Arc::new(client));
            clients.insert(config.database.clone(), Arc::new(KustoClient::new(auth.clone(), &config.database)));
        }
        
        // Add clients for additional databases
        for (db_name, db_config) in &config.databases {
            let cluster_url = db_config.cluster_url.as_ref().unwrap_or(&config.cluster_url);
            let auth = if cluster_url != &config.cluster_url {
                KustoAuthHandler::from_oauth(cluster_url, &codex_home).await?
            } else {
                auth.clone()
            };
            
            let client = Arc::new(KustoClient::new(auth.clone(), &db_config.name));
            clients.insert(db_name.clone(), client.clone());
            clients.insert(db_config.name.clone(), client);
        }
            
        Ok(Self {
            clients,
            config: config.clone(),
            knowledge_base_path,
            knowledge_base,
        })
    }
    
    /// Get the appropriate client for a database
    fn get_client(&self, database: Option<&str>) -> Result<Arc<KustoClient>> {
        let db_name = database.unwrap_or("default");
        
        self.clients.get(db_name)
            .or_else(|| self.clients.get("default"))
            .cloned()
            .ok_or_else(|| CodexErr::Other(format!("No client found for database: {}", db_name)))
    }
    
    /// Update knowledge base with query results
    async fn update_knowledge_base_with_query(&self, database: &str, query: &str, results: &[HashMap<String, serde_json::Value>]) -> Result<()> {
        if !self.config.auto_update_knowledge_base {
            return Ok(());
        }
        
        let mut kb = self.knowledge_base.write().await;
        
        // Update database info
        let cluster_url = self.config.databases.get(database)
            .and_then(|db| db.cluster_url.as_ref())
            .unwrap_or(&self.config.cluster_url);
        kb.update_database(database.to_string(), cluster_url.clone(), None);
        
        // Try to extract table information from the query
        if let Some(table_name) = Self::extract_main_table_from_query(query) {
            // Limit sample data
            let limited_results: Vec<_> = results.iter()
                .take(self.config.max_knowledge_base_rows)
                .cloned()
                .collect();
            
            // Extract column info from results
            if let Some(first_row) = results.first() {
                let columns: Vec<ColumnInfo> = first_row.iter().map(|(name, value)| {
                    let data_type = match value {
                        serde_json::Value::String(_) => "string",
                        serde_json::Value::Number(_) => "real",
                        serde_json::Value::Bool(_) => "bool",
                        serde_json::Value::Array(_) => "dynamic",
                        serde_json::Value::Object(_) => "dynamic",
                        serde_json::Value::Null => "string",
                    };
                    
                    ColumnInfo {
                        name: name.clone(),
                        data_type: data_type.to_string(),
                        description: None,
                        is_nullable: Some(value.is_null()),
                        sample_values: vec![value.clone()],
                        is_commonly_queried: false,
                    }
                }).collect();
                
                kb.update_table(database, table_name, columns, limited_results);
            }
        }
        
        // Save knowledge base
        kb.save_to_file(&self.knowledge_base_path).await?;
        
        Ok(())
    }
    
    /// Extract the main table name from a Kusto query (simple heuristic)
    fn extract_main_table_from_query(query: &str) -> Option<String> {
        let query = query.trim();
        
        // Look for patterns like "TableName |" or "TableName\n"
        if let Some(first_line) = query.lines().next() {
            let first_line = first_line.trim();
            if let Some(pipe_pos) = first_line.find('|') {
                let table_part = first_line[..pipe_pos].trim();
                if !table_part.is_empty() && !table_part.contains(' ') {
                    return Some(table_part.to_string());
                }
            } else if !first_line.contains(' ') && !first_line.is_empty() {
                return Some(first_line.to_string());
            }
        }
        
        None
    }

    /// Execute a Kusto query
    pub async fn execute_query(&self, args: Value) -> Result<Value> {
        let query = args["query"].as_str().ok_or_else(|| {
            CodexErr::Other("query parameter is required".to_string())
        })?;
        
        let database = args["database"].as_str();
        let client = self.get_client(database)?;
        let db_name = database.unwrap_or(&self.config.database);
        
        // Execute the query
        let result: KustoQueryResult = client
            .execute_query(query)
            .await?;
            
        // Process the results into a more usable format
        let processed_results = process_kusto_results(result);
        
        // Update knowledge base with results
        if let Err(e) = self.update_knowledge_base_with_query(db_name, query, &processed_results).await {
            tracing::warn!("Failed to update knowledge base: {}", e);
        }
        
        Ok(json!({
            "results": processed_results,
            "database": db_name,
            "query": query,
            "row_count": processed_results.len()
        }))
    }
    
    /// Get schema information for a table
    pub async fn get_table_schema(&self, args: Value) -> Result<Value> {
        let table_name = args["table_name"].as_str().ok_or_else(|| {
            CodexErr::Other("table_name parameter is required".to_string())
        })?;
        
        let database = args["database"].as_str();
        let client = self.get_client(database)?;
        let db_name = database.unwrap_or(&self.config.database);
        
        // First check knowledge base for cached schema
        let kb = self.knowledge_base.read().await;
        if let Some(table_info) = kb.get_table_info(db_name, table_name) {
            return Ok(json!({
                "table_name": table_name,
                "database": db_name,
                "columns": table_info.columns,
                "description": table_info.description,
                "sample_data": table_info.sample_data,
                "estimated_row_count": table_info.estimated_row_count,
                "source": "knowledge_base"
            }));
        }
        drop(kb);
        
        // Use .schema command to get table information
        let query = format!(".show table {} schema as json", table_name);
        
        // Execute the query
        let result: KustoQueryResult = client
            .execute_query(&query)
            .await?;
            
        // Process the results
        let processed_results = process_kusto_results(result);
        
        Ok(json!({
            "table_name": table_name,
            "database": db_name,
            "schema": processed_results,
            "source": "live_query"
        }))
    }
    
    /// List available tables
    pub async fn list_tables(&self, args: Value) -> Result<Value> {
        let database = args["database"].as_str();
        let client = self.get_client(database)?;
        let db_name = database.unwrap_or(&self.config.database);
        
        // First check knowledge base for cached tables
        let kb = self.knowledge_base.read().await;
        if let Some(tables) = kb.get_database_tables(db_name) {
            let table_list: Vec<_> = tables.iter().map(|(name, info)| {
                json!({
                    "table_name": name,
                    "description": info.description,
                    "column_count": info.columns.len(),
                    "query_count": info.query_count,
                    "last_accessed": info.last_accessed,
                    "source": "knowledge_base"
                })
            }).collect();
            
            if !table_list.is_empty() {
                return Ok(json!({
                    "database": db_name,
                    "tables": table_list,
                    "source": "knowledge_base"
                }));
            }
        }
        drop(kb);
        
        // Use .show tables command
        let query = ".show tables | project TableName, DatabaseName";
        
        // Execute the query
        let result: KustoQueryResult = client
            .execute_query(query)
            .await?;
            
        // Process the results
        let processed_results = process_kusto_results(result);
        
        Ok(json!({
            "database": db_name,
            "tables": processed_results,
            "source": "live_query"
        }))
    }
    
    /// List available databases
    pub async fn list_databases(&self, _args: Value) -> Result<Value> {
        let mut databases = Vec::new();
        
        // Add default database
        if !self.config.database.is_empty() {
            databases.push(json!({
                "name": self.config.database,
                "cluster_url": self.config.cluster_url,
                "is_default": true,
                "description": "Default database"
            }));
        }
        
        // Add configured databases
        for (db_name, db_config) in &self.config.databases {
            databases.push(json!({
                "name": db_config.name,
                "alias": db_name,
                "cluster_url": db_config.cluster_url.as_ref().unwrap_or(&self.config.cluster_url),
                "is_default": db_config.is_default,
                "description": db_config.description
            }));
        }
        
        Ok(json!({
            "databases": databases,
            "total_count": databases.len()
        }))
    }
    
    /// Get knowledge base summary
    pub async fn get_knowledge_base_summary(&self, _args: Value) -> Result<Value> {
        let kb = self.knowledge_base.read().await;
        let summary = kb.generate_summary();
        
        Ok(json!({
            "summary": summary,
            "last_updated": kb.last_updated,
            "database_count": kb.databases.len(),
            "pattern_count": kb.query_patterns.len(),
            "function_count": kb.functions.len()
        }))
    }
    
    /// Update table description in knowledge base
    pub async fn update_table_description(&self, args: Value) -> Result<Value> {
        let database = args["database"].as_str().ok_or_else(|| {
            CodexErr::Other("database parameter is required".to_string())
        })?;
        
        let table_name = args["table_name"].as_str().ok_or_else(|| {
            CodexErr::Other("table_name parameter is required".to_string())
        })?;
        
        let description = args["description"].as_str().ok_or_else(|| {
            CodexErr::Other("description parameter is required".to_string())
        })?;
        
        let mut kb = self.knowledge_base.write().await;
        
        // Update table description
        if let Some(db_info) = kb.databases.get_mut(database) {
            if let Some(table_info) = db_info.tables.get_mut(table_name) {
                table_info.description = Some(description.to_string());
                kb.last_updated = chrono::Utc::now();
                
                // Save knowledge base
                if let Err(e) = kb.save_to_file(&self.knowledge_base_path).await {
                    return Err(CodexErr::Other(format!("Failed to save knowledge base: {}", e)));
                }
                
                return Ok(json!({
                    "success": true,
                    "message": format!("Updated description for table {}.{}", database, table_name)
                }));
            }
        }
        
        Err(CodexErr::Other(format!("Table {}.{} not found in knowledge base", database, table_name)))
    }
    
    /// Search knowledge base
    pub async fn search_knowledge_base(&self, args: Value) -> Result<Value> {
        let search_term = args["search_term"].as_str().ok_or_else(|| {
            CodexErr::Other("search_term parameter is required".to_string())
        })?;
        
        let search_type = args["search_type"].as_str().unwrap_or("all");
        
        let kb = self.knowledge_base.read().await;
        let mut results = json!({
            "search_term": search_term,
            "search_type": search_type,
            "tables": [],
            "columns": [],
            "patterns": []
        });
        
        let search_term_lower = search_term.to_lowercase();
        
        // Search tables
        if search_type == "tables" || search_type == "all" {
            let mut table_matches = Vec::new();
            for (db_name, db_info) in &kb.databases {
                for (table_name, table_info) in &db_info.tables {
                    if table_name.to_lowercase().contains(&search_term_lower) ||
                       table_info.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&search_term_lower)) {
                        table_matches.push(json!({
                            "database": db_name,
                            "table_name": table_name,
                            "description": table_info.description,
                            "column_count": table_info.columns.len(),
                            "query_count": table_info.query_count
                        }));
                    }
                }
            }
            results["tables"] = json!(table_matches);
        }
        
        // Search columns
        if search_type == "columns" || search_type == "all" {
            let mut column_matches = Vec::new();
            for (db_name, db_info) in &kb.databases {
                for (table_name, table_info) in &db_info.tables {
                    for column in &table_info.columns {
                        if column.name.to_lowercase().contains(&search_term_lower) ||
                           column.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&search_term_lower)) {
                            column_matches.push(json!({
                                "database": db_name,
                                "table_name": table_name,
                                "column_name": column.name,
                                "data_type": column.data_type,
                                "description": column.description
                            }));
                        }
                    }
                }
            }
            results["columns"] = json!(column_matches);
        }
        
        // Search patterns
        if search_type == "patterns" || search_type == "all" {
            let pattern_matches: Vec<_> = kb.query_patterns.iter()
                .filter(|pattern| {
                    pattern.name.to_lowercase().contains(&search_term_lower) ||
                    pattern.description.to_lowercase().contains(&search_term_lower) ||
                    pattern.query.to_lowercase().contains(&search_term_lower)
                })
                .map(|pattern| json!({
                    "name": pattern.name,
                    "description": pattern.description,
                    "query": pattern.query,
                    "tables": pattern.tables,
                    "usage_count": pattern.usage_count
                }))
                .collect();
            results["patterns"] = json!(pattern_matches);
        }
        
        Ok(results)
    }
}