//! Kusto (Azure Data Explorer) integration for Codex CLI.
//!
//! This module provides functionality to interact with Azure Data Explorer (Kusto),
//! allowing users to run queries and manage data.

pub mod auth;
pub mod client;
pub mod models;
pub mod tool_handler;
pub mod tools;
pub mod tools_impl;

// Re-export key components for easier access
pub use auth::KustoAuth;
pub use client::KustoClient;
pub use integration::register_kusto_tools_with_openai;
pub use models::*;
pub use tool_handler::handle_kusto_tool_call;
pub use tools::register_kusto_tools;
pub use tools_impl::KustoTools;

// Integration module
pub mod integration {
    use crate::error::Result;
    use crate::config_types::KustoConfig;
    use crate::openai_tools::OpenAiTool;
    use super::tools::create_kusto_tools;

    /// Register Kusto tools with OpenAI
    pub fn register_kusto_tools_with_openai(config: &KustoConfig) -> Result<Vec<OpenAiTool>> {
        let tools = create_kusto_tools();
        Ok(tools)
    }
}