# Recovery Services and Backup REST API Reference

This document provides a comprehensive reference for the Azure Recovery Services and Backup REST APIs based on the official Microsoft documentation.

## API Documentation Sources

- [Recovery Services API](https://learn.microsoft.com/en-us/rest/api/recoveryservices/operation-groups?view=rest-recoveryservices-2025-02-01)
- [Backup API](https://learn.microsoft.com/en-us/rest/api/backup/operation-groups?view=rest-backup-2025-02-01)
- [Recovery Services Vaults API](https://learn.microsoft.com/en-us/rest/api/recoveryservices/vaults)
- [Backup Protected Items API](https://learn.microsoft.com/en-us/rest/api/backup/backup-protected-items)
- [Backup Jobs API](https://learn.microsoft.com/en-us/rest/api/backup/backup-jobs)
- [Backup Policies API](https://learn.microsoft.com/en-us/rest/api/backup/backup-policies)
- [Recovery Points API](https://learn.microsoft.com/en-us/rest/api/backup/recovery-points)
- [Protection Containers API](https://learn.microsoft.com/en-us/rest/api/backup/protection-containers)
- [Protectable Items API](https://learn.microsoft.com/en-us/rest/api/backup/protectable-items)

## API Versions

- **Recovery Services API**: `2023-04-01` (current stable)
- **Backup API**: `2023-04-01` (current stable)
- **Preview API**: `2025-02-01` (preview version)

## Base URLs

- **Management API**: `https://management.azure.com`
- **Resource Provider**: `Microsoft.RecoveryServices`

## Authentication

All APIs require Azure Active Directory (AAD) authentication with appropriate RBAC permissions:
- **Reader**: View operations
- **Contributor**: Full access
- **Backup Contributor**: Backup-specific operations
- **Backup Operator**: Restore operations

## Recovery Services API Endpoints

### 1. Vaults Operations

#### List Vaults by Subscription
```http
GET https://management.azure.com/subscriptions/{subscriptionId}/providers/Microsoft.RecoveryServices/vaults?api-version=2023-04-01
```

Example curl:
```bash
curl -X GET \
  "https://management.azure.com/subscriptions/{subscriptionId}/providers/Microsoft.RecoveryServices/vaults?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

Response:
```json
{
  "value": [
    {
      "id": "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}",
      "name": "{vaultName}",
      "type": "Microsoft.RecoveryServices/vaults",
      "location": "westus",
      "properties": {
        "provisioningState": "Succeeded"
      }
    }
  ]
}
```

#### List Vaults by Resource Group
```http
GET https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults?api-version=2023-04-01
```

Example curl:
```bash
curl -X GET \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

#### Get Vault
```http
GET https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}?api-version=2023-04-01
```

Example curl:
```bash
curl -X GET \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

#### Create or Update Vault
```http
PUT https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}?api-version=2023-04-01
```

Example curl:
```bash
curl -X PUT \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "location": "westus",
    "properties": {},
    "sku": {
      "name": "Standard"
    }
  }'
```

#### Delete Vault
```http
DELETE https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}?api-version=2023-04-01
```

Example curl:
```bash
curl -X DELETE \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

### 2. Vault Properties

#### Get Vault Properties
```http
GET https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupconfig?api-version=2023-04-01
```

Example curl:
```bash
curl -X GET \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupconfig?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

Response:
```json
{
  "id": "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupconfig/vaultconfig",
  "name": "vaultconfig",
  "type": "Microsoft.RecoveryServices/vaults/backupconfig",
  "properties": {
    "enhancedSecurityState": "Enabled",
    "softDeleteFeatureState": "Enabled",
    "storageModelType": "GeoRedundant",
    "storageType": "GeoRedundant",
    "storageTypeState": "Locked",
    "crossRegionRestoreFlag": false,
    "isSiteRecovery": false
  }
}
```

#### Update Vault Properties
```http
PATCH https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupconfig?api-version=2023-04-01
```

Example curl:
```bash
curl -X PATCH \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupconfig?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "properties": {
      "enhancedSecurityState": "Enabled",
      "softDeleteFeatureState": "Enabled"
    }
  }'
```

### 3. Vault Storage Configuration

#### Get Storage Configuration
```http
GET https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupstorageconfig/vaultstorageconfig?api-version=2023-04-01
```

Example curl:
```bash
curl -X GET \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupstorageconfig/vaultstorageconfig?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

Response:
```json
{
  "id": "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupstorageconfig/vaultstorageconfig",
  "name": "vaultstorageconfig",
  "type": "Microsoft.RecoveryServices/vaults/backupstorageconfig",
  "properties": {
    "storageModelType": "GeoRedundant",
    "storageType": "GeoRedundant",
    "storageTypeState": "Locked",
    "crossRegionRestoreFlag": false
  }
}
```

#### Update Storage Configuration
```http
PATCH https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupstorageconfig/vaultstorageconfig?api-version=2023-04-01
```

Example curl:
```bash
curl -X PATCH \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupstorageconfig/vaultstorageconfig?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "properties": {
      "storageModelType": "GeoRedundant",
      "crossRegionRestoreFlag": true
    }
  }'
```

## Backup API Endpoints

### 1. Protection Containers

#### List Protection Containers
```http
GET https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers?api-version=2023-04-01
```

Example curl:
```bash
curl -X GET \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

Response:
```json
{
  "value": [
    {
      "id": "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}",
      "name": "iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}",
      "type": "Microsoft.RecoveryServices/vaults/backupFabrics/protectionContainers",
      "properties": {
        "friendlyName": "{vmName}",
        "backupManagementType": "AzureIaasVM",
        "registrationStatus": "Registered",
        "healthStatus": "Healthy",
        "containerType": "AzureIaasVMContainer"
      }
    }
  ]
}
```

#### Get Protection Container
```http
GET https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}?api-version=2023-04-01
```

Example curl:
```bash
curl -X GET \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

#### Register Protection Container (VM)
```http
PUT https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}?api-version=2023-04-01
```

Example curl:
```bash
curl -X PUT \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "properties": {
      "containerType": "VMAppContainer",
      "sourceResourceId": "/subscriptions/{subscriptionId}/resourceGroups/{vmResourceGroup}/providers/Microsoft.Compute/virtualMachines/{vmName}",
      "virtualMachineId": "/subscriptions/{subscriptionId}/resourceGroups/{vmResourceGroup}/providers/Microsoft.Compute/virtualMachines/{vmName}",
      "virtualMachineVersion": "Compute"
    }
  }'
```

#### Unregister Protection Container
```http
DELETE https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}?api-version=2023-04-01
```

Example curl:
```bash
curl -X DELETE \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

#### Container Naming Conventions

For Azure VMs, the container name follows this pattern:
```
iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}
```

For SQL databases in Azure VMs:
```
iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}
```

For Azure Storage accounts:
```
storagecontainer;storage;{storageResourceGroup};{storageAccountName}
```

### 2. Protectable Items

#### List Protectable Items
```http
GET https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupProtectableItems?api-version=2023-04-01
```

Example curl:
```bash
curl -X GET \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupProtectableItems?api-version=2023-04-01&$filter=backupManagementType eq 'AzureIaasVM'" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

Response:
```json
{
  "value": [
    {
      "id": "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupProtectableItems/{protectableItemId}",
      "name": "{protectableItemId}",
      "type": "Microsoft.RecoveryServices/vaults/backupProtectableItems",
      "properties": {
        "friendlyName": "{vmName}",
        "protectableItemType": "Microsoft.Compute/virtualMachines",
        "backupManagementType": "AzureIaasVM",
        "workloadType": "VM",
        "virtualMachineId": "/subscriptions/{subscriptionId}/resourceGroups/{vmResourceGroup}/providers/Microsoft.Compute/virtualMachines/{vmName}",
        "protectionState": "NotProtected"
      }
    }
  ]
}
```

**Query Parameters:**
- `$filter`: Filter expression (e.g., `backupManagementType eq 'AzureIaasVM'`)
- `$skipToken`: Pagination token for large result sets

#### Get Protectable Item
```http
GET https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupProtectableItems/{protectableItemId}?api-version=2023-04-01
```

Example curl:
```bash
curl -X GET \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupProtectableItems/{protectableItemId}?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

### 3. Protected Items

#### List Protected Items
```http
GET https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupProtectedItems?api-version=2023-04-01
```

Example curl:
```bash
curl -X GET \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupProtectedItems?api-version=2023-04-01&$filter=backupManagementType eq 'AzureIaasVM'" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

Response:
```json
{
  "value": [
    {
      "id": "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}/protectedItems/vm;iaasvmcontainerv2;{vmResourceGroup};{vmName}",
      "name": "vm;iaasvmcontainerv2;{vmResourceGroup};{vmName}",
      "type": "Microsoft.RecoveryServices/vaults/backupFabrics/protectionContainers/protectedItems",
      "properties": {
        "friendlyName": "{vmName}",
        "virtualMachineId": "/subscriptions/{subscriptionId}/resourceGroups/{vmResourceGroup}/providers/Microsoft.Compute/virtualMachines/{vmName}",
        "protectionStatus": "Healthy",
        "protectionState": "Protected",
        "lastBackupStatus": "Completed",
        "lastBackupTime": "2023-05-01T12:00:00Z",
        "protectedItemType": "Microsoft.Compute/virtualMachines",
        "backupManagementType": "AzureIaasVM",
        "workloadType": "VM",
        "containerName": "iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}",
        "sourceResourceId": "/subscriptions/{subscriptionId}/resourceGroups/{vmResourceGroup}/providers/Microsoft.Compute/virtualMachines/{vmName}",
        "policyId": "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupPolicies/{policyName}"
      }
    }
  ]
}
```

#### Get Protected Item
```http
GET https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}/protectedItems/{protectedItemName}?api-version=2023-04-01
```

Example curl:
```bash
curl -X GET \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}/protectedItems/vm;iaasvmcontainerv2;{vmResourceGroup};{vmName}?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

#### Enable Protection (Create Protected Item)
```http
PUT https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}/protectedItems/{protectedItemName}?api-version=2023-04-01
```

Example curl for VM protection:
```bash
curl -X PUT \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}/protectedItems/vm;iaasvmcontainerv2;{vmResourceGroup};{vmName}?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "properties": {
      "protectedItemType": "Microsoft.Compute/virtualMachines",
      "sourceResourceId": "/subscriptions/{subscriptionId}/resourceGroups/{vmResourceGroup}/providers/Microsoft.Compute/virtualMachines/{vmName}",
      "policyId": "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupPolicies/{policyName}",
      "protectionState": "ProtectionEnabled"
    }
  }'
