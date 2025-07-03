//! Data models for Recovery Services operations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported workload types for Recovery Services
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkloadType {
    #[serde(rename = "SAPHANA")]
    SapHana,
    #[serde(rename = "SQLDataBase")]
    SqlServer,
    #[serde(rename = "VM")]
    VM,
}

impl std::fmt::Display for WorkloadType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkloadType::SapHana => write!(f, "SAPHANA"),
            WorkloadType::SqlServer => write!(f, "SQLDataBase"),
            WorkloadType::VM => write!(f, "VM"),
        }
    }
}

/// Backup management types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BackupManagementType {
    #[serde(rename = "AzureWorkload")]
    AzureWorkload,
}

impl std::fmt::Display for BackupManagementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupManagementType::AzureWorkload => write!(f, "AzureWorkload"),
        }
    }
}

/// Recovery Services vault information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultInfo {
    pub id: String,
    pub name: String,
    pub resource_group: String,
    pub subscription_id: String,
    pub location: String,
    pub properties: Option<VaultProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultProperties {
    #[serde(rename = "provisioningState")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "upgradeDetails")]
    pub upgrade_details: Option<serde_json::Value>,
}

/// Protectable item (database that can be protected)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectableItem {
    pub id: String,
    pub name: String,
    pub properties: ProtectableItemProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectableItemProperties {
    #[serde(rename = "friendlyName")]
    pub friendly_name: String,
    #[serde(rename = "serverName")]
    pub server_name: String,
    #[serde(rename = "parentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "protectableItemType")]
    pub protectable_item_type: String,
    #[serde(rename = "protectionState")]
    pub protection_state: String,
    #[serde(rename = "workloadType")]
    pub workload_type: String,
}

/// Protected item (database that is currently protected)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedItem {
    pub id: String,
    pub name: String,
    pub properties: ProtectedItemProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedItemProperties {
    #[serde(rename = "friendlyName")]
    pub friendly_name: String,
    #[serde(rename = "serverName")]
    pub server_name: String,
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[serde(rename = "workloadType")]
    pub workload_type: String,
    #[serde(rename = "protectionState")]
    pub protection_state: String,
    #[serde(rename = "healthStatus")]
    pub health_status: Option<String>,
    #[serde(rename = "lastBackupStatus")]
    pub last_backup_status: Option<String>,
    #[serde(rename = "lastBackupTime")]
    pub last_backup_time: Option<String>,
    #[serde(rename = "policyId")]
    pub policy_id: Option<String>,
}

/// Backup job information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupJob {
    pub id: String,
    pub name: String,
    pub properties: BackupJobProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupJobProperties {
    #[serde(rename = "jobType")]
    pub job_type: String,
    pub operation: String,
    pub status: String,
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    pub duration: Option<String>,
    #[serde(rename = "entityFriendlyName")]
    pub entity_friendly_name: String,
    #[serde(rename = "backupManagementType")]
    pub backup_management_type: String,
    #[serde(rename = "workloadType")]
    pub workload_type: String,
    #[serde(rename = "errorDetails")]
    pub error_details: Option<Vec<serde_json::Value>>,
}

/// Recovery point information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryPoint {
    pub id: String,
    pub name: String,
    pub properties: RecoveryPointProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryPointProperties {
    #[serde(rename = "recoveryPointTimeInUtc")]
    pub recovery_point_time_in_utc: Option<String>,
    #[serde(rename = "type")]
    pub recovery_point_type: Option<String>,
    #[serde(rename = "recoveryPointTierDetails")]
    pub recovery_point_tier_details: Option<Vec<RecoveryPointTierDetail>>,
    #[serde(rename = "recoveryPointProperties")]
    pub recovery_point_properties: Option<RecoveryPointPropertiesDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryPointTierDetail {
    #[serde(rename = "type")]
    pub tier_type: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryPointPropertiesDetail {
    #[serde(rename = "ruleName")]
    pub rule_name: Option<String>,
    #[serde(rename = "expiryTime")]
    pub expiry_time: Option<String>,
}

/// Backup policy information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupPolicy {
    pub id: String,
    pub name: String,
    pub properties: BackupPolicyProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupPolicyProperties {
    #[serde(rename = "backupManagementType")]
    pub backup_management_type: String,
    #[serde(rename = "workLoadType")]
    pub workload_type: String,
    pub settings: Option<serde_json::Value>,
    #[serde(rename = "subProtectionPolicy")]
    pub sub_protection_policy: Option<Vec<serde_json::Value>>,
}

/// Container (VM) registration information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupContainer {
    pub id: String,
    pub name: String,
    pub properties: BackupContainerProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupContainerProperties {
    #[serde(rename = "friendlyName")]
    pub friendly_name: String,
    #[serde(rename = "backupManagementType")]
    pub backup_management_type: String,
    #[serde(rename = "registrationStatus")]
    pub registration_status: String,
    #[serde(rename = "healthStatus")]
    pub health_status: Option<String>,
    #[serde(rename = "containerType")]
    pub container_type: String,
}

/// Restore operation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreRequest {
    pub properties: RestoreRequestProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreRequestProperties {
    #[serde(rename = "objectType")]
    pub object_type: String,
    #[serde(rename = "recoveryType")]
    pub recovery_type: String,
    #[serde(rename = "sourceResourceId")]
    pub source_resource_id: Option<String>,
    #[serde(rename = "propertyBag")]
    pub property_bag: Option<HashMap<String, String>>,
    #[serde(rename = "targetInfo")]
    pub target_info: Option<TargetRestoreInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetRestoreInfo {
    #[serde(rename = "overwriteOption")]
    pub overwrite_option: String,
    #[serde(rename = "containerId")]
    pub container_id: String,
    #[serde(rename = "databaseName")]
    pub database_name: Option<String>,
    #[serde(rename = "targetDirectoryForFileRestore")]
    pub target_directory_for_file_restore: Option<String>,
}

/// Backup trigger request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupRequest {
    pub properties: BackupRequestProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupRequestProperties {
    #[serde(rename = "objectType")]
    pub object_type: String,
    #[serde(rename = "backupType")]
    pub backup_type: String,
    #[serde(rename = "enableCompression")]
    pub enable_compression: Option<bool>,
    #[serde(rename = "copyOnly")]
    pub copy_only: Option<bool>,
}

/// Protection enable request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectionRequest {
    pub properties: ProtectionRequestProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectionRequestProperties {
    #[serde(rename = "protectedItemType")]
    pub protected_item_type: String,
    #[serde(rename = "policyId")]
    pub policy_id: String,
    #[serde(rename = "workloadType")]
    pub workload_type: String,
    #[serde(rename = "sourceResourceId")]
    pub source_resource_id: Option<String>,
}

/// API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub value: Vec<T>,
    #[serde(rename = "nextLink")]
    pub next_link: Option<String>,
}

/// Error response from Azure API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    pub details: Option<Vec<ErrorDetail>>,
}