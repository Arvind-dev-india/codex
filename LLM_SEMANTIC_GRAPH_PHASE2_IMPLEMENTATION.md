# Phase 2: LLM Integration Implementation Guide

## 🎯 Phase 2 Goals (Weeks 3-4)

**Objective**: Enable LLM to intelligently enrich symbols with semantic business context

**Deliverables**:
- LLM-driven semantic analysis engine
- Intelligent enrichment strategies
- Confidence-based update mechanisms
- Batch processing for efficiency
- Real-time enrichment during code analysis

**Success Criteria**:
- LLM can successfully enrich symbols with business context
- Confidence scoring works accurately
- Batch processing improves performance by >50%
- Real-time enrichment doesn't impact tool response times

---

## 📋 Phase 2 Task Breakdown

### Week 3: LLM Semantic Analysis Engine

#### Task 3.1: Semantic Analysis Framework (3 days)
**Files to create**:
```
codex-rs/core/src/code_analysis/
├── semantic_analyzer.rs         # New - Core semantic analysis engine
├── pattern_detector.rs          # New - Architectural pattern detection
├── business_extractor.rs        # New - Business logic extraction
└── quality_analyzer.rs          # New - Code quality assessment
```

**Core Semantic Analyzer**:
```rust
// semantic_analyzer.rs
pub struct SemanticAnalyzer {
    pattern_detector: PatternDetector,
    business_extractor: BusinessLogicExtractor,
    quality_analyzer: QualityAnalyzer,
    confidence_calculator: ConfidenceCalculator,
}

impl SemanticAnalyzer {
    pub async fn analyze_symbol_semantics(
        &self, 
        symbol: &Symbol, 
        context: &AnalysisContext
    ) -> Result<SemanticEnrichment> {
        
        // 1. Detect architectural patterns
        let patterns = self.pattern_detector.detect_patterns(symbol, context).await?;
        
        // 2. Extract business logic context
        let business_context = self.business_extractor.extract_context(symbol, context).await?;
        
        // 3. Analyze code quality
        let quality_metrics = self.quality_analyzer.analyze_quality(symbol, context).await?;
        
        // 4. Calculate confidence score
        let confidence = self.confidence_calculator.calculate_confidence(
            &patterns, &business_context, &quality_metrics
        );
        
        Ok(SemanticEnrichment {
            business_purpose: business_context.purpose,
            business_rules: business_context.rules,
            architectural_role: patterns.primary_pattern,
            quality_metrics,
            usage_patterns: business_context.usage_patterns,
            improvement_suggestions: quality_metrics.suggestions,
            confidence_score: confidence,
        })
    }
    
    pub async fn analyze_relationship_semantics(
        &self,
        edge: &Edge,
        from_symbol: &Symbol,
        to_symbol: &Symbol,
        context: &AnalysisContext
    ) -> Result<RelationshipEnrichment> {
        // Analyze semantic meaning of relationships between symbols
    }
}
```

