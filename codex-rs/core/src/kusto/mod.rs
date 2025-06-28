//! Kusto (Azure Data Explorer) integration for Codex CLI.
//!
//! This module provides functionality to interact with Azure Data Explorer (Kusto),
//! allowing users to run queries and manage data.

pub mod auth;
pub mod client;
pub mod client_rest;
pub mod client_sdk;
pub mod diagnostics;
pub mod knowledge_base;
pub mod models;
pub mod tool_handler;
pub mod tools;
pub mod tools_impl;

// Re-export key components for easier access
pub use auth::KustoAuth;
pub use client::KustoClient;
pub use client_rest::KustoRestClient;
pub use client_sdk::KustoSdkClient;
pub use integration::register_kusto_tools_with_openai;
pub use knowledge_base::*;
pub use models::*;
pub use tool_handler::handle_kusto_tool_call;
pub use tools::create_kusto_tools;
pub use tools_impl::KustoTools;

// Integration module
pub mod integration {
    use crate::config_types::KustoConfig;
    use crate::openai_tools::OpenAiTool;
    use super::tools::create_kusto_tools;

    /// Register Kusto tools with OpenAI if Kusto is configured
    pub fn register_kusto_tools_with_openai(config: &Option<KustoConfig>) -> Option<Vec<OpenAiTool>> {
        if let Some(kusto_config) = config {
            // Check if Kusto is properly configured
            if !kusto_config.cluster_url.is_empty() && 
               (!kusto_config.database.is_empty() || !kusto_config.databases.is_empty()) {
                let tools = create_kusto_tools();
                return Some(tools);
            }
        }
        None
    }
}