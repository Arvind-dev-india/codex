# Enhanced Kusto Integration for Codex-RS

This document describes the comprehensive enhancements made to the Kusto (Azure Data Explorer) integration in codex-rs, including multiple database support and an intelligent knowledge base system.

## Overview

The enhanced Kusto integration provides:

1. **Multiple Database Support**: Configure and query multiple Kusto databases across different clusters
2. **Intelligent Knowledge Base**: Automatically learns and caches table schemas, query patterns, and metadata
3. **Enhanced Tools**: Extended set of tools for database exploration and knowledge management
4. **Backward Compatibility**: Maintains compatibility with existing single-database configurations

## Configuration

### Configuration Options Overview

The enhanced Kusto integration supports multiple configuration approaches, from simple single-database setups to complex multi-cluster environments.

### Option 1: Basic Configuration (Single Database)

**Minimum required configuration** - works exactly like before:

```toml
[kusto]
cluster_url = "https://help.kusto.windows.net"
database = "Samples"
```

This provides:
- Single database access
- Automatic knowledge base creation
- All enhanced tools available

### Option 2: Multiple Databases - Same Cluster

**Configure multiple databases on the same cluster:**

```toml
[kusto]
cluster_url = "https://help.kusto.windows.net"
database = "Samples"  # Fallback default

# Knowledge base configuration
knowledge_base_path = "kusto_knowledge_base.json"
auto_update_knowledge_base = true
max_knowledge_base_rows = 100

# Multiple databases on same cluster
[kusto.databases.samples]
name = "Samples"
description = "Sample database with demo data including StormEvents"
is_default = true

[kusto.databases.analytics]
name = "AnalyticsDB"
description = "Analytics and reporting database"

[kusto.databases.logs]
name = "ApplicationLogs"
description = "Application and system logs"
```

### Option 3: Multiple Databases - Different Clusters

**Configure databases across multiple clusters:**

```toml
[kusto]
cluster_url = "https://help.kusto.windows.net"  # Primary cluster
database = "Samples"

# Knowledge base configuration
knowledge_base_path = "kusto_knowledge_base.json"
auto_update_knowledge_base = true
max_knowledge_base_rows = 100

# Databases across different clusters
[kusto.databases.samples]
name = "Samples"
cluster_url = "https://help.kusto.windows.net"
description = "Sample database with demo data"
is_default = true

[kusto.databases.production]
name = "ProductionDB"
cluster_url = "https://prod-cluster.kusto.windows.net"
description = "Production database with real application data"

[kusto.databases.staging]
name = "StagingDB"
cluster_url = "https://staging-cluster.kusto.windows.net"
description = "Staging environment database"

[kusto.databases.analytics]
name = "AnalyticsWarehouse"
cluster_url = "https://analytics-cluster.kusto.windows.net"
description = "Data warehouse for analytics and reporting"
```

### Option 4: Auto-Discovery Configuration (Recommended)

**Minimal configuration with automatic database discovery:**

```toml
[kusto]
cluster_url = "https://help.kusto.windows.net"
auto_discover_databases = true
default_database = "Samples"  # Optional: specify which database to use by default

# Knowledge base configuration
knowledge_base_path = "kusto_knowledge_base.json"
auto_update_knowledge_base = true
max_knowledge_base_rows = 100

# Optional: Manual configuration for important databases
[kusto.databases.production]
name = "ProductionDB"
cluster_url = "https://prod-cluster.kusto.windows.net"
description = "Critical production data - manually configured"
priority = "high"
is_default = false
```

### Database Selection Behavior

#### **Automatic Database Selection**
When you don't specify a database in your query:
```bash
codex "show me the top 10 storm events"
```

The system selects databases in this priority order:
1. Database marked with `is_default = true`
2. The `default_database` field (if using auto-discovery)
3. The main `database` field
4. First discovered database (if using auto-discovery)

#### **Explicit Database Selection**
You can specify which database to use in natural language:
```bash
codex "query the production database for user activity"
codex "show tables in the analytics database"
codex "get schema from staging database for Users table"
```

The AI maps natural language to configured database names:
- "production database" → `ProductionDB`
- "analytics database" → `AnalyticsDB` or `AnalyticsWarehouse`
- "staging database" → `StagingDB`

#### **Tool-Level Database Selection**
For precise control, specify the exact database name:
```bash
codex "execute query 'Users | take 10' on database ProductionDB"
codex "list tables in database AnalyticsWarehouse"
```

