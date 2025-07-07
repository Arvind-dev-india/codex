# Microsoft Azure Backup for SAP ASE - Complete API Documentation

## Overview

This documentation is extracted from the official Microsoft Azure Backup for SAP ASE Postman collection.
It provides comprehensive coverage of Azure Recovery Services and Backup REST APIs with real examples,
request/response formats, and complete workflows for LLM consumption.

**Collection Description:**
REST API Specs for SAP ASE Workload Backup to Azure Prerequisite: ASE Pre-registration script download location http://aka.ms/ScriptForPermsOnASE Prerequisite: Install dotnet core runtime in VM https://dotnet.microsoft.com/download/linux-package-manager/sles/runtime-2.1.0 Backup REST APIs for ASE Documentation for reference (which was shown in Demo) http://aka.ms/ase/rest If you want to use Postman to test REST APIs (as shown in Demo) https://www.getpostman.com Official public documentation for Azure Backup https://docs.microsoft.com/en-us/rest/api/backup Azure Backup Team Demo Recording Will be updated in a day

**Source:** https://documenter.getpostman.com/view/198900/RztoMoMS
**Collection ID:** 198900
**API Version:** 2018-01-10 (as used in examples)
**Workload Type:** SAP ASE Database Backup

## Table of Contents

1. [01 - Common Operations](#01---common-operations)
2. [02 - Create Recovery Services Vault](#02---create-recovery-services-vault)
3. [03 - Register Container](#03---register-container)
4. [04 - Trigger Inquiry](#04---trigger-inquiry)
5. [05 - List Protectable Items](#05---list-protectable-items)
6. [06 - Create Policy](#06---create-policy)
7. [07 - Configure Protection](#07---configure-protection)
8. [08 - List Protected Items](#08---list-protected-items)
9. [09 - Trigger Backup](#09---trigger-backup)
10. [10 - Trigger Restore](#10---trigger-restore)
11. [11 - Stop Protection](#11---stop-protection)
12. [12 - Unregister Container](#12---unregister-container)
13. [13 - Delete Recovery Services Vault](#13---delete-recovery-services-vault)

## Authentication

All endpoints require Azure Active Directory (Azure AD) authentication.

### OAuth 2.0 Flow
```http
POST https://login.microsoftonline.com/{tenantId}/oauth2/token
Content-Type: application/x-www-form-urlencoded

grant_type=client_credentials
&client_id={clientId}
&client_secret={clientSecret}
&resource=https://management.core.windows.net/
```

### Required Headers
```
Authorization: Bearer {access_token}
Content-Type: application/json
Accept: application/json
```

## 1. 01 - Common Operations

AAD LoginTrack Location HeaderTrack Azure Async OperationTrack Backup JobList Jobs

### AAD Login

### AAD Login

**Method:** `POST`

**URL:** `https://login.microsoftonline.com/{{tenantId}}/oauth2/token`

**Headers:**
- `Content-Type: application/x-www-form-urlencoded`

**Request Body:**

Form Data:
- `grant_type`: `client_credentials`
- `client_id`: `{{clientId}}`
- `client_secret`: `{{clientSecret}}`
- `resource`: `https://management.core.windows.net/`

**Example cURL:**

```bash
curl -X POST \
  'https://login.microsoftonline.com/{{tenantId}}/oauth2/token' \
  -H 'Content-Type: application/x-www-form-urlencoded'
```

### Track Location Header

### Track Location Header

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/operationResults/fbd8fcf1-643e-4787-a312-1f77be9208bd?api-version=2018-01-10`

**Headers:**
- `authorization: {{bearer}}`
- `accept: application/json`
- `content-type: application/json`

**Example cURL:**

```bash
curl -X GET \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/operationResults/fbd8fcf1-643e-4787-a312-1f77be9208bd?api-version=2018-01-10' \
  -H 'authorization: $(BEARER)' \
  -H 'accept: application/json' \
  -H 'content-type: application/json'
```

### Track Azure Async Operation

### Track Azure Async Operation

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu/operationsStatus/b1f97612-c4c9-4c60-a12f-2b5ebcdde6b1?api-version=2018-01-10`

**Headers:**
- `authorization: {{bearer}}`
- `accept: application/json`
- `content-type: application/json`

**Example cURL:**

```bash
curl -X GET \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu/operationsStatus/b1f97612-c4c9-4c60-a12f-2b5ebcdde6b1?api-version=2018-01-10' \
  -H 'authorization: $(BEARER)' \
  -H 'accept: application/json' \
  -H 'content-type: application/json'
```

### Track Backup Job

### Track Backup Job

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupJobs/ed748c97-f070-4db5-8972-638c6e69d22b?api-version=2018-01-10`

**Headers:**
- `authorization: {{bearer}}`
- `accept: application/json`
- `content-type: application/json`

**Example cURL:**

```bash
curl -X GET \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupJobs/ed748c97-f070-4db5-8972-638c6e69d22b?api-version=2018-01-10' \
  -H 'authorization: $(BEARER)' \
  -H 'accept: application/json' \
  -H 'content-type: application/json'
```

### List Backup Jobs

### List Backup Jobs

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupJobs?api-version=2018-01-10`

**Headers:**
- `authorization: {{bearer}}`
- `accept: application/json`
- `content-type: application/json`

**Example cURL:**

```bash
curl -X GET \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupJobs?api-version=2018-01-10' \
  -H 'authorization: $(BEARER)' \
  -H 'accept: application/json' \
  -H 'content-type: application/json'
```

## 2. 02 - Create Recovery Services Vault

Creates and Gets Recovery Services Vault

### Create Recovery Services Vault

### Create Recovery Services Vault

**Method:** `PUT`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete?api-version=2018-01-10`

**Headers:**
- `Authorization: {{bearer}}`
- `Accept: application/json`
- `Content-Type: application/json`

**Request Body:**

```json
{
  "name": "aseecyvault-donotdelete",
  "location": "eastus2euap",
  "type": "Microsoft.RecoveryServices/vaults",
  "sku": {
    "name": "RS0",
    "tier": "Standard"
  },
  "properties": {
  }
}
```

**Example cURL:**

```bash
curl -X PUT \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete?api-version=2018-01-10' \
  -H 'Authorization: $(BEARER)' \
  -H 'Accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
  "name": "aseecyvault-donotdelete",
  "location": "eastus2euap",
  "type": "Microsoft.RecoveryServices/vaults",
  "sku": {
    "name": "RS0",
    "tier": "Standard"
  },
  "properties": {
  }
}'
```

### Get Recovery Services Vault

### Get Recovery Services Vault

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete?api-version=2018-01-10`

**Headers:**
- `Authorization: {{bearer}}`
- `Accept: application/json`
- `Content-Type: application/json`

**Example cURL:**

```bash
curl -X GET \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete?api-version=2018-01-10' \
  -H 'Authorization: $(BEARER)' \
  -H 'Accept: application/json' \
  -H 'Content-Type: application/json'
```

## 3. 03 - Register Container

Registers Container

### Register Container

### Register Container

**Method:** `PUT`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1?api-version=2018-01-10`

**Headers:**
- `authorization: {{bearer}}`
- `accept: application/json`
- `content-type: application/json`

**Request Body:**

```json
{
    "SubscriptionId":"da364f0f-307b-41c9-9d47-b7413ec45535",
    "properties": {
        "containerType": "VMAppContainer",
        "friendlyName": "aseecyvm1",
        "backupManagementType": "AzureWorkload",
        "sourceResourceId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.Compute/virtualMachines/aseecyvm1",
    	"workloadType":"SAPAseDatabase"
    }
}

```

**Example cURL:**

```bash
curl -X PUT \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1?api-version=2018-01-10' \
  -H 'authorization: $(BEARER)' \
  -H 'accept: application/json' \
  -H 'content-type: application/json' \
  -d '{
    "SubscriptionId":"da364f0f-307b-41c9-9d47-b7413ec45535",
    "properties": {
        "containerType": "VMAppContainer",
        "friendlyName": "aseecyvm1",
        "backupManagementType": "AzureWorkload",
        "sourceResourceId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.Compute/virtualMachines/aseecyvm1",
    	"workloadType":"SAPAseDatabase"
    }
}
'
```

### List Protection Containers

### List Protection Containers

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupProtectionContainers?api-version=2018-01-10&$filter=providertype eq 'AzureWorkload'`

**Headers:**
- `Authorization: {{bearer}}`
- `Accept: application/json`
- `Content-Type: application/json`

**Example cURL:**

```bash
curl -X GET \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupProtectionContainers?api-version=2018-01-10&$filter=providertype eq 'AzureWorkload'' \
  -H 'Authorization: $(BEARER)' \
  -H 'Accept: application/json' \
  -H 'Content-Type: application/json'
```

## 4. 04 - Trigger Inquiry

Discovers Protectable Databases in Container

### Trigger Inquiry

### Trigger Inquiry

**Method:** `POST`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/inquire?api-version=2018-01-10&$filter=workloadType eq 'SAPAseDatabase'`

**Headers:**
- `authorization: {{bearer}}`
- `accept: application/json`
- `content-type: application/json`

**Example cURL:**

```bash
curl -X POST \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/inquire?api-version=2018-01-10&$filter=workloadType eq 'SAPAseDatabase'' \
  -H 'authorization: $(BEARER)' \
  -H 'accept: application/json' \
  -H 'content-type: application/json'
```

## 5. 05 - List Protectable Items

Lists Protectable Items/Databases

### List Protectable Items

### List Protectable Items

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupProtectableItems?api-version=2018-01-10&$filter=backupManagementType eq 'AzureWorkload' and workloadType eq 'SAPAseDatabase'`

**Headers:**
- `authorization: {{bearer}}`
- `accept: application/json`
- `content-type: application/json`

**Example cURL:**

```bash
curl -X GET \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupProtectableItems?api-version=2018-01-10&$filter=backupManagementType eq 'AzureWorkload' and workloadType eq 'SAPAseDatabase'' \
  -H 'authorization: $(BEARER)' \
  -H 'accept: application/json' \
  -H 'content-type: application/json'
```

## 6. 06 - Create Policy

Creates and Gets Policy

### Create Policy (Daily Full and Hourly Log)

### Create Policy (Daily Full and Hourly Log)

**Method:** `PUT`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupPolicies/DailyFullHourlyLog?api-version=2018-01-10`

**Headers:**
- `Authorization: {{bearer}}`
- `Accept: application/json`
- `Content-Type: application/json`

**Request Body:**

```json
{
            "id": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupPolicies/DailyFullHourlyLog",
            "name": "DailyFullHourlyLog",
            "type": "Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupPolicies",
            "properties": {
                "backupManagementType": "AzureWorkload",
                "workLoadType": "SAPAseDatabase",
                "settings": {
                    "timeZone": "UTC",
                    "issqlcompression": false,
                    "isCompression": false
                },
                "subProtectionPolicy": [
                    {
                        "policyType": "Full",
                        "schedulePolicy": {
                            "schedulePolicyType": "SimpleSchedulePolicy",
                            "scheduleRunFrequency": "Daily",
                            "scheduleRunTimes": [
                                "2018-09-14T16:00:00Z"
                            ],
                            "scheduleWeeklyFrequency": 0
                        },
                        "retentionPolicy": {
                            "retentionPolicyType": "LongTermRetentionPolicy",
                            "dailySchedule": {
                                "retentionTimes": [
                                    "2018-09-14T16:00:00Z"
                                ],
                                "retentionDuration": {
                                    "count": 30,
                                    "durationType": "Days"
                                }
                            }
                        }
                    },
                    {
                        "policyType": "Log",
                        "schedulePolicy": {
                            "schedulePolicyType": "LogSchedulePolicy",
                            "scheduleFrequencyInMins": 60
                        },
                        "retentionPolicy": {
                            "retentionPolicyType": "SimpleRetentionPolicy",
                            "retentionDuration": {
                                "count": 30,
                                "durationType": "Days"
                            }
                        }
                    }
                ],
                "protectedItemsCount": 0
            }
        }
```

**Example cURL:**

```bash
curl -X PUT \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupPolicies/DailyFullHourlyLog?api-version=2018-01-10' \
  -H 'Authorization: $(BEARER)' \
  -H 'Accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
            "id": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupPolicies/DailyFullHourlyLog",
            "name": "DailyFullHourlyLog",
            "type": "Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupPolicies",
            "properties": {
                "backupManagementType": "AzureWorkload",
                "workLoadType": "SAPAseDatabase",
                "settings": {
                    "timeZone": "UTC",
                    "issqlcompression": false,
                    "isCompression": false
                },
                "subProtectionPolicy": [
                    {
                        "policyType": "Full",
                        "schedulePolicy": {
                            "schedulePolicyType": "SimpleSchedulePolicy",
                            "scheduleRunFrequency": "Daily",
                            "scheduleRunTimes": [
                                "2018-09-14T16:00:00Z"
                            ],
                            "scheduleWeeklyFrequency": 0
                        },
                        "retentionPolicy": {
                            "retentionPolicyType": "LongTermRetentionPolicy",
                            "dailySchedule": {
                                "retentionTimes": [
                                    "2018-09-14T16:00:00Z"
                                ],
                                "retentionDuration": {
                                    "count": 30,
                                    "durationType": "Days"
                                }
                            }
                        }
                    },
                    {
                        "policyType": "Log",
                        "schedulePolicy": {
                            "schedulePolicyType": "LogSchedulePolicy",
                            "scheduleFrequencyInMins": 60
                        },
                        "retentionPolicy": {
                            "retentionPolicyType": "SimpleRetentionPolicy",
                            "retentionDuration": {
                                "count": 30,
                                "durationType": "Days"
                            }
                        }
                    }
                ],
                "protectedItemsCount": 0
            }
        }'
```

### List Policies

### List Policies

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupPolicies?api-version=2018-01-10`

**Headers:**
- `Authorization: {{bearer}}`
- `Accept: application/json`
- `Content-Type: application/json`

**Example cURL:**

```bash
curl -X GET \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupPolicies?api-version=2018-01-10' \
  -H 'Authorization: $(BEARER)' \
  -H 'Accept: application/json' \
  -H 'Content-Type: application/json'
```

## 7. 07 - Configure Protection

Configures Protaction of datasource with specified Backup Policy

### Configure Protection

### Configure Protection

**Method:** `PUT`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu?api-version=2018-01-10`

**Headers:**
- `authorization: {{bearer}}`
- `accept: application/json`
- `content-type: application/json`

**Request Body:**

```json
{
    "id": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu",
    "name": "SAPAseDatabase;azu;azu",
  "type": "Microsoft.RecoveryServices/vaults/protectedItems",
  "location": "eastus2euap",
    "properties": {
        "backupManagementType": "AzureWorkload",
        "workloadType": "SAPAse",
        "protectedItemType": "SAPAseDatabase",
        "friendlyName": "azu",
        "policyId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupPolicies/DailyFullHourlyLog"
    }
}

```

**Example cURL:**

```bash
curl -X PUT \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu?api-version=2018-01-10' \
  -H 'authorization: $(BEARER)' \
  -H 'accept: application/json' \
  -H 'content-type: application/json' \
  -d '{
    "id": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu",
    "name": "SAPAseDatabase;azu;azu",
  "type": "Microsoft.RecoveryServices/vaults/protectedItems",
  "location": "eastus2euap",
    "properties": {
        "backupManagementType": "AzureWorkload",
        "workloadType": "SAPAse",
        "protectedItemType": "SAPAseDatabase",
        "friendlyName": "azu",
        "policyId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupPolicies/DailyFullHourlyLog"
    }
}
'
```

## 8. 08 - List Protected Items

Lists Protected Items

### List Protected Items

### List Protected Items

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupProtectedItems?api-version=2018-01-10&$filter=providertype eq 'AzureWorkload'`

**Headers:**
- `Authorization: {{bearer}}`
- `Accept: application/json`
- `Content-Type: application/json`

**Example cURL:**

```bash
curl -X GET \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupProtectedItems?api-version=2018-01-10&$filter=providertype eq 'AzureWorkload'' \
  -H 'Authorization: $(BEARER)' \
  -H 'Accept: application/json' \
  -H 'Content-Type: application/json'
```

## 9. 09 - Trigger Backup

Triggers Backup

### Trigger Backup (Full)

### Trigger Backup (Full)

**Method:** `POST`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu/backup?api-version=2018-01-10`

**Headers:**
- `authorization: {{bearer}}`
- `accept: application/json`
- `content-type: application/json`

**Request Body:**

```json
{
    "properties": {
        "objectType": "AzureWorkloadBackupRequest",
        "backupType": "Full",
        "enableCompression": true,
        "recoveryPointExpiryTimeInUTC": "2019-02-28T18:29:59.000Z"
    }
    
}

```

**Example cURL:**

```bash
curl -X POST \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu/backup?api-version=2018-01-10' \
  -H 'authorization: $(BEARER)' \
  -H 'accept: application/json' \
  -H 'content-type: application/json' \
  -d '{
    "properties": {
        "objectType": "AzureWorkloadBackupRequest",
        "backupType": "Full",
        "enableCompression": true,
        "recoveryPointExpiryTimeInUTC": "2019-02-28T18:29:59.000Z"
    }
    
}
'
```

### Trigger Backup (Log)

### Trigger Backup (Log)

**Method:** `POST`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu/backup?api-version=2018-01-10`

**Headers:**
- `authorization: {{bearer}}`
- `accept: application/json`
- `content-type: application/json`

**Request Body:**

```json
{
    "properties": {
        "objectType": "AzureWorkloadBackupRequest",
        "backupType": "Log",
        "enableCompression": true,
        "recoveryPointExpiryTimeInUTC": "2019-02-28T18:29:59.000Z"
    }
    
}

```

**Example cURL:**

```bash
curl -X POST \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu/backup?api-version=2018-01-10' \
  -H 'authorization: $(BEARER)' \
  -H 'accept: application/json' \
  -H 'content-type: application/json' \
  -d '{
    "properties": {
        "objectType": "AzureWorkloadBackupRequest",
        "backupType": "Log",
        "enableCompression": true,
        "recoveryPointExpiryTimeInUTC": "2019-02-28T18:29:59.000Z"
    }
    
}
'
```

## 10. 10 - Trigger Restore

Triggers Restore

### List Recovery Points

### List Recovery Points

**Method:** `GET`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu/recoveryPoints?api-version=2018-01-10&$filter=startDate eq '2019-01-01 05:23:52 AM' and endDate eq '2019-02-07 05:23:52 AM'`

**Headers:**
- `authorization: {{bearer}}`
- `accept: application/json`
- `content-type: application/json`

**Example cURL:**

```bash
curl -X GET \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu/recoveryPoints?api-version=2018-01-10&$filter=startDate eq '2019-01-01 05:23:52 AM' and endDate eq '2019-02-07 05:23:52 AM'' \
  -H 'authorization: $(BEARER)' \
  -H 'accept: application/json' \
  -H 'content-type: application/json'
```

### Trigger Restore (Full) (Original Location)

### Trigger Restore (Full) (Original Location)

**Method:** `POST`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu/recoveryPoints/82592972300173/restore?api-version=2018-01-10`

**Headers:**
- `authorization: {{bearer}}`
- `accept: application/json`
- `content-type: application/json`

**Request Body:**

```json
{
  "properties": {
    "objectType": "AzureWorkloadRestoreRequest",
    "targetInfo": {
      "containerId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1",
      "databaseName": "azu/azu",
      "overwriteOption": "Overwrite"
    },
    "RecoveryType": "OriginalLocation",
    "SourceResourceId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/VMAppContainer/aseecyvm1"
  }
}
```

**Example cURL:**

```bash
curl -X POST \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu/recoveryPoints/82592972300173/restore?api-version=2018-01-10' \
  -H 'authorization: $(BEARER)' \
  -H 'accept: application/json' \
  -H 'content-type: application/json' \
  -d '{
  "properties": {
    "objectType": "AzureWorkloadRestoreRequest",
    "targetInfo": {
      "containerId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1",
      "databaseName": "azu/azu",
      "overwriteOption": "Overwrite"
    },
    "RecoveryType": "OriginalLocation",
    "SourceResourceId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/VMAppContainer/aseecyvm1"
  }
}'
```

### Trigger Restore (Full) (Alternate Location) (Different Host)

### Trigger Restore (Full) (Alternate Location) (Different Host)

**Method:** `POST`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu/recoveryPoints/82592972300173/restore?api-version=2018-01-10`

**Headers:**
- `authorization: {{bearer}}`
- `accept: application/json`
- `content-type: application/json`

**Request Body:**

```json
{
  "properties": {
    "objectType": "AzureWorkloadRestoreRequest",
    "targetInfo": {
      "containerId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm2",
      "databaseName": "azu/AZU_RestoreTest",
      "overwriteOption": "Overwrite"
    },
    "RecoveryType": "AlternateLocation",
    "SourceResourceId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/VMAppContainer/aseecyvm1"
  }
}
```

**Example cURL:**

```bash
curl -X POST \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu/recoveryPoints/82592972300173/restore?api-version=2018-01-10' \
  -H 'authorization: $(BEARER)' \
  -H 'accept: application/json' \
  -H 'content-type: application/json' \
  -d '{
  "properties": {
    "objectType": "AzureWorkloadRestoreRequest",
    "targetInfo": {
      "containerId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm2",
      "databaseName": "azu/AZU_RestoreTest",
      "overwriteOption": "Overwrite"
    },
    "RecoveryType": "AlternateLocation",
    "SourceResourceId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/VMAppContainer/aseecyvm1"
  }
}'
```

### Trigger Restore (Log) (Original Location)

### Trigger Restore (Log) (Original Location)

**Method:** `POST`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu/recoveryPoints/DefaultRangeRecoveryPoint/restore?api-version=2018-01-10`

**Headers:**
- `authorization: {{bearer}}`
- `accept: application/json`
- `content-type: application/json`

**Request Body:**

```json
{
  "properties": {
    "objectType": "AzureWorkloadPointInTimeRestoreRequest",
    "targetInfo": {
      "containerId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1",
      "databaseName": "azu/azu",
      "overwriteOption": "Overwrite"
    },
    "pointInTime": "2019-02-01T13:00:32.017Z",
    "RecoveryType": "OriginalLocation",
    "SourceResourceId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/VMAppContainer/aseecyvm1"
  }
}
```

**Example cURL:**

```bash
curl -X POST \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu/recoveryPoints/DefaultRangeRecoveryPoint/restore?api-version=2018-01-10' \
  -H 'authorization: $(BEARER)' \
  -H 'accept: application/json' \
  -H 'content-type: application/json' \
  -d '{
  "properties": {
    "objectType": "AzureWorkloadPointInTimeRestoreRequest",
    "targetInfo": {
      "containerId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1",
      "databaseName": "azu/azu",
      "overwriteOption": "Overwrite"
    },
    "pointInTime": "2019-02-01T13:00:32.017Z",
    "RecoveryType": "OriginalLocation",
    "SourceResourceId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/VMAppContainer/aseecyvm1"
  }
}'
```

### Trigger Restore (Log) (Alternate Location) (Different Host)

### Trigger Restore (Log) (Alternate Location) (Different Host)

**Method:** `POST`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu/recoveryPoints/DefaultRangeRecoveryPoint/restore?api-version=2018-01-10`

**Headers:**
- `authorization: {{bearer}}`
- `accept: application/json`
- `content-type: application/json`

**Request Body:**

```json
{
  "properties": {
    "objectType": "AzureWorkloadPointInTimeRestoreRequest",
    "targetInfo": {
      "containerId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm2",
      "databaseName": "azu/AZU_RestoreTest",
      "overwriteOption": "Overwrite"
    },
    "pointInTime": "2019-02-01T13:00:32.017Z",
    "RecoveryType": "AlternateLocation",
    "SourceResourceId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/VMAppContainer/aseecyvm1"
  }
}
```

**Example cURL:**

```bash
curl -X POST \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu/recoveryPoints/DefaultRangeRecoveryPoint/restore?api-version=2018-01-10' \
  -H 'authorization: $(BEARER)' \
  -H 'accept: application/json' \
  -H 'content-type: application/json' \
  -d '{
  "properties": {
    "objectType": "AzureWorkloadPointInTimeRestoreRequest",
    "targetInfo": {
      "containerId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm2",
      "databaseName": "azu/AZU_RestoreTest",
      "overwriteOption": "Overwrite"
    },
    "pointInTime": "2019-02-01T13:00:32.017Z",
    "RecoveryType": "AlternateLocation",
    "SourceResourceId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/VMAppContainer/aseecyvm1"
  }
}'
```

## 11. 11 - Stop Protection

Stops Protection

### Stop Protection (Delete Data)

### Stop Protection (Delete Data)

**Method:** `DELETE`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu?api-version=2018-01-10`

**Headers:**
- `Authorization: {{bearer}}`
- `Accept: application/json`
- `Content-Type: application/json`

**Example cURL:**

```bash
curl -X DELETE \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1/protectedItems/SAPAseDatabase;azu;azu?api-version=2018-01-10' \
  -H 'Authorization: $(BEARER)' \
  -H 'Accept: application/json' \
  -H 'Content-Type: application/json'
```

## 12. 12 - Unregister Container

Unregisters Container

### Unregister Container

### Unregister Container

**Method:** `DELETE`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1?api-version=2018-01-10`

**Headers:**
- `authorization: {{bearer}}`
- `accept: application/json`
- `content-type: application/json`

**Request Body:**

```json
{
    "Id": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServicesBVTD/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1",
    "name": "VMAppContainer;compute;ASERG;aseecyvm1",
    "type": "",
    "properties": {
        "containerType": "VMAppContainer",
        "friendlyName": "aseecyvm1",
        "backupManagementType": "AzureWorkload",
        "sourceResourceId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.Compute/virtualMachines/aseecyvm1",
		"workloadType":"SAPAseDatabase"
    },
	"SubscriptionId":"da364f0f-307b-41c9-9d47-b7413ec45535"
}

```

**Example cURL:**

```bash
curl -X DELETE \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1?api-version=2018-01-10' \
  -H 'authorization: $(BEARER)' \
  -H 'accept: application/json' \
  -H 'content-type: application/json' \
  -d '{
    "Id": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.RecoveryServicesBVTD/vaults/aseecyvault-donotdelete/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;ASERG;aseecyvm1",
    "name": "VMAppContainer;compute;ASERG;aseecyvm1",
    "type": "",
    "properties": {
        "containerType": "VMAppContainer",
        "friendlyName": "aseecyvm1",
        "backupManagementType": "AzureWorkload",
        "sourceResourceId": "/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourceGroups/ASERG/providers/Microsoft.Compute/virtualMachines/aseecyvm1",
		"workloadType":"SAPAseDatabase"
    },
	"SubscriptionId":"da364f0f-307b-41c9-9d47-b7413ec45535"
}
'
```

## 13. 13 - Delete Recovery Services Vault

Deletes Recovery Services Vault

### Delete Recovery Services Vault

### Delete Recovery Services Vault

**Method:** `DELETE`

**URL:** `https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete?api-version=2018-01-10`

**Headers:**
- `Authorization: {{bearer}}`
- `Accept: application/json`
- `Content-Type: application/json`

**Request Body:**


**Example cURL:**

```bash
curl -X DELETE \
  'https://management.azure.com/subscriptions/da364f0f-307b-41c9-9d47-b7413ec45535/resourcegroups/ASERG/providers/Microsoft.RecoveryServices/vaults/aseecyvault-donotdelete?api-version=2018-01-10' \
  -H 'Authorization: $(BEARER)' \
  -H 'Accept: application/json' \
  -H 'Content-Type: application/json'
```

## Error Handling

### HTTP Status Codes

- `200 OK` - Request successful
- `201 Created` - Resource created successfully
- `202 Accepted` - Request accepted for asynchronous processing
- `204 No Content` - Request successful, no content returned
- `400 Bad Request` - Invalid request parameters
- `401 Unauthorized` - Authentication required or failed
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

## Asynchronous Operations

Many Azure Backup operations are asynchronous and return `202 Accepted` with tracking headers:

### Tracking Headers
- `Location` - URL to poll for operation status
- `Azure-AsyncOperation` - URL for detailed operation status
- `Retry-After` - Recommended polling interval in seconds

### Polling Pattern
```bash
# 1. Initial request returns 202 with Location header
curl -X POST 'https://management.azure.com/.../backup' \
  -H 'Authorization: Bearer $TOKEN'

# 2. Poll the Location URL until completion
curl -X GET 'https://management.azure.com/.../operationResults/...' \
  -H 'Authorization: Bearer $TOKEN'
```

## Best Practices

### Authentication
- Use service principal authentication for automated scenarios
- Implement token refresh logic for long-running operations
- Store credentials securely (Azure Key Vault recommended)

### API Usage
- Respect rate limits and implement exponential backoff
- Use appropriate API versions for stability
- Implement proper error handling and retry logic
- Poll asynchronous operations at recommended intervals

### Resource Naming
- Use consistent naming conventions for containers and items
- Follow Azure naming guidelines for resources
- Include environment and purpose in resource names

## Complete Workflow Examples

### End-to-End SAP ASE Backup Setup

1. **Create Recovery Services Vault**
2. **Register VM Container**
3. **Trigger Database Discovery**
4. **Create Backup Policy**
5. **Configure Protection**
6. **Trigger Backup**
7. **Monitor Job Status**

### Restore Workflow

1. **List Recovery Points**
2. **Choose Restore Type (Full/Point-in-Time)**
3. **Trigger Restore Operation**
4. **Monitor Restore Job**
5. **Verify Restored Data**

### Cleanup Workflow

1. **Stop Protection (with/without data deletion)**
2. **Unregister Container**
3. **Delete Recovery Services Vault (if needed)**
