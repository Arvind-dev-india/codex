# Kusto (Azure Data Explorer) Integration for Codex CLI

## Overview

Codex CLI now includes Kusto (Azure Data Explorer) integration, allowing you to run queries against your Kusto clusters using natural language commands. This integration leverages the same OAuth authentication mechanism as Azure DevOps, providing a seamless experience.

## Features

- **Query Execution**: Run Kusto Query Language (KQL) queries against your Kusto clusters
- **Schema Exploration**: Get table schemas and list available tables
- **OAuth Authentication**: Uses the same authentication as Azure DevOps (no need for separate login)
- **Token Sharing**: Shares tokens with Azure DevOps for a unified experience

## Configuration

Add Kusto configuration to your `~/.codex/config.toml`:

```toml
[kusto]
cluster_url = "https://help.kusto.windows.net"
database = "Samples"
```

## Authentication

The Kusto integration uses the same OAuth device code flow as Azure DevOps:

1. **First Time**: You'll be prompted to authenticate via browser
2. **Token Sharing**: If you've already authenticated with Azure DevOps, those tokens will be reused
3. **Automatic Refresh**: Tokens are automatically refreshed when needed
4. **Persistent**: Authentication lasts for ~90 days before requiring re-authentication

## Available Tools

### kusto_execute_query

Execute a Kusto Query Language (KQL) query against your cluster.

```
codex "run the query 'StormEvents | take 10' against Kusto"
```

### kusto_get_table_schema

Get schema information for a specific table.

```
codex "show me the schema for the StormEvents table in Kusto"
```

### kusto_list_tables

List all available tables in the configured database.

```
codex "list all tables in the Kusto database"
```

## Example Queries

```
codex "find the top 5 states with the most storm events in the StormEvents table"
```

```
codex "calculate the average damage amount by event type from StormEvents"
```

```
codex "create a query to find outliers in the StormEvents data"
```

## Technical Details

- **Authentication**: OAuth 2.0 device code flow with Microsoft identity platform
- **Token Storage**: `~/.codex/azure_devops_auth.json` (shared with Azure DevOps)
- **API**: Uses the Kusto REST API
- **Query Format**: KQL (Kusto Query Language)

## Troubleshooting

### Authentication Issues

If you encounter authentication issues:

1. Delete `~/.codex/azure_devops_auth.json` to force re-authentication
2. Ensure your account has access to the Kusto cluster
3. Check that the cluster URL is correct

### Query Errors

Common query errors:

1. **Table not found**: Verify the table exists in the configured database
2. **Syntax errors**: Check your KQL syntax
3. **Permission denied**: Ensure your account has query permissions

## Security Notes

- **OAuth tokens** are stored in `~/.codex/azure_devops_auth.json` with restricted file permissions (600)
- **No credentials** are stored in the config file
- **Automatic token refresh** reduces the need for re-authentication
- **Scoped access** ensures only necessary permissions are requested