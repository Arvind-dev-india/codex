//! Parser pool for managing Tree-sitter parsers for different languages.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use tree_sitter::{Language, Parser, Query, QueryCursor, Tree, StreamingIterator};
use once_cell::sync::Lazy;

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

/// Get the query file for a language and query type
fn get_query_file(language: SupportedLanguage, _query_type: QueryType) -> Result<String, String> {
    let lang_file = match language {
        SupportedLanguage::Rust => "rust.scm",
        SupportedLanguage::JavaScript => "javascript.scm",
        SupportedLanguage::TypeScript => "typescript.scm",
        SupportedLanguage::Python => "python.scm",
        SupportedLanguage::Go => "go.scm",
        SupportedLanguage::Cpp => "cpp.scm",
        SupportedLanguage::CSharp => "csharp.scm",
        SupportedLanguage::Java => "java.scm",
    };
    
    Ok(lang_file.to_string())
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
    queries: Mutex<HashMap<(SupportedLanguage, QueryType), Query>>,
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
        // Check if we already have the query loaded
        let mut queries = self.queries.lock().unwrap();
        if let Some(_query) = queries.get(&(language, query_type)) {
            // We need to create a new Query since Query doesn't implement Clone
            // In a real implementation, we would load the query from a file or string
            // We'll load the query again from the file
            let lang = self.load_language(language)?;
            let query_file = get_query_file(language, query_type)?;
            let query_path = format!("src/code_analysis/queries/{}", query_file);
            let query_content = fs::read_to_string(&query_path)
                .map_err(|e| format!("Failed to read query file {}: {}", query_path, e))?;
            
            let query = Query::new(&lang, &query_content)
                .map_err(|e| format!("Failed to parse query: {}", e))?;
            
            return Ok(query);
        }
        
        // Load the language
        let lang = self.load_language(language)?;
        
        // Get the query file path
        let query_file = match language {
            SupportedLanguage::Rust => "rust.scm",
            SupportedLanguage::CSharp => "csharp.scm",
            SupportedLanguage::Python => "python.scm",
            SupportedLanguage::JavaScript => "javascript.scm",
            SupportedLanguage::TypeScript => "typescript.scm",
            SupportedLanguage::Java => "java.scm",
            SupportedLanguage::Cpp => "cpp.scm",
            SupportedLanguage::Go => "go.scm",
        };
        
        // Load the query file
        let query_path = format!("src/code_analysis/queries/{}", query_file);
        let query_content = fs::read_to_string(&query_path)
            .map_err(|e| format!("Failed to read query file {}: {}", query_path, e))?;
        
        // Create the query
        let query = Query::new(&lang, &query_content)
            .map_err(|e| format!("Failed to parse query: {}", e))?;
        
        // Store the query and create a new one to return
        queries.insert((language, query_type), query);
        
        // Create a new query instance to return since Query doesn't implement Clone
        let return_query = Query::new(&lang, &query_content)
            .map_err(|e| format!("Failed to parse query: {}", e))?;
        
        Ok(return_query)
    }

    /// Parse a file and return the parsed result
    /// Uses incremental parsing if the file has been parsed before
    pub fn parse_file(&self, path: &str, source: &str) -> Result<ParsedFile, String> {
        let path_obj = Path::new(path);
        let language = SupportedLanguage::from_path(path_obj)
            .ok_or_else(|| format!("Unsupported file extension: {}", path))?;

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
        // Read the file content
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file {}: {}", path, e))?;
            
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

/// Singleton instance of the parser pool
pub fn get_parser_pool() -> &'static ParserPool {
    static PARSER_POOL: once_cell::sync::Lazy<ParserPool> = once_cell::sync::Lazy::new(ParserPool::new);
    &PARSER_POOL
}