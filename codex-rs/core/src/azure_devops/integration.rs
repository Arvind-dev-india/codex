//! Integration of Azure DevOps tools with the OpenAI tools system.

use crate::azure_devops::tools::register_azure_devops_tools;
use crate::config_types::AzureDevOpsConfig;
use crate::openai_tools::OpenAiTool;

/// Register Azure DevOps tools with the OpenAI tools system
pub fn register_azure_devops_tools_with_openai(
    config: &Option<AzureDevOpsConfig>,
) -> Option<Vec<OpenAiTool>> {
    if config.is_some() {
        // Get the tools and prefix them with "azure_devops__OAI_CODEX_MCP__" to match
        // the MCP connection manager's expected format
        let tools = register_azure_devops_tools();
        Some(tools)
    } else {
        None
    }
}

/// Create a fully qualified tool name for Azure DevOps tools
pub fn azure_devops_tool_name(tool_name: &str) -> String {
    format!("azure_devops.{}", tool_name)
}