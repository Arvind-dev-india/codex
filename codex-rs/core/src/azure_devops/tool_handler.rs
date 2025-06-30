//! Handler for Azure DevOps tool calls.

use serde_json::Value;
use std::sync::Arc;

use crate::azure_devops::tools_impl::AzureDevOpsTools;
use crate::config_types::AzureDevOpsConfig;
use crate::error::{CodexErr, Result};
use crate::mcp_tool_call::ToolCall;

/// Handle Azure DevOps tool calls
pub async fn handle_azure_devops_tool_call(
    tool_call: &ToolCall,
    config: &AzureDevOpsConfig,
) -> Result<Value> {
    // Create tools instance
    let tools = Arc::new(AzureDevOpsTools::new(config).await?);
    
    // Extract tool name and arguments
    let name = &tool_call.name;
    let mut args = tool_call.arguments.clone();
    
    // If the arguments don't include a project and we have a default project in the config,
    // add the default project to the arguments
    if !args.is_object() {
        args = serde_json::Value::Object(serde_json::Map::new());
    }
    
    if args.get("project").is_none() {
        if let Some(default_project) = &config.default_project {
            if let serde_json::Value::Object(ref mut map) = args {
                map.insert("project".to_string(), serde_json::Value::String(default_project.clone()));
            }
        }
    }
    
    // Dispatch to appropriate tool function
    match name.as_str() {
        "azure_devops_query_work_items" => {
            tools.query_work_items(args).await
        },
        "azure_devops_get_work_item" => {
            tools.get_work_item(args).await
        },
        "azure_devops_create_work_item" => {
            tools.create_work_item(args).await
        },
        "azure_devops_update_work_item" => {
            tools.update_work_item(args).await
        },
        "azure_devops_add_work_item_comment" => {
            tools.add_work_item_comment(args).await
        },
        "azure_devops_query_pull_requests" => {
            tools.query_pull_requests(args).await
        },
        "azure_devops_get_pull_request" => {
            tools.get_pull_request(args).await
        },
        "azure_devops_comment_on_pull_request" => {
            tools.comment_on_pull_request(args).await
        },
        "azure_devops_get_wiki_page" => {
            tools.get_wiki_page(args).await
        },
        "azure_devops_update_wiki_page" => {
            tools.update_wiki_page(args).await
        },
        "azure_devops_run_pipeline" => {
            tools.run_pipeline(args).await
        },
        "azure_devops_get_pipeline_status" => {
            tools.get_pipeline_status(args).await
        },
        _ => {
            Err(CodexErr::Other(format!("Unknown Azure DevOps tool: {}", name)))
        }
    }
}