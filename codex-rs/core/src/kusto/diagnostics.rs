//! Kusto diagnostics and troubleshooting utilities

use super::client_sdk::KustoSdkClient;
use crate::error::{CodexErr, Result};

/// Diagnostic utilities for Kusto connectivity and query issues
pub struct KustoDiagnostics;

impl KustoDiagnostics {
    /// Run comprehensive diagnostics on a Kusto client
    pub async fn run_diagnostics(client: &KustoSdkClient, database: &str) -> Result<String> {
        let mut report = String::new();
        report.push_str("=== Kusto Diagnostics Report ===\n\n");
        
        // Test 1: Basic connectivity
        report.push_str("1. Testing basic connectivity...\n");
        match Self::test_basic_connectivity(client).await {
            Ok(_) => report.push_str("   ✅ Basic connectivity: PASSED\n"),
            Err(e) => report.push_str(&format!("   ❌ Basic connectivity: FAILED - {}\n", e)),
        }
        
        // Test 2: Database access
        report.push_str("\n2. Testing database access...\n");
        match Self::test_database_access(client, database).await {
            Ok(_) => report.push_str("   ✅ Database access: PASSED\n"),
            Err(e) => report.push_str(&format!("   ❌ Database access: FAILED - {}\n", e)),
        }
        
        // Test 3: Management commands
        report.push_str("\n3. Testing management commands...\n");
        match Self::test_management_commands(client).await {
            Ok(_) => report.push_str("   ✅ Management commands: PASSED\n"),
            Err(e) => report.push_str(&format!("   ❌ Management commands: FAILED - {}\n", e)),
        }
        
        // Test 4: Table listing
        report.push_str("\n4. Testing table listing...\n");
        match Self::test_table_listing(client).await {
            Ok(tables) => {
                report.push_str("   ✅ Table listing: PASSED\n");
                report.push_str(&format!("   Found {} tables\n", tables.len()));
                if !tables.is_empty() {
                    report.push_str("   Sample tables:\n");
                    for (i, table) in tables.iter().take(3).enumerate() {
                        report.push_str(&format!("     {}. {}\n", i + 1, table));
                    }
                }
            },
            Err(e) => report.push_str(&format!("   ❌ Table listing: FAILED - {}\n", e)),
        }
        
        report.push_str("\n=== End Diagnostics Report ===\n");
        Ok(report)
    }
    
    /// Test basic connectivity with a simple print statement
    async fn test_basic_connectivity(client: &KustoSdkClient) -> Result<()> {
        let query = "print 'test'";
        tracing::info!("Running basic connectivity test: {}", query);
        
        let _result = client.execute_query(query).await?;
        Ok(())
    }
    
    /// Test database access with a database-specific query
    async fn test_database_access(client: &KustoSdkClient, database: &str) -> Result<()> {
        // Try a simple query that should work if database access is correct
        let query = "print database_name = database()";
        tracing::info!("Running database access test: {}", query);
        
        let result = client.execute_query(query).await?;
        tracing::info!("Database access test result: {:?}", result);
        Ok(())
    }
    
    /// Test management commands
    async fn test_management_commands(client: &KustoSdkClient) -> Result<()> {
        // Try the simplest management command
        let query = ".show version";
        tracing::info!("Running management command test: {}", query);
        
        let _result = client.execute_query(query).await?;
        Ok(())
    }
    
    /// Test table listing with different approaches
    async fn test_table_listing(client: &KustoSdkClient) -> Result<Vec<String>> {
        let queries = vec![
            ".show tables | project TableName",
            ".show tables",
            ".show tables | take 5",
        ];
        
        for query in queries {
            tracing::info!("Trying table listing query: {}", query);
            match client.execute_query(query).await {
                Ok(result) => {
                    let tables: Vec<String> = result.iter()
                        .filter_map(|row| {
                            row.get("TableName")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string())
                        })
                        .collect();
                    
                    if !tables.is_empty() {
                        tracing::info!("Successfully got {} tables with query: {}", tables.len(), query);
                        return Ok(tables);
                    }
                },
                Err(e) => {
                    tracing::warn!("Query '{}' failed: {}", query, e);
                    continue;
                }
            }
        }
        
        Err(CodexErr::Other("All table listing queries failed".to_string()))
    }
}