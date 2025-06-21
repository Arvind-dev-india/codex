//! Azure DevOps integration for Codex CLI.
//!
//! This module provides functionality to interact with Azure DevOps services,
//! including work items, pull requests, repositories, and wikis.

pub mod auth;
pub mod client;
pub mod models;
pub mod tools;

// Re-export key components for easier access
pub use auth::AzureDevOpsAuth;
pub use client::AzureDevOpsClient;
pub use models::*;
pub use tools::register_azure_devops_tools;