```

#### Disable Protection (Delete Protected Item)
```http
DELETE https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}/protectedItems/{protectedItemName}?api-version=2023-04-01
```

Example curl:
```bash
curl -X DELETE \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}/protectedItems/vm;iaasvmcontainerv2;{vmResourceGroup};{vmName}?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

#### Protected Item Naming Conventions

For Azure VMs:
```
vm;iaasvmcontainerv2;{vmResourceGroup};{vmName}
```

For SQL databases in Azure VMs:
```
AzureBackupServerContainer;mssql;{vmResourceGroup};{vmName}
```

### 4. Backup Policies

#### List Backup Policies
```http
GET https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupPolicies?api-version=2023-04-01
```

Example curl:
```bash
curl -X GET \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupPolicies?api-version=2023-04-01&$filter=backupManagementType eq 'AzureIaasVM'" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

Response:
```json
{
  "value": [
    {
      "id": "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupPolicies/{policyName}",
      "name": "{policyName}",
      "type": "Microsoft.RecoveryServices/vaults/backupPolicies",
      "properties": {
        "backupManagementType": "AzureIaasVM",
        "schedulePolicy": {
          "schedulePolicyType": "SimpleSchedulePolicy",
          "scheduleRunFrequency": "Daily",
          "scheduleRunTimes": [
            "2023-05-01T01:00:00Z"
          ],
          "scheduleWeeklyFrequency": 0
        },
        "retentionPolicy": {
          "retentionPolicyType": "LongTermRetentionPolicy",
          "dailySchedule": {
            "retentionTimes": [
              "2023-05-01T01:00:00Z"
            ],
            "retentionDuration": {
              "count": 30,
              "durationType": "Days"
            }
          }
        },
        "timeZone": "UTC",
        "protectedItemsCount": 2
      }
    }
  ]
}
```

#### Get Backup Policy
```http
GET https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupPolicies/{policyName}?api-version=2023-04-01
```

Example curl:
```bash
curl -X GET \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupPolicies/{policyName}?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