**Pattern Detector Implementation**:
```rust
// pattern_detector.rs
pub struct PatternDetector {
    pattern_rules: Vec<PatternRule>,
}

impl PatternDetector {
    pub async fn detect_patterns(
        &self, 
        symbol: &Symbol, 
        context: &AnalysisContext
    ) -> Result<DetectedPatterns> {
        
        let mut detected_patterns = Vec::new();
        
        // Repository Pattern Detection
        if self.is_repository_pattern(symbol, context) {
            detected_patterns.push(ArchitecturalPattern {
                name: "Repository Pattern".to_string(),
                confidence: self.calculate_repository_confidence(symbol, context),
                description: "Data access layer abstraction".to_string(),
                responsibilities: vec![
                    "Data persistence".to_string(),
                    "Query abstraction".to_string(),
                    "Domain model mapping".to_string(),
                ],
            });
        }
        
        // MVC Pattern Detection
        if self.is_controller_pattern(symbol, context) {
            detected_patterns.push(ArchitecturalPattern {
                name: "MVC Controller".to_string(),
                confidence: self.calculate_controller_confidence(symbol, context),
                description: "HTTP request handling and response coordination".to_string(),
                responsibilities: vec![
                    "Request validation".to_string(),
                    "Business logic coordination".to_string(),
                    "Response formatting".to_string(),
                ],
            });
        }
        
        // Service Layer Pattern Detection
        if self.is_service_pattern(symbol, context) {
            detected_patterns.push(ArchitecturalPattern {
                name: "Service Layer".to_string(),
                confidence: self.calculate_service_confidence(symbol, context),
                description: "Business logic orchestration".to_string(),
                responsibilities: vec![
                    "Business rule enforcement".to_string(),
                    "Transaction management".to_string(),
                    "Cross-cutting concerns".to_string(),
                ],
            });
        }
        
        Ok(DetectedPatterns {
            patterns: detected_patterns,
            primary_pattern: self.select_primary_pattern(&detected_patterns),
        })
    }
    
    fn is_repository_pattern(&self, symbol: &Symbol, context: &AnalysisContext) -> bool {
        // Check for Repository pattern indicators:
        // 1. Class name ends with "Repository"
        // 2. Has CRUD methods (Save, Update, Delete, Find, Get)
        // 3. Depends on database/ORM types
        // 4. Returns domain entities
        
        let name_matches = symbol.name.ends_with("Repository") || 
                          symbol.name.ends_with("Repo");
        
        let has_crud_methods = context.get_methods_for_symbol(&symbol.name)
            .iter()
            .any(|method| {
                let method_name = method.name.to_lowercase();
                method_name.contains("save") || 
                method_name.contains("update") || 
                method_name.contains("delete") || 
                method_name.contains("find") || 
                method_name.contains("get")
            });
        
        let has_database_dependencies = context.get_dependencies_for_symbol(&symbol.name)
            .iter()
            .any(|dep| {
                dep.to_lowercase().contains("database") ||
                dep.to_lowercase().contains("context") ||
                dep.to_lowercase().contains("connection")
            });
        
        name_matches && has_crud_methods && has_database_dependencies
    }
}
```

**Business Logic Extractor**:
```rust
// business_extractor.rs
pub struct BusinessLogicExtractor {
    domain_vocabulary: DomainVocabulary,
    workflow_detector: WorkflowDetector,
}

impl BusinessLogicExtractor {
    pub async fn extract_context(
        &self, 
        symbol: &Symbol, 
        context: &AnalysisContext
    ) -> Result<BusinessContext> {
        
        // 1. Analyze method names for business intent
        let business_intent = self.analyze_business_intent(symbol, context);
        
        // 2. Extract business rules from validation logic
        let business_rules = self.extract_business_rules(symbol, context);
        
        // 3. Identify workflow participation
        let workflow_participation = self.identify_workflows(symbol, context);
        
        // 4. Analyze domain concepts
        let domain_concepts = self.analyze_domain_concepts(symbol, context);
        
        Ok(BusinessContext {
            purpose: business_intent.primary_purpose,
            rules: business_rules,
            workflows: workflow_participation,
            domain_concepts,
            usage_patterns: self.extract_usage_patterns(symbol, context),
        })
    }
    
    fn analyze_business_intent(&self, symbol: &Symbol, context: &AnalysisContext) -> BusinessIntent {
        // Analyze method names and parameters to understand business purpose
        let methods = context.get_methods_for_symbol(&symbol.name);
        
        let mut intent_indicators = Vec::new();
        
        for method in methods {
            // Authentication patterns
            if method.name.to_lowercase().contains("authenticate") ||
               method.name.to_lowercase().contains("login") ||
               method.name.to_lowercase().contains("verify") {
                intent_indicators.push("Authentication".to_string());
            }
            
            // User management patterns
            if method.name.to_lowercase().contains("create") && 
               method.name.to_lowercase().contains("user") {
                intent_indicators.push("User Management".to_string());
            }
            
            // Payment processing patterns
            if method.name.to_lowercase().contains("process") && 
               method.name.to_lowercase().contains("payment") {
                intent_indicators.push("Payment Processing".to_string());
            }
            
            // Validation patterns
            if method.name.to_lowercase().contains("validate") ||
               method.name.to_lowercase().contains("check") {
                intent_indicators.push("Validation".to_string());
            }
        }
        
        BusinessIntent {
            primary_purpose: self.determine_primary_purpose(&intent_indicators),
            secondary_purposes: intent_indicators,
        }
    }
    
    fn extract_business_rules(&self, symbol: &Symbol, context: &AnalysisContext) -> Vec<String> {
        let mut rules = Vec::new();
        
        // Look for validation patterns in method bodies
        let method_bodies = context.get_method_bodies_for_symbol(&symbol.name);
        
        for body in method_bodies {
            // Email validation rules
            if body.contains("@") && body.contains("email") {
                rules.push("Email address must be valid format".to_string());
            }
            
            // Password strength rules
            if body.contains("password") && body.contains("length") {
                rules.push("Password must meet minimum length requirements".to_string());
            }
            
            // Uniqueness constraints
            if body.contains("unique") || body.contains("duplicate") {
                rules.push("Must enforce uniqueness constraints".to_string());
            }
            
            // Authorization rules
            if body.contains("permission") || body.contains("role") {
                rules.push("Must enforce authorization rules".to_string());
            }
        }
        
        rules
    }
}
```

