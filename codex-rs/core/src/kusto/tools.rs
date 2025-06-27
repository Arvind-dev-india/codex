//! Tool definitions for Kusto (Azure Data Explorer) operations.

use crate::openai_tools::OpenAiTool;

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
    OpenAiTool {
        name: "kusto_execute_query".to_string(),
        description: "Execute a Kusto query against Azure Data Explorer".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The Kusto Query Language (KQL) query to execute"
                }
            },
            "required": ["query"]
        }),
    }
}

/// Create a tool for getting schema information for a table
fn create_get_table_schema_tool() -> OpenAiTool {
    OpenAiTool {
        name: "kusto_get_table_schema".to_string(),
        description: "Get schema information for a Kusto table".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "table_name": {
                    "type": "string",
                    "description": "The name of the table to get schema for"
                }
            },
            "required": ["table_name"]
        }),
    }
}

/// Create a tool for listing available tables
fn create_list_tables_tool() -> OpenAiTool {
    OpenAiTool {
        name: "kusto_list_tables".to_string(),
        description: "List available tables in the Kusto database".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {}
        }),
    }
}