#### Create or Update Backup Policy
```http
PUT https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupPolicies/{policyName}?api-version=2023-04-01
```

Example curl for VM backup policy:
```bash
curl -X PUT \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupPolicies/{policyName}?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "properties": {
      "backupManagementType": "AzureIaasVM",
      "schedulePolicy": {
        "schedulePolicyType": "SimpleSchedulePolicy",
        "scheduleRunFrequency": "Daily",
        "scheduleRunTimes": [
          "2023-05-01T01:00:00Z"
        ],
        "scheduleWeeklyFrequency": 0
      },
      "retentionPolicy": {
        "retentionPolicyType": "LongTermRetentionPolicy",
        "dailySchedule": {
          "retentionTimes": [
            "2023-05-01T01:00:00Z"
          ],
          "retentionDuration": {
            "count": 30,
            "durationType": "Days"
          }
        }
      },
      "timeZone": "UTC"
    }
  }'
```

#### Delete Backup Policy
```http
DELETE https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupPolicies/{policyName}?api-version=2023-04-01
```

Example curl:
```bash
curl -X DELETE \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupPolicies/{policyName}?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

#### Policy Types

Different workload types support different policy configurations:

1. **Azure VM Backup Policy**:
   - Daily or weekly schedule
   - Retention: daily, weekly, monthly, yearly

2. **SQL Database Backup Policy**:
   - Full backup: daily
   - Differential backup: days of week
   - Log backup: every 15/30/60 minutes
   - Retention: days, weeks, months, years

3. **Azure Files Backup Policy**:
   - Daily schedule
   - Retention: daily, weekly, monthly, yearly

### 5. Backup Jobs

#### List Backup Jobs
```http
GET https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupJobs?api-version=2023-04-01
```

Example curl:
```bash
curl -X GET \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupJobs?api-version=2023-04-01&$filter=status eq 'InProgress'" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

