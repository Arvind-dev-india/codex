//! Code analysis tools using Tree-sitter for parsing and generating code reference graphs.

pub mod parser_pool;
pub mod context_extractor;
pub mod repo_mapper;
pub mod tools;
pub mod supplementary_registry;
pub mod enhanced_graph_structures;
pub mod enhanced_bfs_traversal;
pub mod integration;
pub mod tool_handler;
pub mod graph_manager;
pub mod memory_optimized_storage;
pub mod repo_mapper_memory_optimized;
pub mod memory_optimization_example;

pub use tools::register_code_analysis_tools;
pub use integration::register_code_analysis_tools_with_openai;
pub use tool_handler::handle_code_analysis_tool_call;
pub use tools::{
    handle_analyze_code,
    handle_find_symbol_references,
    handle_find_symbol_definitions,
    handle_get_symbol_subgraph,
    handle_get_related_files_skeleton,
    handle_get_multiple_files_skeleton,
};
pub use parser_pool::{get_parser_pool, SupportedLanguage, QueryType};