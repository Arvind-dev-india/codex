//! Data models for Kusto (Azure Data Explorer) entities.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a Kusto query result
#[derive(Debug, Deserialize, Serialize)]
pub struct KustoQueryResult {
    /// Tables returned by the query
    pub tables: Vec<KustoTable>,
}

/// Represents a table in a Kusto query result
#[derive(Debug, Deserialize, Serialize)]
pub struct KustoTable {
    /// Name of the table
    pub name: String,
    /// Column definitions
    pub columns: Vec<KustoColumn>,
    /// Rows of data
    pub rows: Vec<Vec<serde_json::Value>>,
}

/// Represents a column in a Kusto table
#[derive(Debug, Deserialize, Serialize)]
pub struct KustoColumn {
    /// Name of the column
    pub name: String,
    /// Data type of the column
    #[serde(rename = "type")]
    pub data_type: String,
}

/// Represents a row in a Kusto table with named columns
#[derive(Debug, Deserialize, Serialize)]
pub struct KustoRow {
    /// Column name to value mapping
    #[serde(flatten)]
    pub values: HashMap<String, serde_json::Value>,
}

/// Convert raw Kusto query results to a more usable format
pub fn process_kusto_results(result: KustoQueryResult) -> Vec<HashMap<String, serde_json::Value>> {
    let mut processed_results = Vec::new();
    
    for table in result.tables {
        // Skip empty tables
        if table.rows.is_empty() {
            continue;
        }
        
        // Process each row
        for row in table.rows {
            let mut row_map = HashMap::new();
            
            // Map column names to values
            for (i, value) in row.iter().enumerate() {
                if i < table.columns.len() {
                    let column_name = &table.columns[i].name;
                    row_map.insert(column_name.clone(), value.clone());
                }
            }
            
            processed_results.push(row_map);
        }
    }
    
    processed_results
}