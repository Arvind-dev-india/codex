# Quickstart Guide: Codex CLI with Azure AI and Azure DevOps

This guide will help you set up and run Codex CLI with Azure OpenAI as the model provider and Azure DevOps integration.

## Step 1: Build the Rust Client

Follow these steps to build the Rust client:

```bash
# Clone the repository (if you haven't already)
git clone https://github.com/openai/codex.git
cd codex

# Build the Rust client
cd codex-rs
cargo build --release

# Create an alias for easier access (optional)
alias codex-rs="$(pwd)/target/release/codex"
```

For detailed build instructions, see [BUILD_INSTRUCTIONS.md](./BUILD_INSTRUCTIONS.md).

## Step 2: Set Up Environment Variables

Set the required environment variables for Azure OpenAI and Azure DevOps:

```bash
# Azure OpenAI API Key
export AZURE_OPENAI_API_KEY="your-azure-openai-api-key"

# Azure DevOps Personal Access Token
export AZURE_DEVOPS_PAT="your-azure-devops-pat"
```

Add these to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.) to make them permanent.

## Step 3: Create Configuration File

Create a configuration file at `~/.codex/config.toml` with the following content:

```toml
# Use Azure AI as the model provider
model = "gpt-4"
model_provider = "azure_openai"

# Azure OpenAI configuration
[model_providers.azure_openai]
name = "Azure OpenAI"
base_url = "https://YOUR_RESOURCE_NAME.openai.azure.com"
api_key_env_var = "AZURE_OPENAI_API_KEY"
api_version = "2023-12-01-preview"
deployment_id = "YOUR_DEPLOYMENT_NAME"  # Replace with your GPT-4 deployment name
wire_api = "chat"

# Azure DevOps configuration
[azure_devops]
organization_url = "https://dev.azure.com/your-organization"
pat_env_var = "AZURE_DEVOPS_PAT"
default_project = "YourProject"
```

Make sure to replace:
- `YOUR_RESOURCE_NAME` with your Azure OpenAI resource name
- `YOUR_DEPLOYMENT_NAME` with your GPT-4 deployment name
- `your-organization` with your Azure DevOps organization name
- `YourProject` with your default Azure DevOps project name

For a complete configuration example, see [azure_config_sample.toml](./azure_config_sample.toml).

## Step 4: Test Your Setup

Run a simple command to test your setup:

```bash
./target/release/codex "Hello, what can you do with Azure DevOps?"
```

## Step 5: Try Azure DevOps Integration

Try some Azure DevOps commands:

```bash
# Query work items
./target/release/codex "Show me all active bugs in my project"

# Create a work item
./target/release/codex "Create a new task titled 'Update documentation' with description 'We need to update the API docs'"

# Get wiki content
./target/release/codex "Show me the content of the 'Getting Started' page in our wiki"
```

## Troubleshooting

### Azure OpenAI Issues

If you encounter issues with Azure OpenAI:

1. Verify your API key is correct
2. Check that your deployment name matches the one in your Azure portal
3. Ensure your resource has the appropriate quota for the model you're using

### Azure DevOps Issues

If you encounter issues with Azure DevOps:

1. Verify your PAT is correct and not expired
2. Check that your PAT has the necessary permissions:
   - Work Items: Read & Write
   - Code: Read & Write
   - Pull Request Threads: Read & Write
   - Wiki: Read & Write
   - Build: Read & Execute
3. Ensure your organization URL is correct

For more detailed information, see:
- [Azure DevOps Usage Guide](./AZURE_DEVOPS_USAGE.md)
- [Configuration Guide](./codex-rs/config.md)