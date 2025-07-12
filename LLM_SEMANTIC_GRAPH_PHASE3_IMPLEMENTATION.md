# Phase 3: Enhanced Tools Integration Implementation Guide

## 🎯 Phase 3 Goals (Weeks 5-6)

**Objective**: Integrate semantic enrichment into existing MCP tools for enhanced user experience

**Deliverables**:
- Enhanced versions of all existing MCP tools
- Seamless fallback mechanisms
- Enriched response formats
- Migration utilities for existing users
- Performance optimization for enriched responses

**Success Criteria**:
- All existing tools return enriched data when available
- Response times remain within acceptable limits
- Backward compatibility maintained
- User experience significantly improved with semantic context

---

## 📋 Phase 3 Task Breakdown

### Week 5: Enhanced Tool Implementation

#### Task 5.1: Enhanced analyze_code Tool (2 days)
**Files to modify**:
```
codex-rs/core/src/code_analysis/
├── tools.rs                     # Modified - Enhanced analyze_code
└── enhanced_responses.rs        # New - Rich response formatting
```

**Enhanced analyze_code Implementation**:
```rust
// Enhanced analyze_code tool
pub fn handle_analyze_code_enhanced(args: Value) -> Option<Result<Value, String>> {
    Some(match serde_json::from_value::<AnalyzeCodeInput>(args) {
        Ok(input) => {
            // 1. Get structural analysis (existing functionality)
            let structural_symbols = analyze_file_symbols(&input.file_path)?;
            
            // 2. Get semantic enrichments if available
            let storage = get_semantic_storage()?;
            let mut enriched_symbols = Vec::new();
            
            for symbol in structural_symbols {
                match storage.get_enriched_symbol(&symbol.id)? {
                    Some(enriched) => {
                        enriched_symbols.push(EnrichedSymbolResponse {
                            // Structural data
                            name: enriched.name,
                            symbol_type: enriched.symbol_type.to_string(),
                            file_path: enriched.file_path,
                            start_line: enriched.start_line,
                            end_line: enriched.end_line,
                            parent: enriched.parent,
                            
                            // Semantic enrichments
                            business_purpose: enriched.business_purpose,
                            business_rules: enriched.business_rules,
                            architectural_role: enriched.architectural_role,
                            quality_score: enriched.quality_metrics
                                .as_ref()
                                .map(|qm| qm.overall_score()),
                            usage_patterns: enriched.usage_patterns
                                .iter()
                                .map(|up| up.pattern_name.clone())
                                .collect(),
                            improvement_suggestions: enriched.improvement_suggestions,
                            
                            // Enrichment metadata
                            enrichment_confidence: enriched.llm_confidence,
                            last_enriched: enriched.semantic_last_updated,
                            needs_enrichment: should_re_enrich(&enriched),
                        });
                    }
                    None => {
                        // Fallback to structural data only
                        enriched_symbols.push(EnrichedSymbolResponse::from_structural(symbol));
                    }
                }
            }
            
            // 3. Calculate file-level insights
            let file_insights = calculate_file_insights(&enriched_symbols);
            
            Ok(json!({
                "symbols": enriched_symbols,
                "file_insights": file_insights,
                "enrichment_status": {
                    "total_symbols": enriched_symbols.len(),
                    "enriched_symbols": enriched_symbols.iter()
                        .filter(|s| s.enrichment_confidence.is_some())
                        .count(),
                    "average_confidence": calculate_average_confidence(&enriched_symbols),
                    "needs_enrichment": enriched_symbols.iter()
                        .filter(|s| s.needs_enrichment)
                        .map(|s| s.name.clone())
                        .collect::<Vec<_>>()
                }
            }))
        },
        Err(e) => Err(format!("Invalid arguments: {}", e)),
    })
}

#[derive(Serialize)]
struct EnrichedSymbolResponse {
    // Structural data
    pub name: String,
    pub symbol_type: String,
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub parent: Option<String>,
    
    // Semantic enrichments (optional)
    pub business_purpose: Option<String>,
    pub business_rules: Vec<String>,
    pub architectural_role: Option<String>,
    pub quality_score: Option<f32>,
    pub usage_patterns: Vec<String>,
    pub improvement_suggestions: Vec<String>,
    
    // Enrichment metadata
    pub enrichment_confidence: Option<f32>,
    pub last_enriched: Option<DateTime<Utc>>,
    pub needs_enrichment: bool,
}

#[derive(Serialize)]
struct FileInsights {
    pub architectural_patterns: Vec<String>,
    pub business_domains: Vec<String>,
    pub quality_summary: QualitySummary,
    pub improvement_priorities: Vec<String>,
    pub complexity_assessment: String,
}

fn calculate_file_insights(symbols: &[EnrichedSymbolResponse]) -> FileInsights {
    let mut patterns = HashSet::new();
    let mut domains = HashSet::new();
    let mut quality_scores = Vec::new();
    let mut all_suggestions = Vec::new();
    
    for symbol in symbols {
        if let Some(role) = &symbol.architectural_role {
            patterns.insert(role.clone());
        }
        
        if let Some(purpose) = &symbol.business_purpose {
            // Extract domain from business purpose
            if let Some(domain) = extract_business_domain(purpose) {
                domains.insert(domain);
            }
        }
        
        if let Some(score) = symbol.quality_score {
            quality_scores.push(score);
        }
        
        all_suggestions.extend(symbol.improvement_suggestions.clone());
    }
    
    FileInsights {
        architectural_patterns: patterns.into_iter().collect(),
        business_domains: domains.into_iter().collect(),
        quality_summary: QualitySummary::from_scores(&quality_scores),
        improvement_priorities: prioritize_suggestions(all_suggestions),
        complexity_assessment: assess_file_complexity(symbols),
    }
}
```