## Auto-Discovery Feature

### Overview

The auto-discovery feature automatically finds and configures all Kusto databases you have access to, eliminating the need for manual configuration of each database.

### How Auto-Discovery Works

1. **Cluster Scanning**: Uses `.show databases` command to list all accessible databases
2. **Metadata Collection**: Gathers database properties, descriptions, and access permissions
3. **Knowledge Base Integration**: Automatically adds discovered databases to the knowledge base
4. **Smart Caching**: Caches discovery results with configurable refresh intervals

### Enabling Auto-Discovery

#### Minimal Configuration
```toml
[kusto]
cluster_url = "https://help.kusto.windows.net"
auto_discover_databases = true
```

#### Advanced Auto-Discovery Configuration
```toml
[kusto]
cluster_url = "https://help.kusto.windows.net"
auto_discover_databases = true
default_database = "Samples"
discovery_cache_ttl_hours = 24  # Refresh discovery every 24 hours
discovery_include_system_databases = false  # Exclude system databases

# Additional clusters for discovery
discovery_clusters = [
    "https://prod-cluster.kusto.windows.net",
    "https://analytics-cluster.kusto.windows.net"
]
```

### Auto-Discovery Benefits

1. **Zero Configuration**: Works immediately with just cluster URL
2. **Always Current**: Automatically discovers new databases as they're created
3. **Reduced Maintenance**: No need to update config files when databases change
4. **Better AI Context**: AI knows about all available databases for better suggestions

### Manual Override

You can combine auto-discovery with manual configuration for important databases:

```toml
[kusto]
auto_discover_databases = true
cluster_url = "https://help.kusto.windows.net"

# Manual configuration takes precedence over auto-discovery
[kusto.databases.production]
name = "ProductionDB"
cluster_url = "https://prod-cluster.kusto.windows.net"
description = "Critical production data - manually configured for better control"
priority = "high"
is_default = false
```

### Discovery Commands

- **Trigger Discovery**: `codex "discover all available Kusto databases"`
- **Refresh Discovery**: `codex "refresh the database list"`
- **Show Discovered**: `codex "show me all discovered databases"`

## Knowledge Base System

### Automatic Learning

The knowledge base automatically captures and stores:

- **Database Information**: Names, descriptions, cluster URLs, access patterns
- **Table Schemas**: Column names, data types, descriptions, sample data
- **Query Patterns**: Common query structures, usage frequency, performance insights
- **Function Definitions**: Built-in Kusto functions with examples and documentation

### Knowledge Base Structure

The knowledge base is stored as a JSON file with the following structure:

```json
{
  "version": "1.0",
  "last_updated": "2024-01-15T10:30:00Z",
  "databases": {
    "Samples": {
      "name": "Samples",
      "description": "Sample database",
      "cluster_url": "https://help.kusto.windows.net",
      "tables": {
        "StormEvents": {
          "name": "StormEvents",
          "description": "Weather event data",
          "columns": [
            {
              "name": "StartTime",
              "data_type": "datetime",
              "description": "Event start time",
              "sample_values": ["2007-01-01T00:00:00Z"]
            }
          ],
          "sample_data": [...],
          "query_count": 15,
          "last_accessed": "2024-01-15T10:25:00Z"
        }
      }
    }
  },
  "query_patterns": [...],
  "functions": {...}
}
```

### Benefits

1. **Faster Queries**: Cached schema information reduces API calls
2. **Better Suggestions**: AI can suggest relevant tables and columns
3. **Learning Over Time**: Knowledge base improves with usage
4. **Offline Capability**: Basic information available without network access

## Available Tools

### Core Query Tools

#### `kusto_execute_query`
Execute Kusto queries with automatic knowledge base updates.

**Parameters:**
- `query` (required): The KQL query to execute
- `database` (optional): Database name or alias

**Example:**
```
codex "Execute query: StormEvents | take 10"
```

#### `kusto_get_table_schema`
Get detailed schema information for a table.

**Parameters:**
- `table_name` (required): Name of the table
- `database` (optional): Database name or alias

**Example:**
```
codex "Get schema for StormEvents table"
```

#### `kusto_list_tables`
List all tables in a database.

**Parameters:**
- `database` (optional): Database name or alias

**Example:**
```
codex "List all tables in the analytics database"
```

### Database Management Tools

#### `kusto_list_databases`
List all configured databases and their information.

