//! Parser pool for managing Tree-sitter parsers for different languages.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use std::time::SystemTime;
use tree_sitter::{Language, Parser, Query, QueryCursor, Tree, StreamingIterator};
use tracing;

// Import the tree-sitter language parsers
extern crate tree_sitter_rust;
extern crate tree_sitter_c_sharp;
extern crate tree_sitter_python;
extern crate tree_sitter_javascript;
extern crate tree_sitter_typescript;
extern crate tree_sitter_java;
extern crate tree_sitter_cpp;
extern crate tree_sitter_go;

/// Supported languages for parsing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SupportedLanguage {
    Rust,
    JavaScript,
    TypeScript,
    Python,
    Go,
    Cpp,
    CSharp,
    Java,
    // Add more languages as needed
}

/// Get the query content for a language and query type (embedded at compile time)
fn get_query_content(language: SupportedLanguage, _query_type: QueryType) -> Result<&'static str, String> {
    let content = match language {
        SupportedLanguage::Rust => include_str!("queries/rust.scm"),
        SupportedLanguage::JavaScript => include_str!("queries/javascript.scm"),
        SupportedLanguage::TypeScript => include_str!("queries/typescript.scm"),
        SupportedLanguage::Python => include_str!("queries/python.scm"),
        SupportedLanguage::Go => include_str!("queries/go.scm"),
        SupportedLanguage::Cpp => include_str!("queries/cpp.scm"),
        SupportedLanguage::CSharp => include_str!("queries/csharp.scm"),
        SupportedLanguage::Java => include_str!("queries/java.scm"),
    };
    
    Ok(content)
}

impl SupportedLanguage {
    /// Get the file extensions associated with this language
    pub fn get_extensions(&self) -> Vec<&'static str> {
        match self {
            SupportedLanguage::Rust => vec!["rs"],
            SupportedLanguage::JavaScript => vec!["js", "jsx", "mjs"],
            SupportedLanguage::TypeScript => vec!["ts", "tsx"],
            SupportedLanguage::Python => vec!["py", "pyw"],
            SupportedLanguage::Go => vec!["go"],
            SupportedLanguage::Cpp => vec!["cpp", "cc", "cxx", "c++", "hpp", "hh", "hxx", "h++", "h"],
            SupportedLanguage::CSharp => vec!["cs"],
            SupportedLanguage::Java => vec!["java"],
        }
    }

    /// Detect language from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        let ext = ext.to_lowercase();
        match ext.as_str() {
            "rs" => Some(SupportedLanguage::Rust),
            "js" | "jsx" | "mjs" => Some(SupportedLanguage::JavaScript),
            "ts" | "tsx" => Some(SupportedLanguage::TypeScript),
            "py" | "pyw" => Some(SupportedLanguage::Python),
            "go" => Some(SupportedLanguage::Go),
            "cpp" | "cc" | "cxx" | "c++" | "hpp" | "hh" | "hxx" | "h++" | "h" | "c" => Some(SupportedLanguage::Cpp),
            "cs" => Some(SupportedLanguage::CSharp),
            "java" => Some(SupportedLanguage::Java),
            _ => None,
        }
    }

    /// Detect language from file path
    pub fn from_path(path: &Path) -> Option<Self> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(Self::from_extension)
    }
}

#[derive(Clone)]
/// Parsed file with its Tree-sitter tree and source code
pub struct ParsedFile {
    pub path: String,
    pub language: SupportedLanguage,
    pub tree: Tree,
    pub source: String,
}

impl ParsedFile {
    /// Execute a Tree-sitter query on this file
    pub fn execute_query(&self, query_str: &str) -> Result<Vec<QueryMatch>, String> {
        // Get the parser pool
        let parser_pool = get_parser_pool();
        
        // Load the language
        let lang = parser_pool.load_language(self.language)?;
        
        // Create a query from the query string
        let query = Query::new(&lang, query_str)
            .map_err(|e| format!("Failed to parse query: {}", e))?;
        
        // Execute the query
        self.execute_query_with_query(&query)
    }
    
