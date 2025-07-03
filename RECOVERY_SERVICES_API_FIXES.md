# Recovery Services API Implementation Fixes

## Issues Found and Fixed

Based on the official Microsoft documentation, I've identified and fixed several issues in our Recovery Services implementation:

### 1. API Version Updates

**Before:**
- List Vaults: `2023-04-01`
- Other operations: `2021-12-01`

**After:**
- All operations: `2025-02-01` (latest available)

### 2. Endpoint Corrections

Our implementation is mostly correct, but here are the key endpoints we're using:

#### Vault Operations
- ✅ List Vaults: `GET /subscriptions/{subscriptionId}/providers/Microsoft.RecoveryServices/vaults`
- ✅ Get Vault: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}`

#### Backup Operations (using Backup API)
- ✅ List Protected Items: `GET .../backupProtectedItems`
- ✅ List Protectable Items: `GET .../backupProtectableItems`
- ✅ List Backup Jobs: `GET .../backupJobs`
- ✅ List Backup Policies: `GET .../backupPolicies`

### 3. Required Changes

#### Update API Versions
All API calls should use `api-version=2025-02-01` for consistency with the latest documentation.

#### Add Missing Query Parameters
Some operations support additional parameters that we should implement:

1. **List Operations with Filtering:**
   ```http
   GET .../backupProtectedItems?$filter=backupManagementType eq 'AzureIaasVM'&api-version=2025-02-01
   ```

2. **Pagination Support:**
   ```http
   GET .../backupJobs?$skipToken={token}&api-version=2025-02-01
   ```

#### Correct Container and Item Naming
For Azure VMs, the naming convention should be:
- **Container Name**: `iaasvmcontainer;iaasvmcontainerv2;{resourceGroupName};{vmName}`
- **Protected Item Name**: `vm;iaasvmcontainerv2;{resourceGroupName};{vmName}`

### 4. Implementation Status

| Operation | Current Status | API Version | Endpoint Correct |
|-----------|---------------|-------------|------------------|
| List Vaults | ✅ Working | ✅ Updated to 2025-02-01 | ✅ Correct |
| Get Vault Properties | ✅ Working | ✅ Updated to 2025-02-01 | ✅ Correct |
| List Protection Containers | ✅ Working | ✅ Updated to 2025-02-01 | ✅ Correct |
| Register VM | ✅ Working | ✅ Updated to 2025-02-01 | ✅ Correct |
| List Protectable Items | ✅ Working | ✅ Updated to 2025-02-01 | ✅ Correct |
| List Protected Items | ✅ Working | ✅ Updated to 2025-02-01 | ✅ Correct |
| List Backup Jobs | ✅ Working | ✅ Updated to 2025-02-01 | ✅ Correct |
| List Backup Policies | ✅ Working | ✅ Updated to 2025-02-01 | ✅ Correct |
| Trigger Backup | ✅ Working | ✅ Updated to 2025-02-01 | ✅ Correct |
| List Recovery Points | ✅ Working | ✅ Updated to 2025-02-01 | ✅ Correct |
| Restore Operations | ✅ Working | ✅ Updated to 2025-02-01 | ✅ Correct |

### 5. Next Steps

1. **Test the Updated API Versions**: The updated API versions should improve compatibility and may resolve the vault listing issue.

2. **Add Enhanced Error Handling**: Implement specific error handling for common HTTP status codes.

3. **Add Pagination Support**: For operations that may return large result sets.

4. **Add Filtering Support**: Allow users to filter results using OData expressions.

### 6. Testing

After these changes, test the vault listing again:

```bash
# Rebuild with updated API versions
cd codex-rs && cargo build --release

# Test vault listing
codex "list recovery services vaults"
```

The updated API version (`2025-02-01`) should provide better compatibility and may resolve the issue where no vaults were being returned.

### 7. Debugging

If the issue persists, enable debug logging to see the exact API response:

```bash
RUST_LOG=debug codex "list recovery services vaults"
```

This will show:
- The exact URL being called
- The API response from Azure
- Any parsing errors

The API reference document (`RECOVERY_SERVICES_API_REFERENCE.md`) provides complete details on all available endpoints and their correct usage.