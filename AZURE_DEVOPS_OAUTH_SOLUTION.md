# Azure DevOps OAuth Authentication - Solution Summary

## Problem Resolved

**Original Issue**: You were getting "either provide pat" errors even when you had configured OAuth in your TOML file.

**Root Cause**: The Azure DevOps tools implementation was still using the old authentication logic that only looked for Personal Access Tokens (PATs), ignoring the new `auth_method` configuration.

## Solution Implemented

### ✅ **Fixed Authentication Logic**

**File**: `codex-rs/core/src/azure_devops/tools_impl.rs`
- Updated `AzureDevOpsTools::new()` to be async and respect the `auth_method` configuration
- Added support for OAuth, PAT, and Auto authentication methods
- Integrated with the new OAuth handler

### ✅ **OAuth Device Code Flow**

**File**: `codex-rs/core/src/azure_devops/auth/oauth_auth.rs`
- Complete OAuth 2.0 Device Code Flow implementation
- Uses Microsoft's public client (same as Azure CLI)
- Automatic token refresh and persistence
- Secure token storage with proper file permissions

### ✅ **Enhanced Configuration**

**File**: `codex-rs/core/src/config_types/config_types_azure.rs`
- Added `AzureDevOpsAuthMethod` enum with `oauth`, `pat`, and `auto` options
- Backward compatible with existing PAT configurations
- Flexible configuration options

### ✅ **Updated Tool Handler**

**File**: `codex-rs/core/src/azure_devops/tool_handler.rs`
- Updated to use the new async authentication
- Properly handles OAuth token management

## How to Use

### 1. Update Your Configuration

Edit your `~/.codex/config.toml`:

```toml
[azure_devops]
organization_url = "https://dev.azure.com/your-organization"
auth_method = "oauth"  # 🔑 This is the key change!
default_project = "YourProject"
```

### 2. First Time Usage

Run any Azure DevOps command:

```bash
codex "find all bugs assigned to me"
```

You'll see:
```
🔐 Azure DevOps Authentication Required
To sign in, use a web browser to open the page:
    https://microsoft.com/devicelogin
And enter the code: ABC123XYZ

Waiting for authentication...
```

### 3. Complete Authentication

1. Open the URL in your browser
2. Enter the device code
3. Sign in with your Microsoft account
4. ✅ Authentication successful!

### 4. Subsequent Usage

Commands now work automatically - no more authentication needed for ~90 days:

```bash
codex "create a new task for API documentation"
codex "show me all pull requests that need review"
codex "update work item 1234 to set priority to high"
```

## Configuration Options

### Option 1: OAuth Only (Recommended)
```toml
[azure_devops]
organization_url = "https://dev.azure.com/your-organization"
auth_method = "oauth"
default_project = "YourProject"
```

### Option 2: Auto (OAuth with PAT Fallback)
```toml
[azure_devops]
organization_url = "https://dev.azure.com/your-organization"
auth_method = "auto"  # Try OAuth first, fall back to PAT
pat_env_var = "AZURE_DEVOPS_PAT"  # Optional fallback
default_project = "YourProject"
```

### Option 3: PAT Only (For Automation)
```toml
[azure_devops]
organization_url = "https://dev.azure.com/your-organization"
auth_method = "pat"
pat_env_var = "AZURE_DEVOPS_PAT"
default_project = "YourProject"
```

## Token Management

- **Storage**: `~/.codex/azure_devops_auth.json` (secure permissions)
- **Access Token**: Expires in ~1 hour, refreshes automatically
- **Refresh Token**: Expires in ~90 days
- **Re-authentication**: Only needed every ~90 days

## Benefits

### ✅ **User Experience**
- **Easy Setup**: No need to create PATs or app registrations
- **One-Time Auth**: Authenticate once, works for 90 days
- **Automatic Refresh**: No manual token management
- **Same as Azure CLI**: Familiar authentication flow

### ✅ **Security**
- **Short-lived Tokens**: Access tokens expire in 1 hour
- **Secure Storage**: Tokens stored with restricted file permissions
- **No App Registration**: Uses Microsoft's public client
- **Scoped Access**: Only requests necessary permissions

### ✅ **Compatibility**
- **Backward Compatible**: Existing PAT configs still work
- **Flexible**: Support for OAuth, PAT, or Auto modes
- **Migration Friendly**: Easy to switch between methods

## Files Modified/Created

### Core Implementation
- `codex-rs/core/src/azure_devops/auth/oauth_auth.rs` - OAuth implementation
- `codex-rs/core/src/azure_devops/auth.rs` - Enhanced auth handler
- `codex-rs/core/src/azure_devops/tools_impl.rs` - Updated tools with OAuth support
- `codex-rs/core/src/azure_devops/tool_handler.rs` - Async auth integration
- `codex-rs/core/src/config_types/config_types_azure.rs` - Configuration types

### Documentation
- `AZURE_DEVOPS_USAGE.md` - Comprehensive usage guide
- `azure_oauth_config_example.toml` - Configuration examples
- `AZURE_DEVOPS_OAUTH_IMPLEMENTATION.md` - Technical details

### Tests
- `codex-rs/core/tests/azure_devops_oauth.rs` - OAuth unit tests
- `codex-rs/core/tests/azure_devops_oauth_integration.rs` - Integration tests
- `codex-rs/core/tests/azure_devops_config_parsing.rs` - Configuration tests

## Answer to Your Original Question

**"Will device code flow need to run every time?"**

**NO!** 🎉

- **First time**: One-time device code authentication
- **Daily usage**: Automatic, no authentication needed
- **Token refresh**: Happens transparently every hour
- **Re-authentication**: Only every ~90 days when refresh token expires

The OAuth implementation provides the same seamless experience as Azure CLI and VS Code - authenticate once, then it "just works" for months.

## Testing

All tests pass:
```bash
cd codex-rs && cargo test -p codex-core azure_devops
```

The implementation is ready for use! 🚀