// Unified Relationship Query Interface
// Phase 1: Single interface for querying all relationships (main + cross-project)

use std::collections::HashMap;
use serde_json::{json, Value};
use super::context_extractor::{CodeSymbol, SymbolReference};
use super::supplementary_registry::{SupplementarySymbolRegistry, SupplementarySymbolInfo};

/// Unified result combining main graph edges, cross-project relationships, and AST analysis
#[derive(Debug, Clone)]
pub struct UnifiedRelationshipResult {
    pub main_references: Vec<MainProjectReference>,
    pub cross_project_relationships: Vec<CrossProjectRelationship>,
    pub ast_relationships: Vec<ASTRelationship>,
    pub summary: RelationshipSummary,
}

/// Reference within the main project
#[derive(Debug, Clone)]
pub struct MainProjectReference {
    pub file_path: String,
    pub line: usize,
    pub column: usize,
    pub reference_type: String,
    pub source_symbol: String,
    pub target_symbol: String,
}

/// Cross-project relationship (traditional FQN/name matching)
#[derive(Debug, Clone)]
pub struct CrossProjectRelationship {
    pub cross_project_symbol_fqn: String,
    pub cross_project_file_path: String,
    pub cross_project_name: String,
    pub project_name: String,
    pub relationship_type: String, // "reference", "definition", "usage"
    pub confidence: f32,
    pub detection_method: String, // "fqn_match", "name_match"
}

/// AST-based relationship (enhanced same-named symbol analysis)
#[derive(Debug, Clone)]
pub struct ASTRelationship {
    pub main_symbol_fqn: String,
    pub cross_project_symbol_fqn: String,
    pub relationship_type: String, // "wrapper", "implementation", "inheritance", "unrelated"
    pub confidence: f32,
    pub detection_method: String, // "ast_analysis"
    pub ast_patterns_found: Vec<String>, // Debug info about what AST patterns were detected
}

/// Summary statistics for the relationship query
#[derive(Debug, Clone)]
pub struct RelationshipSummary {
    pub total_main_references: usize,
    pub total_cross_project_relationships: usize,
    pub total_ast_relationships: usize,
    pub strong_relationships: usize, // High confidence relationships
    pub cross_project_boundaries_detected: bool,
}

impl UnifiedRelationshipResult {
    /// Create a new empty result
    pub fn new() -> Self {
        Self {
            main_references: Vec::new(),
            cross_project_relationships: Vec::new(),
            ast_relationships: Vec::new(),
            summary: RelationshipSummary {
                total_main_references: 0,
                total_cross_project_relationships: 0,
                total_ast_relationships: 0,
                strong_relationships: 0,
                cross_project_boundaries_detected: false,
            },
        }
    }

    /// Combine results from different layers
    pub fn combine(
        main_refs: Vec<MainProjectReference>,
        cross_project_rels: Vec<CrossProjectRelationship>,
        ast_rels: Vec<ASTRelationship>,
    ) -> Self {
        let strong_relationships = cross_project_rels.iter().filter(|r| r.confidence > 0.5).count()
            + ast_rels.iter().filter(|r| r.confidence > 0.5).count();

        let cross_project_boundaries_detected = !cross_project_rels.is_empty() || !ast_rels.is_empty();

        Self {
            summary: RelationshipSummary {
                total_main_references: main_refs.len(),
                total_cross_project_relationships: cross_project_rels.len(),
                total_ast_relationships: ast_rels.len(),
                strong_relationships,
                cross_project_boundaries_detected,
            },
            main_references: main_refs,
            cross_project_relationships: cross_project_rels,
            ast_relationships: ast_rels,
        }
    }