#### Task 3.2: Confidence Calculation System (2 days)
**Files to create**:
```
codex-rs/core/src/code_analysis/
├── confidence_calculator.rs     # New - Confidence scoring system
└── enrichment_quality.rs       # New - Quality assessment for enrichments
```

**Confidence Calculator**:
```rust
// confidence_calculator.rs
pub struct ConfidenceCalculator {
    pattern_weights: HashMap<String, f32>,
    context_weights: ContextWeights,
}

impl ConfidenceCalculator {
    pub fn calculate_confidence(
        &self,
        patterns: &DetectedPatterns,
        business_context: &BusinessContext,
        quality_metrics: &QualityMetrics,
    ) -> f32 {
        
        let mut confidence_score = 0.0;
        let mut total_weight = 0.0;
        
        // Pattern detection confidence (30% weight)
        let pattern_confidence = self.calculate_pattern_confidence(patterns);
        confidence_score += pattern_confidence * 0.3;
        total_weight += 0.3;
        
        // Business context confidence (40% weight)
        let business_confidence = self.calculate_business_confidence(business_context);
        confidence_score += business_confidence * 0.4;
        total_weight += 0.4;
        
        // Code quality confidence (20% weight)
        let quality_confidence = self.calculate_quality_confidence(quality_metrics);
        confidence_score += quality_confidence * 0.2;
        total_weight += 0.2;
        
        // Context richness (10% weight)
        let context_confidence = self.calculate_context_confidence(business_context);
        confidence_score += context_confidence * 0.1;
        total_weight += 0.1;
        
        // Normalize to 0.0-1.0 range
        if total_weight > 0.0 {
            confidence_score / total_weight
        } else {
            0.0
        }
    }
    
    fn calculate_pattern_confidence(&self, patterns: &DetectedPatterns) -> f32 {
        if patterns.patterns.is_empty() {
            return 0.1; // Low confidence if no patterns detected
        }
        
        // Average confidence of detected patterns, weighted by pattern importance
        let mut weighted_sum = 0.0;
        let mut total_weight = 0.0;
        
        for pattern in &patterns.patterns {
            let pattern_weight = self.pattern_weights
                .get(&pattern.name)
                .unwrap_or(&1.0);
            
            weighted_sum += pattern.confidence * pattern_weight;
            total_weight += pattern_weight;
        }
        
        if total_weight > 0.0 {
            weighted_sum / total_weight
        } else {
            0.5
        }
    }
    
    fn calculate_business_confidence(&self, business_context: &BusinessContext) -> f32 {
        let mut confidence_factors = Vec::new();
        
        // Purpose clarity
        if business_context.purpose.len() > 10 {
            confidence_factors.push(0.8);
        } else {
            confidence_factors.push(0.3);
        }
        
        // Business rules richness
        let rules_confidence = match business_context.rules.len() {
            0 => 0.1,
            1..=2 => 0.5,
            3..=5 => 0.7,
            _ => 0.9,
        };
        confidence_factors.push(rules_confidence);
        
        // Workflow participation
        if !business_context.workflows.is_empty() {
            confidence_factors.push(0.8);
        } else {
            confidence_factors.push(0.4);
        }
        
        // Domain concept richness
        let domain_confidence = match business_context.domain_concepts.len() {
            0 => 0.2,
            1..=3 => 0.6,
            _ => 0.8,
        };
        confidence_factors.push(domain_confidence);
        
        // Average the confidence factors
        confidence_factors.iter().sum::<f32>() / confidence_factors.len() as f32
    }
}
```

