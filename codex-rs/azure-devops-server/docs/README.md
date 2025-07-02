# Azure DevOps MCP Server Documentation

Welcome to the Azure DevOps MCP Server documentation. This server provides Azure DevOps functionality through the Model Context Protocol (MCP).

## Documentation Index

### Getting Started
- **[Quick Start Guide](quick-start.md)** - Get up and running in minutes
- **[Installation Guide](../README.md#installation)** - Detailed installation instructions

### Reference
- **[Tool Reference](tool-reference.md)** - Complete reference for all available tools
- **[Configuration Reference](../README.md#configuration)** - Configuration options and examples

### Examples
- **[Example Configurations](../README.md#example-tool-calls)** - Sample tool calls and configurations
- **[WIQL Query Examples](tool-reference.md#common-wiql-query-examples)** - Common Work Item Query Language examples

## Overview

The Azure DevOps MCP Server provides access to:

### Work Items
- Query work items using WIQL (Work Item Query Language)
- Create, read, update work items
- Add comments to work items
- Manage work item relationships

### Pull Requests
- Query and filter pull requests
- Get detailed pull request information
- Add comments and reviews
- Track pull request status

### Wiki
- Read wiki page content
- Update wiki pages
- Manage wiki structure

### Pipelines
- Trigger pipeline runs
- Monitor pipeline status
- Get build logs and results
- Manage pipeline parameters

## Key Features

- **MCP Protocol Compliance**: Full compatibility with MCP-enabled clients
- **Comprehensive API Coverage**: Access to major Azure DevOps features
- **Flexible Authentication**: Support for PAT tokens and OAuth
- **Rich Query Support**: Full WIQL support for complex work item queries
- **Error Handling**: Detailed error messages and status reporting
- **Configuration Options**: Multiple ways to configure and deploy

## Quick Example

Here's a simple example of querying work items:

```json
{
  "name": "azure_devops_query_work_items",
  "arguments": {
    "project": "MyProject",
    "query": "SELECT [System.Id], [System.Title] FROM WorkItems WHERE [System.State] = 'Active'"
  }
}
```

## Support

For issues and questions:

1. Check the [troubleshooting section](../README.md#troubleshooting)
2. Review the [tool reference](tool-reference.md) for parameter details
3. Verify your Azure DevOps permissions and configuration
4. Test with the provided [test script](../test-server.sh)

## Contributing

This server is part of the larger Codex project. Contributions are welcome following the project's contribution guidelines.