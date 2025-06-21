//! Integration of Azure DevOps tools with the OpenAI tools system.

use crate::azure_devops::tools::register_azure_devops_tools;
use crate::config_types::AzureDevOpsConfig;
use crate::openai_tools::OpenAiTool;

/// Register Azure DevOps tools with the OpenAI tools system
pub fn register_azure_devops_tools_with_openai(
    config: &Option<AzureDevOpsConfig>,
) -> Option<Vec<OpenAiTool>> {
    if config.is_some() {
        Some(register_azure_devops_tools())
    } else {
        None
    }
}

/// Create a fully qualified tool name for Azure DevOps tools
pub fn azure_devops_tool_name(tool_name: &str) -> String {
    format!("azure_devops.{}", tool_name)
}