    /// Convert to JSON for tool responses (find_symbol_references format)
    pub fn to_references_json(&self) -> Value {
        let mut references = Vec::new();

        // Add main project references
        for main_ref in &self.main_references {
            references.push(json!({
                "file_path": main_ref.file_path,
                "line": main_ref.line,
                "column": main_ref.column,
                "reference_type": main_ref.reference_type,
                "project_type": "main"
            }));
        }

        // Add cross-project relationships
        for cross_rel in &self.cross_project_relationships {
            references.push(json!({
                "file_path": cross_rel.cross_project_file_path,
                "line": 1, // Default line for cross-project
                "column": 0,
                "reference_type": cross_rel.relationship_type,
                "project_type": "cross-project",
                "project_name": cross_rel.project_name,
                "confidence": cross_rel.confidence,
                "detection_method": cross_rel.detection_method
            }));
        }

        // Add AST relationships
        for ast_rel in &self.ast_relationships {
            if ast_rel.relationship_type != "unrelated" {
                references.push(json!({
                    "file_path": "cross-project", // Will be resolved later
                    "line": 1,
                    "column": 0,
                    "reference_type": ast_rel.relationship_type,
                    "project_type": "cross-project",
                    "main_symbol_fqn": ast_rel.main_symbol_fqn,
                    "cross_symbol_fqn": ast_rel.cross_project_symbol_fqn,
                    "confidence": ast_rel.confidence,
                    "detection_method": ast_rel.detection_method,
                    "ast_patterns": ast_rel.ast_patterns_found
                }));
            }
        }

        json!({
            "references": references,
            "summary": {
                "total_references": references.len(),
                "main_project_references": self.summary.total_main_references,
                "cross_project_references": self.summary.total_cross_project_relationships + self.summary.total_ast_relationships,
                "strong_relationships": self.summary.strong_relationships,
                "cross_project_boundaries_detected": self.summary.cross_project_boundaries_detected
            }
        })
    }

    /// Convert to JSON for tool responses (find_symbol_definitions format)
    pub fn to_definitions_json(&self) -> Value {
        let mut definitions = Vec::new();

        // Add cross-project definitions with relationship information
        for cross_rel in &self.cross_project_relationships {
            if cross_rel.relationship_type == "definition" {
                // Find related AST relationships for this cross-project symbol
                let related_ast_rels: Vec<_> = self.ast_relationships
                    .iter()
                    .filter(|ast| ast.cross_project_symbol_fqn == cross_rel.cross_project_symbol_fqn)
                    .collect();

                let related_main_symbols: Vec<Value> = related_ast_rels
                    .iter()
                    .map(|ast| json!({
                        "main_fqn": ast.main_symbol_fqn,
                        "relationship": ast.relationship_type,
                        "confidence": ast.confidence
                    }))
                    .collect();

                definitions.push(json!({
                    "symbol": cross_rel.cross_project_name,
                    "file_path": cross_rel.cross_project_file_path,
                    "start_line": 1, // Will be resolved from supplementary registry
                    "end_line": 1,
                    "symbol_type": "unknown", // Will be resolved from supplementary registry
                    "project_type": "cross-project",
                    "project_name": cross_rel.project_name,
                    "cross_symbol_fqn": cross_rel.cross_project_symbol_fqn,
                    "related_main_symbols": related_main_symbols,
                    "relationship_detected": !related_main_symbols.is_empty(),
                    "confidence": cross_rel.confidence,
                    "detection_method": cross_rel.detection_method
                }));
            }
        }

        json!({
            "definitions": definitions,
            "summary": {
                "total_definitions": definitions.len(),
                "cross_project_definitions": definitions.len(),
                "relationships_detected": self.summary.total_ast_relationships,
                "cross_project_boundaries_detected": self.summary.cross_project_boundaries_detected
            }
        })
    }
}

/// Unified query interface for all relationship types
pub struct UnifiedRelationshipQuery<'a> {
    main_symbols: &'a HashMap<String, CodeSymbol>,
    main_references: &'a [SymbolReference],
    supplementary_registry: Option<&'a SupplementarySymbolRegistry>,
}

impl<'a> UnifiedRelationshipQuery<'a> {
    pub fn new(
        main_symbols: &'a HashMap<String, CodeSymbol>,
        main_references: &'a [SymbolReference],
        supplementary_registry: Option<&'a SupplementarySymbolRegistry>,
    ) -> Self {
        Self {
            main_symbols,
            main_references,
            supplementary_registry,
        }
    }

