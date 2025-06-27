# Azure DevOps OAuth Authentication Demo

## Problem Solved

You were getting errors like "either provide pat" even when you had configured OAuth in your TOML file. This was because the Azure DevOps tools were still using the old authentication logic that only looked for Personal Access Tokens (PATs).

## What Was Fixed

1. **Updated Authentication Logic**: The `AzureDevOpsTools::new()` method now respects the `auth_method` configuration
2. **OAuth Integration**: Added full OAuth device code flow support
3. **Backward Compatibility**: Existing PAT configurations continue to work
4. **Flexible Configuration**: Support for `oauth`, `pat`, and `auto` authentication methods

## How to Use OAuth Authentication

### 1. Update Your Configuration

Edit your `~/.codex/config.toml`:

```toml
[azure_devops]
organization_url = "https://dev.azure.com/your-organization"
auth_method = "oauth"  # This is the key change!
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
And enter the code: FGH789XYZ

Waiting for authentication...
```

### 3. Complete Authentication

1. Open https://microsoft.com/devicelogin in your browser
2. Enter the code shown (e.g., `FGH789XYZ`)
3. Sign in with your Microsoft account
4. You'll see: `✅ Authentication successful!`

### 4. Subsequent Usage

From now on, commands work automatically:

```bash
codex "create a new task for updating documentation"
codex "show me all pull requests that need my review"
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

- **Storage**: Tokens are saved to `~/.codex/azure_devops_auth.json`
- **Security**: File has restricted permissions (600 - owner read/write only)
- **Refresh**: Access tokens refresh automatically every hour
- **Expiration**: Only need to re-authenticate every ~90 days

## Troubleshooting

### Still Getting PAT Errors?

1. **Check your config**: Make sure `auth_method = "oauth"` is set
2. **Restart**: Try restarting your terminal/shell
3. **Clear tokens**: Delete `~/.codex/azure_devops_auth.json` and try again

### Authentication Failed?

1. **Check organization URL**: Ensure it's correct (e.g., `https://dev.azure.com/your-org`)
2. **Network access**: Make sure you can reach `login.microsoftonline.com`
3. **Browser issues**: Try a different browser or incognito mode

### Permission Denied?

1. **Account permissions**: Ensure your Microsoft account has access to the Azure DevOps organization
2. **Organization settings**: Check if your organization allows OAuth applications

## Benefits of OAuth vs PAT

| Feature | OAuth | PAT |
|---------|-------|-----|
| Setup | Easy (no token creation) | Manual (create PAT) |
| Security | High (short-lived tokens) | Medium (long-lived) |
| Maintenance | Automatic | Manual renewal |
| User Experience | Best | Good |
| Browser Required | Yes (first time only) | No |

## Technical Details

- **Client ID**: Uses Azure CLI's public client (`04b07795-8ddb-461a-bbee-02f9e1bf7b46`)
- **Flow**: OAuth 2.0 Device Code Flow
- **Scopes**: `https://app.vssps.visualstudio.com/user_impersonation offline_access`
- **Endpoints**: Microsoft's standard OAuth endpoints

This implementation provides the same user experience as Azure CLI and VS Code extensions!