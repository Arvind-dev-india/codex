# Using Azure DevOps Integration with Codex CLI

This guide explains how to set up and use the Azure DevOps integration with Codex CLI.

## Setup

### 1. Configure Azure DevOps in your Codex config

Add the following to your `~/.codex/config.toml` file:

```toml
[azure_devops]
organization_url = "https://dev.azure.com/your-organization"
auth_method = "oauth"  # Recommended: use OAuth device code flow
default_project = "YourProject"  # Optional: Default project to use
```

### 2. Choose your authentication method

Codex CLI supports multiple authentication methods for Azure DevOps:

#### Option A: OAuth Device Code Flow (Recommended)

This is the easiest method - no need to create tokens or app registrations:

```toml
[azure_devops]
organization_url = "https://dev.azure.com/your-organization"
auth_method = "oauth"  # Use OAuth device code flow
default_project = "YourProject"
```

**First time usage:**
1. Run any Azure DevOps command: `codex "find all bugs assigned to me"`
2. You'll see a prompt like:
   ```
   Azure DevOps Authentication Required
   To sign in, use a web browser to open the page:
       https://microsoft.com/devicelogin
   And enter the code: FGH789XYZ
   
   Waiting for authentication...
   ```
3. Open the URL in your browser and enter the code
4. Sign in with your Microsoft account
5. Authentication successful! Tokens are saved automatically

**Subsequent usage:**
- Just run commands normally - authentication is automatic
- Tokens refresh automatically when needed
- Only need to re-authenticate every ~90 days

#### Option B: Personal Access Token (PAT)

If you prefer using PATs or need them for automation:

```toml
[azure_devops]
organization_url = "https://dev.azure.com/your-organization"
auth_method = "pat"  # Use PAT only
pat_env_var = "AZURE_DEVOPS_PAT"  # Environment variable containing your PAT
default_project = "YourProject"
```

Create a PAT:
1. Go to `https://dev.azure.com/your-organization/_usersSettings/tokens`
2. Click "New Token"
3. Give it a name like "Codex CLI"
4. Select appropriate scopes (Work Items: Read & Write, Code: Read, etc.)
5. Copy the token and set it as an environment variable:

```bash
export AZURE_DEVOPS_PAT="your-pat-here"
```

#### Option C: Auto (Default)

Tries OAuth first, falls back to PAT if available:

```toml
[azure_devops]
organization_url = "https://dev.azure.com/your-organization"
auth_method = "auto"  # Try OAuth first, fall back to PAT
pat_env_var = "AZURE_DEVOPS_PAT"  # Optional fallback PAT
default_project = "YourProject"
```

## Usage Examples

### Work Items

```bash
# Query work items
codex "Find all high priority bugs assigned to me"
codex "Show me all tasks in the current sprint"
codex "List work items created this week"

# Create work items
codex "Create a new bug titled 'Login page crashes on mobile'"
codex "Create a task to update the documentation"

# Update work items
codex "Update work item 1234 to set priority to high"
codex "Add a comment to work item 5678 saying 'Fixed in latest build'"
```

### Pull Requests

```bash
# Query pull requests
codex "Show me all active pull requests that need my review"
codex "Find pull requests created by john.doe@company.com"

# Comment on pull requests
codex "Add a comment to PR #45 asking for more test coverage"
```

### Pipelines

```bash
# Run pipelines
codex "Run the 'Build and Deploy' pipeline on the main branch"
codex "Trigger the CI pipeline for the feature/new-login branch"

# Check pipeline status
codex "Show me the status of the last 5 pipeline runs"
```

### Wiki

```bash
# Read wiki pages
codex "Show me the content of the 'Getting Started' wiki page"

# Update wiki pages
codex "Update the API documentation wiki page with the new endpoints"
```

## Authentication Management

### Check authentication status
```bash
codex "What Azure DevOps organization am I connected to?"
```

### Re-authenticate (if needed)
If you need to switch accounts or re-authenticate:

1. **For OAuth**: Delete the saved tokens:
   ```bash
   rm ~/.codex/azure_devops_auth.json
   ```
   Next command will prompt for authentication again.

2. **For PAT**: Update your environment variable:
   ```bash
   export AZURE_DEVOPS_PAT="new-pat-token"
   ```

### Logout
```bash
# This would clear saved OAuth tokens
codex "logout from Azure DevOps"
```

## Troubleshooting

### "Authentication failed" errors
1. **OAuth**: Try deleting `~/.codex/azure_devops_auth.json` and re-authenticating
2. **PAT**: Check that your PAT hasn't expired and has the right scopes
3. **Organization URL**: Ensure the URL is correct (e.g., `https://dev.azure.com/your-org`)

### "Permission denied" errors
- Check that your account has the necessary permissions in Azure DevOps
- For PATs, verify the token has the required scopes

### Network/proxy issues
- OAuth authentication requires internet access to `login.microsoftonline.com`
- If behind a corporate proxy, ensure it allows access to Microsoft authentication endpoints

## Security Notes

- **OAuth tokens** are stored in `~/.codex/azure_devops_auth.json` with restricted file permissions (600)
- **PATs** should be stored in environment variables, not directly in config files
- Tokens are automatically refreshed when possible
- OAuth tokens expire after ~90 days and will prompt for re-authentication

## Comparison: OAuth vs PAT

| Feature | OAuth Device Code | Personal Access Token |
|---------|------------------|----------------------|
| Setup complexity | Easy (no token creation) | Medium (create PAT) |
| Security | High (short-lived tokens) | Medium (long-lived token) |
| User experience | Best (automatic refresh) | Good (manual renewal) |
| Automation friendly | Good | Excellent |
| Requires browser | Yes (first time only) | No |
| Token management | Automatic | Manual |

**Recommendation**: Use OAuth for interactive use, PAT for automation/CI scenarios.