Response:
```json
{
  "value": [
    {
      "id": "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupJobs/{jobId}",
      "name": "{jobId}",
      "type": "Microsoft.RecoveryServices/vaults/backupJobs",
      "properties": {
        "jobType": "AzureIaasVMJob",
        "entityFriendlyName": "{vmName}",
        "operation": "Backup",
        "status": "InProgress",
        "startTime": "2023-05-01T01:00:00Z",
        "activityId": "12345678-1234-1234-1234-123456789012",
        "backupManagementType": "AzureIaasVM"
      }
    }
  ]
}
```

**Query Parameters:**
- `$filter`: Filter expression (e.g., `status eq 'InProgress'`)
- `$skipToken`: Pagination token

**Common Filter Values:**
- Status: `InProgress`, `Completed`, `Failed`, `Cancelled`
- Operation: `Backup`, `Restore`, `ConfigureBackup`, `DisableBackup`

#### Get Backup Job
```http
GET https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupJobs/{jobId}?api-version=2023-04-01
```

Example curl:
```bash
curl -X GET \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupJobs/{jobId}?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

Response:
```json
{
  "id": "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupJobs/{jobId}",
  "name": "{jobId}",
  "type": "Microsoft.RecoveryServices/vaults/backupJobs",
  "properties": {
    "jobType": "AzureIaasVMJob",
    "entityFriendlyName": "{vmName}",
    "operation": "Backup",
    "status": "InProgress",
    "startTime": "2023-05-01T01:00:00Z",
    "duration": "PT1H30M",
    "activityId": "12345678-1234-1234-1234-123456789012",
    "backupManagementType": "AzureIaasVM",
    "extendedInfo": {
      "tasksList": [
        {
          "taskId": "TakeSnapshotTask",
          "status": "Completed",
          "startTime": "2023-05-01T01:00:00Z",
          "endTime": "2023-05-01T01:30:00Z"
        },
        {
          "taskId": "TransferDataToVaultTask",
          "status": "InProgress",
          "startTime": "2023-05-01T01:30:00Z"
        }
      ],
      "propertyBag": {
        "VM Size": "Standard_DS2_v2",
        "OS Type": "Windows"
      }
    }
  }
}
```

#### Cancel Backup Job
```http
POST https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupJobs/{jobId}/cancel?api-version=2023-04-01
```

Example curl:
```bash
curl -X POST \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupJobs/{jobId}/cancel?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