### Week 4: Intelligent Enrichment Strategies

#### Task 4.1: Smart Enrichment Orchestrator (2 days)
**Files to create**:
```
codex-rs/core/src/code_analysis/
├── enrichment_orchestrator.rs   # New - Coordinates enrichment process
├── enrichment_strategies.rs     # New - Different enrichment approaches
└── batch_processor.rs           # New - Batch processing for efficiency
```

**Enrichment Orchestrator**:
```rust
// enrichment_orchestrator.rs
pub struct EnrichmentOrchestrator {
    semantic_analyzer: SemanticAnalyzer,
    storage: Arc<Mutex<SemanticGraphStorage>>,
    strategies: Vec<Box<dyn EnrichmentStrategy>>,
    batch_processor: BatchProcessor,
}

impl EnrichmentOrchestrator {
    pub async fn enrich_symbol_intelligently(
        &self,
        symbol_id: &str,
        force_refresh: bool,
    ) -> Result<EnrichmentResult> {
        
        // 1. Check if enrichment is needed
        if !force_refresh && !self.needs_enrichment(symbol_id).await? {
            return Ok(EnrichmentResult::AlreadyEnriched);
        }
        
        // 2. Get symbol and context
        let symbol = self.get_symbol_with_context(symbol_id).await?;
        let context = self.build_analysis_context(&symbol).await?;
        
        // 3. Select appropriate enrichment strategy
        let strategy = self.select_enrichment_strategy(&symbol, &context);
        
        // 4. Perform semantic analysis
        let enrichment = strategy.enrich_symbol(&symbol, &context).await?;
        
        // 5. Validate and store enrichment
        let validated_enrichment = self.validate_enrichment(&enrichment)?;
        self.store_enrichment(symbol_id, &validated_enrichment).await?;
        
        // 6. Schedule related symbols for enrichment
        self.schedule_related_enrichments(&symbol, &context).await?;
        
        Ok(EnrichmentResult::Enriched(validated_enrichment))
    }
    
    pub async fn batch_enrich_symbols(
        &self,
        symbol_ids: Vec<String>,
        batch_size: usize,
    ) -> Result<BatchEnrichmentResult> {
        
        self.batch_processor.process_batch(
            symbol_ids,
            batch_size,
            |batch| async move {
                let mut results = Vec::new();
                
                for symbol_id in batch {
                    match self.enrich_symbol_intelligently(&symbol_id, false).await {
                        Ok(result) => results.push((symbol_id, result)),
                        Err(e) => {
                            tracing::warn!("Failed to enrich symbol {}: {}", symbol_id, e);
                            results.push((symbol_id, EnrichmentResult::Failed(e.to_string())));
                        }
                    }
                }
                
                Ok(results)
            }
        ).await
    }
    
    async fn needs_enrichment(&self, symbol_id: &str) -> Result<bool> {
        let storage = self.storage.lock().await;
        
        match storage.get_enriched_symbol(symbol_id)? {
            Some(symbol) => {
                // Check if enrichment is stale or low confidence
                let is_stale = symbol.semantic_last_updated
                    .map(|updated| Utc::now() - updated > Duration::days(30))
                    .unwrap_or(true);
                
                let is_low_confidence = symbol.llm_confidence
                    .map(|conf| conf < 0.7)
                    .unwrap_or(true);
                
                Ok(is_stale || is_low_confidence)
            }
            None => Ok(true), // No enrichment exists
        }
    }
    
    fn select_enrichment_strategy(
        &self,
        symbol: &Symbol,
        context: &AnalysisContext,
    ) -> &dyn EnrichmentStrategy {
        
        // Select strategy based on symbol characteristics
        match symbol.symbol_type {
            SymbolType::Class => {
                if context.has_many_methods() {
                    &self.strategies[0] // ComplexClassStrategy
                } else {
                    &self.strategies[1] // SimpleClassStrategy
                }
            }
            SymbolType::Function => {
                if context.is_public_api() {
                    &self.strategies[2] // PublicApiStrategy
                } else {
                    &self.strategies[3] // InternalFunctionStrategy
                }
            }
            _ => &self.strategies[4], // DefaultStrategy
        }
    }
}
```

