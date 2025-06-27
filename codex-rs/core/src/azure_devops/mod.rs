//! Azure DevOps integration for Codex CLI.
//!
//! This module provides functionality to interact with Azure DevOps services,
//! including work items, pull requests, repositories, and wikis.

pub mod auth;
pub mod client;
pub mod integration;
pub mod models;
pub mod tool_handler;
pub mod tools;
pub mod tools_impl;

// Re-export key components for easier access
pub use auth::AzureDevOpsAuth;
pub use client::AzureDevOpsClient;
pub use integration::register_azure_devops_tools_with_openai;
pub use models::*;
pub use tool_handler::handle_azure_devops_tool_call;
pub use tools::register_azure_devops_tools;
pub use tools_impl::AzureDevOpsTools;