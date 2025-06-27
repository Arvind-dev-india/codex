//! Handler for Kusto (Azure Data Explorer) tool calls.

use serde_json::Value;
use std::sync::Arc;

use crate::kusto::tools_impl::KustoTools;
use crate::config_types::KustoConfig;
use crate::error::{CodexErr, Result};
use crate::mcp_tool_call::ToolCall;

/// Handle Kusto tool calls
pub async fn handle_kusto_tool_call(
    tool_call: &ToolCall,
    config: &KustoConfig,
) -> Result<Value> {
    // Create tools instance
    let tools = Arc::new(KustoTools::new(config).await?);
    
    // Extract tool name and arguments
    let name = &tool_call.name;
    let args = tool_call.arguments.clone();
    
    // Dispatch to appropriate tool function
    match name.as_str() {
        "kusto_execute_query" => {
            tools.execute_query(args).await
        },
        "kusto_get_table_schema" => {
            tools.get_table_schema(args).await
        },
        "kusto_list_tables" => {
            tools.list_tables(args).await
        },
        "kusto_list_databases" => {
            tools.list_databases(args).await
        },
        "kusto_get_knowledge_base_summary" => {
            tools.get_knowledge_base_summary(args).await
        },
        "kusto_update_table_description" => {
            tools.update_table_description(args).await
        },
        "kusto_search_knowledge_base" => {
            tools.search_knowledge_base(args).await
        },
        _ => {
            Err(CodexErr::Other(format!("Unknown Kusto tool: {}", name)))
        }
    }
}