    /// Execute a predefined Tree-sitter query on this file
    pub fn execute_predefined_query(&self, query_type: QueryType) -> Result<Vec<QueryMatch>, String> {
        // Get the parser pool
        let parser_pool = get_parser_pool();
        
        // Load the query
        let query = parser_pool.load_query(self.language, query_type)?;
        
        // Execute the query
        self.execute_query_with_query(&query)
    }
    
    /// Execute a Tree-sitter query on this file with a given Query object
    fn execute_query_with_query(&self, query: &Query) -> Result<Vec<QueryMatch>, String> {
        // Create a query cursor
        let mut cursor = QueryCursor::new();
        
        // Execute the query
        let mut matches = cursor.matches(
            query,
            self.tree.root_node(),
            self.source.as_bytes(),
        );
        
        // Convert the matches to our QueryMatch struct
        let mut result = Vec::new();
        // Use next() from StreamingIterator trait
        while let Some(match_) = matches.next() {
            let mut captures = Vec::new();
            for capture in match_.captures {
                let node = capture.node;
                let capture_name = query.capture_names()[capture.index as usize].to_string();
                let text = node.utf8_text(self.source.as_bytes())
                    .map_err(|e| format!("Failed to get text for node: {}", e))?
                    .to_string();
                
                let start_position = node.start_position();
                let end_position = node.end_position();
                
                captures.push(QueryCapture {
                    name: capture_name,
                    text,
                    start_byte: node.start_byte(),
                    end_byte: node.end_byte(),
                    start_point: (start_position.row, start_position.column),
                    end_point: (end_position.row, end_position.column),
                });
            }
            
            result.push(QueryMatch {
                pattern_index: match_.pattern_index,
                captures,
            });
        }
        
        Ok(result)
    }
}

/// A match from a Tree-sitter query
pub struct QueryMatch {
    pub pattern_index: usize,
    pub captures: Vec<QueryCapture>,
}

/// A capture from a Tree-sitter query match
pub struct QueryCapture {
    pub name: String,
    pub text: String,
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_point: (usize, usize),
    pub end_point: (usize, usize),
}

/// Query type for different kinds of queries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QueryType {
    Functions,
    Classes,
    Methods,
    Variables,
    References,
    Imports,
    All,
}

/// Parser pool for managing Tree-sitter parsers
pub struct ParserPool {
    parsers: Mutex<HashMap<SupportedLanguage, Parser>>,
    parsed_files: Mutex<HashMap<String, (ParsedFile, std::time::SystemTime)>>,
    languages: Mutex<HashMap<SupportedLanguage, Language>>,
    queries: Mutex<HashMap<(SupportedLanguage, QueryType), String>>, // Store query strings instead of Query objects
}

impl ParserPool {
    /// Create a new parser pool
    pub fn new() -> Self {
        Self {
            parsers: Mutex::new(HashMap::new()),
            parsed_files: Mutex::new(HashMap::new()),
            languages: Mutex::new(HashMap::new()),
            queries: Mutex::new(HashMap::new()),
        }
    }
    
    /// Load a language for a supported language
    fn load_language(&self, language: SupportedLanguage) -> Result<Language, String> {
        // Check if we already have the language loaded
        let mut languages = self.languages.lock().unwrap();
        if let Some(lang) = languages.get(&language) {
            // Language is not Copy, so we need to clone it
            return Ok(lang.clone());
        }
        
        // Load the language
        let lang_fn = match language {
            SupportedLanguage::Rust => tree_sitter_rust::LANGUAGE,
            SupportedLanguage::CSharp => tree_sitter_c_sharp::LANGUAGE,
            SupportedLanguage::Python => tree_sitter_python::LANGUAGE,
            SupportedLanguage::JavaScript => tree_sitter_javascript::LANGUAGE,
            SupportedLanguage::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
            SupportedLanguage::Java => tree_sitter_java::LANGUAGE,
            SupportedLanguage::Cpp => tree_sitter_cpp::LANGUAGE,
            SupportedLanguage::Go => tree_sitter_go::LANGUAGE,
        };
        
        // Convert LanguageFn to Language
        let lang: Language = lang_fn.into();
        
        // Store the language - clone it since we need to return it too
        languages.insert(language, lang.clone());
        
        // Return the language
        Ok(lang)
    }
    
