# Windows Authentication Guide for MCP Servers

This guide covers how to use the authentication features of the Azure DevOps, Kusto, and Recovery Services MCP servers on Windows.

## 📋 Prerequisites

### Required Software
- **Rust** (latest stable version)
  - Download from: https://rustup.rs/
  - Run the installer and follow the prompts
  - Restart your terminal after installation

### Optional Tools
- **Git for Windows** (for cloning the repository)
- **Windows Terminal** (recommended for better terminal experience)
- **PowerShell 7+** (recommended over Command Prompt)

## 🏗️ Building the Servers

### 1. Clone the Repository
```cmd
git clone https://github.com/your-org/codex.git
cd codex
```

### 2. Build All MCP Servers
```cmd
cd codex-rs
cargo build --release --bin azure-devops-server
cargo build --release --bin kusto-server
cargo build --release --bin recovery-services-server
```

### 3. Locate the Executables
After building, the executables will be located at:
```
codex-rs\target\release\azure-devops-server.exe
codex-rs\target\release\kusto-server.exe
codex-rs\target\release\recovery-services-server.exe
```

## 🔐 Authentication Commands

Each MCP server now includes built-in authentication commands that work independently of the MCP protocol.

### Azure DevOps Server

#### Login
```cmd
# Command Prompt
azure-devops-server.exe login

# PowerShell
.\azure-devops-server.exe login
```

#### Check Status
```cmd
# Command Prompt
azure-devops-server.exe status

# PowerShell
.\azure-devops-server.exe status
```

#### Logout
```cmd
# Command Prompt
azure-devops-server.exe logout

# PowerShell
.\azure-devops-server.exe logout
```

### Kusto (Azure Data Explorer) Server

#### Login
```cmd
# Command Prompt
kusto-server.exe login

# PowerShell
.\kusto-server.exe login
```

#### Check Status
```cmd
# Command Prompt
kusto-server.exe status

# PowerShell
.\kusto-server.exe status
```

#### Logout
```cmd
# Command Prompt
kusto-server.exe logout

# PowerShell
.\kusto-server.exe logout
```

### Recovery Services Server

#### Login
```cmd
# Command Prompt
recovery-services-server.exe login

# PowerShell
.\recovery-services-server.exe login
```

#### Check Status
```cmd
# Command Prompt
recovery-services-server.exe status

# PowerShell
.\recovery-services-server.exe status
```

#### Logout
```cmd
# Command Prompt
recovery-services-server.exe logout

# PowerShell
.\recovery-services-server.exe logout
```

## 📁 Token Storage Locations

Authentication tokens are stored in your Windows user profile:

- **Azure DevOps**: `C:\Users\{username}\.codex\azure_devops_auth.json`
- **Kusto**: `C:\Users\{username}\.codex\kusto_auth.json`
- **Recovery Services**: `C:\Users\{username}\.codex\recovery_services_auth.json`

These tokens are compatible with the main Codex application and Azure CLI tools.

## 🚀 Getting Started Workflow

### 1. First-Time Setup
```cmd
# Navigate to the built executables
cd codex-rs\target\release

# Check current authentication status
azure-devops-server.exe status
kusto-server.exe status
recovery-services-server.exe status
```

### 2. Authenticate with Services
```cmd
# Authenticate with Azure DevOps
azure-devops-server.exe login

# Authenticate with Kusto
kusto-server.exe login

# Authenticate with Recovery Services
recovery-services-server.exe login
```

### 3. Verify Authentication
```cmd
# Check all authentication statuses
azure-devops-server.exe status
kusto-server.exe status
recovery-services-server.exe status
```

### 4. Start MCP Servers
```cmd
# Start servers (default behavior - no subcommand needed)
azure-devops-server.exe
# or explicitly
azure-devops-server.exe serve
```

## 🔧 Configuration

### Server Configuration Files
Each server can use configuration files located at:
- `%USERPROFILE%\.codex\config.toml` (main Codex config)
- `azure_devops_config.toml` (standalone Azure DevOps config)
- `kusto_config.toml` (standalone Kusto config)
- `recovery_services_config.toml` (standalone Recovery Services config)