Response:
```json
{
  "id": "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupJobs/{jobId}",
  "name": "{jobId}",
  "type": "Microsoft.RecoveryServices/vaults/backupJobs",
  "properties": {
    "jobType": "AzureIaasVMJob",
    "operation": "Backup",
    "status": "Cancelling",
    "startTime": "2023-05-01T01:00:00Z"
  }
}
```

### 6. Backup Operations

#### Trigger Backup
```http
POST https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}/protectedItems/{protectedItemName}/backup?api-version=2023-04-01
```

Example curl for triggering VM backup:
```bash
curl -X POST \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}/protectedItems/vm;iaasvmcontainerv2;{vmResourceGroup};{vmName}/backup?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "properties": {
      "objectType": "IaasVMBackupRequest",
      "recoveryPointExpiryTimeInUTC": "2023-06-01T01:00:00Z"
    }
  }'
```

Response:
```json
{
  "id": "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}/protectedItems/vm;iaasvmcontainerv2;{vmResourceGroup};{vmName}/backup",
  "name": "backup",
  "type": "Microsoft.RecoveryServices/vaults/backupFabrics/protectionContainers/protectedItems/backup",
  "properties": {
    "objectType": "IaasVMBackupRequest",
    "recoveryPointExpiryTimeInUTC": "2023-06-01T01:00:00Z",
    "backupJobId": "{jobId}"
  }
}
```

#### Check Backup Status
You can check the status of a backup operation by querying the backup job using the `backupJobId` returned from the trigger backup operation:

```http
GET https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupJobs/{jobId}?api-version=2023-04-01
```

### 7. Recovery Points

#### List Recovery Points
```http
GET https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}/protectedItems/{protectedItemName}/recoveryPoints?api-version=2023-04-01
```

