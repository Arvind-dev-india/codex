# LLM-Enhanced Semantic Graph Implementation Plan

## 🎯 Project Overview

**Goal**: Transform the existing MCP code analysis server into a self-improving system where LLMs enrich structural code graphs with semantic business context, stored persistently for future use.

**Core Concept**: LLM acts as semantic intelligence layer that enriches existing tree-sitter structural graph with business logic understanding, quality assessments, and architectural insights.

## 📋 Executive Summary

### Current State
- Excellent structural analysis using tree-sitter
- Fast symbol navigation and reference tracking
- Multi-language support (Rust, C#, Python, C++, Java, JS/TS)
- Graph-based storage for relationships

### Target State
- Structural analysis + semantic understanding
- Business context and workflow mapping
- Code quality and architectural insights
- Self-improving system that learns over time
- Persistent semantic knowledge across sessions

### Key Benefits
1. **Self-Learning**: System gets smarter with each LLM interaction
2. **Performance**: Avoids re-analyzing same code repeatedly
3. **Persistence**: Knowledge survives restarts and updates
4. **Incremental**: Can be implemented without breaking existing functionality
5. **Scalable**: Works for enterprise codebases

---

## 🏗️ Architecture Overview

### Enhanced Data Flow
```
Source Code → Tree-sitter → Structural Graph → MCP Tools → LLM Analysis → Semantic Enrichment → Persistent Storage
                                    ↑                                                              ↓
                                    ←←←←←← Future Queries Read Enriched Data ←←←←←←←←←←←←←←←←←←←←←←←
```

### Storage Architecture
```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────────┐
│   Structural    │    │    Semantic      │    │   Persistent        │
│   Graph         │───▶│   Enrichment     │───▶│   Storage           │
│   (Tree-sitter) │    │   (LLM Layer)    │    │   (SQLite/JSON)     │
└─────────────────┘    └──────────────────┘    └─────────────────────┘
```

---

## 📊 Implementation Phases

### Phase 1: Foundation (Weeks 1-2)
- **Goal**: Add persistent storage layer without breaking existing functionality
- **Deliverables**: Enhanced graph schema, storage implementation, basic enrichment tools
- **Risk**: Low - additive changes only

### Phase 2: LLM Integration (Weeks 3-4)
- **Goal**: Enable LLM to enrich symbols with semantic data
- **Deliverables**: Enrichment MCP tools, confidence-based updates, basic semantic analysis
- **Risk**: Medium - new LLM interaction patterns

### Phase 3: Enhanced Tools (Weeks 5-6)
- **Goal**: Existing tools return enriched data when available
- **Deliverables**: Enhanced tool responses, fallback mechanisms, migration utilities
- **Risk**: Low - backward compatible enhancements

### Phase 4: Advanced Features (Weeks 7-8)
- **Goal**: Advanced semantic analysis and batch processing
- **Deliverables**: Workflow detection, architectural pattern recognition, performance optimization
- **Risk**: Medium - complex semantic analysis

### Phase 5: Production Optimization (Weeks 9-10)
- **Goal**: Production-ready performance and reliability
- **Deliverables**: Caching, monitoring, error handling, documentation
- **Risk**: Low - optimization and polish

---

## 🗄️ Data Schema Design

### Enhanced Symbol Structure
```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnhancedSymbol {
    // === Existing Structural Data ===
    pub id: String,                    // Unique identifier
    pub name: String,                  // Symbol name
    pub symbol_type: SymbolType,       // Function, Class, Variable, etc.
    pub file_path: String,             // Source file path
    pub start_line: usize,             // Start line number
    pub end_line: usize,               // End line number
    pub parent: Option<String>,        // Parent symbol (for methods in classes)
    
    // === LLM-Enriched Semantic Data ===
    pub business_purpose: Option<String>,           // "User authentication workflow"
    pub business_rules: Vec<String>,                // ["Email must be unique", "Password min 8 chars"]
    pub architectural_role: Option<String>,         // "Repository layer for user data access"
    pub quality_metrics: Option<QualityMetrics>,    // Code quality assessment
    pub usage_patterns: Vec<UsagePattern>,          // Common usage scenarios
    pub improvement_suggestions: Vec<String>,       // LLM recommendations
    pub security_considerations: Vec<String>,       // Security implications
    pub performance_notes: Vec<String>,             // Performance characteristics
    
    // === Enrichment Metadata ===
    pub semantic_last_updated: Option<DateTime<Utc>>,  // When semantic data was last updated
    pub llm_confidence: Option<f32>,                    // 0.0-1.0 confidence in semantic data
    pub enrichment_version: u32,                        // Track semantic model versions
    pub enrichment_source: Option<String>,              // Which LLM/model provided enrichment
    pub manual_overrides: Vec<ManualOverride>,          // Human corrections to LLM analysis
    
    // === Timestamps ===
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Enhanced Edge Structure
```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnhancedEdge {
    // === Existing Structural Data ===
    pub id: String,
    pub from_symbol: String,           // Source symbol ID
    pub to_symbol: String,             // Target symbol ID
    pub edge_type: EdgeType,           // Call, Contains, Usage, etc.
    
    // === LLM-Enriched Semantic Data ===
    pub business_relationship: Option<String>,      // "Validates user input before saving"
    pub data_flow_description: Option<String>,      // "User data flows from controller to service"
    pub dependency_reason: Option<String>,          // "Needs UserRepository for data persistence"
    pub workflow_step: Option<WorkflowStep>,        // Position in business workflow
    pub interaction_frequency: Option<String>,      // "High", "Medium", "Low"
    pub error_propagation: Option<String>,          // How errors flow through this relationship
    
    // === Enrichment Metadata ===
    pub semantic_last_updated: Option<DateTime<Utc>>,
    pub llm_confidence: Option<f32>,
    pub enrichment_version: u32,
    
    // === Timestamps ===
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Supporting Data Structures
```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QualityMetrics {
    pub maintainability_score: Option<f32>,     // 0.0-1.0
    pub complexity_score: Option<f32>,          // Cyclomatic complexity normalized
    pub testability_score: Option<f32>,         // How easy to test
    pub performance_score: Option<f32>,         // Performance characteristics
    pub security_score: Option<f32>,            // Security assessment
    pub code_smells: Vec<String>,               // Identified issues
    pub best_practices_adherence: Vec<String>,  // Which practices are followed
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UsagePattern {
    pub pattern_name: String,                   // "Factory Pattern", "Validation Chain"
    pub description: String,                    // How this symbol is typically used
    pub frequency: String,                      // "Common", "Rare", "Critical"
    pub examples: Vec<String>,                  // Example usage scenarios
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkflowStep {
    pub workflow_name: String,                  // "User Registration", "Payment Processing"
    pub step_number: u32,                       // Position in workflow
    pub step_description: String,               // What happens in this step
    pub preconditions: Vec<String>,             // What must be true before this step
    pub postconditions: Vec<String>,            // What is true after this step
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ManualOverride {
    pub field_name: String,                     // Which field was overridden
    pub original_value: String,                 // LLM's original value
    pub override_value: String,                 // Human-corrected value
    pub reason: String,                         // Why the override was made
    pub timestamp: DateTime<Utc>,               // When override was made
    pub user: Option<String>,                   // Who made the override
}
```

---

## 💾 Storage Implementation Details

### Database Schema (SQLite)

```sql
-- Enhanced Symbols Table
CREATE TABLE IF NOT EXISTS semantic_symbols (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    symbol_type TEXT NOT NULL,
    file_path TEXT NOT NULL,
    start_line INTEGER NOT NULL,
    end_line INTEGER NOT NULL,
    parent_id TEXT,
    
    -- Semantic enrichment fields
    business_purpose TEXT,
    business_rules TEXT,                    -- JSON array
    architectural_role TEXT,
    quality_metrics TEXT,                   -- JSON object
    usage_patterns TEXT,                    -- JSON array
    improvement_suggestions TEXT,           -- JSON array
    security_considerations TEXT,           -- JSON array
    performance_notes TEXT,                 -- JSON array
    
    -- Enrichment metadata
    semantic_last_updated DATETIME,
    llm_confidence REAL,
    enrichment_version INTEGER DEFAULT 1,
    enrichment_source TEXT,
    manual_overrides TEXT,                  -- JSON array
    
    -- Timestamps
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (parent_id) REFERENCES semantic_symbols(id)
);

-- Enhanced Edges Table
CREATE TABLE IF NOT EXISTS semantic_edges (
    id TEXT PRIMARY KEY,
    from_symbol_id TEXT NOT NULL,
    to_symbol_id TEXT NOT NULL,
    edge_type TEXT NOT NULL,
    
    -- Semantic enrichment fields
    business_relationship TEXT,
    data_flow_description TEXT,
    dependency_reason TEXT,
    workflow_step TEXT,                     -- JSON object
    interaction_frequency TEXT,
    error_propagation TEXT,
    
    -- Enrichment metadata
    semantic_last_updated DATETIME,
    llm_confidence REAL,
    enrichment_version INTEGER DEFAULT 1,
    
    -- Timestamps
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (from_symbol_id) REFERENCES semantic_symbols(id),
    FOREIGN KEY (to_symbol_id) REFERENCES semantic_symbols(id)
);

-- Workflow Tracking Table
CREATE TABLE IF NOT EXISTS business_workflows (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    steps TEXT,                             -- JSON array of workflow steps
    entry_points TEXT,                      -- JSON array of symbol IDs
    exit_points TEXT,                       -- JSON array of symbol IDs
    business_rules TEXT,                    -- JSON array
    stakeholders TEXT,                      -- JSON array
    
    -- Enrichment metadata
    llm_confidence REAL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_symbols_name ON semantic_symbols(name);
CREATE INDEX IF NOT EXISTS idx_symbols_file_path ON semantic_symbols(file_path);
CREATE INDEX IF NOT EXISTS idx_symbols_type ON semantic_symbols(symbol_type);
CREATE INDEX IF NOT EXISTS idx_symbols_semantic_updated ON semantic_symbols(semantic_last_updated);
CREATE INDEX IF NOT EXISTS idx_edges_from ON semantic_edges(from_symbol_id);
CREATE INDEX IF NOT EXISTS idx_edges_to ON semantic_edges(to_symbol_id);
CREATE INDEX IF NOT EXISTS idx_edges_type ON semantic_edges(edge_type);
```

### Storage Implementation

```rust
// codex-rs/core/src/code_analysis/semantic_storage.rs

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde_json;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use anyhow::{Result, Context};

pub struct SemanticGraphStorage {
    connection: SqliteConnection,
    cache: HashMap<String, EnhancedSymbol>,
    cache_dirty: bool,
}

impl SemanticGraphStorage {
    pub fn new(database_url: &str) -> Result<Self> {
        let connection = SqliteConnection::establish(database_url)
            .context("Failed to connect to semantic graph database")?;
        
        // Run migrations
        Self::run_migrations(&connection)?;
        
        Ok(Self {
            connection,
            cache: HashMap::new(),
            cache_dirty: false,
        })
    }
    
    fn run_migrations(conn: &SqliteConnection) -> Result<()> {
        // Execute schema creation SQL
        diesel::sql_query(include_str!("../migrations/001_create_semantic_tables.sql"))
            .execute(conn)
            .context("Failed to run database migrations")?;
        Ok(())
    }
    
    // === Symbol Operations ===
    
    pub fn save_enriched_symbol(&mut self, symbol: &EnhancedSymbol) -> Result<()> {
        use crate::schema::semantic_symbols::dsl::*;
        
        let symbol_json = SemanticSymbolRow::from_enhanced_symbol(symbol)?;
        
        diesel::replace_into(semantic_symbols)
            .values(&symbol_json)
            .execute(&self.connection)
            .context("Failed to save enriched symbol")?;
        
        // Update cache
        self.cache.insert(symbol.id.clone(), symbol.clone());
        self.cache_dirty = true;
        
        Ok(())
    }
    
    pub fn get_enriched_symbol(&mut self, symbol_id: &str) -> Result<Option<EnhancedSymbol>> {
        // Check cache first
        if let Some(symbol) = self.cache.get(symbol_id) {
            return Ok(Some(symbol.clone()));
        }
        
        // Query database
        use crate::schema::semantic_symbols::dsl::*;
        
        let symbol_row: Option<SemanticSymbolRow> = semantic_symbols
            .filter(id.eq(symbol_id))
            .first(&self.connection)
            .optional()
            .context("Failed to query enriched symbol")?;
        
        if let Some(row) = symbol_row {
            let enhanced_symbol = row.to_enhanced_symbol()?;
            self.cache.insert(symbol_id.to_string(), enhanced_symbol.clone());
            Ok(Some(enhanced_symbol))
        } else {
            Ok(None)
        }
    }
    
    pub fn get_symbols_by_file(&mut self, file_path: &str) -> Result<Vec<EnhancedSymbol>> {
        use crate::schema::semantic_symbols::dsl::*;
        
        let symbol_rows: Vec<SemanticSymbolRow> = semantic_symbols
            .filter(file_path.eq(file_path))
            .load(&self.connection)
            .context("Failed to query symbols by file")?;
        
        let mut symbols = Vec::new();
        for row in symbol_rows {
            let enhanced_symbol = row.to_enhanced_symbol()?;
            self.cache.insert(enhanced_symbol.id.clone(), enhanced_symbol.clone());
            symbols.push(enhanced_symbol);
        }
        
        Ok(symbols)
    }
    
    pub fn is_symbol_enriched(&self, symbol_id: &str) -> Result<bool> {
        use crate::schema::semantic_symbols::dsl::*;
        
        let has_enrichment: bool = semantic_symbols
            .filter(id.eq(symbol_id))
            .filter(business_purpose.is_not_null())
            .select(diesel::dsl::exists(
                semantic_symbols.filter(id.eq(symbol_id))
            ))
            .get_result(&self.connection)
            .context("Failed to check symbol enrichment status")?;
        
        Ok(has_enrichment)
    }
    
    pub fn get_symbols_needing_enrichment(&self, limit: Option<i64>) -> Result<Vec<String>> {
        use crate::schema::semantic_symbols::dsl::*;
        
        let mut query = semantic_symbols
            .filter(business_purpose.is_null().or(
                llm_confidence.lt(0.7).or(
                    semantic_last_updated.lt(
                        diesel::dsl::sql::<diesel::sql_types::Timestamp>(
                            "datetime('now', '-30 days')"
                        )
                    )
                )
            ))
            .select(id)
            .into_boxed();
        
        if let Some(limit_val) = limit {
            query = query.limit(limit_val);
        }
        
        let symbol_ids: Vec<String> = query
            .load(&self.connection)
            .context("Failed to query symbols needing enrichment")?;
        
        Ok(symbol_ids)
    }
    
    // === Edge Operations ===
    
    pub fn save_enriched_edge(&mut self, edge: &EnhancedEdge) -> Result<()> {
        use crate::schema::semantic_edges::dsl::*;
        
        let edge_json = SemanticEdgeRow::from_enhanced_edge(edge)?;
        
        diesel::replace_into(semantic_edges)
            .values(&edge_json)
            .execute(&self.connection)
            .context("Failed to save enriched edge")?;
        
        Ok(())
    }
    
    pub fn get_enriched_edges_for_symbol(&self, symbol_id: &str) -> Result<Vec<EnhancedEdge>> {
        use crate::schema::semantic_edges::dsl::*;
        
        let edge_rows: Vec<SemanticEdgeRow> = semantic_edges
            .filter(from_symbol_id.eq(symbol_id).or(to_symbol_id.eq(symbol_id)))
            .load(&self.connection)
            .context("Failed to query enriched edges")?;
        
        let mut edges = Vec::new();
        for row in edge_rows {
            edges.push(row.to_enhanced_edge()?);
        }
        
        Ok(edges)
    }
    
    // === Workflow Operations ===
    
    pub fn save_business_workflow(&mut self, workflow: &BusinessWorkflow) -> Result<()> {
        use crate::schema::business_workflows::dsl::*;
        
        let workflow_row = BusinessWorkflowRow::from_workflow(workflow)?;
        
        diesel::replace_into(business_workflows)
            .values(&workflow_row)
            .execute(&self.connection)
            .context("Failed to save business workflow")?;
        
        Ok(())
    }
    
    pub fn get_workflows_for_symbol(&self, symbol_id: &str) -> Result<Vec<BusinessWorkflow>> {
        use crate::schema::business_workflows::dsl::*;
        
        // Query workflows that contain this symbol in entry_points or exit_points
        let workflow_rows: Vec<BusinessWorkflowRow> = business_workflows
            .filter(
                entry_points.like(format!("%{}%", symbol_id))
                .or(exit_points.like(format!("%{}%", symbol_id)))
            )
            .load(&self.connection)
            .context("Failed to query workflows for symbol")?;
        
        let mut workflows = Vec::new();
        for row in workflow_rows {
            workflows.push(row.to_workflow()?);
        }
        
        Ok(workflows)
    }
    
    // === Cache Management ===
    
    pub fn flush_cache(&mut self) -> Result<()> {
        if self.cache_dirty {
            // Could implement batch writes here for performance
            self.cache_dirty = false;
        }
        Ok(())
    }
    
    pub fn clear_cache(&mut self) {
        self.cache.clear();
        self.cache_dirty = false;
    }
    
    // === Statistics and Monitoring ===
    
    pub fn get_enrichment_statistics(&self) -> Result<EnrichmentStatistics> {
        use crate::schema::semantic_symbols::dsl::*;
        
        let total_symbols: i64 = semantic_symbols
            .count()
            .get_result(&self.connection)
            .context("Failed to count total symbols")?;
        
        let enriched_symbols: i64 = semantic_symbols
            .filter(business_purpose.is_not_null())
            .count()
            .get_result(&self.connection)
            .context("Failed to count enriched symbols")?;
        
        let high_confidence_symbols: i64 = semantic_symbols
            .filter(llm_confidence.gt(0.8))
            .count()
            .get_result(&self.connection)
            .context("Failed to count high confidence symbols")?;
        
        Ok(EnrichmentStatistics {
            total_symbols,
            enriched_symbols,
            enrichment_percentage: (enriched_symbols as f32 / total_symbols as f32) * 100.0,
            high_confidence_symbols,
            high_confidence_percentage: (high_confidence_symbols as f32 / enriched_symbols as f32) * 100.0,
        })
    }
}

// === Data Transfer Objects ===

#[derive(Queryable, Insertable, AsChangeset)]
#[table_name = "semantic_symbols"]
struct SemanticSymbolRow {
    pub id: String,
    pub name: String,
    pub symbol_type: String,
    pub file_path: String,
    pub start_line: i32,
    pub end_line: i32,
    pub parent_id: Option<String>,
    pub business_purpose: Option<String>,
    pub business_rules: Option<String>,
    pub architectural_role: Option<String>,
    pub quality_metrics: Option<String>,
    pub usage_patterns: Option<String>,
    pub improvement_suggestions: Option<String>,
    pub security_considerations: Option<String>,
    pub performance_notes: Option<String>,
    pub semantic_last_updated: Option<chrono::NaiveDateTime>,
    pub llm_confidence: Option<f32>,
    pub enrichment_version: i32,
    pub enrichment_source: Option<String>,
    pub manual_overrides: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl SemanticSymbolRow {
    fn from_enhanced_symbol(symbol: &EnhancedSymbol) -> Result<Self> {
        Ok(Self {
            id: symbol.id.clone(),
            name: symbol.name.clone(),
            symbol_type: symbol.symbol_type.to_string(),
            file_path: symbol.file_path.clone(),
            start_line: symbol.start_line as i32,
            end_line: symbol.end_line as i32,
            parent_id: symbol.parent.clone(),
            business_purpose: symbol.business_purpose.clone(),
            business_rules: if symbol.business_rules.is_empty() {
                None
            } else {
                Some(serde_json::to_string(&symbol.business_rules)?)
            },
            architectural_role: symbol.architectural_role.clone(),
            quality_metrics: symbol.quality_metrics.as_ref()
                .map(|qm| serde_json::to_string(qm))
                .transpose()?,
            usage_patterns: if symbol.usage_patterns.is_empty() {
                None
            } else {
                Some(serde_json::to_string(&symbol.usage_patterns)?)
            },
            improvement_suggestions: if symbol.improvement_suggestions.is_empty() {
                None
            } else {
                Some(serde_json::to_string(&symbol.improvement_suggestions)?)
            },
            security_considerations: if symbol.security_considerations.is_empty() {
                None
            } else {
                Some(serde_json::to_string(&symbol.security_considerations)?)
            },
            performance_notes: if symbol.performance_notes.is_empty() {
                None
            } else {
                Some(serde_json::to_string(&symbol.performance_notes)?)
            },
            semantic_last_updated: symbol.semantic_last_updated.map(|dt| dt.naive_utc()),
            llm_confidence: symbol.llm_confidence,
            enrichment_version: symbol.enrichment_version as i32,
            enrichment_source: symbol.enrichment_source.clone(),
            manual_overrides: if symbol.manual_overrides.is_empty() {
                None
            } else {
                Some(serde_json::to_string(&symbol.manual_overrides)?)
            },
            created_at: symbol.created_at.naive_utc(),
            updated_at: symbol.updated_at.naive_utc(),
        })
    }
    
    fn to_enhanced_symbol(&self) -> Result<EnhancedSymbol> {
        Ok(EnhancedSymbol {
            id: self.id.clone(),
            name: self.name.clone(),
            symbol_type: self.symbol_type.parse()?,
            file_path: self.file_path.clone(),
            start_line: self.start_line as usize,
            end_line: self.end_line as usize,
            parent: self.parent_id.clone(),
            business_purpose: self.business_purpose.clone(),
            business_rules: self.business_rules.as_ref()
                .map(|br| serde_json::from_str(br))
                .transpose()?
                .unwrap_or_default(),
            architectural_role: self.architectural_role.clone(),
            quality_metrics: self.quality_metrics.as_ref()
                .map(|qm| serde_json::from_str(qm))
                .transpose()?,
            usage_patterns: self.usage_patterns.as_ref()
                .map(|up| serde_json::from_str(up))
                .transpose()?
                .unwrap_or_default(),
            improvement_suggestions: self.improvement_suggestions.as_ref()
                .map(|is| serde_json::from_str(is))
                .transpose()?
                .unwrap_or_default(),
            security_considerations: self.security_considerations.as_ref()
                .map(|sc| serde_json::from_str(sc))
                .transpose()?
                .unwrap_or_default(),
            performance_notes: self.performance_notes.as_ref()
                .map(|pn| serde_json::from_str(pn))
                .transpose()?
                .unwrap_or_default(),
            semantic_last_updated: self.semantic_last_updated.map(|dt| DateTime::from_utc(dt, Utc)),
            llm_confidence: self.llm_confidence,
            enrichment_version: self.enrichment_version as u32,
            enrichment_source: self.enrichment_source.clone(),
            manual_overrides: self.manual_overrides.as_ref()
                .map(|mo| serde_json::from_str(mo))
                .transpose()?
                .unwrap_or_default(),
            created_at: DateTime::from_utc(self.created_at, Utc),
            updated_at: DateTime::from_utc(self.updated_at, Utc),
        })
    }
}

#[derive(Debug)]
pub struct EnrichmentStatistics {
    pub total_symbols: i64,
    pub enriched_symbols: i64,
    pub enrichment_percentage: f32,
    pub high_confidence_symbols: i64,
    pub high_confidence_percentage: f32,
}
```