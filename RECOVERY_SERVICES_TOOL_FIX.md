# Recovery Services Tool Fix

## Issue

Recovery Services tools are registered correctly with OpenAI, but they're not being handled in the `handle_function_call` function in `codex.rs`. This is why the tools appear in the tool list but return "unsupported call" when used.

## Root Cause

In `codex.rs`, the `handle_function_call` function has special handling for:
- Azure DevOps tools (line 1299)
- Code Analysis tools (line 1308)
- Kusto tools (line 1328)

But it's missing a handler for Recovery Services tools that start with `recovery_services_`.

## Solution

Add a handler for Recovery Services tools in the `handle_function_call` function, similar to how Azure DevOps and Kusto tools are handled:

```rust
// Check for Recovery Services tools directly
name if name.starts_with("recovery_services_") => {
    // For Recovery Services tools, use "recovery_services" as server name
    let timeout = None;
    handle_mcp_tool_call(
        sess, &sub_id, call_id, "recovery_services".to_string(), name.to_string(), arguments, timeout,
    )
    .await
}
```

This should be added right after the Kusto tools handler (around line 1336).

## Implementation

```diff
// Check for Kusto tools directly
name if name.starts_with("kusto_") => {
    // For Kusto tools, use "kusto" as server name
    let timeout = None;
    handle_mcp_tool_call(
        sess, &sub_id, call_id, "kusto".to_string(), name.to_string(), arguments, timeout,
    )
    .await
}
+ // Check for Recovery Services tools directly
+ name if name.starts_with("recovery_services_") => {
+     // For Recovery Services tools, use "recovery_services" as server name
+     let timeout = None;
+     handle_mcp_tool_call(
+         sess, &sub_id, call_id, "recovery_services".to_string(), name.to_string(), arguments, timeout,
+     )
+     .await
+ }
_ => {
    match try_parse_fully_qualified_tool_name(&name) {
        // ...
    }
}
```

## Expected Behavior

After this change, when you call a Recovery Services tool like `recovery_services_test_connection`, it will be properly routed to the Recovery Services handler and you'll get the expected authentication prompt and functionality.

## Testing

After making this change, rebuild Codex and test with:

```bash
codex "test recovery services connection"
```

You should now get the authentication prompt and the tool should work correctly.