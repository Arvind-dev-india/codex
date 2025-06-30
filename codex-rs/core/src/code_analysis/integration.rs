//! Integration of Code Analysis tools with the OpenAI tools system.

use crate::code_analysis::tools::register_code_analysis_tools;
use crate::openai_tools::OpenAiTool;

/// Register Code Analysis tools with the OpenAI tools system
pub fn register_code_analysis_tools_with_openai() -> Vec<OpenAiTool> {
    // Get the tools and prefix them with "code_analysis_" to match
    // the expected format in the mcp_connection_manager
    let tools = register_code_analysis_tools();
    tools
}

/// Create a fully qualified tool name for Code Analysis tools
pub fn code_analysis_tool_name(tool_name: &str) -> String {
    format!("code_analysis_{}", tool_name)
}