**Enrichment Strategies**:
```rust
// enrichment_strategies.rs
#[async_trait]
pub trait EnrichmentStrategy: Send + Sync {
    async fn enrich_symbol(
        &self,
        symbol: &Symbol,
        context: &AnalysisContext,
    ) -> Result<SemanticEnrichment>;
    
    fn strategy_name(&self) -> &str;
    fn confidence_multiplier(&self) -> f32;
}

pub struct ComplexClassStrategy {
    semantic_analyzer: SemanticAnalyzer,
}

#[async_trait]
impl EnrichmentStrategy for ComplexClassStrategy {
    async fn enrich_symbol(
        &self,
        symbol: &Symbol,
        context: &AnalysisContext,
    ) -> Result<SemanticEnrichment> {
        
        // For complex classes, do comprehensive analysis
        let mut enrichment = self.semantic_analyzer
            .analyze_symbol_semantics(symbol, context)
            .await?;
        
        // Additional analysis for complex classes
        enrichment.architectural_role = Some(
            self.determine_architectural_role(symbol, context).await?
        );
        
        enrichment.usage_patterns = self.analyze_usage_patterns(symbol, context).await?;
        
        // Higher confidence for complex classes due to more context
        enrichment.confidence_score *= 1.2;
        enrichment.confidence_score = enrichment.confidence_score.min(1.0);
        
        Ok(enrichment)
    }
    
    fn strategy_name(&self) -> &str {
        "ComplexClassStrategy"
    }
    
    fn confidence_multiplier(&self) -> f32 {
        1.2
    }
}

pub struct PublicApiStrategy {
    semantic_analyzer: SemanticAnalyzer,
}

#[async_trait]
impl EnrichmentStrategy for PublicApiStrategy {
    async fn enrich_symbol(
        &self,
        symbol: &Symbol,
        context: &AnalysisContext,
    ) -> Result<SemanticEnrichment> {
        
        let mut enrichment = self.semantic_analyzer
            .analyze_symbol_semantics(symbol, context)
            .await?;
        
        // For public APIs, focus on usage patterns and documentation
        enrichment.usage_patterns = self.analyze_public_usage_patterns(symbol, context).await?;
        enrichment.improvement_suggestions.extend(
            self.suggest_api_improvements(symbol, context).await?
        );
        
        // Add security considerations for public APIs
        enrichment.security_considerations = self.analyze_security_implications(symbol, context).await?;
        
        Ok(enrichment)
    }
    
    fn strategy_name(&self) -> &str {
        "PublicApiStrategy"
    }
    
    fn confidence_multiplier(&self) -> f32 {
        1.1
    }
}
```

#### Task 4.2: Batch Processing System (2 days)
**Files to create**:
```
codex-rs/core/src/code_analysis/
├── batch_processor.rs           # New - Efficient batch processing
└── enrichment_queue.rs          # New - Queue management for enrichments
```

