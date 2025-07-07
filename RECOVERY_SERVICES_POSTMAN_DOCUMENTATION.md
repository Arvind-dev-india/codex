# Recovery Services Vault API - Postman Collection Documentation

## Overview

This documentation is based on the official Microsoft Azure Recovery Services Vault API Postman collection (Collection ID: 198900). It provides comprehensive coverage of all Azure Backup and Recovery Services operations in an LLM-friendly format.

**Source:** https://documenter.getpostman.com/view/198900/RztoMoMS?version=latest

## Table of Contents

- [Authentication](#authentication)
- [Base URLs and API Versions](#base-urls-and-api-versions)
- [Vault Operations](#vault-operations)
- [Backup Operations](#backup-operations)
- [Restore Operations](#restore-operations)
- [Protection Management](#protection-management)
- [Policy Management](#policy-management)
- [Job Management](#job-management)
- [Common Headers](#common-headers)
- [Error Handling](#error-handling)
- [Examples and Use Cases](#examples-and-use-cases)

## Authentication

All endpoints require Azure Active Directory (Azure AD) authentication with appropriate permissions for Recovery Services operations.

### Required Permissions
- **Backup Contributor**: Full backup and restore operations
- **Backup Operator**: Restore operations only
- **Backup Reader**: Read-only access to backup data
- **Recovery Services Contributor**: Full vault management

### Authentication Flow
```bash
# Get access token
curl -X POST https://login.microsoftonline.com/{tenant-id}/oauth2/v2.0/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "client_id={client-id}&scope=https://management.azure.com/.default&client_secret={client-secret}&grant_type=client_credentials"
```

## Base URLs and API Versions

- **Base URL**: `https://management.azure.com`
- **API Version**: `2023-04-01` (stable)
- **Latest API Version**: `2025-02-01` (preview)
- **Resource Provider**: `Microsoft.RecoveryServices`

## Vault Operations

### List Recovery Services Vaults by Subscription

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/providers/Microsoft.RecoveryServices/vaults?api-version=2023-04-01`

**Path Variables:**
- `subscriptionId`: Azure subscription ID

**Query Parameters:**
- `api-version`: API version (required)
- `$filter`: OData filter expression (optional)
- `$top`: Number of results to return (optional)

**Headers:**
- `Authorization: Bearer {access_token}`
- `Content-Type: application/json`
- `Accept: application/json`

**Example cURL:**
```bash
curl -X GET \
  'https://management.azure.com/subscriptions/{subscriptionId}/providers/Microsoft.RecoveryServices/vaults?api-version=2023-04-01' \
  -H 'Authorization: Bearer {access_token}' \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json'
```

**Response Example:**
```json
{
  "value": [
    {
      "id": "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}",
      "name": "{vaultName}",
      "type": "Microsoft.RecoveryServices/vaults",
      "location": "eastus",
      "properties": {
        "provisioningState": "Succeeded",
        "upgradeDetails": {},
        "privateEndpointConnections": []
      },
      "sku": {
        "name": "Standard"
      }
    }
  ]
}
```

### Get Recovery Services Vault

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}?api-version=2023-04-01`

**Path Variables:**
- `subscriptionId`: Azure subscription ID
- `resourceGroupName`: Resource group name
- `vaultName`: Recovery Services vault name

**Example cURL:**
```bash
curl -X GET \
  'https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}?api-version=2023-04-01' \
  -H 'Authorization: Bearer {access_token}' \
  -H 'Content-Type: application/json'
```

### Create or Update Recovery Services Vault

**Method:** `PUT`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}?api-version=2023-04-01`

**Request Body:**
```json
{
  "location": "eastus",
  "sku": {
    "name": "Standard"
  },
  "properties": {
    "publicNetworkAccess": "Enabled"
  }
}
```

## Backup Operations

### List Protected Items

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupProtectedItems?api-version=2023-04-01`

**Query Parameters:**
- `$filter`: Filter by backup management type, workload type, etc.
  - Example: `backupManagementType eq 'AzureIaasVM'`
- `$skipToken`: Pagination token

**Example cURL:**
```bash
curl -X GET \
  'https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupProtectedItems?api-version=2023-04-01&$filter=backupManagementType eq '\''AzureIaasVM'\''' \
  -H 'Authorization: Bearer {access_token}' \
  -H 'Content-Type: application/json'
```

### Trigger Backup

**Method:** `POST`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}/protectedItems/{protectedItemName}/backup?api-version=2023-04-01`

**Path Variables:**
- `fabricName`: Usually "Azure"
- `containerName`: Container name (e.g., "iaasvmcontainer;iaasvmcontainerv2;{resourceGroup};{vmName}")
- `protectedItemName`: Protected item name (e.g., "vm;iaasvmcontainerv2;{resourceGroup};{vmName}")

**Request Body:**
```json
{
  "properties": {
    "objectType": "IaasVMBackupRequest",
    "recoveryPointExpiryTimeInUTC": "2024-12-31T23:59:59.000Z"
  }
}
```

### List Backup Jobs

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupJobs?api-version=2023-04-01`

**Query Parameters:**
- `$filter`: Filter by operation, status, start time, end time
  - Example: `operation eq 'Backup' and status eq 'InProgress'`
- `$skipToken`: Pagination token

## Restore Operations

### List Recovery Points

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}/protectedItems/{protectedItemName}/recoveryPoints?api-version=2023-04-01`

**Query Parameters:**
- `$filter`: Filter by date range
  - Example: `startDate eq '2024-01-01' and endDate eq '2024-01-31'`

### Trigger Restore

**Method:** `POST`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}/protectedItems/{protectedItemName}/recoveryPoints/{recoveryPointId}/restore?api-version=2023-04-01`

**Request Body for VM Restore:**
```json
{
  "properties": {
    "objectType": "IaasVMRestoreRequest",
    "recoveryPointId": "{recoveryPointId}",
    "recoveryType": "RestoreDisks",
    "sourceResourceId": "/subscriptions/{subscriptionId}/resourceGroups/{sourceResourceGroup}/providers/Microsoft.Compute/virtualMachines/{sourceVmName}",
    "storageAccountId": "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroup}/providers/Microsoft.Storage/storageAccounts/{storageAccountName}",
    "region": "eastus",
    "createNewCloudService": false,
    "originalStorageAccountOption": false,
    "encryptionDetails": {
      "encryptionEnabled": false
    }
  }
}
```

## Protection Management

### Enable Protection for VM

**Method:** `PUT`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}/protectedItems/{protectedItemName}?api-version=2023-04-01`

**Request Body:**
```json
{
  "properties": {
    "protectedItemType": "Microsoft.Compute/virtualMachines",
    "sourceResourceId": "/subscriptions/{subscriptionId}/resourceGroups/{vmResourceGroup}/providers/Microsoft.Compute/virtualMachines/{vmName}",
    "policyId": "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupPolicies/{policyName}"
  }
}
```

### Disable Protection

**Method:** `DELETE`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}/protectedItems/{protectedItemName}?api-version=2023-04-01`

**Query Parameters:**
- `deleteBackupData`: true/false - Whether to delete backup data

## Policy Management

### List Backup Policies

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupPolicies?api-version=2023-04-01`

**Query Parameters:**
- `$filter`: Filter by backup management type
  - Example: `backupManagementType eq 'AzureIaasVM'`

### Create Backup Policy

**Method:** `PUT`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupPolicies/{policyName}?api-version=2023-04-01`

**Request Body:**
```json
{
  "properties": {
    "backupManagementType": "AzureIaasVM",
    "schedulePolicy": {
      "schedulePolicyType": "SimpleSchedulePolicy",
      "scheduleRunFrequency": "Daily",
      "scheduleRunTimes": ["2024-01-01T02:00:00.000Z"],
      "scheduleWeeklyFrequency": 0
    },
    "retentionPolicy": {
      "retentionPolicyType": "LongTermRetentionPolicy",
      "dailySchedule": {
        "retentionTimes": ["2024-01-01T02:00:00.000Z"],
        "retentionDuration": {
          "count": 30,
          "durationType": "Days"
        }
      }
    }
  }
}
```

## Job Management

### Get Job Details

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupJobs/{jobName}?api-version=2023-04-01`

### Cancel Job

**Method:** `POST`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupJobs/{jobName}/cancel?api-version=2023-04-01`

## Common Headers

All API requests should include these headers:

```
Authorization: Bearer {access_token}
Content-Type: application/json
Accept: application/json
User-Agent: {your-application-name}
```

## Error Handling

### Standard HTTP Status Codes

- `200 OK` - Success
- `201 Created` - Resource created successfully
- `202 Accepted` - Request accepted for processing
- `400 Bad Request` - Invalid request parameters
- `401 Unauthorized` - Authentication required
- `403 Forbidden` - Insufficient permissions
- `404 Not Found` - Resource not found
- `409 Conflict` - Resource conflict
- `429 Too Many Requests` - Rate limit exceeded
- `500 Internal Server Error` - Server error

### Error Response Format

```json
{
  "error": {
    "code": "InvalidParameter",
    "message": "The parameter 'subscriptionId' is invalid.",
    "details": [
      {
        "code": "InvalidSubscriptionId",
        "message": "The subscription ID is not valid."
      }
    ]
  }
}
```

## Examples and Use Cases

### Complete VM Backup Workflow

1. **List available VMs for backup:**
```bash
curl -X GET \
  'https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupProtectableItems?api-version=2023-04-01&$filter=backupManagementType eq '\''AzureIaasVM'\''' \
  -H 'Authorization: Bearer {access_token}'
```

2. **Create a backup policy:**
```bash
curl -X PUT \
  'https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupPolicies/DailyBackupPolicy?api-version=2023-04-01' \
  -H 'Authorization: Bearer {access_token}' \
  -H 'Content-Type: application/json' \
  -d '{policy_json}'
```

3. **Enable protection for VM:**
```bash
curl -X PUT \
  'https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/{containerName}/protectedItems/{protectedItemName}?api-version=2023-04-01' \
  -H 'Authorization: Bearer {access_token}' \
  -H 'Content-Type: application/json' \
  -d '{protection_json}'
```

4. **Trigger on-demand backup:**
```bash
curl -X POST \
  'https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/{containerName}/protectedItems/{protectedItemName}/backup?api-version=2023-04-01' \
  -H 'Authorization: Bearer {access_token}' \
  -H 'Content-Type: application/json' \
  -d '{backup_request_json}'
```

### Monitoring and Management

1. **Monitor backup jobs:**
```bash
curl -X GET \
  'https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupJobs?api-version=2023-04-01&$filter=status eq '\''InProgress'\''' \
  -H 'Authorization: Bearer {access_token}'
```

2. **List recovery points:**
```bash
curl -X GET \
  'https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/{containerName}/protectedItems/{protectedItemName}/recoveryPoints?api-version=2023-04-01' \
  -H 'Authorization: Bearer {access_token}'
```

## Integration with LLMs

This documentation is structured to be easily consumed by Large Language Models (LLMs) for:

- **Code Generation**: Generating Azure Backup automation scripts
- **Troubleshooting**: Diagnosing backup and restore issues
- **Best Practices**: Implementing proper backup strategies
- **API Integration**: Building applications that use Azure Backup services

### Key Features for LLM Consumption

1. **Structured Format**: Clear sections with consistent formatting
2. **Complete Examples**: Full cURL commands with all required parameters
3. **Error Handling**: Comprehensive error codes and responses
4. **Workflow Guidance**: Step-by-step procedures for common tasks
5. **Parameter Documentation**: Detailed explanation of all parameters and their usage

## Additional Resources

- [Official Azure Backup REST API Documentation](https://docs.microsoft.com/en-us/rest/api/backup/)
- [Recovery Services REST API Documentation](https://docs.microsoft.com/en-us/rest/api/recoveryservices/)
- [Azure Backup PowerShell Documentation](https://docs.microsoft.com/en-us/powershell/module/az.recoveryservices/)
- [Azure CLI Backup Commands](https://docs.microsoft.com/en-us/cli/azure/backup/)

---

*This documentation is based on the official Microsoft Azure Recovery Services Vault API Postman collection and is designed to be comprehensive and LLM-friendly for automated processing and code generation.*