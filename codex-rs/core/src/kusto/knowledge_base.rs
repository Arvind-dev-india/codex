//! Knowledge base for Kusto tables, schemas, and query patterns.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;
use chrono::{DateTime, Utc};

use crate::error::{CodexErr, Result};

/// Knowledge base containing information about Kusto databases, tables, and query patterns
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KustoKnowledgeBase {
    /// Version of the knowledge base format
    pub version: String,
    
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
    
    /// Database information
    pub databases: HashMap<String, DatabaseInfo>,
    
    /// Query patterns and examples
    pub query_patterns: Vec<QueryPattern>,
    
    /// Common functions and operators
    pub functions: HashMap<String, FunctionInfo>,
}

/// Information about a specific database
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseInfo {
    /// Database name
    pub name: String,
    
    /// Database description
    pub description: Option<String>,
    
    /// Cluster URL
    pub cluster_url: String,
    
    /// Tables in this database
    pub tables: HashMap<String, TableInfo>,
    
    /// Last time this database was queried
    pub last_accessed: DateTime<Utc>,
    
    /// Number of times this database has been queried
    pub query_count: u64,
}

/// Information about a specific table
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableInfo {
    /// Table name
    pub name: String,
    
    /// Table description
    pub description: Option<String>,
    
    /// Column information
    pub columns: Vec<ColumnInfo>,
    
    /// Sample data (limited rows)
    pub sample_data: Vec<HashMap<String, serde_json::Value>>,
    
    /// Common query patterns for this table
    pub common_queries: Vec<String>,
    
    /// Last time this table was queried
    pub last_accessed: DateTime<Utc>,
    
    /// Number of times this table has been queried
    pub query_count: u64,
    
    /// Estimated row count
    pub estimated_row_count: Option<u64>,
    
    /// Data retention period
    pub retention_period: Option<String>,
}

/// Information about a table column
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ColumnInfo {
    /// Column name
    pub name: String,
    
    /// Data type
    pub data_type: String,
    
    /// Column description
    pub description: Option<String>,
    
    /// Whether the column is nullable
    pub is_nullable: Option<bool>,
    
    /// Sample values
    pub sample_values: Vec<serde_json::Value>,
    
    /// Whether this column is commonly used in queries
    pub is_commonly_queried: bool,
}

/// Query pattern information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryPattern {
    /// Pattern name/title
    pub name: String,
    
    /// Pattern description
    pub description: String,
    
    /// Example query
    pub query: String,
    
    /// Tables involved in this pattern
    pub tables: Vec<String>,
    
    /// Use case or scenario
    pub use_case: String,
    
    /// How often this pattern is used
    pub usage_count: u64,
}

/// Information about Kusto functions
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FunctionInfo {
    /// Function name
    pub name: String,
    
    /// Function description
    pub description: String,
    
    /// Function syntax
    pub syntax: String,
    
    /// Example usage
    pub example: String,
    
    /// Category (e.g., "aggregation", "string", "datetime")
    pub category: String,
}

impl KustoKnowledgeBase {
    /// Create a new empty knowledge base
    pub fn new() -> Self {
        Self {
            version: "1.0".to_string(),
            last_updated: Utc::now(),
            databases: HashMap::new(),
            query_patterns: Vec::new(),
            functions: Self::default_functions(),
        }
    }
    