### Example Configuration
Create `%USERPROFILE%\.codex\config.toml`:
```toml
[azure_devops]
organization_url = "https://dev.azure.com/your-organization"
auth_method = "oauth"
default_project = "your-project"

[kusto]
cluster_url = "https://your-cluster.kusto.windows.net"
database = "your-database"

[recovery_services]
subscription_id = "your-subscription-id"
resource_group = "your-resource-group"
```

## 🛠️ Troubleshooting

### Common Issues

#### 1. "Command not found" Error
**Problem**: Windows can't find the executable
**Solution**: 
- Use the full path to the executable
- Or add the directory to your PATH environment variable

#### 2. Authentication Timeout
**Problem**: OAuth flow times out
**Solution**:
- Ensure your default browser is working
- Check firewall settings
- Try running with `--verbose` flag for more details

#### 3. Token File Permissions
**Problem**: Cannot read/write token files
**Solution**:
- Check that `%USERPROFILE%\.codex\` directory exists
- Ensure you have write permissions to your user profile
- Try running as administrator if needed

#### 4. Conditional Access Policy Issues
**Problem**: Tokens expire quickly due to corporate policies
**Solution**:
- Use the `status` command to check token validity
- Re-authenticate when tokens expire: `logout` then `login`

### Verbose Logging
For debugging, use the `--verbose` flag:
```cmd
azure-devops-server.exe --verbose login
kusto-server.exe --verbose status
```

### Manual Token Cleanup
If you need to manually clear tokens:
```cmd
# Delete token files
del "%USERPROFILE%\.codex\azure_devops_auth.json"
del "%USERPROFILE%\.codex\kusto_auth.json"
del "%USERPROFILE%\.codex\recovery_services_auth.json"
```

## 🔄 Integration with Other Tools

### Using with Codex
Once authenticated via the MCP servers, the tokens are automatically available to:
- Main Codex application
- Azure CLI (compatible format)
- Other tools that read from `%USERPROFILE%\.codex\`

### Using with MCP Clients
After authentication, you can use the servers with any MCP client:

#### Claude Desktop Configuration
Add to your Claude Desktop config file:
```json
{
  "mcpServers": {
    "azure-devops": {
      "command": "C:\\path\\to\\azure-devops-server.exe"
    },
    "kusto": {
      "command": "C:\\path\\to\\kusto-server.exe"
    },
    "recovery-services": {
      "command": "C:\\path\\to\\recovery-services-server.exe"
    }
  }
}
```

## 📚 Additional Resources

- [Azure DevOps REST API Documentation](https://docs.microsoft.com/en-us/rest/api/azure/devops/)
- [Kusto Query Language (KQL) Reference](https://docs.microsoft.com/en-us/azure/data-explorer/kusto/query/)
- [Azure Recovery Services Documentation](https://docs.microsoft.com/en-us/azure/backup/)
- [MCP Protocol Specification](https://modelcontextprotocol.io/)

## 🆘 Getting Help

If you encounter issues:

1. **Check Status First**: Run `{server}.exe status` to see current authentication state
2. **Try Verbose Mode**: Add `--verbose` flag for detailed logging
3. **Clear and Re-authenticate**: Use `logout` then `login` commands
4. **Check Configuration**: Verify your config files are properly formatted
5. **Review Logs**: Check Windows Event Viewer for system-level issues

## 🎯 Quick Reference

| Action | Azure DevOps | Kusto | Recovery Services |
|--------|--------------|-------|-------------------|
| **Login** | `azure-devops-server.exe login` | `kusto-server.exe login` | `recovery-services-server.exe login` |
| **Status** | `azure-devops-server.exe status` | `kusto-server.exe status` | `recovery-services-server.exe status` |
| **Logout** | `azure-devops-server.exe logout` | `kusto-server.exe logout` | `recovery-services-server.exe logout` |
| **Start Server** | `azure-devops-server.exe` | `kusto-server.exe` | `recovery-services-server.exe` |
| **Help** | `azure-devops-server.exe --help` | `kusto-server.exe --help` | `recovery-services-server.exe --help` |

---

**Note**: Replace `{username}` with your actual Windows username in file paths. The `.exe` extension is automatically added on Windows builds.