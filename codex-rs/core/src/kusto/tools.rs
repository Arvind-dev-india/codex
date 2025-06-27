//! Tool definitions for Kusto (Azure Data Explorer) operations.

use std::collections::BTreeMap;
use crate::openai_tools::{OpenAiTool, JsonSchema, create_function_tool};

/// Create all Kusto tools
pub fn create_kusto_tools() -> Vec<OpenAiTool> {
    vec![
        create_execute_query_tool(),
        create_get_table_schema_tool(),
        create_list_tables_tool(),
    ]
}

/// Create a tool for executing Kusto queries
fn create_execute_query_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("query".to_string(), JsonSchema::String);
    
    create_function_tool(
        "kusto_execute_query",
        "Execute a Kusto query against Azure Data Explorer",
        parameters,
        &["query"],
    )
}

/// Create a tool for getting schema information for a table
fn create_get_table_schema_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("table_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "kusto_get_table_schema",
        "Get schema information for a Kusto table",
        parameters,
        &["table_name"],
    )
}

/// Create a tool for listing available tables
fn create_list_tables_tool() -> OpenAiTool {
    let parameters = BTreeMap::new();
    
    create_function_tool(
        "kusto_list_tables",
        "List available tables in the Kusto database",
        parameters,
        &[],
    )
}