# Recovery Services Vault API - Postman Collection Reference

## Overview

This documentation provides a comprehensive reference for the Azure Recovery Services Vault API based on the official Postman collection. It serves as an LLM-friendly resource for understanding and implementing Recovery Services operations.

**Source:** [Official Postman Documentation](https://documenter.getpostman.com/view/198900/RztoMoMS?version=latest#733bb521-058b-4c94-b68e-2487cd74c25a)

## Table of Contents

- [Authentication](#authentication)
- [Base URLs and Common Parameters](#base-urls-and-common-parameters)
- [Vault Management Operations](#vault-management-operations)
- [Backup Policy Operations](#backup-policy-operations)
- [Protection Container Operations](#protection-container-operations)
- [Protected Items Operations](#protected-items-operations)
- [Backup Jobs Operations](#backup-jobs-operations)
- [Recovery Points Operations](#recovery-points-operations)
- [Restore Operations](#restore-operations)
- [Common Response Formats](#common-response-formats)
- [Error Handling](#error-handling)
- [Code Examples](#code-examples)

## Authentication

All Recovery Services Vault API endpoints require Azure Active Directory (Azure AD) authentication using OAuth 2.0.

### Required Headers
```
Authorization: Bearer {access_token}
Content-Type: application/json
Accept: application/json
```

### Required Permissions
- **Backup Contributor**: Full backup and restore operations
- **Backup Operator**: Restore operations only
- **Backup Reader**: Read-only access to backup data

## Base URLs and Common Parameters

### Base URL
```
https://management.azure.com
```

### Common Path Parameters
- `{subscriptionId}`: Azure subscription ID
- `{resourceGroupName}`: Resource group containing the vault
- `{vaultName}`: Name of the Recovery Services vault
- `{fabricName}`: Protection fabric name (typically "Azure")
- `{containerName}`: Protection container name
- `{protectedItemName}`: Protected item name
- `{policyName}`: Backup policy name

### API Version
```
api-version=2023-04-01
```

## Vault Management Operations

### 1. List Recovery Services Vaults

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/providers/Microsoft.RecoveryServices/vaults`

**Description:** Retrieves all Recovery Services vaults in a subscription.

**Query Parameters:**
- `api-version`: API version (required)
- `$filter`: OData filter expression (optional)

**Example cURL:**
```bash
curl -X GET \
  'https://management.azure.com/subscriptions/{subscriptionId}/providers/Microsoft.RecoveryServices/vaults?api-version=2023-04-01' \
  -H 'Authorization: Bearer {access_token}' \
  -H 'Content-Type: application/json'
```

**Response:**
```json
{
  "value": [
    {
      "id": "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}",
      "name": "{vaultName}",
      "type": "Microsoft.RecoveryServices/vaults",
      "location": "eastus",
      "properties": {
        "provisioningState": "Succeeded"
      }
    }
  ]
}
```

### 2. Get Recovery Services Vault

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}`

**Description:** Retrieves details of a specific Recovery Services vault.

**Example cURL:**
```bash
curl -X GET \
  'https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}?api-version=2023-04-01' \
  -H 'Authorization: Bearer {access_token}' \
  -H 'Content-Type: application/json'
```

### 3. Create or Update Recovery Services Vault

**Method:** `PUT`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}`

**Request Body:**
```json
{
  "location": "eastus",
  "properties": {},
  "sku": {
    "name": "Standard"
  }
}
```

## Backup Policy Operations

### 1. List Backup Policies

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupPolicies`

**Query Parameters:**
- `api-version`: API version (required)
- `$filter`: Filter by backup management type (optional)

**Example cURL:**
```bash
curl -X GET \
  'https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupPolicies?api-version=2023-04-01' \
  -H 'Authorization: Bearer {access_token}' \
  -H 'Content-Type: application/json'
```

### 2. Get Backup Policy

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupPolicies/{policyName}`

### 3. Create or Update Backup Policy

**Method:** `PUT`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupPolicies/{policyName}`

**Request Body (VM Policy):**
```json
{
  "properties": {
    "backupManagementType": "AzureIaasVM",
    "schedulePolicy": {
      "schedulePolicyType": "SimpleSchedulePolicy",
      "scheduleRunFrequency": "Daily",
      "scheduleRunTimes": ["2023-01-01T02:00:00Z"]
    },
    "retentionPolicy": {
      "retentionPolicyType": "LongTermRetentionPolicy",
      "dailySchedule": {
        "retentionTimes": ["2023-01-01T02:00:00Z"],
        "retentionDuration": {
          "count": 30,
          "durationType": "Days"
        }
      }
    }
  }
}
```

## Protection Container Operations

### 1. List Protection Containers

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupProtectionContainers`

**Query Parameters:**
- `api-version`: API version (required)
- `$filter`: Filter by backup management type (optional)

### 2. Register Protection Container

**Method:** `PUT`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}`

**Request Body:**
```json
{
  "properties": {
    "backupManagementType": "AzureIaasVM",
    "containerType": "IaasVMContainer",
    "sourceResourceId": "/subscriptions/{subscriptionId}/resourceGroups/{vmResourceGroup}/providers/Microsoft.Compute/virtualMachines/{vmName}"
  }
}
```

## Protected Items Operations

### 1. List Protected Items

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupProtectedItems`

**Query Parameters:**
- `api-version`: API version (required)
- `$filter`: Filter by backup management type, item type, policy name, etc.

**Example Filters:**
- `backupManagementType eq 'AzureIaasVM'`
- `itemType eq 'VM'`
- `policyName eq 'DefaultPolicy'`

### 2. Get Protected Item

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}/protectedItems/{protectedItemName}`

### 3. Enable Protection (Create Protected Item)

**Method:** `PUT`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}/protectedItems/{protectedItemName}`

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

### 4. Disable Protection (Delete Protected Item)

**Method:** `DELETE`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}/protectedItems/{protectedItemName}`

## Backup Jobs Operations

### 1. List Backup Jobs

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupJobs`

**Query Parameters:**
- `api-version`: API version (required)
- `$filter`: Filter by operation, status, start time, end time
- `$skip`: Number of items to skip
- `$top`: Number of items to return

**Example Filters:**
- `operation eq 'Backup'`
- `status eq 'InProgress'`
- `startTime eq '2023-01-01T00:00:00Z' and endTime eq '2023-01-02T00:00:00Z'`

### 2. Get Backup Job

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupJobs/{jobName}`

### 3. Cancel Backup Job

**Method:** `POST`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupJobs/{jobName}/cancel`

### 4. Trigger Backup

**Method:** `POST`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}/protectedItems/{protectedItemName}/backup`

**Request Body:**
```json
{
  "properties": {
    "objectType": "IaasVMBackupRequest",
    "recoveryPointExpiryTimeInUTC": "2023-12-31T23:59:59Z"
  }
}
```

## Recovery Points Operations

### 1. List Recovery Points

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}/protectedItems/{protectedItemName}/recoveryPoints`

**Query Parameters:**
- `api-version`: API version (required)
- `$filter`: Filter by start time and end time

### 2. Get Recovery Point

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}/protectedItems/{protectedItemName}/recoveryPoints/{recoveryPointId}`

## Restore Operations

### 1. Trigger Restore

**Method:** `POST`

**URL:** `https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/{fabricName}/protectionContainers/{containerName}/protectedItems/{protectedItemName}/recoveryPoints/{recoveryPointId}/restore`

**Request Body (Original Location Restore):**
```json
{
  "properties": {
    "objectType": "IaasVMRestoreRequest",
    "recoveryPointId": "{recoveryPointId}",
    "recoveryType": "OriginalLocation",
    "sourceResourceId": "/subscriptions/{subscriptionId}/resourceGroups/{vmResourceGroup}/providers/Microsoft.Compute/virtualMachines/{vmName}",
    "createNewCloudService": false,
    "originalStorageAccountOption": false
  }
}
```

**Request Body (Alternate Location Restore):**
```json
{
  "properties": {
    "objectType": "IaasVMRestoreRequest",
    "recoveryPointId": "{recoveryPointId}",
    "recoveryType": "AlternateLocation",
    "targetVirtualMachineId": "/subscriptions/{subscriptionId}/resourceGroups/{targetResourceGroup}/providers/Microsoft.Compute/virtualMachines/{targetVmName}",
    "targetResourceGroupId": "/subscriptions/{subscriptionId}/resourceGroups/{targetResourceGroup}",
    "storageAccountId": "/subscriptions/{subscriptionId}/resourceGroups/{storageResourceGroup}/providers/Microsoft.Storage/storageAccounts/{storageAccountName}",
    "virtualNetworkId": "/subscriptions/{subscriptionId}/resourceGroups/{networkResourceGroup}/providers/Microsoft.Network/virtualNetworks/{vnetName}",
    "subnetId": "/subscriptions/{subscriptionId}/resourceGroups/{networkResourceGroup}/providers/Microsoft.Network/virtualNetworks/{vnetName}/subnets/{subnetName}",
    "createNewCloudService": true,
    "originalStorageAccountOption": false
  }
}
```

## Common Response Formats

### Success Response
```json
{
  "id": "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupJobs/{jobId}",
  "name": "{jobId}",
  "type": "Microsoft.RecoveryServices/vaults/backupJobs",
  "properties": {
    "jobType": "AzureIaaSVMJob",
    "duration": "00:05:30",
    "actionsInfo": [],
    "errorDetails": [],
    "extendedInfo": {},
    "entityFriendlyName": "{vmName}",
    "backupManagementType": "AzureIaasVM",
    "operation": "Backup",
    "status": "Completed",
    "startTime": "2023-01-01T02:00:00Z",
    "endTime": "2023-01-01T02:05:30Z",
    "activityId": "{activityId}"
  }
}
```

### Error Response
```json
{
  "error": {
    "code": "BadRequest",
    "message": "The request is invalid.",
    "details": [
      {
        "code": "InvalidParameter",
        "message": "The parameter 'policyName' is required."
      }
    ]
  }
}
```

## Error Handling

### Common HTTP Status Codes
- `200 OK`: Request successful
- `201 Created`: Resource created successfully
- `202 Accepted`: Request accepted for processing
- `400 Bad Request`: Invalid request parameters
- `401 Unauthorized`: Authentication required
- `403 Forbidden`: Insufficient permissions
- `404 Not Found`: Resource not found
- `409 Conflict`: Resource already exists or conflict
- `500 Internal Server Error`: Server error

### Error Response Structure
All error responses follow the standard Azure REST API error format with `error.code`, `error.message`, and optional `error.details` array.

## Code Examples

### Python Example - List Vaults
```python
import requests

def list_recovery_vaults(subscription_id, access_token):
    url = f"https://management.azure.com/subscriptions/{subscription_id}/providers/Microsoft.RecoveryServices/vaults"
    headers = {
        'Authorization': f'Bearer {access_token}',
        'Content-Type': 'application/json'
    }
    params = {'api-version': '2023-04-01'}
    
    response = requests.get(url, headers=headers, params=params)
    return response.json()
```

### PowerShell Example - Enable Protection
```powershell
$subscriptionId = "your-subscription-id"
$resourceGroupName = "your-resource-group"
$vaultName = "your-vault-name"
$vmName = "your-vm-name"
$policyName = "your-policy-name"

$uri = "https://management.azure.com/subscriptions/$subscriptionId/resourceGroups/$resourceGroupName/providers/Microsoft.RecoveryServices/vaults/$vaultName/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;$resourceGroupName;$vmName/protectedItems/vm;iaasvmcontainerv2;$resourceGroupName;$vmName"

$body = @{
    properties = @{
        protectedItemType = "Microsoft.Compute/virtualMachines"
        sourceResourceId = "/subscriptions/$subscriptionId/resourceGroups/$resourceGroupName/providers/Microsoft.Compute/virtualMachines/$vmName"
        policyId = "/subscriptions/$subscriptionId/resourceGroups/$resourceGroupName/providers/Microsoft.RecoveryServices/vaults/$vaultName/backupPolicies/$policyName"
    }
} | ConvertTo-Json -Depth 3

Invoke-RestMethod -Uri "$uri?api-version=2023-04-01" -Method PUT -Body $body -Headers @{
    'Authorization' = "Bearer $accessToken"
    'Content-Type' = 'application/json'
}
```

### cURL Example - Trigger Backup
```bash
#!/bin/bash

SUBSCRIPTION_ID="your-subscription-id"
RESOURCE_GROUP="your-resource-group"
VAULT_NAME="your-vault-name"
VM_NAME="your-vm-name"
ACCESS_TOKEN="your-access-token"

curl -X POST \
  "https://management.azure.com/subscriptions/$SUBSCRIPTION_ID/resourceGroups/$RESOURCE_GROUP/providers/Microsoft.RecoveryServices/vaults/$VAULT_NAME/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;$RESOURCE_GROUP;$VM_NAME/protectedItems/vm;iaasvmcontainerv2;$RESOURCE_GROUP;$VM_NAME/backup?api-version=2023-04-01" \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "properties": {
      "objectType": "IaasVMBackupRequest",
      "recoveryPointExpiryTimeInUTC": "2023-12-31T23:59:59Z"
    }
  }'
```

## Best Practices

1. **Authentication**: Always use service principals or managed identities for automated operations
2. **Error Handling**: Implement proper retry logic for transient failures
3. **Rate Limiting**: Respect Azure API rate limits and implement exponential backoff
4. **Monitoring**: Monitor backup job status and set up alerts for failures
5. **Resource Naming**: Use consistent naming conventions for containers and protected items
6. **Security**: Store access tokens securely and rotate them regularly

## Related Documentation

- [Azure Recovery Services REST API Reference](https://docs.microsoft.com/en-us/rest/api/recoveryservices/)
- [Azure Backup REST API Reference](https://docs.microsoft.com/en-us/rest/api/backup/)
- [Azure Backup PowerShell Reference](https://docs.microsoft.com/en-us/powershell/module/az.recoveryservices/)
- [Azure CLI Backup Reference](https://docs.microsoft.com/en-us/cli/azure/backup/)

---

*This documentation is based on the official Azure Recovery Services Vault API Postman collection and is designed to be LLM-friendly for automated code generation and API integration tasks.*