#### Task 5.2: Enhanced find_symbol_references Tool (1 day)
**Enhanced find_symbol_references Implementation**:
```rust
pub fn handle_find_symbol_references_enhanced(args: Value) -> Option<Result<Value, String>> {
    Some(match serde_json::from_value::<FindSymbolReferencesInput>(args) {
        Ok(input) => {
            // 1. Get structural references (existing functionality)
            let structural_refs = if super::graph_manager::is_graph_initialized() {
                super::graph_manager::find_symbol_references(&input.symbol_name)
            } else {
                find_references_fallback(&input.symbol_name)?
            };
            
            // 2. Enrich references with semantic context
            let storage = get_semantic_storage()?;
            let mut enriched_refs = Vec::new();
            
            for reference in structural_refs {
                let enriched_ref = EnrichedReferenceResponse {
                    // Structural data
                    file: reference.reference_file,
                    line: reference.reference_line,
                    column: reference.reference_col,
                    reference_type: match reference.reference_type {
                        ReferenceType::Call => "call",
                        ReferenceType::Declaration => "declaration",
                        ReferenceType::Implementation => "implementation",
                        ReferenceType::Import => "import",
                        ReferenceType::Inheritance => "inheritance",
                        ReferenceType::Usage => "usage",
                    }.to_string(),
                    
                    // Semantic enrichments
                    business_context: get_reference_business_context(&reference, &storage)?,
                    usage_pattern: classify_usage_pattern(&reference, &storage)?,
                    impact_assessment: assess_reference_impact(&reference, &storage)?,
                };
                
                enriched_refs.push(enriched_ref);
            }
            
            // 3. Group references by context
            let grouped_refs = group_references_by_context(&enriched_refs);
            
            // 4. Calculate usage analytics
            let usage_analytics = calculate_usage_analytics(&enriched_refs);
            
            Ok(json!({
                "references": enriched_refs,
                "grouped_references": grouped_refs,
                "usage_analytics": usage_analytics,
                "summary": {
                    "total_references": enriched_refs.len(),
                    "reference_types": count_reference_types(&enriched_refs),
                    "files_affected": count_affected_files(&enriched_refs),
                    "business_contexts": extract_business_contexts(&enriched_refs),
                }
            }))
        },
        Err(e) => Err(format!("Invalid arguments: {}", e)),
    })
}

#[derive(Serialize)]
struct EnrichedReferenceResponse {
    // Structural data
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub reference_type: String,
    
    // Semantic enrichments
    pub business_context: Option<String>,
    pub usage_pattern: Option<String>,
    pub impact_assessment: ImpactAssessment,
}

#[derive(Serialize)]
struct ImpactAssessment {
    pub criticality: String,        // "High", "Medium", "Low"
    pub change_risk: String,        // "High", "Medium", "Low"
    pub test_coverage: Option<bool>,
    pub dependencies_count: usize,
}

fn get_reference_business_context(
    reference: &SymbolReference,
    storage: &SemanticGraphStorage,
) -> Result<Option<String>> {
    // Get the symbol that contains this reference
    if let Some(containing_symbol) = find_containing_symbol(&reference.reference_file, reference.reference_line)? {
        if let Some(enriched) = storage.get_enriched_symbol(&containing_symbol.id)? {
            return Ok(enriched.business_purpose);
        }
    }
    Ok(None)
}
```

