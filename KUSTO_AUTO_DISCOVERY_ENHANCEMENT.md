# Kusto Auto-Discovery Enhancement

## Proposed Feature: Automatic Database Discovery

### Current State
Users must manually configure each database in their `config.toml`:

```toml
[kusto.databases.mydb]
name = "MyDatabase"
description = "My database"
```

### Proposed Enhancement
Add automatic database discovery that:

1. **Scans Available Databases**: Automatically discovers all databases you have access to
2. **Updates Knowledge Base**: Adds discovered databases to the knowledge base
3. **Smart Configuration**: Minimal manual configuration required

### New Configuration Options

#### Minimal Configuration (Auto-Discovery Enabled)
```toml
[kusto]
cluster_url = "https://help.kusto.windows.net"
auto_discover_databases = true
default_database = "Samples"  # Optional: specify default
```

#### Mixed Configuration (Some Manual, Some Auto)
```toml
[kusto]
cluster_url = "https://help.kusto.windows.net"
auto_discover_databases = true

# Manual configuration for important databases
[kusto.databases.production]
name = "ProductionDB"
cluster_url = "https://prod.kusto.windows.net"
description = "Critical production data"
is_default = false
priority = "high"  # New field for important databases
```

### New Tools

#### `kusto_discover_databases`
Manually trigger database discovery:
```bash
codex "discover all available Kusto databases"
```

#### `kusto_refresh_database_list`
Refresh the list of available databases:
```bash
codex "refresh the list of Kusto databases"
```

### Implementation Plan

1. **Discovery Query**: Use `.show databases` command to list available databases
2. **Metadata Enrichment**: Get database properties and descriptions
3. **Knowledge Base Integration**: Store discovered databases with metadata
4. **Caching Strategy**: Cache discovery results with TTL (time-to-live)
5. **Permission Handling**: Gracefully handle databases with limited access

### Benefits

1. **Zero Configuration**: Works out of the box with just cluster URL
2. **Always Up-to-Date**: Automatically discovers new databases
3. **Reduced Maintenance**: No need to manually update config for new databases
4. **Better Discovery**: AI knows about all available databases

### Usage Examples

```bash
# Auto-discovery in action
codex "what databases are available?"
# → Automatically scans and shows all accessible databases

codex "show me tables in the marketing database"
# → Automatically discovers and connects to MarketingDB

codex "find databases related to analytics"
# → Searches discovered databases by name and description
```