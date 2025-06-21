# Azure DevOps Integration for Codex CLI

## Overview

Codex CLI now includes Azure DevOps integration, allowing you to interact with your Azure DevOps projects using natural language commands. This integration enables you to:

- Query, create, and update work items
- Search and comment on pull requests
- Access and modify wiki pages
- Run and monitor pipelines

## Configuration

Add Azure DevOps configuration to your `~/.codex/config.toml`:

```toml
[azure_devops]
organization_url = "https://dev.azure.com/your-organization"
pat_env_var = "AZURE_DEVOPS_PAT"
default_project = "YourProject"
```

Set your Personal Access Token:

```bash
export AZURE_DEVOPS_PAT="your-pat-here"
```

## Usage Examples

```bash
# Work with work items
codex "Find all high priority bugs assigned to me"
codex "Create a new task titled 'Update documentation'"

# Work with pull requests
codex "Show me all active pull requests that need my review"
codex "Add a comment to PR #45"

# Work with wiki
codex "Show me the content of the 'Getting Started' wiki page"

# Work with pipelines
codex "Run the 'Build and Deploy' pipeline on the main branch"
```

For detailed usage instructions, see [AZURE_DEVOPS_USAGE.md](./AZURE_DEVOPS_USAGE.md).

## Building from Source

To build Codex CLI with Azure DevOps integration:

```bash
git clone https://github.com/your-username/codex.git
cd codex
git checkout feat/azure-devops-integration
cargo build --release -p codex-cli
```

The built binary will be available at `./target/release/codex`.