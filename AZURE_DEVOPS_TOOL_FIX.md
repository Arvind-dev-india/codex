# Azure DevOps Tool Integration Fix

## Issue Description

The Azure DevOps integration was properly implemented in the codebase, but there was a mismatch between how the tools were registered and how they were expected to be called. This resulted in the error message:

```
unsupported call: azure_devops_query_work_items
```

## Root Cause

The issue was in the tool name handling:

1. The Azure DevOps tools were registered with names like `azure_devops_query_work_items` in `tools.rs`.
2. The MCP connection manager in `mcp_connection_manager.rs` was expecting the server name to be `azure_devops` and the tool name to be just `query_work_items`.
3. When the model called `azure_devops_query_work_items`, the MCP connection manager couldn't properly route the call.

## Solution

We modified the `call_tool` method in `mcp_connection_manager.rs` to handle both formats:

1. The original format: server=`azure_devops`, tool=`query_work_items`
2. The direct format: server=`""` (empty), tool=`azure_devops_query_work_items`

The changes allow the MCP connection manager to recognize Azure DevOps tool calls regardless of how they're formatted, making the integration more robust.

## Changes Made

1. Updated `mcp_connection_manager.rs` to detect Azure DevOps tools by checking if:
   - The server is explicitly "azure_devops", OR
   - The server is empty and the tool name starts with "azure_devops_"

2. Added logic to extract the correct tool name in both cases.

## Testing

To test this fix:

1. Configure Azure DevOps in your `~/.codex/config.toml` file as described in `AZURE_DEVOPS_USAGE.md`
2. Set your Azure DevOps PAT in the environment variable
3. Try querying work items with:
   ```
   codex "Find all work items assigned to me in the project"
   ```

The tool should now correctly process the Azure DevOps function calls.

## Additional Notes

This fix maintains backward compatibility with any existing code that might be using the original format (server=`azure_devops`, tool=`query_work_items`), while also supporting the direct format that the model seems to prefer.