    /// Load a query for a supported language and query type
    fn load_query(&self, language: SupportedLanguage, query_type: QueryType) -> Result<Query, String> {
        // Check if we already have the query cached
        let cached_query = {
            let queries = self.queries.lock().unwrap();
            queries.get(&(language, query_type)).cloned()
        };
        
        let query_content = if let Some(content) = cached_query {
            // Use cached query string
            content
        } else {
            // Get the embedded query content
            let content = get_query_content(language, query_type)?.to_string();
            
            // Cache the query string
            {
                let mut queries = self.queries.lock().unwrap();
                queries.insert((language, query_type), content.clone());
            }
            
            content
        };
        
        // Load the language and create the query
        let lang = self.load_language(language)?;
        
        // Create the query from the content
        let query = Query::new(&lang, &query_content)
            .map_err(|e| {
                tracing::warn!("Query parsing failed for language {:?}: {}", language, e);
                format!("Failed to parse query for {:?}: {}", language, e)
            })?;
        
        Ok(query)
    }

    /// Parse a file and return the parsed result
    /// Uses incremental parsing if the file has been parsed before
    pub fn parse_file(&self, path: &str, source: &str) -> Result<ParsedFile, String> {
        let path_obj = Path::new(path);
        let language = SupportedLanguage::from_path(path_obj)
            .ok_or_else(|| format!("Unsupported file extension: {}", path))?;
        
        // Removed verbose logging - language detection is working fine

        // Load the language first
        let lang = self.load_language(language)?;
        
        // Get or create a parser for this language
        let mut parsers = self.parsers.lock().unwrap();
        let parser = parsers.entry(language).or_insert_with(|| {
            let mut parser = Parser::new();
            parser.set_language(&lang).unwrap();
            parser
        });

        // Check if we've parsed this file before
        let mut parsed_files = self.parsed_files.lock().unwrap();
        let old_tree = parsed_files.get(path).map(|(parsed_file, _)| &parsed_file.tree);
        
        // Parse the source code, using the old tree for incremental parsing if available
        let tree = parser
            .parse(source, old_tree)
            .ok_or_else(|| format!("Failed to parse file: {}", path))?;

        // Create the parsed file
        let parsed_file = ParsedFile {
            path: path.to_string(),
            language,
            tree,
            source: source.to_string(),
        };
        
        // Store the parsed file with the current timestamp
        parsed_files.insert(path.to_string(), (parsed_file.clone(), SystemTime::now()));
        
        Ok(parsed_file)
    }
    
    /// Check if a file needs to be reparsed based on its modification time
    pub fn needs_reparse(&self, path: &str) -> Result<bool, String> {
        let file_path = Path::new(path);
        
        // Get the file's last modification time
        let metadata = fs::metadata(file_path)
            .map_err(|e| format!("Failed to get metadata for file {}: {}", path, e))?;
        let mtime = metadata.modified()
            .map_err(|e| format!("Failed to get modification time for file {}: {}", path, e))?;
        
        // Check if we've parsed this file before
        let parsed_files = self.parsed_files.lock().unwrap();
        if let Some((_, parse_time)) = parsed_files.get(path) {
            // If the file has been modified since we last parsed it, we need to reparse
            Ok(mtime > *parse_time)
        } else {
            // If we haven't parsed this file before, we need to parse it
            Ok(true)
        }
    }
    
    /// Parse a file from disk
    pub fn parse_file_from_disk(&self, path: &str) -> Result<ParsedFile, String> {
        // Read the file content with UTF-8 error handling
        let content = match fs::read(path) {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(content) => content,
                Err(_) => {
                    // Try with lossy conversion for files with invalid UTF-8
                    match fs::read(path) {
                        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
                        Err(e) => return Err(format!("Failed to read file {}: {}", path, e)),
                    }
                }
            },
            Err(e) => return Err(format!("Failed to read file {}: {}", path, e)),
        };
            
        // Parse the file
        self.parse_file(path, &content)
    }
    
    /// Parse a file from disk if it needs to be reparsed
    pub fn parse_file_if_needed(&self, path: &str) -> Result<ParsedFile, String> {
        if self.needs_reparse(path)? {
            self.parse_file_from_disk(path)
        } else {
            // Return the cached parsed file
            let parsed_files = self.parsed_files.lock().unwrap();
            let (parsed_file, _) = parsed_files.get(path)
                .ok_or_else(|| format!("File not found in cache: {}", path))?;
            Ok(parsed_file.clone())
        }
    }
}