#### Task 5.3: Enhanced get_symbol_subgraph Tool (1 day)
**Enhanced get_symbol_subgraph Implementation**:
```rust
pub fn handle_get_symbol_subgraph_enhanced(args: Value) -> Option<Result<Value, String>> {
    Some(match serde_json::from_value::<GetSymbolSubgraphInput>(args) {
        Ok(input) => {
            // 1. Get structural subgraph (existing functionality)
            let structural_subgraph = super::graph_manager::get_symbol_subgraph(
                &input.symbol_name,
                input.max_depth,
            );
            
            // 2. Enrich nodes and edges with semantic data
            let storage = get_semantic_storage()?;
            let enriched_subgraph = enrich_subgraph(&structural_subgraph, &storage)?;
            
            // 3. Analyze subgraph patterns
            let subgraph_analysis = analyze_subgraph_patterns(&enriched_subgraph)?;
            
            // 4. Generate insights
            let insights = generate_subgraph_insights(&enriched_subgraph, &subgraph_analysis)?;
            
            Ok(json!({
                "nodes": enriched_subgraph.nodes,
                "edges": enriched_subgraph.edges,
                "analysis": subgraph_analysis,
                "insights": insights,
                "metadata": {
                    "total_nodes": enriched_subgraph.nodes.len(),
                    "total_edges": enriched_subgraph.edges.len(),
                    "enrichment_coverage": calculate_enrichment_coverage(&enriched_subgraph),
                    "architectural_patterns": identify_architectural_patterns(&enriched_subgraph),
                    "business_workflows": identify_business_workflows(&enriched_subgraph),
                }
            }))
        },
        Err(e) => Err(format!("Invalid arguments: {}", e)),
    })
}

#[derive(Serialize)]
struct EnrichedSubgraph {
    pub nodes: Vec<EnrichedNode>,
    pub edges: Vec<EnrichedEdge>,
}

#[derive(Serialize)]
struct EnrichedNode {
    pub id: String,
    pub name: String,
    pub symbol_type: String,
    
    // Semantic enrichments
    pub business_purpose: Option<String>,
    pub architectural_role: Option<String>,
    pub quality_score: Option<f32>,
    pub criticality: String,
    pub enrichment_confidence: Option<f32>,
}

#[derive(Serialize)]
struct EnrichedEdge {
    pub from: String,
    pub to: String,
    pub edge_type: String,
    
    // Semantic enrichments
    pub business_relationship: Option<String>,
    pub data_flow_description: Option<String>,
    pub dependency_strength: String,
    pub workflow_step: Option<String>,
}

#[derive(Serialize)]
struct SubgraphAnalysis {
    pub architectural_layers: Vec<ArchitecturalLayer>,
    pub data_flow_paths: Vec<DataFlowPath>,
    pub dependency_clusters: Vec<DependencyCluster>,
    pub business_processes: Vec<BusinessProcess>,
}

fn enrich_subgraph(
    structural_subgraph: &StructuralSubgraph,
    storage: &SemanticGraphStorage,
) -> Result<EnrichedSubgraph> {
    let mut enriched_nodes = Vec::new();
    let mut enriched_edges = Vec::new();
    
    // Enrich nodes
    for node in &structural_subgraph.nodes {
        let enriched_node = match storage.get_enriched_symbol(&node.id)? {
            Some(enriched) => EnrichedNode {
                id: node.id.clone(),
                name: enriched.name,
                symbol_type: enriched.symbol_type.to_string(),
                business_purpose: enriched.business_purpose,
                architectural_role: enriched.architectural_role,
                quality_score: enriched.quality_metrics
                    .as_ref()
                    .map(|qm| qm.overall_score()),
                criticality: assess_node_criticality(&enriched),
                enrichment_confidence: enriched.llm_confidence,
            },
            None => EnrichedNode::from_structural(node),
        };
        enriched_nodes.push(enriched_node);
    }
    
    // Enrich edges
    for edge in &structural_subgraph.edges {
        let enriched_edge = match storage.get_enriched_edge(&edge.id)? {
            Some(enriched) => EnrichedEdge {
                from: edge.from.clone(),
                to: edge.to.clone(),
                edge_type: edge.edge_type.to_string(),
                business_relationship: enriched.business_relationship,
                data_flow_description: enriched.data_flow_description,
                dependency_strength: assess_dependency_strength(&enriched),
                workflow_step: enriched.workflow_step
                    .as_ref()
                    .map(|ws| ws.step_description.clone()),
            },
            None => EnrichedEdge::from_structural(edge),
        };
        enriched_edges.push(enriched_edge);
    }
    
    Ok(EnrichedSubgraph {
        nodes: enriched_nodes,
        edges: enriched_edges,
    })
}
```