**Example:**
```
codex "Show me all available Kusto databases"
```

#### `kusto_discover_databases`
Automatically discover all databases you have access to on the configured clusters.

**Example:**
```
codex "Discover all available Kusto databases"
```

#### `kusto_refresh_database_list`
Refresh the list of discovered databases and update the knowledge base.

**Example:**
```
codex "Refresh the list of Kusto databases"
```

### Knowledge Base Tools

#### `kusto_get_knowledge_base_summary`
Get a comprehensive summary of the knowledge base.

**Example:**
```
codex "Show me a summary of what we know about our Kusto databases"
```

#### `kusto_search_knowledge_base`
Search the knowledge base for tables, columns, or patterns.

**Parameters:**
- `search_term` (required): Term to search for
- `search_type` (optional): Type of search ("tables", "columns", "patterns", "all")

**Example:**
```
codex "Search for tables related to 'user' in the knowledge base"
```

#### `kusto_update_table_description`
Update the description of a table in the knowledge base.

**Parameters:**
- `database` (required): Database name
- `table_name` (required): Table name
- `description` (required): New description

**Example:**
```
codex "Update the description of the Users table to 'Contains user profile and activity data'"
```

## Usage Examples

### Basic Queries

```bash
# Simple query (uses default database)
codex "Show me the top 10 storm events by damage"

# Query specific database by name
codex "Query the production database for recent user logins"
codex "Show tables in the analytics database"

# Get table information
codex "What columns are available in the StormEvents table?"
codex "Get schema for Users table in production database"
```

### Multi-Database Operations

```bash
# Database discovery and exploration
codex "What databases do we have access to?"
codex "Discover all available Kusto databases"
codex "Show me databases that contain user data"

# Cross-database queries
codex "Compare user counts between production and staging databases"
codex "Find tables with similar schemas across all databases"

# Database-specific operations
codex "List all tables in the analytics warehouse"
codex "Show recent activity in the logs database"
```

### Knowledge Base Interactions

```bash
# Explore cached knowledge
codex "Show me a summary of our Kusto knowledge base"
codex "What tables have we queried most frequently?"

# Search for specific information
codex "Find all tables that contain user information"
codex "Search for columns related to timestamps across all databases"
codex "Show me query patterns for the StormEvents table"

# Update and maintain knowledge base
codex "Update the description of the Events table to include that it contains clickstream data"
codex "Refresh the database list to find new databases"
```

### Advanced Analytics

```bash
# Pattern-based queries
codex "Show me common query patterns for the StormEvents table"
codex "What are the most efficient ways to query user activity data?"

# Cross-database analysis
codex "Compare data quality between production and staging environments"
codex "Find relationships between tables across different databases"

# Schema exploration and optimization
codex "What are the most commonly queried tables across all databases?"
codex "Show me tables that might benefit from indexing based on query patterns"
```

### Auto-Discovery Workflows

```bash
# Initial setup with auto-discovery
codex "Discover all databases I have access to"
codex "Show me what databases were found"

# Ongoing maintenance
codex "Refresh the list of available databases"
codex "Check if any new databases have been added"

# Smart database selection
codex "Which database should I use for user analytics?"
codex "Find the best database for storing application logs"
```

## Architecture

### Components

1. **KustoConfig**: Enhanced configuration supporting multiple databases
2. **KustoKnowledgeBase**: Intelligent caching and learning system
3. **KustoTools**: Extended tool implementations
4. **KustoClient**: Multi-database client management
5. **Integration Layer**: Seamless integration with codex-rs tool system

### Data Flow

1. **Query Execution**: User submits query → Tool determines database → Execute query → Update knowledge base
2. **Knowledge Learning**: Query results → Extract schema information → Update table metadata → Save to disk
3. **Smart Suggestions**: User request → Search knowledge base → Provide relevant context → Generate response

### Authentication

- **Dedicated Kusto OAuth**: Uses proper Kusto-specific OAuth scopes (not Azure DevOps scopes)
- **Automatic Authentication**: Prompts for authentication on first use
- **Token Management**: Separate token storage for Kusto (`~/.codex/kusto_auth.json`)
- **Multi-Cluster Support**: Supports multiple clusters with separate authentication
- **Automatic Refresh**: Tokens are automatically refreshed when needed
- **Secure Storage**: Tokens are stored securely with proper file permissions

