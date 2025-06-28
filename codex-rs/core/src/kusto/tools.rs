//! Tool definitions for Kusto (Azure Data Explorer) operations.

use std::collections::BTreeMap;
use crate::openai_tools::{OpenAiTool, JsonSchema, create_function_tool};

/// Create all Kusto tools
pub fn create_kusto_tools() -> Vec<OpenAiTool> {
    vec![
        create_execute_query_tool(),
        create_get_table_schema_tool(),
        create_list_tables_tool(),
        create_list_databases_tool(),
        create_get_knowledge_base_summary_tool(),
        create_update_table_description_tool(),
        create_search_knowledge_base_tool(),
        create_list_functions_tool(),
        create_describe_function_tool(),
        create_test_connection_tool(),
        create_clear_auth_cache_tool(),
    ]
}

/// Create a tool for executing Kusto queries
fn create_execute_query_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("query".to_string(), JsonSchema::String);
    parameters.insert("database".to_string(), JsonSchema::String);
    
    create_function_tool(
        "kusto_execute_query",
        "Execute a Kusto query against Azure Data Explorer. Optionally specify a database name, otherwise uses the default database.",
        parameters,
        &["query"],
    )
}

/// Create a tool for getting schema information for a table
fn create_get_table_schema_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("table_name".to_string(), JsonSchema::String);
    parameters.insert("database".to_string(), JsonSchema::String);
    
    create_function_tool(
        "kusto_get_table_schema",
        "Get schema information for a Kusto table. Optionally specify a database name, otherwise uses the default database.",
        parameters,
        &["table_name"],
    )
}

/// Create a tool for listing available tables
fn create_list_tables_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("database".to_string(), JsonSchema::String);
    
    create_function_tool(
        "kusto_list_tables",
        "List available tables in the Kusto database. Optionally specify a database name, otherwise uses the default database.",
        parameters,
        &[],
    )
}

/// Create a tool for listing available databases
fn create_list_databases_tool() -> OpenAiTool {
    let parameters = BTreeMap::new();
    
    create_function_tool(
        "kusto_list_databases",
        "List all configured Kusto databases and their information",
        parameters,
        &[],
    )
}

/// Create a tool for getting knowledge base summary
fn create_get_knowledge_base_summary_tool() -> OpenAiTool {
    let parameters = BTreeMap::new();
    
    create_function_tool(
        "kusto_get_knowledge_base_summary",
        "Get a summary of the Kusto knowledge base including databases, tables, and common query patterns",
        parameters,
        &[],
    )
}

/// Create a tool for updating table descriptions
fn create_update_table_description_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("database".to_string(), JsonSchema::String);
    parameters.insert("table_name".to_string(), JsonSchema::String);
    parameters.insert("description".to_string(), JsonSchema::String);
    
    create_function_tool(
        "kusto_update_table_description",
        "Update the description of a table in the knowledge base",
        parameters,
        &["database", "table_name", "description"],
    )
}

/// Create a tool for searching the knowledge base
fn create_search_knowledge_base_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("search_term".to_string(), JsonSchema::String);
    parameters.insert("search_type".to_string(), JsonSchema::String);
    
    create_function_tool(
        "kusto_search_knowledge_base",
        "Search the knowledge base for tables, columns, or query patterns. search_type can be 'tables', 'columns', 'patterns', or 'all'",
        parameters,
        &["search_term"],
    )
}

/// Create a tool for listing available functions
fn create_list_functions_tool() -> OpenAiTool {
    let parameters = BTreeMap::new();
    
    create_function_tool(
        "kusto_list_functions",
        "List available functions in the Kusto database",
        parameters,
        &[],
    )
}

/// Create a tool for describing a specific function
fn create_describe_function_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("function_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "kusto_describe_function",
        "Get detailed information about a specific Kusto function",
        parameters,
        &["function_name"],
    )
}

/// Create a tool for testing connection and queries
fn create_test_connection_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("test_query".to_string(), JsonSchema::String);
    parameters.insert("database".to_string(), JsonSchema::String);
    
    create_function_tool(
        "kusto_test_connection",
        "Test Kusto connection and run a diagnostic query. Use this to debug connection issues and validate queries before running them.",
        parameters,
        &[],
    )
}

/// Create a tool for clearing authentication cache
fn create_clear_auth_cache_tool() -> OpenAiTool {
    let parameters = BTreeMap::new();
    
    create_function_tool(
        "kusto_clear_auth_cache",
        "Clear Kusto authentication cache and force re-authentication. Use this when getting authentication errors.",
        parameters,
        &[],
    )
}