### Week 6: Skeleton Tools Enhancement and Migration

#### Task 6.1: Enhanced Skeleton Generation (2 days)
**Files to modify**:
```
codex-rs/core/src/code_analysis/
├── tools.rs                     # Modified - Enhanced skeleton tools
└── intelligent_skeleton.rs     # New - Smart skeleton generation
```

**Enhanced get_multiple_files_skeleton Implementation**:
```rust
pub fn handle_get_multiple_files_skeleton_enhanced(args: Value) -> Option<Result<Value, String>> {
    Some(match serde_json::from_value::<GetMultipleFilesSkeletonInput>(args) {
        Ok(input) => {
            // 1. Validate and filter files
            let (valid_files, invalid_files) = validate_and_filter_files(&input.file_paths);
            
            if valid_files.is_empty() {
                return Some(Err(format!("No valid files found. Invalid files: {:?}", invalid_files)));
            }
            
            // 2. Generate intelligent skeletons with semantic context
            let storage = get_semantic_storage()?;
            let mut enriched_skeletons = Vec::new();
            
            for file_path in &valid_files {
                let skeleton = generate_intelligent_skeleton(
                    file_path,
                    &storage,
                    input.max_tokens / valid_files.len(), // Distribute tokens evenly
                )?;
                enriched_skeletons.push(skeleton);
            }
            
            // 3. Optimize token usage across files
            let optimized_skeletons = optimize_skeleton_tokens(
                enriched_skeletons,
                input.max_tokens,
            )?;
            
            // 4. Generate cross-file insights
            let cross_file_insights = generate_cross_file_insights(&optimized_skeletons)?;
            
            let mut result = json!({
                "files": optimized_skeletons,
                "cross_file_insights": cross_file_insights,
                "metadata": {
                    "total_files": valid_files.len(),
                    "total_tokens_used": calculate_total_tokens(&optimized_skeletons),
                    "token_limit": input.max_tokens,
                    "enrichment_coverage": calculate_skeleton_enrichment_coverage(&optimized_skeletons),
                }
            });
            
            if !invalid_files.is_empty() {
                result["warnings"] = json!(format!("Skipped invalid files: {:?}", invalid_files));
            }
            
            Ok(result)
        },
        Err(e) => Err(format!("Invalid arguments: {}", e)),
    })
}

#[derive(Serialize)]
struct IntelligentSkeleton {
    pub file_path: String,
    pub language: String,
    pub skeleton_content: String,
    pub symbols: Vec<SkeletonSymbol>,
    pub file_insights: FileSkeletonInsights,
    pub tokens_used: usize,
}

#[derive(Serialize)]
struct SkeletonSymbol {
    pub name: String,
    pub symbol_type: String,
    pub line_range: String,
    pub signature: String,
    
    // Semantic enrichments
    pub business_purpose: Option<String>,
    pub architectural_role: Option<String>,
    pub quality_indicators: Vec<String>,
    pub usage_frequency: Option<String>,
    pub importance_score: Option<f32>,
}

#[derive(Serialize)]
struct FileSkeletonInsights {
    pub primary_purpose: Option<String>,
    pub architectural_patterns: Vec<String>,
    pub business_domains: Vec<String>,
    pub quality_summary: String,
    pub complexity_level: String,
    pub key_responsibilities: Vec<String>,
}

fn generate_intelligent_skeleton(
    file_path: &str,
    storage: &SemanticGraphStorage,
    token_budget: usize,
) -> Result<IntelligentSkeleton> {
    // 1. Get structural symbols
    let structural_symbols = analyze_file_symbols(file_path)?;
    
    // 2. Enrich with semantic data
    let mut enriched_symbols = Vec::new();
    for symbol in structural_symbols {
        let enriched = match storage.get_enriched_symbol(&symbol.id)? {
            Some(enriched) => enriched,
            None => EnhancedSymbol::from_structural(symbol),
        };
        enriched_symbols.push(enriched);
    }
    
    // 3. Prioritize symbols by importance
    let prioritized_symbols = prioritize_symbols_for_skeleton(&enriched_symbols);
    
    // 4. Generate skeleton content with token budget
    let skeleton_content = generate_skeleton_content_with_budget(
        file_path,
        &prioritized_symbols,
        token_budget,
    )?;
    
    // 5. Create skeleton symbols
    let skeleton_symbols = create_skeleton_symbols(&prioritized_symbols);
    
    // 6. Generate file insights
    let file_insights = generate_file_skeleton_insights(&enriched_symbols);
    
    Ok(IntelligentSkeleton {
        file_path: file_path.to_string(),
        language: detect_language(file_path),
        skeleton_content,
        symbols: skeleton_symbols,
        file_insights,
        tokens_used: approximate_token_count(&skeleton_content),
    })
}

fn prioritize_symbols_for_skeleton(symbols: &[EnhancedSymbol]) -> Vec<&EnhancedSymbol> {
    let mut prioritized = symbols.iter().collect::<Vec<_>>();
    
    prioritized.sort_by(|a, b| {
        let score_a = calculate_symbol_importance_score(a);
        let score_b = calculate_symbol_importance_score(b);
        score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    prioritized
}

fn calculate_symbol_importance_score(symbol: &EnhancedSymbol) -> f32 {
    let mut score = 0.0;
    
    // Base score by symbol type
    score += match symbol.symbol_type {
        SymbolType::Class => 3.0,
        SymbolType::Function => 2.0,
        SymbolType::Method => 1.5,
        SymbolType::Interface => 2.5,
        _ => 1.0,
    };
    
    // Boost for public symbols
    if symbol.name.starts_with(char::is_uppercase) {
        score += 1.0;
    }
    
    // Boost for enriched symbols
    if symbol.business_purpose.is_some() {
        score += 2.0;
    }
    
    // Boost for architectural significance
    if symbol.architectural_role.is_some() {
        score += 1.5;
    }
    
    // Boost for quality
    if let Some(quality) = &symbol.quality_metrics {
        score += quality.overall_score() * 0.5;
    }
    
    score
}
```