/// Debug function to show JavaScript AST structure
fn _debug_javascript_ast(lang: &Language) {
    eprintln!("=== Debugging JavaScript AST ===");
    
    let mut parser = Parser::new();
    parser.set_language(lang).unwrap();
    
    let test_code = r#"
function testFunction() {
    return "hello";
}

const arrowFunc = () => {
    return "world";
};

class TestClass {
    method() {
        return "test";
    }
}
"#;
    
    if let Some(tree) = parser.parse(test_code, None) {
        _print_ast_node(tree.root_node(), test_code, 0);
    }
    eprintln!("=== End JavaScript AST Debug ===");
}

/// Debug function to show Python AST structure
fn _debug_python_ast(lang: &Language) {
    eprintln!("=== Debugging Python AST ===");
    
    let mut parser = Parser::new();
    parser.set_language(lang).unwrap();
    
    let test_code = r#"
class Calculator:
    def __init__(self):
        self.value = 0
    
    def add(self, x):
        return x + self.value

def simple_function():
    return 42
"#;
    
    if let Some(tree) = parser.parse(test_code, None) {
        _print_ast_node(tree.root_node(), test_code, 0);
    }
    eprintln!("=== End Python AST Debug ===");
}

/// Debug function to test Python query directly
fn _debug_python_query(lang: &Language) {
    eprintln!("=== Debugging Python Query ===");
    
    let mut parser = Parser::new();
    parser.set_language(lang).unwrap();
    
    // Test with the actual test file content
    let test_code = r#""""
A test module for demonstrating line number detection
"""

import math
from typing import Optional

class Calculator:
    """A simple calculator class."""
    
    def __init__(self, initial_value: float = 0.0):
        """Initialize the calculator with an optional initial value."""
        self.value = initial_value
        self.history = []

def simple_function():
    print("This is a simple function")
    return 42
"#;
    
    if let Some(tree) = parser.parse(test_code, None) {
        // Test the current Python query
        let query_content = r#"(class_definition
  name: (identifier) @name.definition.class) @definition.class

(function_definition
  name: (identifier) @name.definition.function) @definition.function

(call
  function: [
      (identifier) @name.reference.call
      (attribute
        attribute: (identifier) @name.reference.call)
  ]) @reference.call"#;
        
        match Query::new(lang, query_content) {
            Ok(query) => {
                let mut cursor = QueryCursor::new();
                let mut matches = cursor.matches(&query, tree.root_node(), test_code.as_bytes());
                
                eprintln!("Query executed successfully, checking matches...");
                let mut match_count = 0;
                while let Some(match_) = matches.next() {
                    match_count += 1;
                    eprintln!("Match {}: pattern {}", match_count, match_.pattern_index);
                    for capture in match_.captures {
                        let capture_name = query.capture_names()[capture.index as usize];
                        let text = capture.node.utf8_text(test_code.as_bytes()).unwrap_or("<error>");
                        eprintln!("  Capture: {} = '{}'", capture_name, text);
                    }
                }
                eprintln!("Total matches found: {}", match_count);
            }
            Err(e) => {
                eprintln!("Query failed: {}", e);
            }
        }
    }
    eprintln!("=== End Python Query Debug ===");
}

fn _print_ast_node(node: tree_sitter::Node, source: &str, depth: usize) {
    let indent = "  ".repeat(depth);
    let node_text = node.utf8_text(source.as_bytes()).unwrap_or("<error>");
    let node_text_preview = if node_text.len() > 50 {
        format!("{}...", &node_text[..50].replace('\n', "\\n"))
    } else {
        node_text.replace('\n', "\\n")
    };
    
    eprintln!("{}({}) \"{}\"", indent, node.kind(), node_text_preview);
    
    // Only print first few levels to avoid too much output
    if depth < 4 {
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                _print_ast_node(child, source, depth + 1);
            }
        }
    }
}

/// Singleton instance of the parser pool
pub fn get_parser_pool() -> &'static ParserPool {
    static PARSER_POOL: once_cell::sync::Lazy<ParserPool> = once_cell::sync::Lazy::new(ParserPool::new);
    &PARSER_POOL
}