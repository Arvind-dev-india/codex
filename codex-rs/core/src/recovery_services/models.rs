//! Data models for Recovery Services operations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported workload types for Recovery Services (2025-02-01 API)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkloadType {
    #[serde(rename = "Invalid")]
    Invalid,
    #[serde(rename = "VM")]
    VM,
    #[serde(rename = "FileFolder")]
    FileFolder,
    #[serde(rename = "AzureSqlDb")]
    AzureSqlDb,
    #[serde(rename = "SQLDB")]
    SqlDb,
    #[serde(rename = "Exchange")]
    Exchange,
    #[serde(rename = "Sharepoint")]
    Sharepoint,
    #[serde(rename = "VMwareVM")]
    VMwareVM,
    #[serde(rename = "SystemState")]
    SystemState,
    #[serde(rename = "Client")]
    Client,
    #[serde(rename = "GenericDataSource")]
    GenericDataSource,
    #[serde(rename = "SQLDataBase")]
    SqlDatabase,
    #[serde(rename = "AzureFileShare")]
    AzureFileShare,
    #[serde(rename = "SAPHanaDatabase")]
    SapHanaDatabase,
    #[serde(rename = "SAPAseDatabase")]
    SapAseDatabase,
    #[serde(rename = "SAPHanaDBInstance")]
    SapHanaDbInstance,
    #[serde(rename = "AnyDatabase")]
    AnyDatabase,
}

impl std::fmt::Display for WorkloadType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkloadType::Invalid => write!(f, "Invalid"),
            WorkloadType::VM => write!(f, "VM"),
            WorkloadType::FileFolder => write!(f, "FileFolder"),
            WorkloadType::AzureSqlDb => write!(f, "AzureSqlDb"),
            WorkloadType::SqlDb => write!(f, "SQLDB"),
            WorkloadType::Exchange => write!(f, "Exchange"),
            WorkloadType::Sharepoint => write!(f, "Sharepoint"),
            WorkloadType::VMwareVM => write!(f, "VMwareVM"),
            WorkloadType::SystemState => write!(f, "SystemState"),
            WorkloadType::Client => write!(f, "Client"),
            WorkloadType::GenericDataSource => write!(f, "GenericDataSource"),
            WorkloadType::SqlDatabase => write!(f, "SQLDataBase"),
            WorkloadType::AzureFileShare => write!(f, "AzureFileShare"),
            WorkloadType::SapHanaDatabase => write!(f, "SAPHanaDatabase"),
            WorkloadType::SapAseDatabase => write!(f, "SAPAseDatabase"),
            WorkloadType::SapHanaDbInstance => write!(f, "SAPHanaDBInstance"),
            WorkloadType::AnyDatabase => write!(f, "AnyDatabase"),
        }
    }
}

/// Backup management types (2025-02-01 API)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BackupManagementType {
    #[serde(rename = "Invalid")]
    Invalid,
    #[serde(rename = "AzureIaasVM")]
    AzureIaasVM,
    #[serde(rename = "MAB")]
    MAB,
    #[serde(rename = "DPM")]
    DPM,
    #[serde(rename = "AzureSql")]
    AzureSql,
    #[serde(rename = "AzureBackupServer")]
    AzureBackupServer,
    #[serde(rename = "AzureWorkload")]
    AzureWorkload,
    #[serde(rename = "AzureStorage")]
    AzureStorage,
    #[serde(rename = "DefaultBackup")]
    DefaultBackup,
}

impl std::fmt::Display for BackupManagementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupManagementType::Invalid => write!(f, "Invalid"),
            BackupManagementType::AzureIaasVM => write!(f, "AzureIaasVM"),
            BackupManagementType::MAB => write!(f, "MAB"),
            BackupManagementType::DPM => write!(f, "DPM"),
            BackupManagementType::AzureSql => write!(f, "AzureSql"),
            BackupManagementType::AzureBackupServer => write!(f, "AzureBackupServer"),
            BackupManagementType::AzureWorkload => write!(f, "AzureWorkload"),
            BackupManagementType::AzureStorage => write!(f, "AzureStorage"),
            BackupManagementType::DefaultBackup => write!(f, "DefaultBackup"),
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

/// Policy types for workload backup policies (2025-02-01 API)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PolicyType {
    #[serde(rename = "Full")]
    Full,
    #[serde(rename = "Incremental")]
    Incremental,
    #[serde(rename = "Differential")]
    Differential,
    #[serde(rename = "Log")]
    Log,
}

impl std::fmt::Display for PolicyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PolicyType::Full => write!(f, "Full"),
            PolicyType::Incremental => write!(f, "Incremental"),
            PolicyType::Differential => write!(f, "Differential"),
            PolicyType::Log => write!(f, "Log"),
        }
    }
}

/// Sub-protection policy for workload backup policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubProtectionPolicy {
    #[serde(rename = "policyType")]
    pub policy_type: PolicyType,
    #[serde(rename = "schedulePolicy")]
    pub schedule_policy: SchedulePolicy,
    #[serde(rename = "retentionPolicy")]
    pub retention_policy: RetentionPolicy,
}