Example curl for listing VM recovery points:
```bash
curl -X GET \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}/protectedItems/vm;iaasvmcontainerv2;{vmResourceGroup};{vmName}/recoveryPoints?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

Response:
```json
{
  "value": [
    {
      "id": "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}/protectedItems/vm;iaasvmcontainerv2;{vmResourceGroup};{vmName}/recoveryPoints/{recoveryPointId}",
      "name": "{recoveryPointId}",
      "type": "Microsoft.RecoveryServices/vaults/backupFabrics/protectionContainers/protectedItems/recoveryPoints",
      "properties": {
        "objectType": "IaasVMRecoveryPoint",
        "recoveryPointType": "AppConsistent",
        "recoveryPointTime": "2023-05-01T01:00:00Z",
        "recoveryPointAdditionalInfo": "",
        "sourceVMStorageType": "GeoRedundant",
        "isSourceVMEncrypted": false,
        "isInstantIlrSessionActive": false,
        "recoveryPointTierDetails": [
          {
            "tierType": "VaultStandard",
            "status": "Valid"
          }
        ]
      }
    }
  ]
}
```

#### Get Recovery Point
```http
GET https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}/protectedItems/{protectedItemName}/recoveryPoints/{recoveryPointId}?api-version=2023-04-01
```

Example curl:
```bash
curl -X GET \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}/protectedItems/vm;iaasvmcontainerv2;{vmResourceGroup};{vmName}/recoveryPoints/{recoveryPointId}?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json"
```

Response:
```json
{
  "id": "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}/protectedItems/vm;iaasvmcontainerv2;{vmResourceGroup};{vmName}/recoveryPoints/{recoveryPointId}",
  "name": "{recoveryPointId}",
  "type": "Microsoft.RecoveryServices/vaults/backupFabrics/protectionContainers/protectedItems/recoveryPoints",
  "properties": {
    "objectType": "IaasVMRecoveryPoint",
    "recoveryPointType": "AppConsistent",
    "recoveryPointTime": "2023-05-01T01:00:00Z",
    "recoveryPointAdditionalInfo": "",
    "sourceVMStorageType": "GeoRedundant",
    "isSourceVMEncrypted": false,
    "isInstantIlrSessionActive": false,
    "recoveryPointTierDetails": [
      {
        "tierType": "VaultStandard",
        "status": "Valid"
      }
    ]
  }
}
```

#### Recovery Point Types

For Azure VMs, recovery points can be of the following types:
- `AppConsistent`: Application-consistent recovery point
- `CrashConsistent`: Crash-consistent recovery point
- `FileSystemConsistent`: File system-consistent recovery point

### 8. Restore Operations

#### Trigger Restore
```http
POST https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}/protectedItems/{protectedItemName}/recoveryPoints/{recoveryPointId}/restore?api-version=2023-04-01
```

Example curl for VM restore to original location:
```bash
curl -X POST \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}/protectedItems/vm;iaasvmcontainerv2;{vmResourceGroup};{vmName}/recoveryPoints/{recoveryPointId}/restore?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "properties": {
      "objectType": "IaasVMRestoreRequest",
      "recoveryPointId": "{recoveryPointId}",
      "recoveryType": "OriginalLocation",
      "sourceResourceId": "/subscriptions/{subscriptionId}/resourceGroups/{vmResourceGroup}/providers/Microsoft.Compute/virtualMachines/{vmName}"
    }
  }'
```

Example curl for VM restore to alternate location:
```bash
curl -X POST \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}/protectedItems/vm;iaasvmcontainerv2;{vmResourceGroup};{vmName}/recoveryPoints/{recoveryPointId}/restore?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "properties": {
      "objectType": "IaasVMRestoreRequest",
      "recoveryPointId": "{recoveryPointId}",
      "recoveryType": "AlternateLocation",
      "sourceResourceId": "/subscriptions/{subscriptionId}/resourceGroups/{vmResourceGroup}/providers/Microsoft.Compute/virtualMachines/{vmName}",
      "targetResourceGroupId": "/subscriptions/{subscriptionId}/resourceGroups/{targetResourceGroup}",
      "targetVirtualMachineName": "{targetVmName}",
      "targetNetworkId": "/subscriptions/{subscriptionId}/resourceGroups/{targetResourceGroup}/providers/Microsoft.Network/virtualNetworks/{targetVnet}",
      "targetSubnetName": "{targetSubnet}"
    }
  }'
```

Response:
```json
{
  "id": "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}/protectedItems/vm;iaasvmcontainerv2;{vmResourceGroup};{vmName}/recoveryPoints/{recoveryPointId}/restore",
  "name": "restore",
  "type": "Microsoft.RecoveryServices/vaults/backupFabrics/protectionContainers/protectedItems/recoveryPoints/restore",
  "properties": {
    "objectType": "IaasVMRestoreRequest",
    "recoveryPointId": "{recoveryPointId}",
    "recoveryType": "OriginalLocation",
    "sourceResourceId": "/subscriptions/{subscriptionId}/resourceGroups/{vmResourceGroup}/providers/Microsoft.Compute/virtualMachines/{vmName}",
    "jobId": "{jobId}"
  }
}
```

#### Restore Types

For Azure VMs, the following restore types are supported:
- `OriginalLocation`: Restore to the original VM location (overwrites existing VM)
- `AlternateLocation`: Restore to a different location or as a new VM
- `RestoreDisks`: Restore only the disks to a storage account

#### File Recovery (Item Level Recovery)

For file-level recovery from VM backups:

```http
POST https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}/protectedItems/{protectedItemName}/recoveryPoints/{recoveryPointId}/provisionInstantItemRecovery?api-version=2023-04-01
```

Example curl:
```bash
curl -X POST \
  "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;{vmResourceGroup};{vmName}/protectedItems/vm;iaasvmcontainerv2;{vmResourceGroup};{vmName}/recoveryPoints/{recoveryPointId}/provisionInstantItemRecovery?api-version=2023-04-01" \
  -H "Authorization: Bearer {access_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "properties": {
      "objectType": "ILRRequestProperties",
      "recoveryPointId": "{recoveryPointId}",
      "sourceResourceId": "/subscriptions/{subscriptionId}/resourceGroups/{vmResourceGroup}/providers/Microsoft.Compute/virtualMachines/{vmName}",
      "recoveryMode": "FileRecovery"
    }
  }'