**Batch Processor**:
```rust
// batch_processor.rs
pub struct BatchProcessor {
    max_concurrent_batches: usize,
    batch_timeout: Duration,
    retry_policy: RetryPolicy,
}

impl BatchProcessor {
    pub async fn process_batch<T, F, Fut>(
        &self,
        items: Vec<T>,
        batch_size: usize,
        processor: F,
    ) -> Result<Vec<ProcessingResult<T>>>
    where
        F: Fn(Vec<T>) -> Fut + Send + Sync + Clone,
        Fut: Future<Output = Result<Vec<(T, EnrichmentResult)>>> + Send,
        T: Send + Clone,
    {
        let batches: Vec<Vec<T>> = items
            .chunks(batch_size)
            .map(|chunk| chunk.to_vec())
            .collect();
        
        let semaphore = Arc::new(Semaphore::new(self.max_concurrent_batches));
        let mut tasks = Vec::new();
        
        for batch in batches {
            let permit = semaphore.clone().acquire_owned().await?;
            let processor_clone = processor.clone();
            let timeout = self.batch_timeout;
            
            let task = tokio::spawn(async move {
                let _permit = permit; // Keep permit until task completes
                
                // Process batch with timeout
                let result = tokio::time::timeout(
                    timeout,
                    processor_clone(batch.clone())
                ).await;
                
                match result {
                    Ok(Ok(results)) => results,
                    Ok(Err(e)) => {
                        // Return error for all items in batch
                        batch.into_iter()
                            .map(|item| (item, EnrichmentResult::Failed(e.to_string())))
                            .collect()
                    }
                    Err(_) => {
                        // Timeout - return timeout error for all items
                        batch.into_iter()
                            .map(|item| (item, EnrichmentResult::Failed("Timeout".to_string())))
                            .collect()
                    }
                }
            });
            
            tasks.push(task);
        }
        
        // Wait for all batches to complete
        let mut all_results = Vec::new();
        for task in tasks {
            match task.await {
                Ok(batch_results) => {
                    for (item, result) in batch_results {
                        all_results.push(ProcessingResult {
                            item,
                            result,
                        });
                    }
                }
                Err(e) => {
                    tracing::error!("Batch processing task failed: {}", e);
                }
            }
        }
        
        Ok(all_results)
    }
}

#[derive(Debug)]
pub struct ProcessingResult<T> {
    pub item: T,
    pub result: EnrichmentResult,
}

#[derive(Debug)]
pub enum EnrichmentResult {
    Enriched(SemanticEnrichment),
    AlreadyEnriched,
    Failed(String),
    Skipped(String),
}
```

---

## 🧪 Phase 2 Testing Strategy

### Integration Tests
```rust
#[tokio::test]
async fn test_semantic_analysis_engine() {
    let analyzer = SemanticAnalyzer::new();
    let symbol = create_test_symbol("UserService");
    let context = create_test_context();
    
    let enrichment = analyzer.analyze_symbol_semantics(&symbol, &context).await.unwrap();
    
    assert!(enrichment.business_purpose.is_some());
    assert!(!enrichment.business_rules.is_empty());
    assert!(enrichment.confidence_score > 0.5);
}

#[tokio::test]
async fn test_batch_enrichment() {
    let orchestrator = EnrichmentOrchestrator::new();
    let symbol_ids = vec!["UserService".to_string(), "UserRepository".to_string()];
    
    let results = orchestrator.batch_enrich_symbols(symbol_ids, 2).await.unwrap();
    
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|r| matches!(r.result, EnrichmentResult::Enriched(_))));
}
```

### Performance Tests
```rust
#[tokio::test]
async fn test_enrichment_performance() {
    let orchestrator = EnrichmentOrchestrator::new();
    let symbol_ids: Vec<String> = (0..100).map(|i| format!("Symbol{}", i)).collect();
    
    let start = Instant::now();
    let results = orchestrator.batch_enrich_symbols(symbol_ids, 10).await.unwrap();
    let duration = start.elapsed();
    
    // Should complete 100 symbols in under 30 seconds
    assert!(duration < Duration::from_secs(30));
    assert_eq!(results.len(), 100);
}
```

---

## 📊 Phase 2 Success Metrics

1. **Functional Requirements**:
   - Semantic analysis engine produces meaningful enrichments
   - Confidence scoring accurately reflects enrichment quality
   - Batch processing handles large symbol sets efficiently

2. **Performance Requirements**:
   - Single symbol enrichment completes within 2 seconds
   - Batch processing achieves >50% performance improvement over sequential
   - Memory usage scales linearly with batch size

3. **Quality Requirements**:
   - Enrichment confidence scores correlate with manual quality assessment
   - Pattern detection accuracy >80% on test cases
   - Business context extraction provides meaningful insights

**Ready for Phase 3**: Enhanced Tools Integration