#### Task 6.2: Migration Utilities (1 day)
**Files to create**:
```
codex-rs/core/src/code_analysis/
├── migration_utils.rs           # New - Migration utilities
└── compatibility_layer.rs      # New - Backward compatibility
```

**Migration Utilities**:
```rust
// migration_utils.rs
pub struct MigrationManager {
    storage: Arc<Mutex<SemanticGraphStorage>>,
    config: MigrationConfig,
}

impl MigrationManager {
    pub async fn migrate_to_enhanced_tools(&self) -> Result<MigrationReport> {
        let mut report = MigrationReport::new();
        
        // 1. Backup existing data
        self.backup_existing_data().await?;
        report.backup_completed = true;
        
        // 2. Migrate structural graph to semantic storage
        let migration_result = self.migrate_structural_to_semantic().await?;
        report.symbols_migrated = migration_result.symbols_migrated;
        report.edges_migrated = migration_result.edges_migrated;
        
        // 3. Update tool configurations
        self.update_tool_configurations().await?;
        report.tools_updated = true;
        
        // 4. Validate migration
        let validation_result = self.validate_migration().await?;
        report.validation_passed = validation_result.success;
        report.validation_errors = validation_result.errors;
        
        Ok(report)
    }
    
    async fn migrate_structural_to_semantic(&self) -> Result<StructuralMigrationResult> {
        let structural_graph = load_existing_structural_graph()?;
        let mut symbols_migrated = 0;
        let mut edges_migrated = 0;
        
        let storage = self.storage.lock().await;
        
        // Migrate symbols
        for structural_symbol in structural_graph.symbols {
            let enhanced_symbol = EnhancedSymbol::from_structural_symbol(structural_symbol);
            storage.save_enriched_symbol(&enhanced_symbol)?;
            symbols_migrated += 1;
        }
        
        // Migrate edges
        for structural_edge in structural_graph.edges {
            let enhanced_edge = EnhancedEdge::from_structural_edge(structural_edge);
            storage.save_enriched_edge(&enhanced_edge)?;
            edges_migrated += 1;
        }
        
        Ok(StructuralMigrationResult {
            symbols_migrated,
            edges_migrated,
        })
    }
}

#[derive(Debug)]
pub struct MigrationReport {
    pub backup_completed: bool,
    pub symbols_migrated: usize,
    pub edges_migrated: usize,
    pub tools_updated: bool,
    pub validation_passed: bool,
    pub validation_errors: Vec<String>,
    pub migration_duration: Duration,
}
```