```

## Common Query Parameters

### Filtering
- `$filter`: OData filter expression
- `$orderby`: Sort order
- `$top`: Number of items to return (default: 200)
- `$skip`: Number of items to skip

### Common Filter Expressions

#### By Backup Management Type
```
$filter=backupManagementType eq 'AzureIaasVM'
```

Valid values:
- `AzureIaasVM`: Azure Virtual Machines
- `AzureStorage`: Azure Storage
- `AzureWorkload`: Azure Workload (SQL, SAP HANA)
- `MAB`: Microsoft Azure Backup Server
- `DPM`: Data Protection Manager

#### By Status
```
$filter=status eq 'InProgress'
```

Valid values:
- `InProgress`
- `Completed`
- `Failed`
- `Cancelled`
- `CompletedWithWarnings`

#### By Operation
```
$filter=operation eq 'Backup'
```

Valid values:
- `Backup`
- `Restore`
- `ConfigureBackup`
- `DisableBackup`

#### By Date Range
```
$filter=startTime ge 2023-01-01T00:00:00Z and startTime le 2023-01-31T23:59:59Z
```

### Pagination
- `$skipToken`: Continuation token for pagination
- `nextLink`: URL for next page of results

### Example Complex Filter
```
$filter=backupManagementType eq 'AzureIaasVM' and status eq 'Completed' and operation eq 'Backup' and startTime ge 2023-01-01T00:00:00Z
```

## Common Headers

### Request Headers
```http
Authorization: Bearer {access_token}
Content-Type: application/json
Accept: application/json
```

### Response Headers
```http
Content-Type: application/json
x-ms-request-id: {request-id}
x-ms-correlation-request-id: {correlation-id}
```

## Error Handling

### Common Error Codes
- `400 Bad Request`: Invalid request parameters
- `401 Unauthorized`: Authentication required
- `403 Forbidden`: Insufficient permissions
- `404 Not Found`: Resource not found
- `409 Conflict`: Resource already exists
- `429 Too Many Requests`: Rate limit exceeded
- `500 Internal Server Error`: Server error

### Error Response Format
```json
{
  "error": {
    "code": "ErrorCode",
    "message": "Error message",
    "details": [
      {
        "code": "DetailedErrorCode",
        "message": "Detailed error message"
      }
    ]
  }
}
```

## Rate Limits

- **Read operations**: 15,000 requests per hour
- **Write operations**: 1,200 requests per hour
- **Long-running operations**: 500 requests per hour

## Best Practices

1. **Use the latest API version** for new implementations
2. **Implement proper error handling** for all API calls
3. **Use pagination** for list operations that may return large result sets
4. **Implement retry logic** with exponential backoff for transient failures
5. **Cache authentication tokens** and refresh before expiration
6. **Use appropriate RBAC roles** for least privilege access
7. **Monitor rate limits** and implement throttling if necessary

## Implementation Notes

### Fabric Names
- For Azure VMs: `Azure`
- For on-premises: Varies by backup agent

### Container Names
- For Azure VMs: `iaasvmcontainer;iaasvmcontainerv2;{resourceGroupName};{vmName}`
- Format varies by workload type

### Protected Item Names
- For Azure VMs: `vm;iaasvmcontainerv2;{resourceGroupName};{vmName}`
- Format varies by workload type

This reference should be used to ensure our Recovery Services implementation follows the official API specifications and uses the correct endpoints, parameters, and response formats.