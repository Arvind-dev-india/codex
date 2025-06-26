//! Code analysis tools using Tree-sitter for parsing and generating code reference graphs.

mod parser_pool;
pub mod context_extractor;
pub mod repo_mapper;
pub mod tools;
pub mod integration;
pub mod tool_handler;
pub mod graph_manager;

pub use tools::register_code_analysis_tools;
pub use integration::register_code_analysis_tools_with_openai;
pub use tool_handler::handle_code_analysis_tool_call;
pub use tools::{
    handle_analyze_code,
    handle_find_symbol_references,
    handle_find_symbol_definitions,
    handle_get_symbol_subgraph,
};