    /// Query all relationships for a given symbol name
    pub fn query_symbol_relationships(&self, symbol_name: &str) -> UnifiedRelationshipResult {
        tracing::debug!("🔍 Unified query for symbol: {}", symbol_name);

        // Layer 1: Query main project references
        let main_refs = self.query_main_project_references(symbol_name);
        tracing::debug!("Layer 1 - Main references: {}", main_refs.len());

        // Layer 2: Query cross-project relationships (traditional matching)
        let cross_project_rels = self.query_cross_project_relationships(symbol_name);
        tracing::debug!("Layer 2 - Cross-project relationships: {}", cross_project_rels.len());

        // Layer 3: Query AST relationships (enhanced same-named symbol analysis)
        let ast_rels = self.query_ast_relationships(symbol_name);
        tracing::debug!("Layer 3 - AST relationships: {}", ast_rels.len());

        // Combine all layers
        let result = UnifiedRelationshipResult::combine(main_refs, cross_project_rels, ast_rels);
        
        tracing::info!("🎯 Unified query complete: {} main refs, {} cross-project rels, {} AST rels", 
                      result.summary.total_main_references,
                      result.summary.total_cross_project_relationships,
                      result.summary.total_ast_relationships);

        result
    }

    /// Layer 1: Query main project references
    fn query_main_project_references(&self, symbol_name: &str) -> Vec<MainProjectReference> {
        self.main_references
            .iter()
            .filter(|r| r.symbol_name.contains(symbol_name))
            .map(|r| MainProjectReference {
                file_path: r.reference_file.clone(),
                line: r.reference_line,
                column: r.reference_col,
                reference_type: format!("{:?}", r.reference_type).to_lowercase(),
                source_symbol: r.symbol_name.clone(),
                target_symbol: r.symbol_fqn.clone(),
            })
            .collect()
    }

    /// Layer 2: Query cross-project relationships (traditional FQN/name matching)
    fn query_cross_project_relationships(&self, symbol_name: &str) -> Vec<CrossProjectRelationship> {
        if let Some(registry) = self.supplementary_registry {
            registry.symbols
                .iter()
                .filter(|(_, symbol)| symbol.name.contains(symbol_name))
                .map(|(fqn, symbol)| CrossProjectRelationship {
                    cross_project_symbol_fqn: fqn.clone(),
                    cross_project_file_path: symbol.file_path.clone(),
                    cross_project_name: symbol.name.clone(),
                    project_name: symbol.project_name.clone(),
                    relationship_type: "definition".to_string(),
                    confidence: if symbol.name == symbol_name { 1.0 } else { 0.7 },
                    detection_method: if symbol.name == symbol_name { "exact_name_match" } else { "partial_name_match" }.to_string(),
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Layer 3: Query AST relationships (enhanced same-named symbol analysis)
    fn query_ast_relationships(&self, symbol_name: &str) -> Vec<ASTRelationship> {
        if let Some(registry) = self.supplementary_registry {
            let mut ast_relationships = Vec::new();

            // Get main project symbols with this name
            let main_symbols_with_name: Vec<_> = self.main_symbols
                .values()
                .filter(|s| s.name == symbol_name)
                .collect();

            // Get cross-project symbols with this name
            let cross_symbols_with_name: Vec<_> = registry.symbols
                .iter()
                .filter(|(_, s)| s.name == symbol_name)
                .collect();

            // Analyze relationships between same-named symbols
            for main_symbol in &main_symbols_with_name {
                for (cross_fqn, cross_symbol) in &cross_symbols_with_name {
                    // Use the existing AST-based relationship detection
                    let relationship_type = super::tools::detect_symbol_relationship(main_symbol, cross_symbol);
                    
                    if relationship_type != "unrelated" {
                        let confidence = match relationship_type.as_str() {
                            "wrapper" => 0.8,
                            "implementation" => 0.9,
                            "inheritance" => 0.9,
                            "possible_wrapper" => 0.4,
                            _ => 0.2,
                        };

                        ast_relationships.push(ASTRelationship {
                            main_symbol_fqn: main_symbol.fqn.clone(),
                            cross_project_symbol_fqn: cross_fqn.to_string(),
                            relationship_type,
                            confidence,
                            detection_method: "ast_analysis".to_string(),
                            ast_patterns_found: vec!["tree_sitter_analysis".to_string()], // TODO: Add actual patterns
                        });
                    }
                }
            }

            ast_relationships
        } else {
            Vec::new()
        }
    }
}