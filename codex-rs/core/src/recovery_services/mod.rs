//! Recovery Services (Azure Backup) integration for Codex CLI.
//!
//! This module provides functionality to interact with Azure Recovery Services,
//! allowing users to manage backup and recovery operations for SAP HANA and SQL Server workloads.

pub mod auth;
pub mod client;
pub mod models;
pub mod tool_handler;
pub mod tools;
pub mod tools_impl;

// Re-export key components for easier access
pub use auth::RecoveryServicesAuth;
pub use client::RecoveryServicesClient;
pub use integration::register_recovery_services_tools_with_openai;
pub use models::*;
pub use tool_handler::handle_recovery_services_tool_call;
pub use tools::create_recovery_services_tools;
pub use tools_impl::RecoveryServicesTools;

// Integration module
pub mod integration {
    use crate::config_types::RecoveryServicesConfig;
    use crate::openai_tools::OpenAiTool;
    use super::tools::create_recovery_services_tools;

    /// Register Recovery Services tools with OpenAI if Recovery Services is configured and enabled
    pub fn register_recovery_services_tools_with_openai(config: &Option<RecoveryServicesConfig>) -> Option<Vec<OpenAiTool>> {
        if let Some(recovery_services_config) = config {
            // Check if Recovery Services is enabled (default: true)
            if recovery_services_config.enabled.unwrap_or(true) {
                // Check if Recovery Services is properly configured
                // The actual structure has enabled, subscription_id, resource_group, vault_name, and vaults
                if !recovery_services_config.subscription_id.is_empty() && 
                   !recovery_services_config.resource_group.is_empty() &&
                   !recovery_services_config.vault_name.is_empty() {
                    let tools = create_recovery_services_tools();
                    return Some(tools);
                }
            }
        }
        None
    }
}