//! Parser pool for managing Tree-sitter parsers for different languages.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use tree_sitter::{Language, Parser, Query, QueryCursor, Tree};
use once_cell::sync::Lazy;

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

impl SupportedLanguage {
    /// Get the file extensions associated with this language
    pub fn get_extensions(&self) -> Vec<&'static str> {
        match self {
            SupportedLanguage::Rust => vec!["rs"],
            SupportedLanguage::JavaScript => vec!["js", "jsx", "mjs"],
            SupportedLanguage::TypeScript => vec!["ts", "tsx"],
            SupportedLanguage::Python => vec!["py"],
            SupportedLanguage::Go => vec!["go"],
            SupportedLanguage::Cpp => vec!["cpp", "cc", "cxx", "h", "hpp"],
            SupportedLanguage::CSharp => vec!["cs"],
            SupportedLanguage::Java => vec!["java"],
        }
    }

    /// Detect language from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        let ext = ext.to_lowercase();
        if ext == "rs" {
            Some(SupportedLanguage::Rust)
        } else if ext == "js" || ext == "jsx" || ext == "mjs" {
            Some(SupportedLanguage::JavaScript)
        } else if ext == "ts" || ext == "tsx" {
            Some(SupportedLanguage::TypeScript)
        } else if ext == "py" {
            Some(SupportedLanguage::Python)
        } else if ext == "go" {
            Some(SupportedLanguage::Go)
        } else if ext == "cpp" || ext == "cc" || ext == "cxx" || ext == "h" || ext == "hpp" {
            Some(SupportedLanguage::Cpp)
        } else if ext == "cs" {
            Some(SupportedLanguage::CSharp)
        } else if ext == "java" {
            Some(SupportedLanguage::Java)
        } else {
            None
        }
    }

    /// Detect language from file path
    pub fn from_path(path: &Path) -> Option<Self> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(Self::from_extension)
    }
}

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
        let query = Query::new(lang, query_str)
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
        let matches = cursor.matches(
            query,
            self.tree.root_node(),
            self.source.as_bytes(),
        );
        
        // Convert the matches to our QueryMatch struct
        let mut result = Vec::new();
        for match_ in matches {
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
            return Ok(*lang);
        }
        
        // Load the language
        let lang = match language {
            SupportedLanguage::Rust => tree_sitter_rust::language(),
            SupportedLanguage::JavaScript => tree_sitter_javascript::language(),
            SupportedLanguage::TypeScript => tree_sitter_typescript::language_typescript(),
            SupportedLanguage::Python => tree_sitter_python::language(),
            SupportedLanguage::CSharp => tree_sitter_c_sharp::language(),
            SupportedLanguage::Cpp => tree_sitter_cpp::language(),
            SupportedLanguage::Java => tree_sitter_java::language(),
            SupportedLanguage::Go => tree_sitter_go::language(),
            _ => return Err(format!("Language not supported: {:?}", language)),
        };
        
        // Store the language
        languages.insert(language, lang);
        
        Ok(lang)
    }
    
    /// Load a query for a supported language and query type
    fn load_query(&self, language: SupportedLanguage, query_type: QueryType) -> Result<Query, String> {
        // Check if we already have the query loaded
        let mut queries = self.queries.lock().unwrap();
        if let Some(query) = queries.get(&(language, query_type)) {
            return Ok(query.clone());
        }
        
        // Load the language
        let lang = self.load_language(language)?;
        
        // Get the query file path
        let query_file = match language {
            SupportedLanguage::Rust => "rust.scm",
            SupportedLanguage::JavaScript => "javascript.scm",
            SupportedLanguage::TypeScript => "typescript.scm",
            SupportedLanguage::Python => "python.scm",
            SupportedLanguage::CSharp => "csharp.scm",
            SupportedLanguage::Cpp => "cpp.scm",
            SupportedLanguage::Java => "java.scm",
            SupportedLanguage::Go => "go.scm",
            _ => return Err(format!("Query not available for language: {:?}", language)),
        };
        
        // Load the query file
        let query_path = format!("src/code_analysis/queries/{}", query_file);
        let query_content = fs::read_to_string(&query_path)
            .map_err(|e| format!("Failed to read query file {}: {}", query_path, e))?;
        
        // Create the query
        let query = Query::new(lang, &query_content)
            .map_err(|e| format!("Failed to parse query: {}", e))?;
        
        // Store the query
        queries.insert((language, query_type), query.clone());
        
        Ok(query)
    }

    /// Parse a file and return the parsed result
    /// Uses incremental parsing if the file has been parsed before
    pub fn parse_file(&self, path: &str, source: &str) -> Result<ParsedFile, String> {
        let path_obj = Path::new(path);
        let language = SupportedLanguage::from_path(path_obj)
            .ok_or_else(|| format!("Unsupported file extension: {}", path))?;

        // Get or create a parser for this language
        let mut parsers = self.parsers.lock().unwrap();
        let parser = parsers.entry(language).or_insert_with(|| {
            let mut parser = Parser::new();
            // In a real implementation, we would set the language here
            // parser.set_language(&get_language(language)).unwrap();
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