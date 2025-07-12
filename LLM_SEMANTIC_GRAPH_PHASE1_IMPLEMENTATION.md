# Phase 1: Foundation Implementation Guide

## 🎯 Phase 1 Goals (Weeks 1-2)

**Objective**: Add persistent storage layer without breaking existing functionality

**Deliverables**:
- Enhanced graph schema with semantic fields
- SQLite storage implementation
- Basic enrichment MCP tools
- Migration utilities
- Unit tests for storage layer

**Success Criteria**:
- All existing tools continue to work unchanged
- New storage layer can persist and retrieve enhanced symbols
- Basic enrichment tools are functional
- Performance impact < 10% on existing operations

---

## 📋 Phase 1 Task Breakdown

### Week 1: Storage Foundation

#### Task 1.1: Database Schema Setup (2 days)
**Files to create/modify**:
```
codex-rs/core/src/code_analysis/
├── semantic_storage.rs          # New - Main storage implementation
├── migrations/
│   └── 001_create_semantic_tables.sql  # New - Database schema
└── schema.rs                    # New - Diesel schema definitions
```

**Implementation Steps**:
1. Create SQLite database schema with semantic tables
2. Set up Diesel ORM integration
3. Create migration scripts
4. Add database connection management
5. Implement basic CRUD operations

**Code Example**:
```rust
// codex-rs/core/src/code_analysis/semantic_storage.rs
pub struct SemanticGraphStorage {
    connection: SqliteConnection,
    cache: HashMap<String, EnhancedSymbol>,
}

impl SemanticGraphStorage {
    pub fn new(database_path: &str) -> Result<Self> {
        // Initialize database connection
        // Run migrations
        // Set up cache
    }
    
    pub fn save_enriched_symbol(&mut self, symbol: &EnhancedSymbol) -> Result<()> {
        // Save to database and update cache
    }
    
    pub fn get_enriched_symbol(&self, symbol_id: &str) -> Result<Option<EnhancedSymbol>> {
        // Check cache first, then database
    }
}
```

#### Task 1.2: Enhanced Data Structures (1 day)
**Files to create/modify**:
```
codex-rs/core/src/code_analysis/
├── semantic_types.rs            # New - Enhanced symbol/edge types
└── mod.rs                       # Modified - Export new types
```

**Implementation Steps**:
1. Define EnhancedSymbol and EnhancedEdge structures
2. Add semantic metadata fields
3. Implement serialization/deserialization
4. Add conversion methods from existing types

#### Task 1.3: Storage Integration (2 days)
**Files to modify**:
```
codex-rs/core/src/code_analysis/
├── graph_manager.rs             # Modified - Add semantic storage
├── repo_mapper.rs               # Modified - Use enhanced types
└── tools.rs                     # Modified - Prepare for enrichment
```

**Implementation Steps**:
1. Integrate semantic storage with existing graph manager
2. Add fallback mechanisms for missing semantic data
3. Ensure backward compatibility with existing tools
4. Add configuration for storage location

### Week 2: Basic Enrichment Tools

#### Task 2.1: Enrichment MCP Tools (3 days)
**Files to create**:
```
codex-rs/core/src/code_analysis/
├── semantic_tools.rs            # New - LLM enrichment tools
└── enrichment_types.rs          # New - Input/output types for enrichment
```

**New MCP Tools to implement**:
1. `check_semantic_enrichment` - Check if symbol needs enrichment
2. `enrich_symbol_semantics` - LLM enriches symbol with business context
3. `enrich_relationship_semantics` - LLM enriches edges with semantic meaning
4. `get_enrichment_statistics` - Show enrichment progress

**Tool Implementation Example**:
```rust
// check_semantic_enrichment tool
pub fn handle_check_semantic_enrichment(args: Value) -> Option<Result<Value, String>> {
    let input: CheckSemanticEnrichmentInput = serde_json::from_value(args)?;
    
    let storage = get_semantic_storage()?;
    let needs_enrichment = !storage.is_symbol_enriched(&input.symbol_name);
    
    Ok(json!({
        "needs_enrichment": needs_enrichment,
        "last_updated": storage.get_last_updated(&input.symbol_name),
        "confidence": storage.get_confidence(&input.symbol_name)
    }))
}

// enrich_symbol_semantics tool
pub fn handle_enrich_symbol_semantics(args: Value) -> Option<Result<Value, String>> {
    let input: EnrichSymbolSemanticsInput = serde_json::from_value(args)?;
    
    // Get current symbol
    let mut symbol = get_symbol(&input.symbol_name)?;
    
    // Apply LLM enrichment
    symbol.business_purpose = input.business_purpose;
    symbol.business_rules = input.business_rules;
    symbol.llm_confidence = input.confidence;
    symbol.semantic_last_updated = Some(Utc::now());
    
    // Save to storage
    let mut storage = get_semantic_storage()?;
    storage.save_enriched_symbol(&symbol)?;
    
    Ok(json!({"status": "enriched", "symbol": symbol.name}))
}
```

#### Task 2.2: Tool Registration (1 day)
**Files to modify**:
```
codex-rs/core/src/code_analysis/
├── tools.rs                     # Modified - Register new tools
├── integration.rs               # Modified - Add semantic tools to OpenAI
└── tool_handler.rs              # Modified - Handle new tool calls
```

**Implementation Steps**:
1. Register new semantic tools in tool registry
2. Add tool handlers for new enrichment tools
3. Update OpenAI integration to include semantic tools
4. Add proper error handling and validation

#### Task 2.3: Configuration and Setup (1 day)
**Files to create/modify**:
```
codex-rs/core/src/
├── config.rs                    # Modified - Add semantic storage config
└── code_analysis/
    └── semantic_config.rs       # New - Semantic-specific configuration
```