/// Schedule policy for backup policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulePolicy {
    #[serde(rename = "schedulePolicyType")]
    pub schedule_policy_type: String,
    #[serde(rename = "scheduleRunFrequency")]
    pub schedule_run_frequency: Option<String>,
    #[serde(rename = "scheduleRunTimes")]
    pub schedule_run_times: Option<Vec<String>>,
    #[serde(rename = "scheduleRunDays")]
    pub schedule_run_days: Option<Vec<String>>,
    #[serde(rename = "scheduleFrequencyInMins")]
    pub schedule_frequency_in_mins: Option<i32>,
}

/// Retention policy for backup policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    #[serde(rename = "retentionPolicyType")]
    pub retention_policy_type: String,
    #[serde(rename = "retentionDuration")]
    pub retention_duration: Option<RetentionDuration>,
    #[serde(rename = "dailySchedule")]
    pub daily_schedule: Option<DailyRetentionSchedule>,
    #[serde(rename = "weeklySchedule")]
    pub weekly_schedule: Option<WeeklyRetentionSchedule>,
    #[serde(rename = "monthlySchedule")]
    pub monthly_schedule: Option<MonthlyRetentionSchedule>,
}

/// Retention duration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionDuration {
    pub count: i32,
    #[serde(rename = "durationType")]
    pub duration_type: String, // Days, Weeks, Months, Years
}

/// Daily retention schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyRetentionSchedule {
    #[serde(rename = "retentionTimes")]
    pub retention_times: Vec<String>,
    #[serde(rename = "retentionDuration")]
    pub retention_duration: RetentionDuration,
}

/// Weekly retention schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyRetentionSchedule {
    #[serde(rename = "daysOfTheWeek")]
    pub days_of_the_week: Vec<String>,
    #[serde(rename = "retentionTimes")]
    pub retention_times: Vec<String>,
    #[serde(rename = "retentionDuration")]
    pub retention_duration: RetentionDuration,
}

/// Monthly retention schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyRetentionSchedule {
    #[serde(rename = "retentionScheduleFormatType")]
    pub retention_schedule_format_type: String, // Daily, Weekly
    #[serde(rename = "retentionScheduleDaily")]
    pub retention_schedule_daily: Option<DailyRetentionFormat>,
    #[serde(rename = "retentionScheduleWeekly")]
    pub retention_schedule_weekly: Option<WeeklyRetentionFormat>,
    #[serde(rename = "retentionTimes")]
    pub retention_times: Vec<String>,
    #[serde(rename = "retentionDuration")]
    pub retention_duration: RetentionDuration,
}

/// Daily retention format for monthly schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyRetentionFormat {
    #[serde(rename = "daysOfTheMonth")]
    pub days_of_the_month: Vec<Day>,
}

/// Weekly retention format for monthly schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyRetentionFormat {
    #[serde(rename = "daysOfTheWeek")]
    pub days_of_the_week: Vec<String>,
    #[serde(rename = "weeksOfTheMonth")]
    pub weeks_of_the_month: Vec<String>,
}

/// Day representation for retention schedules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Day {
    pub date: i32,
    #[serde(rename = "isLast")]
    pub is_last: bool,
}

/// Workload backup policy settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadSettings {
    #[serde(rename = "timeZone")]
    pub time_zone: Option<String>,
    #[serde(rename = "issqlcompression")]
    pub is_sql_compression: Option<bool>,
    #[serde(rename = "isCompression")]
    pub is_compression: Option<bool>,
}

/// Enhanced backup policy properties for 2025-02-01 API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedBackupPolicyProperties {
    #[serde(rename = "backupManagementType")]
    pub backup_management_type: BackupManagementType,
    #[serde(rename = "workLoadType")]
    pub workload_type: WorkloadType,
    pub settings: Option<WorkloadSettings>,
    #[serde(rename = "subProtectionPolicy")]
    pub sub_protection_policy: Option<Vec<SubProtectionPolicy>>,
    #[serde(rename = "protectedItemsCount")]
    pub protected_items_count: Option<i32>,
}

/// Workload item types for different workloads
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkloadItemType {
    #[serde(rename = "SQLInstance")]
    SqlInstance,
    #[serde(rename = "SQLDataBase")]
    SqlDatabase,
    #[serde(rename = "SAPHanaSystem")]
    SapHanaSystem,
    #[serde(rename = "SAPHanaDatabase")]
    SapHanaDatabase,
    #[serde(rename = "SAPAseSystem")]
    SapAseSystem,
    #[serde(rename = "SAPAseDatabase")]
    SapAseDatabase,
}

impl std::fmt::Display for WorkloadItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkloadItemType::SqlInstance => write!(f, "SQLInstance"),
            WorkloadItemType::SqlDatabase => write!(f, "SQLDataBase"),
            WorkloadItemType::SapHanaSystem => write!(f, "SAPHanaSystem"),
            WorkloadItemType::SapHanaDatabase => write!(f, "SAPHanaDatabase"),
            WorkloadItemType::SapAseSystem => write!(f, "SAPAseSystem"),
            WorkloadItemType::SapAseDatabase => write!(f, "SAPAseDatabase"),
        }
    }
}