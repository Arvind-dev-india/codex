# Recovery Services Async Operations Implementation

## Summary

Successfully implemented comprehensive async operation handling for Azure Recovery Services operations. All major operations now properly detect, track, and provide monitoring capabilities for asynchronous Azure operations.

## Key Changes Made

### 1. Enhanced Response Handling (client.rs)
- Modified `handle_response()` to detect HTTP 202 (Accepted) responses
- Extracts `Location` and `Azure-AsyncOperation` headers for tracking
- Returns structured information about async operations

### 2. Async Operation Tracking Methods (client.rs)
- `track_async_operation()` - Check current status of an async operation
- `wait_for_async_operation()` - Wait for operation completion with timeout
- Both methods use the Location header URL to poll operation status

### 3. Helper Function for Consistent Handling (tools_impl.rs)
- `handle_async_operation()` - Centralized async operation handling
- Provides initial status check after operation initiation
- Returns tracking information and recommendations for monitoring
- Consistent status interpretation across all operations

### 4. New Tool for Manual Tracking (tools.rs)
- `recovery_services_track_async_operation` tool
- Allows users to manually track any async operation using Location URL
- Supports both one-time status checks and waiting for completion

## Operations Updated with Async Handling

All the following operations now properly handle async operations:

1. **VM Registration Operations**
   - `register_vm` - VM registration for backup
   - `reregister_vm` - VM re-registration for backup
   - `unregister_vm` - VM unregistration from backup

2. **Protection Management Operations**
   - `enable_protection` - Enable backup protection for VM
   - `disable_protection` - Disable backup protection for VM

3. **Backup Operations**
   - `trigger_backup` - Trigger on-demand backup

4. **Restore Operations**
   - `restore_original_location` - Restore VM to original location
   - `restore_alternate_location` - Restore VM to alternate location
   - `restore_as_files` - Restore VM as files/disks

5. **Policy Operations**
   - `create_policy` - Create backup policy

## Expected Output Format

All async operations now return consistent output:

```json
{
  "operation": "operation_name",
  "status": "async_operation_initiated|completed|failed|in_progress",
  "async_operation": {
    "status": "accepted",
    "location_header": "https://management.azure.com/...",
    "message": "Operation accepted and is running asynchronously..."
  },
  "tracking_info": {
    "message": "Operation is running asynchronously...",
    "location_url": "https://management.azure.com/...",
    "recommended_action": "Call recovery_services_track_async_operation..."
  },
  "initial_status_check": {
    "status": "InProgress|Succeeded|Failed"
  },
  "result": { /* Original API response */ }
}
```

## Usage Examples

### 1. Automatic Async Handling
When you run any operation (e.g., `recovery_services_reregister_vm`), it will:
- Detect if the operation is async
- Provide tracking information
- Perform an initial status check
- Return recommendations for monitoring

### 2. Manual Tracking
Use the new tracking tool to monitor progress:
```json
{
  "location_url": "https://management.azure.com/subscriptions/.../operationResults/...",
  "wait_for_completion": false
}
```

### 3. Wait for Completion
Set `wait_for_completion: true` to automatically wait:
```json
{
  "location_url": "https://management.azure.com/subscriptions/.../operationResults/...",
  "wait_for_completion": true,
  "max_wait_seconds": 300
}
```

## Benefits

1. **Consistent Experience**: All async operations behave the same way
2. **Proper Job Tracking**: Users can now see and monitor async operations
3. **Immediate Feedback**: Initial status check provides quick feedback
4. **Flexible Monitoring**: Choose between immediate status check or waiting for completion
5. **Clear Guidance**: Recommendations on how to track operations
6. **Error Handling**: Proper handling of async operation failures

## Resolution of Original Issue

The original issue was that `recovery_services_reregister_vm` didn't show any running jobs. Now:
- The operation properly detects Azure's async response (202 Accepted)
- Provides Location header for tracking the actual job
- Shows initial status and provides tracking recommendations
- Users can monitor the real Azure job progress using the tracking tools

This resolves the problem where users couldn't see the actual Azure jobs running in the background.