**Configuration Options**:
```toml
[semantic_analysis]
enabled = true
storage_type = "sqlite"  # or "json"
database_path = "./semantic_graph.db"
cache_size = 10000
auto_enrichment = false
confidence_threshold = 0.7
staleness_days = 30
```

---

## 🧪 Testing Strategy

### Unit Tests
**Files to create**:
```
codex-rs/core/tests/
├── semantic_storage_tests.rs    # New - Storage layer tests
├── semantic_tools_tests.rs      # New - Enrichment tool tests
└── semantic_integration_tests.rs # New - Integration tests
```

**Test Coverage**:
1. **Storage Operations**: Save, retrieve, update, delete enhanced symbols
2. **Cache Management**: Cache hits, misses, invalidation
3. **Migration**: Database schema migrations work correctly
4. **Tool Functionality**: Each enrichment tool works as expected
5. **Backward Compatibility**: Existing tools continue to work
6. **Error Handling**: Graceful handling of database errors

### Integration Tests
```rust
#[tokio::test]
async fn test_semantic_storage_integration() {
    // Test that semantic storage integrates with existing graph manager
    let storage = SemanticGraphStorage::new(":memory:").unwrap();
    let graph_manager = GraphManager::with_semantic_storage(storage);
    
    // Test existing functionality still works
    let symbols = graph_manager.analyze_file("test.rs").await.unwrap();
    assert!(!symbols.is_empty());
    
    // Test semantic enrichment
    let enriched = graph_manager.enrich_symbol("TestClass", enrichment_data).await.unwrap();
    assert!(enriched.business_purpose.is_some());
}

#[tokio::test]
async fn test_enrichment_tools() {
    // Test that enrichment tools work end-to-end
    let result = handle_check_semantic_enrichment(json!({
        "symbol_name": "TestClass",
        "file_path": "test.rs"
    })).unwrap().unwrap();
    
    assert_eq!(result["needs_enrichment"], true);
}
```

---

## 📊 Performance Considerations

### Database Optimization
1. **Indexes**: Create indexes on frequently queried fields
2. **Connection Pooling**: Use connection pooling for concurrent access
3. **Batch Operations**: Implement batch inserts/updates for performance
4. **Cache Strategy**: Implement intelligent caching with LRU eviction

### Memory Management
```rust
pub struct SemanticGraphStorage {
    connection: SqliteConnection,
    cache: LruCache<String, EnhancedSymbol>,  // Use LRU cache
    cache_size: usize,
}

impl SemanticGraphStorage {
    pub fn with_cache_size(database_path: &str, cache_size: usize) -> Result<Self> {
        Ok(Self {
            connection: SqliteConnection::establish(database_path)?,
            cache: LruCache::new(cache_size),
            cache_size,
        })
    }
}
```

### Monitoring and Metrics
```rust
pub struct StorageMetrics {
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub database_queries: u64,
    pub enrichment_operations: u64,
}

impl SemanticGraphStorage {
    pub fn get_metrics(&self) -> StorageMetrics {
        // Return performance metrics
    }
}
```

---

## 🔧 Migration Strategy

### Backward Compatibility
1. **Existing Tools**: All current tools continue to work without changes
2. **Gradual Migration**: Semantic features are opt-in initially
3. **Fallback Mechanisms**: When semantic data unavailable, fall back to structural data
4. **Configuration**: Semantic features can be disabled via configuration

### Data Migration
```rust
pub fn migrate_existing_graph_to_semantic() -> Result<()> {
    // Read existing structural graph
    let structural_symbols = load_existing_symbols()?;
    
    // Convert to enhanced symbols (without semantic data)
    let enhanced_symbols: Vec<EnhancedSymbol> = structural_symbols
        .into_iter()
        .map(|s| EnhancedSymbol::from_structural_symbol(s))
        .collect();
    
    // Save to new semantic storage
    let mut storage = SemanticGraphStorage::new("semantic_graph.db")?;
    for symbol in enhanced_symbols {
        storage.save_enriched_symbol(&symbol)?;
    }
    
    Ok(())
}
```

---

## 📋 Phase 1 Checklist

### Week 1 Deliverables
- [ ] SQLite database schema created and tested
- [ ] Diesel ORM integration working
- [ ] Enhanced symbol/edge data structures defined
- [ ] Basic storage operations implemented (CRUD)
- [ ] Cache management working
- [ ] Migration scripts created and tested
- [ ] Unit tests for storage layer (>80% coverage)

### Week 2 Deliverables
- [ ] `check_semantic_enrichment` tool implemented and tested
- [ ] `enrich_symbol_semantics` tool implemented and tested
- [ ] `enrich_relationship_semantics` tool implemented and tested
- [ ] `get_enrichment_statistics` tool implemented and tested
- [ ] Tool registration and integration complete
- [ ] Configuration system updated
- [ ] Integration tests passing
- [ ] Performance benchmarks established

### Success Metrics
- [ ] All existing tests continue to pass
- [ ] New semantic tools respond correctly
- [ ] Database operations complete within performance targets
- [ ] Memory usage remains within acceptable bounds
- [ ] Documentation updated for new features

---

## 🚀 Phase 1 Completion Criteria

1. **Functional Requirements**:
   - Semantic storage layer operational
   - Basic enrichment tools working
   - Backward compatibility maintained

2. **Performance Requirements**:
   - < 10% performance impact on existing operations
   - Database operations complete within 100ms for typical queries
   - Memory usage increase < 50MB for typical codebases

3. **Quality Requirements**:
   - Unit test coverage > 80%
   - Integration tests passing
   - No regressions in existing functionality

4. **Documentation Requirements**:
   - API documentation for new tools
   - Configuration guide updated
   - Migration guide created

**Ready for Phase 2**: LLM Integration and Semantic Analysis