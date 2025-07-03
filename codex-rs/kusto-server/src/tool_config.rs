//! Configuration for Kusto tools

use mcp_types::{Tool, ToolInputSchema};
use serde_json::json;

/// Create the list of available Kusto tools
pub fn create_kusto_tools() -> Vec<Tool> {
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

/// Create the execute_query tool definition
fn create_execute_query_tool() -> Tool {
    Tool {
        name: "kusto_execute_query".to_string(),
        description: Some("Execute a Kusto query against Azure Data Explorer. Optionally specify a database name, otherwise uses the default database.".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "query": {
                    "type": "string",
                    "description": "Kusto query to execute (KQL)"
                },
                "database": {
                    "type": "string",
                    "description": "Database name (optional, uses default if not specified)"
                }
            })),
            required: Some(vec!["query".to_string()]),
        },
    }
}

/// Create the get_table_schema tool definition
fn create_get_table_schema_tool() -> Tool {
    Tool {
        name: "kusto_get_table_schema".to_string(),
        description: Some("Get schema information for a Kusto table. Optionally specify a database name, otherwise uses the default database.".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "table_name": {
                    "type": "string",
                    "description": "Name of the table to get schema for"
                },
                "database": {
                    "type": "string",
                    "description": "Database name (optional, uses default if not specified)"
                }
            })),
            required: Some(vec!["table_name".to_string()]),
        },
    }
}

/// Create the list_tables tool definition
fn create_list_tables_tool() -> Tool {
    Tool {
        name: "kusto_list_tables".to_string(),
        description: Some("List available tables in the Kusto database. Optionally specify a database name, otherwise uses the default database.".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "database": {
                    "type": "string",
                    "description": "Database name (optional, uses default if not specified)"
                }
            })),
            required: Some(vec![]),
        },
    }
}

/// Create the list_databases tool definition
fn create_list_databases_tool() -> Tool {
    Tool {
        name: "kusto_list_databases".to_string(),
        description: Some("List all configured Kusto databases and their information".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({})),
            required: Some(vec![]),
        },
    }
}

/// Create the get_knowledge_base_summary tool definition
fn create_get_knowledge_base_summary_tool() -> Tool {
    Tool {
        name: "kusto_get_knowledge_base_summary".to_string(),
        description: Some("Get a summary of the Kusto knowledge base including databases, tables, and common query patterns".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({})),
            required: Some(vec![]),
        },
    }
}

/// Create the update_table_description tool definition
fn create_update_table_description_tool() -> Tool {
    Tool {
        name: "kusto_update_table_description".to_string(),
        description: Some("Update the description of a table in the knowledge base".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "database": {
                    "type": "string",
                    "description": "Database name"
                },
                "table_name": {
                    "type": "string",
                    "description": "Name of the table"
                },
                "description": {
                    "type": "string",
                    "description": "New description for the table"
                }
            })),
            required: Some(vec!["database".to_string(), "table_name".to_string(), "description".to_string()]),
        },
    }
}

/// Create the search_knowledge_base tool definition
fn create_search_knowledge_base_tool() -> Tool {
    Tool {
        name: "kusto_search_knowledge_base".to_string(),
        description: Some("Search the knowledge base for tables, columns, or query patterns. search_type can be 'tables', 'columns', 'patterns', or 'all'".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "search_term": {
                    "type": "string",
                    "description": "Term to search for"
                },
                "search_type": {
                    "type": "string",
                    "description": "Type of search to perform",
                    "enum": ["tables", "columns", "patterns", "all"],
                    "default": "all"
                }
            })),
            required: Some(vec!["search_term".to_string()]),
        },
    }
}

/// Create the list_functions tool definition
fn create_list_functions_tool() -> Tool {
    Tool {
        name: "kusto_list_functions".to_string(),
        description: Some("List available functions in the Kusto database".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({})),
            required: Some(vec![]),
        },
    }
}

/// Create the describe_function tool definition
fn create_describe_function_tool() -> Tool {
    Tool {
        name: "kusto_describe_function".to_string(),
        description: Some("Get detailed information about a specific Kusto function".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "function_name": {
                    "type": "string",
                    "description": "Name of the function to describe"
                }
            })),
            required: Some(vec!["function_name".to_string()]),
        },
    }
}

/// Create the test_connection tool definition
fn create_test_connection_tool() -> Tool {
    Tool {
        name: "kusto_test_connection".to_string(),
        description: Some("Test Kusto connection and run a diagnostic query. Use this to debug connection issues and validate queries before running them.".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "test_query": {
                    "type": "string",
                    "description": "Optional test query to run (defaults to 'print \"Hello, Kusto!\"')"
                },
                "database": {
                    "type": "string",
                    "description": "Database name (optional, uses default if not specified)"
                }
            })),
            required: Some(vec![]),
        },
    }
}

/// Create the clear_auth_cache tool definition
fn create_clear_auth_cache_tool() -> Tool {
    Tool {
        name: "kusto_clear_auth_cache".to_string(),
        description: Some("Clear Kusto authentication cache and force re-authentication. Use this when getting authentication errors.".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({})),
            required: Some(vec![]),
        },
    }
}