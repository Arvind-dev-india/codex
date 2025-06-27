# Kusto Authentication Fix - 401 Unauthorized Error Resolution

## Problem

Users were experiencing `401 Unauthorized` errors when trying to use Kusto tools, even though Azure DevOps authentication was working correctly.

## Root Cause

The issue was that the Kusto integration was incorrectly reusing the Azure DevOps OAuth handler, which uses Azure DevOps-specific OAuth scopes:

```rust
// Azure DevOps scopes (WRONG for Kusto)
const AZURE_DEVOPS_SCOPES: &str = "https://app.vssps.visualstudio.com/user_impersonation offline_access";
```

Kusto (Azure Data Explorer) requires different OAuth scopes to access its APIs:

```rust
// Kusto scopes (CORRECT)
const KUSTO_SCOPES: &str = "https://kusto.kusto.windows.net/user_impersonation offline_access";
```

## Solution

Created a dedicated `KustoOAuthHandler` with the correct OAuth scopes for Kusto authentication.

### Key Changes

1. **New Kusto OAuth Handler**: `codex-rs/core/src/kusto/auth.rs`
   - Uses correct Kusto OAuth scopes
   - Separate token storage (`kusto_auth.json` vs `azure_devops_auth.json`)
   - Dedicated device code flow for Kusto

2. **Correct OAuth Scopes**:
   ```rust
   const KUSTO_SCOPES: &str = "https://kusto.kusto.windows.net/user_impersonation offline_access";
   ```

3. **Separate Token Management**:
   - Kusto tokens: `~/.codex/kusto_auth.json`
   - Azure DevOps tokens: `~/.codex/azure_devops_auth.json`

## Authentication Flow

### First Time Setup

1. **User runs Kusto query**:
   ```bash
   codex "show me the top 10 rows from StormEvents table"
   ```

2. **System prompts for authentication**:
   ```
   Kusto (Azure Data Explorer) Authentication Required
   To sign in, use a web browser to open the page:
       https://microsoft.com/devicelogin
   And enter the code: ABC123DEF
   
   Waiting for authentication...
   ```

3. **User completes authentication in browser**

4. **System confirms success**:
   ```
   Kusto authentication successful!
   ```

5. **Tokens are saved** for future use

### Subsequent Usage

- Tokens are automatically loaded from `~/.codex/kusto_auth.json`
- Automatic token refresh when needed
- No re-authentication required until tokens expire

## OAuth Scopes Explained

### Azure DevOps Scope
```
https://app.vssps.visualstudio.com/user_impersonation
```
- Provides access to Azure DevOps services
- Used for work items, repositories, pipelines, etc.

### Kusto Scope
```
https://kusto.kusto.windows.net/user_impersonation
```
- Provides access to Azure Data Explorer clusters
- Used for querying databases, getting schemas, etc.

### Why Different Scopes Matter

Microsoft's OAuth system uses scopes to control access to different services. A token with Azure DevOps scopes cannot access Kusto APIs, and vice versa. This is a security feature to ensure applications only get access to the services they actually need.

## Configuration Requirements

No configuration changes are needed. The authentication will be triggered automatically when you first use a Kusto tool.

### Minimal Configuration
```toml
[kusto]
cluster_url = "https://help.kusto.windows.net"
database = "Samples"
```

### With Auto-Discovery
```toml
[kusto]
cluster_url = "https://help.kusto.windows.net"
database = "Samples"
auto_discover_databases = true
```

## Troubleshooting

### Still Getting 401 Errors?

1. **Clear existing tokens**:
   ```bash
   rm ~/.codex/kusto_auth.json
   ```

2. **Try authentication again**:
   ```bash
   codex "show databases"
   ```

3. **Check cluster URL**: Ensure your cluster URL is correct in the config

4. **Verify permissions**: Make sure your Azure account has access to the Kusto cluster

### Multiple Azure Accounts

If you have multiple Azure accounts:

1. **Use incognito/private browser window** for authentication
2. **Sign in with the correct account** that has Kusto access
3. **Complete the device code flow**

### Corporate Networks

If you're behind a corporate firewall:

1. **Ensure access to Microsoft OAuth endpoints**:
   - `https://login.microsoftonline.com`
   - `https://microsoft.com/devicelogin`

2. **Check proxy settings** if authentication fails

## Security Notes

### Token Storage
- Tokens are stored in `~/.codex/kusto_auth.json`
- File permissions are set to `600` (owner read/write only)
- Tokens are encrypted in transit but stored as JSON locally

### Token Lifecycle
- **Access tokens**: Valid for 1 hour
- **Refresh tokens**: Valid for 90 days
- **Automatic refresh**: Happens transparently when needed

### Logout
To clear stored tokens:
```bash
rm ~/.codex/kusto_auth.json
```

## Testing the Fix

### Test Basic Authentication
```bash
codex "show databases"
```

### Test Query Execution
```bash
codex "show me the top 5 rows from StormEvents table"
```

### Test Schema Access
```bash
codex "what columns are available in the StormEvents table?"
```

## Implementation Details

### File Structure
```
codex-rs/core/src/kusto/
├── auth.rs              # New dedicated Kusto OAuth handler
├── client.rs            # Kusto API client
├── knowledge_base.rs    # Knowledge base system
├── models.rs            # Data models
├── tools.rs             # Tool definitions
├── tools_impl.rs        # Tool implementations
└── tool_handler.rs      # Tool dispatcher
```

### Key Components

1. **KustoOAuthHandler**: Manages OAuth flow with correct scopes
2. **KustoAuthHandler**: Wrapper that provides auth headers
3. **Token Management**: Separate storage and refresh logic

### OAuth Flow Details

1. **Device Code Request**: Get device code with Kusto scopes
2. **User Authentication**: User signs in via browser
3. **Token Exchange**: Exchange device code for access/refresh tokens
4. **Token Storage**: Save tokens securely to disk
5. **Automatic Refresh**: Refresh tokens before expiry

## Future Enhancements

### Planned Improvements

1. **Shared Token Cache**: Explore sharing tokens between Azure services where appropriate
2. **Service Principal Support**: Add support for service principal authentication
3. **Managed Identity**: Support for Azure Managed Identity in cloud environments
4. **Token Encryption**: Encrypt tokens at rest for additional security

### Configuration Options

Future versions may support:

```toml
[kusto]
cluster_url = "https://help.kusto.windows.net"
database = "Samples"

# Authentication options
auth_method = "device_code"  # or "service_principal", "managed_identity"
tenant_id = "your-tenant-id"  # Optional: specific tenant
```

## Summary

The 401 Unauthorized error has been resolved by implementing proper Kusto-specific OAuth authentication with the correct scopes. Users will now be prompted to authenticate specifically for Kusto access, and tokens will be managed separately from Azure DevOps tokens.

This fix ensures:
- ✅ Proper OAuth scopes for Kusto access
- ✅ Separate token management for different services
- ✅ Automatic token refresh
- ✅ Secure token storage
- ✅ Clear authentication prompts
- ✅ No configuration changes required