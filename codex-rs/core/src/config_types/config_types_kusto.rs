//! Kusto (Azure Data Explorer) configuration types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Kusto (Azure Data Explorer) configuration
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct KustoConfig {
    /// Kusto cluster URL (e.g., "https://help.kusto.windows.net")
    pub cluster_url: String,
    
    /// Default database to query (for backward compatibility)
    pub database: String,
    
    /// Multiple databases configuration
    #[serde(default)]
    pub databases: HashMap<String, KustoDatabaseConfig>,
    
    /// Path to the knowledge base file (relative to codex home or absolute)
    #[serde(default = "default_knowledge_base_path")]
    pub knowledge_base_path: String,
    
    /// Whether to automatically update the knowledge base with query results
    #[serde(default = "default_auto_update_kb")]
    pub auto_update_knowledge_base: bool,
    
    /// Maximum number of rows to include in knowledge base updates
    #[serde(default = "default_max_kb_rows")]
    pub max_knowledge_base_rows: usize,
}

/// Configuration for a specific Kusto database
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct KustoDatabaseConfig {
    /// Database name
    pub name: String,
    
    /// Optional description of the database
    pub description: Option<String>,
    
    /// Cluster URL (if different from the main one)
    pub cluster_url: Option<String>,
    
    /// Whether this database is the default for queries
    #[serde(default)]
    pub is_default: bool,
}

fn default_knowledge_base_path() -> String {
    "kusto_knowledge_base.json".to_string()
}

fn default_auto_update_kb() -> bool {
    true
}

fn default_max_kb_rows() -> usize {
    100
}

impl Default for KustoConfig {
    fn default() -> Self {
        Self {
            cluster_url: String::new(),
            database: String::new(),
            databases: HashMap::new(),
            knowledge_base_path: default_knowledge_base_path(),
            auto_update_knowledge_base: default_auto_update_kb(),
            max_knowledge_base_rows: default_max_kb_rows(),
        }
    }
}