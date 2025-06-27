//! Kusto (Azure Data Explorer) configuration types.

use serde::{Deserialize, Serialize};

/// Kusto (Azure Data Explorer) configuration
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct KustoConfig {
    /// Kusto cluster URL (e.g., "https://help.kusto.windows.net")
    pub cluster_url: String,
    
    /// Database to query
    pub database: String,
}

impl Default for KustoConfig {
    fn default() -> Self {
        Self {
            cluster_url: String::new(),
            database: String::new(),
        }
    }
}