#### Task 6.3: Performance Optimization (1 day)
**Files to create/modify**:
```
codex-rs/core/src/code_analysis/
├── performance_optimizer.rs    # New - Performance optimization
├── caching_strategy.rs         # New - Intelligent caching
└── tools.rs                    # Modified - Add performance monitoring
```

**Performance Optimization**:
```rust
// performance_optimizer.rs
pub struct PerformanceOptimizer {
    cache: Arc<RwLock<LruCache<String, CachedResult>>>,
    metrics: Arc<Mutex<PerformanceMetrics>>,
}

impl PerformanceOptimizer {
    pub async fn optimize_tool_response<T>(
        &self,
        tool_name: &str,
        cache_key: &str,
        expensive_operation: impl Future<Output = Result<T>>,
    ) -> Result<T>
    where
        T: Clone + Send + Sync + 'static,
    {
        let start_time = Instant::now();
        
        // Check cache first
        if let Some(cached) = self.get_from_cache(cache_key).await {
            self.record_cache_hit(tool_name, start_time).await;
            return Ok(cached);
        }
        
        // Execute expensive operation
        let result = expensive_operation.await?;
        
        // Cache the result
        self.cache_result(cache_key, &result).await;
        
        // Record metrics
        self.record_cache_miss(tool_name, start_time).await;
        
        Ok(result)
    }
    
    async fn get_from_cache<T>(&self, cache_key: &str) -> Option<T>
    where
        T: Clone + 'static,
    {
        let cache = self.cache.read().await;
        cache.get(cache_key)
            .and_then(|cached| cached.value.downcast_ref::<T>())
            .cloned()
    }
    
    async fn cache_result<T>(&self, cache_key: &str, result: &T)
    where
        T: Clone + Send + Sync + 'static,
    {
        let mut cache = self.cache.write().await;
        cache.put(cache_key.to_string(), CachedResult {
            value: Box::new(result.clone()),
            timestamp: Utc::now(),
        });
    }
}

#[derive(Debug)]
pub struct PerformanceMetrics {
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub average_response_time: Duration,
    pub tool_usage_stats: HashMap<String, ToolUsageStats>,
}

#[derive(Debug)]
pub struct ToolUsageStats {
    pub total_calls: u64,
    pub cache_hit_rate: f32,
    pub average_response_time: Duration,
    pub error_rate: f32,
}
```

---

## 🧪 Phase 3 Testing Strategy

### Integration Tests
```rust
#[tokio::test]
async fn test_enhanced_analyze_code() {
    let result = handle_analyze_code_enhanced(json!({
        "file_path": "test_files/csharp_test_suite/UserService.cs"
    })).unwrap().unwrap();
    
    let symbols = result["symbols"].as_array().unwrap();
    assert!(!symbols.is_empty());
    
    // Check for semantic enrichments
    let first_symbol = &symbols[0];
    assert!(first_symbol["business_purpose"].is_string());
    assert!(first_symbol["enrichment_confidence"].is_number());
}

#[tokio::test]
async fn test_enhanced_find_references() {
    let result = handle_find_symbol_references_enhanced(json!({
        "symbol_name": "UserService"
    })).unwrap().unwrap();
    
    let references = result["references"].as_array().unwrap();
    let usage_analytics = &result["usage_analytics"];
    
    assert!(!references.is_empty());
    assert!(usage_analytics["total_references"].is_number());
}
```

### Performance Tests
```rust
#[tokio::test]
async fn test_enhanced_tools_performance() {
    let start = Instant::now();
    
    let result = handle_analyze_code_enhanced(json!({
        "file_path": "large_test_file.cs"
    })).unwrap().unwrap();
    
    let duration = start.elapsed();
    
    // Should complete within reasonable time even with enrichments
    assert!(duration < Duration::from_secs(5));
    assert!(result["symbols"].as_array().unwrap().len() > 0);
}
```

---

## 📊 Phase 3 Success Metrics

1. **Functional Requirements**:
   - All existing tools return enriched data when available
   - Fallback mechanisms work seamlessly
   - Migration utilities successfully upgrade existing installations

2. **Performance Requirements**:
   - Enhanced tools respond within 150% of original response times
   - Cache hit rate >70% for repeated queries
   - Memory usage increase <100MB for typical workloads

3. **Quality Requirements**:
   - Backward compatibility maintained (all existing tests pass)
   - Enhanced responses provide meaningful additional context
   - User experience significantly improved

**Ready for Phase 4**: Advanced Features and Workflow Detection