# Azure DevOps OAuth Authentication Implementation

## Overview

This implementation adds OAuth device code flow authentication to the Azure DevOps integration in Codex CLI, providing an alternative to Personal Access Tokens (PATs) that doesn't require users to create Azure app registrations.

## What's Been Implemented

### 1. OAuth Device Code Flow Authentication

**File**: `codex-rs/core/src/azure_devops/auth/oauth_auth.rs`

- **Device Code Flow**: Uses Microsoft's public client ID (same as Azure CLI and VS Code)
- **Token Management**: Automatic token refresh and persistence
- **User Experience**: Simple browser-based authentication with device codes
- **Security**: Tokens stored with restricted file permissions (600)

### 2. Enhanced Authentication Handler

**File**: `codex-rs/core/src/azure_devops/auth.rs`

- **Multiple Auth Methods**: OAuth, PAT, or Auto (OAuth with PAT fallback)
- **Seamless Integration**: Works with existing Azure DevOps tools
- **Backward Compatibility**: Existing PAT configurations continue to work

### 3. Configuration Support

**File**: `codex-rs/core/src/config_types/config_types_azure.rs`

- **Auth Method Selection**: Users can choose `oauth`, `pat`, or `auto`
- **Flexible Configuration**: Supports both OAuth and PAT in same config
- **Default Behavior**: Auto mode tries OAuth first, falls back to PAT

### 4. Documentation and Examples

**Files**: 
- `AZURE_DEVOPS_USAGE.md` - Comprehensive usage guide
- `azure_oauth_config_example.toml` - Configuration examples

## How It Works

### First Time Authentication (OAuth)

1. User runs any Azure DevOps command
2. If no valid tokens exist, device code flow initiates:
   ```
   Azure DevOps Authentication Required
   To sign in, use a web browser to open the page:
       https://microsoft.com/devicelogin
   And enter the code: FGH789XYZ
   
   Waiting for authentication...
   ```
3. User opens URL, enters code, signs in with Microsoft account
4. Tokens are saved to `~/.codex/azure_devops_auth.json`
5. Command proceeds with authenticated API calls

### Subsequent Usage

1. Tool checks for existing tokens
2. If access token is valid (not expired) → use directly
3. If access token expired but refresh token valid → automatically refresh
4. If both expired → prompt for re-authentication (every ~90 days)
5. All token management is transparent to the user

### Token Lifecycle

- **Access Token**: Expires in ~1 hour, used for API calls
- **Refresh Token**: Expires in ~90 days, used to get new access tokens
- **Automatic Refresh**: Happens transparently when access token expires
- **Re-authentication**: Only needed when refresh token expires

## Configuration Options

### Option 1: OAuth Only (Recommended)
```toml
[azure_devops]
organization_url = "https://dev.azure.com/your-organization"
auth_method = "oauth"
default_project = "YourProject"
```

### Option 2: PAT Only (For Automation)
```toml
[azure_devops]
organization_url = "https://dev.azure.com/your-organization"
auth_method = "pat"
pat_env_var = "AZURE_DEVOPS_PAT"
default_project = "YourProject"
```

### Option 3: Auto (Default - OAuth with PAT Fallback)
```toml
[azure_devops]
organization_url = "https://dev.azure.com/your-organization"
auth_method = "auto"  # Try OAuth first, fall back to PAT
pat_env_var = "AZURE_DEVOPS_PAT"  # Optional fallback
default_project = "YourProject"
```

## Security Features

1. **No App Registration Required**: Uses Microsoft's public client
2. **Short-lived Tokens**: Access tokens expire in 1 hour
3. **Secure Storage**: Tokens stored with 600 file permissions
4. **Automatic Refresh**: Reduces token exposure time
5. **Scoped Access**: Only requests necessary Azure DevOps permissions

## Comparison: OAuth vs PAT

| Feature | OAuth Device Code | Personal Access Token |
|---------|------------------|----------------------|
| **Setup** | No token creation needed | Must create PAT manually |
| **Security** | High (short-lived tokens) | Medium (long-lived token) |
| **User Experience** | Best (automatic refresh) | Good (manual renewal) |
| **Automation** | Good (90-day refresh) | Excellent (long-lived) |
| **Browser Required** | Yes (first time only) | No |
| **Token Management** | Automatic | Manual |

## Benefits Over PAT-Only Approach

1. **Easier Setup**: No need to navigate Azure DevOps settings to create tokens
2. **Better Security**: Short-lived access tokens vs long-lived PATs
3. **Automatic Renewal**: No manual token rotation needed
4. **Better UX**: One-time browser authentication vs token management
5. **Consistent with Other Tools**: Same flow as Azure CLI, VS Code, etc.

## Implementation Details

### Microsoft OAuth Endpoints Used
- **Device Code**: `https://login.microsoftonline.com/common/oauth2/v2.0/devicecode`
- **Token Exchange**: `https://login.microsoftonline.com/common/oauth2/v2.0/token`

### Client ID
- Uses Azure CLI's public client ID: `04b07795-8ddb-461a-bbee-02f9e1bf7b46`
- No app registration required
- Same client used by Azure CLI and VS Code extensions

### Scopes Requested
- `https://app.vssps.visualstudio.com/user_impersonation` - Azure DevOps access
- `offline_access` - Refresh token capability

### File Storage
- **Location**: `~/.codex/azure_devops_auth.json`
- **Permissions**: 600 (owner read/write only)
- **Format**: JSON with access_token, refresh_token, and expiration times

## Testing

**File**: `codex-rs/core/tests/azure_devops_oauth.rs`

- Unit tests for OAuth handler creation
- Token serialization/deserialization tests
- Authentication method selection tests
- Integration with existing PAT authentication

## Future Enhancements

1. **Azure CLI Integration**: Could leverage existing `az` tokens
2. **Interactive Browser Flow**: Alternative to device code flow
3. **Managed Identity Support**: For Azure-hosted scenarios
4. **Token Caching**: Cross-session token sharing
5. **Logout Command**: Explicit token clearing

## Migration Path

Existing users with PAT configurations:
1. **No Changes Required**: PAT authentication continues to work
2. **Gradual Migration**: Can switch to `auth_method = "auto"` for best of both
3. **Full OAuth**: Can switch to `auth_method = "oauth"` for OAuth-only

## Answer to Original Question

**"Will device code flow need to run every time?"**

**No!** The device code flow only runs:
1. **First time** you use Azure DevOps commands (one-time setup)
2. **Every ~90 days** when refresh tokens expire (automatic prompt)

For day-to-day usage:
- ✅ Tokens are saved and reused automatically
- ✅ Access tokens refresh transparently (every hour)
- ✅ No browser interaction needed for 90 days
- ✅ Same experience as Azure CLI, VS Code, etc.

This provides the best of both worlds: easy setup without ongoing authentication friction.