#### First Time Setup
When you first use a Kusto tool, you'll see:
```
Kusto (Azure Data Explorer) Authentication Required
To sign in, use a web browser to open the page:
    https://microsoft.com/devicelogin
And enter the code: ABC123DEF

Waiting for authentication...
```

After completing authentication in your browser:
```
Kusto authentication successful!
```

Subsequent uses will automatically use the saved tokens.

## Migration Guide

### From Single Database Configuration

Existing configurations continue to work without changes:

```toml
[kusto]
cluster_url = "https://help.kusto.windows.net"
database = "Samples"
```

### Migration Options

#### Option A: Minimal Change (Add Auto-Discovery)

**Recommended for most users** - just add one line:

```toml
[kusto]
cluster_url = "https://help.kusto.windows.net"
database = "Samples"
auto_discover_databases = true  # Add this line
```

Benefits:
- Automatically discovers all accessible databases
- Maintains backward compatibility
- Zero additional configuration needed

#### Option B: Enhanced Manual Configuration

Add multiple databases and knowledge base features:

```toml
[kusto]
cluster_url = "https://help.kusto.windows.net"
database = "Samples"  # Keep for backward compatibility

# Add knowledge base
knowledge_base_path = "kusto_knowledge_base.json"
auto_update_knowledge_base = true

# Add additional databases
[kusto.databases.samples]
name = "Samples"
is_default = true

[kusto.databases.production]
name = "ProductionDB"
cluster_url = "https://prod.kusto.windows.net"
```

#### Option C: Hybrid Approach

Combine auto-discovery with manual configuration for critical databases:

```toml
[kusto]
cluster_url = "https://help.kusto.windows.net"
database = "Samples"
auto_discover_databases = true

# Manual configuration for critical databases
[kusto.databases.production]
name = "ProductionDB"
cluster_url = "https://prod-cluster.kusto.windows.net"
description = "Critical production data"
priority = "high"
```

### Migration Steps

1. **Backup Current Config**: Save your existing `config.toml`
2. **Choose Migration Option**: Pick the approach that fits your needs
3. **Update Configuration**: Add the new settings
4. **Test Discovery**: Run `codex "discover all databases"` if using auto-discovery
5. **Verify Access**: Test queries against different databases

## Performance Considerations

### Knowledge Base Size

- Default limit: 100 rows per table in sample data
- Configurable via `max_knowledge_base_rows`
- Automatic cleanup of old, unused entries

### Caching Strategy

- Schema information cached indefinitely until manually updated
- Query patterns updated with each execution
- Sample data refreshed when new queries provide different results

### Network Optimization

- Batch schema queries when possible
- Use cached information for repeated requests
- Lazy loading of detailed table information

## Security

### Data Privacy

- Sample data limited to configured row count
- No sensitive data stored in knowledge base by default
- Knowledge base file permissions match codex configuration

### Authentication

- Reuses existing Azure DevOps OAuth tokens
- Supports per-cluster authentication
- Automatic token refresh

## Troubleshooting

### Common Issues

1. **Knowledge Base Not Updating**
   - Check `auto_update_knowledge_base` setting
   - Verify write permissions to knowledge base file
   - Check codex logs for update errors

2. **Database Not Found**
   - Verify database configuration in `config.toml`
   - Check cluster URL and database name
   - Ensure proper authentication

3. **Schema Information Missing**
   - Run a simple query to trigger schema discovery
   - Check if table exists in the database
   - Verify permissions to access table metadata

### Debug Commands

```bash
# Check knowledge base status
codex "Show me the knowledge base summary"

# Verify database configuration
codex "List all configured databases"

# Test connectivity
codex "Execute a simple query: print 'test'"
```

## Future Enhancements

### Planned Features

1. **Query Optimization Suggestions**: Analyze query patterns and suggest improvements
2. **Schema Change Detection**: Automatically detect and update schema changes
3. **Cross-Database Relationships**: Map relationships between tables across databases
4. **Performance Metrics**: Track query performance and suggest optimizations
5. **Export/Import**: Share knowledge base between team members

### Extensibility

The knowledge base system is designed to be extensible:

- Custom metadata fields can be added
- Query pattern analysis can be enhanced
- Integration with other data sources is possible

## Contributing

When contributing to the Kusto integration:

1. Maintain backward compatibility
2. Update knowledge base schema version when making breaking changes
3. Add appropriate tests for new functionality
4. Update documentation for new features

## License

This enhancement is part of the codex-rs project and follows the same licensing terms.