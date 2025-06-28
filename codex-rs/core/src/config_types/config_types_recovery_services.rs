//! Configuration types for Recovery Services (Azure Backup) integration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for Recovery Services (Azure Backup) integration
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RecoveryServicesConfig {
    /// Enable/disable Recovery Services tools (default: true)
    pub enabled: Option<bool>,
    
    /// Primary Azure subscription ID
    pub subscription_id: String,
    
    /// Primary resource group name
    pub resource_group: String,
    
    /// Primary Recovery Services vault name
    pub vault_name: String,
    
    /// Optional: Multiple vaults configuration
    #[serde(default)]
    pub vaults: HashMap<String, RecoveryServicesVaultConfig>,
}

/// Configuration for individual Recovery Services vaults
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecoveryServicesVaultConfig {
    /// Vault name
    pub name: String,
    
    /// Subscription ID (if different from primary)
    pub subscription_id: Option<String>,
    
    /// Resource group (if different from primary)
    pub resource_group: Option<String>,
    
    /// Optional description
    pub description: Option<String>,
    
    /// Whether this is the default vault
    #[serde(default)]
    pub is_default: bool,
}