    /// Load knowledge base from file
    pub async fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(Self::new());
        }
        
        let content = fs::read_to_string(path).await
            .map_err(|e| CodexErr::Other(format!("Failed to read knowledge base file: {}", e)))?;
            
        let kb: KustoKnowledgeBase = serde_json::from_str(&content)
            .map_err(|e| CodexErr::Other(format!("Failed to parse knowledge base: {}", e)))?;
            
        Ok(kb)
    }
    
    /// Save knowledge base to file
    pub async fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| CodexErr::Other(format!("Failed to serialize knowledge base: {}", e)))?;
            
        fs::write(path, content).await
            .map_err(|e| CodexErr::Other(format!("Failed to write knowledge base file: {}", e)))?;
            
        Ok(())
    }
    
    /// Add or update database information
    pub fn update_database(&mut self, name: String, cluster_url: String, description: Option<String>) {
        let db_info = self.databases.entry(name.clone()).or_insert_with(|| DatabaseInfo {
            name: name.clone(),
            description: None,
            cluster_url: cluster_url.clone(),
            tables: HashMap::new(),
            last_accessed: Utc::now(),
            query_count: 0,
        });
        
        db_info.cluster_url = cluster_url;
        if let Some(desc) = description {
            db_info.description = Some(desc);
        }
        db_info.last_accessed = Utc::now();
        db_info.query_count += 1;
        
        self.last_updated = Utc::now();
    }
    
    /// Add or update table information
    pub fn update_table(&mut self, database: &str, table_name: String, columns: Vec<ColumnInfo>, sample_data: Vec<HashMap<String, serde_json::Value>>) {
        if let Some(db_info) = self.databases.get_mut(database) {
            let table_info = db_info.tables.entry(table_name.clone()).or_insert_with(|| TableInfo {
                name: table_name.clone(),
                description: None,
                columns: Vec::new(),
                sample_data: Vec::new(),
                common_queries: Vec::new(),
                last_accessed: Utc::now(),
                query_count: 0,
                estimated_row_count: None,
                retention_period: None,
            });
            
            table_info.columns = columns;
            table_info.sample_data = sample_data;
            table_info.last_accessed = Utc::now();
            table_info.query_count += 1;
        }
        
        self.last_updated = Utc::now();
    }
    
    /// Add a query pattern
    pub fn add_query_pattern(&mut self, pattern: QueryPattern) {
        // Check if pattern already exists and update usage count
        if let Some(existing) = self.query_patterns.iter_mut().find(|p| p.query == pattern.query) {
            existing.usage_count += 1;
        } else {
            self.query_patterns.push(pattern);
        }
        
        self.last_updated = Utc::now();
    }
    
    /// Get table information for a specific database
    pub fn get_table_info(&self, database: &str, table: &str) -> Option<&TableInfo> {
        self.databases.get(database)?.tables.get(table)
    }
    
    /// Get all tables for a database
    pub fn get_database_tables(&self, database: &str) -> Option<&HashMap<String, TableInfo>> {
        Some(&self.databases.get(database)?.tables)
    }
    
    /// Get relevant query patterns for specific tables
    pub fn get_relevant_patterns(&self, tables: &[String]) -> Vec<&QueryPattern> {
        self.query_patterns.iter()
            .filter(|pattern| {
                tables.iter().any(|table| pattern.tables.contains(table))
            })
            .collect()
    }
    
    /// Generate a summary of the knowledge base for the LLM
    pub fn generate_summary(&self) -> String {
        let mut summary = String::new();
        
        summary.push_str("# Kusto Knowledge Base Summary\n\n");
        
        // Database overview
        summary.push_str("## Available Databases\n");
        for (db_name, db_info) in &self.databases {
            summary.push_str(&format!("### {}\n", db_name));
            if let Some(desc) = &db_info.description {
                summary.push_str(&format!("Description: {}\n", desc));
            }
            summary.push_str(&format!("Tables: {}\n", db_info.tables.len()));
            summary.push_str(&format!("Query Count: {}\n\n", db_info.query_count));
        }
        
        // Most commonly used tables
        summary.push_str("## Most Commonly Used Tables\n");
        let mut table_usage: Vec<_> = self.databases.iter()
            .flat_map(|(db_name, db_info)| {
                db_info.tables.iter().map(move |(table_name, table_info)| {
                    (format!("{}.{}", db_name, table_name), table_info.query_count)
                })
            })
            .collect();
        table_usage.sort_by(|a, b| b.1.cmp(&a.1));
        
        for (table_name, count) in table_usage.iter().take(10) {
            summary.push_str(&format!("- {}: {} queries\n", table_name, count));
        }
        
        // Common query patterns
        summary.push_str("\n## Common Query Patterns\n");
        let mut patterns = self.query_patterns.clone();
        patterns.sort_by(|a, b| b.usage_count.cmp(&a.usage_count));
        
        for pattern in patterns.iter().take(5) {
            summary.push_str(&format!("### {}\n", pattern.name));
            summary.push_str(&format!("{}\n", pattern.description));
            summary.push_str(&format!("Usage: {} times\n\n", pattern.usage_count));
        }
        
        summary
    }
    
    /// Get default Kusto functions
    fn default_functions() -> HashMap<String, FunctionInfo> {
        let mut functions = HashMap::new();
        
        // Add common Kusto functions
        functions.insert("count".to_string(), FunctionInfo {
            name: "count".to_string(),
            description: "Returns the number of records in the input record set".to_string(),
            syntax: "count()".to_string(),
            example: "StormEvents | count".to_string(),
            category: "aggregation".to_string(),
        });
        
        functions.insert("summarize".to_string(), FunctionInfo {
            name: "summarize".to_string(),
            description: "Produces a table that aggregates the content of the input table".to_string(),
            syntax: "summarize [Column =] Aggregation [, ...] [by Column [, ...]]".to_string(),
            example: "StormEvents | summarize count() by State".to_string(),
            category: "aggregation".to_string(),
        });
        
        functions.insert("where".to_string(), FunctionInfo {
            name: "where".to_string(),
            description: "Filters a table to the subset of rows that satisfy a predicate".to_string(),
            syntax: "where Predicate".to_string(),
            example: "StormEvents | where State == 'TEXAS'".to_string(),
            category: "filter".to_string(),
        });
        
        functions.insert("project".to_string(), FunctionInfo {
            name: "project".to_string(),
            description: "Select the columns to include, rename or drop, and insert new computed columns".to_string(),
            syntax: "project ColumnName [= Expression] [, ...]".to_string(),
            example: "StormEvents | project State, EventType, DamageProperty".to_string(),
            category: "projection".to_string(),
        });
        
        functions.insert("take".to_string(), FunctionInfo {
            name: "take".to_string(),
            description: "Return up to the specified number of rows".to_string(),
            syntax: "take NumberOfRows".to_string(),
            example: "StormEvents | take 10".to_string(),
            category: "sampling".to_string(),
        });
        
        functions.insert("top".to_string(), FunctionInfo {
            name: "top".to_string(),
            description: "Returns the first N records sorted by the specified columns".to_string(),
            syntax: "top NumberOfRows by Expression [asc|desc]".to_string(),
            example: "StormEvents | top 10 by DamageProperty desc".to_string(),
            category: "sorting".to_string(),
        });
        
        functions.insert("join".to_string(), FunctionInfo {
            name: "join".to_string(),
            description: "Merge the rows of two tables to form a new table".to_string(),
            syntax: "join [kind=JoinKind] [hint.strategy=Strategy] (RightTable) on Attributes".to_string(),
            example: "Table1 | join kind=inner (Table2) on CommonColumn".to_string(),
            category: "join".to_string(),
        });
        
        functions.insert("extend".to_string(), FunctionInfo {
            name: "extend".to_string(),
            description: "Create calculated columns and append them to the result set".to_string(),
            syntax: "extend [ColumnName =] Expression [, ...]".to_string(),
            example: "StormEvents | extend DamageRatio = DamageProperty / DamageCrops".to_string(),
            category: "projection".to_string(),
        });
        
        functions
    }
}

impl Default for KustoKnowledgeBase {
    fn default() -> Self {
        Self::new()
    }
}