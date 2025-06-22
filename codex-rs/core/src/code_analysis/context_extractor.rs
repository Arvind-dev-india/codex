//! Context extractor for extracting code context from parsed files.

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::fs;

use super::parser_pool::{get_parser_pool, ParsedFile, SupportedLanguage, QueryType};

/// Code symbol type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolType {
    Function,
    Method,
    Class,
    Struct,
    Enum,
    Interface,
    Variable,
    Constant,
    Import,
    Module,
    Package,
    // Add more symbol types as needed
}

/// Code symbol with its location and type
#[derive(Debug, Clone)]
pub struct CodeSymbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub parent: Option<String>,
}

/// Reference to a code symbol
#[derive(Debug, Clone)]
pub struct SymbolReference {
    pub symbol_name: String,
    pub reference_file: String,
    pub reference_line: usize,
    pub reference_col: usize,
    pub reference_type: ReferenceType,
}

/// Type of reference to a symbol
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferenceType {
    Call,
    Declaration,
    Implementation,
    Import,
    Inheritance,
    Usage,
    // Add more reference types as needed
}

/// Context extractor for extracting code context
pub struct ContextExtractor {
    symbols: HashMap<String, CodeSymbol>,
    references: Vec<SymbolReference>,
    // Map from file path to symbol names defined in that file
    file_symbols: HashMap<String, HashSet<String>>,
}

impl ContextExtractor {
    /// Create a new context extractor
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            references: Vec::new(),
            file_symbols: HashMap::new(),
        }
    }

    /// Extract symbols from a file
    pub fn extract_symbols_from_file(&mut self, file_path: &str) -> Result<(), String> {
        // Read the file content
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {}: {}", file_path, e))?;

        // Parse the file
        let parsed_file = get_parser_pool().parse_file(file_path, &content)?;
        
        // Extract symbols based on the language
        let result = self.extract_symbols_from_parsed_file(&parsed_file);
        
        // If extraction was successful, update the file_symbols map
        if result.is_ok() {
            // Find all symbols defined in this file
            let mut symbols_in_file = HashSet::new();
            for (name, symbol) in &self.symbols {
                if symbol.file_path == file_path {
                    symbols_in_file.insert(name.clone());
                }
            }
            
            // Update the file_symbols map
            self.file_symbols.insert(file_path.to_string(), symbols_in_file);
        }
        
        result
    }
    
    /// Extract symbols from a parsed file
    fn extract_symbols_from_parsed_file(&mut self, parsed_file: &ParsedFile) -> Result<(), String> {
        // Extract symbols based on the language
        match parsed_file.language {
            SupportedLanguage::Rust => self.extract_rust_symbols(parsed_file),
            SupportedLanguage::JavaScript | SupportedLanguage::TypeScript => {
                self.extract_js_ts_symbols(parsed_file)
            }
            SupportedLanguage::Python => self.extract_python_symbols(parsed_file),
            SupportedLanguage::CSharp => self.extract_csharp_symbols(parsed_file),
            SupportedLanguage::Cpp => self.extract_cpp_symbols(parsed_file),
            SupportedLanguage::Java => self.extract_java_symbols(parsed_file),
            SupportedLanguage::Go => self.extract_go_symbols(parsed_file),
        }
    }
    
    /// Extract symbols from a file using incremental parsing if possible
    pub fn extract_symbols_from_file_incremental(&mut self, file_path: &str) -> Result<(), String> {
        // Parse the file if needed
        let parsed_file = get_parser_pool().parse_file_if_needed(file_path)?;
        
        // Extract symbols based on the language
        let result = self.extract_symbols_from_parsed_file(&parsed_file);
        
        // If extraction was successful, update the file_symbols map
        if result.is_ok() {
            // Find all symbols defined in this file
            let mut symbols_in_file = HashSet::new();
            for (name, symbol) in &self.symbols {
                if symbol.file_path == file_path {
                    symbols_in_file.insert(name.clone());
                }
            }
            
            // Update the file_symbols map
            self.file_symbols.insert(file_path.to_string(), symbols_in_file);
        }
        
        result
    }
    
    /// Remove symbols defined in a specific file
    pub fn remove_symbols_for_file(&mut self, file_path: &str) {
        // Get the set of symbols defined in this file
        if let Some(symbol_names) = self.file_symbols.get(file_path) {
            // Remove each symbol from the symbols map
            for name in symbol_names {
                self.symbols.remove(name);
            }
            
            // Remove references to or from symbols in this file
            self.references.retain(|r| r.reference_file != file_path);
        }
        
        // Clear the file's entry in the file_symbols map
        self.file_symbols.remove(file_path);
    }

    /// Extract symbols from a Rust file
    fn extract_rust_symbols(&mut self, parsed_file: &ParsedFile) -> Result<(), String> {
        // Execute the query to find functions, structs, enums, etc.
        let matches = parsed_file.execute_predefined_query(QueryType::All)?;
        
        // Process the matches
        for match_ in matches {
            for capture in &match_.captures {
                match capture.name.as_str() {
                    "function.definition" => {
                        // Find the function name
                        let name = match_.captures.iter()
                            .find(|c| c.name == "function.name")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a symbol for the function
                        let symbol = CodeSymbol {
                            name: name.clone(),
                            symbol_type: SymbolType::Function,
                            file_path: parsed_file.path.clone(),
                            start_line: capture.start_point.0,
                            end_line: capture.end_point.0,
                            start_col: capture.start_point.1,
                            end_col: capture.end_point.1,
                            parent: None,
                        };
                        
                        // Add the symbol to the map
                        self.symbols.insert(name, symbol);
                    }
                    "struct.definition" => {
                        // Find the struct name
                        let name = match_.captures.iter()
                            .find(|c| c.name == "struct.name")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a symbol for the struct
                        let symbol = CodeSymbol {
                            name: name.clone(),
                            symbol_type: SymbolType::Struct,
                            file_path: parsed_file.path.clone(),
                            start_line: capture.start_point.0,
                            end_line: capture.end_point.0,
                            start_col: capture.start_point.1,
                            end_col: capture.end_point.1,
                            parent: None,
                        };
                        
                        // Add the symbol to the map
                        self.symbols.insert(name, symbol);
                    }
                    "call.expression" => {
                        // Find the function being called
                        let function_name = match_.captures.iter()
                            .find(|c| c.name == "call.function" || c.name == "call.method")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a reference to the function
                        let reference = SymbolReference {
                            symbol_name: function_name,
                            reference_file: parsed_file.path.clone(),
                            reference_line: capture.start_point.0,
                            reference_col: capture.start_point.1,
                            reference_type: ReferenceType::Call,
                        };
                        
                        // Add the reference
                        self.references.push(reference);
                    }
                    _ => {}
                }
            }
        }
        
        Ok(())
    }

    /// Extract symbols from a JavaScript/TypeScript file
    fn extract_js_ts_symbols(&mut self, parsed_file: &ParsedFile) -> Result<(), String> {
        // Execute the query to find functions, classes, etc.
        let matches = parsed_file.execute_predefined_query(QueryType::All)?;
        
        // Process the matches
        for match_ in matches {
            for capture in &match_.captures {
                match capture.name.as_str() {
                    "function.definition" => {
                        // Find the function name
                        let name = match_.captures.iter()
                            .find(|c| c.name == "function.name")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a symbol for the function
                        let symbol = CodeSymbol {
                            name: name.clone(),
                            symbol_type: SymbolType::Function,
                            file_path: parsed_file.path.clone(),
                            start_line: capture.start_point.0,
                            end_line: capture.end_point.0,
                            start_col: capture.start_point.1,
                            end_col: capture.end_point.1,
                            parent: None,
                        };
                        
                        // Add the symbol to the map
                        self.symbols.insert(name, symbol);
                    }
                    "class.definition" => {
                        // Find the class name
                        let name = match_.captures.iter()
                            .find(|c| c.name == "class.name")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a symbol for the class
                        let symbol = CodeSymbol {
                            name: name.clone(),
                            symbol_type: SymbolType::Class,
                            file_path: parsed_file.path.clone(),
                            start_line: capture.start_point.0,
                            end_line: capture.end_point.0,
                            start_col: capture.start_point.1,
                            end_col: capture.end_point.1,
                            parent: None,
                        };
                        
                        // Add the symbol to the map
                        self.symbols.insert(name, symbol);
                    }
                    "call.expression" => {
                        // Find the function being called
                        let function_name = match_.captures.iter()
                            .find(|c| c.name == "call.function" || c.name == "call.method")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a reference to the function
                        let reference = SymbolReference {
                            symbol_name: function_name,
                            reference_file: parsed_file.path.clone(),
                            reference_line: capture.start_point.0,
                            reference_col: capture.start_point.1,
                            reference_type: ReferenceType::Call,
                        };
                        
                        // Add the reference
                        self.references.push(reference);
                    }
                    _ => {}
                }
            }
        }
        
        Ok(())
    }

    /// Extract symbols from a Python file
    fn extract_python_symbols(&mut self, parsed_file: &ParsedFile) -> Result<(), String> {
        // Execute the query to find functions, classes, etc.
        let matches = parsed_file.execute_predefined_query(QueryType::All)?;
        
        // Process the matches
        for match_ in matches {
            for capture in &match_.captures {
                match capture.name.as_str() {
                    "function.definition" => {
                        // Find the function name
                        let name = match_.captures.iter()
                            .find(|c| c.name == "function.name")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a symbol for the function
                        let symbol = CodeSymbol {
                            name: name.clone(),
                            symbol_type: SymbolType::Function,
                            file_path: parsed_file.path.clone(),
                            start_line: capture.start_point.0,
                            end_line: capture.end_point.0,
                            start_col: capture.start_point.1,
                            end_col: capture.end_point.1,
                            parent: None,
                        };
                        
                        // Add the symbol to the map
                        self.symbols.insert(name, symbol);
                    }
                    "class.definition" => {
                        // Find the class name
                        let name = match_.captures.iter()
                            .find(|c| c.name == "class.name")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a symbol for the class
                        let symbol = CodeSymbol {
                            name: name.clone(),
                            symbol_type: SymbolType::Class,
                            file_path: parsed_file.path.clone(),
                            start_line: capture.start_point.0,
                            end_line: capture.end_point.0,
                            start_col: capture.start_point.1,
                            end_col: capture.end_point.1,
                            parent: None,
                        };
                        
                        // Add the symbol to the map
                        self.symbols.insert(name, symbol);
                    }
                    "call.expression" => {
                        // Find the function being called
                        let function_name = match_.captures.iter()
                            .find(|c| c.name == "call.function" || c.name == "call.method")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a reference to the function
                        let reference = SymbolReference {
                            symbol_name: function_name,
                            reference_file: parsed_file.path.clone(),
                            reference_line: capture.start_point.0,
                            reference_col: capture.start_point.1,
                            reference_type: ReferenceType::Call,
                        };
                        
                        // Add the reference
                        self.references.push(reference);
                    }
                    _ => {}
                }
            }
        }
        
        Ok(())
    }
    
    /// Extract symbols from a C++ file
    fn extract_cpp_symbols(&mut self, parsed_file: &ParsedFile) -> Result<(), String> {
        // Execute the query to find functions, classes, etc.
        let matches = parsed_file.execute_predefined_query(QueryType::All)?;
        
        // Process the matches
        for match_ in matches {
            for capture in &match_.captures {
                match capture.name.as_str() {
                    "function.definition" => {
                        // Find the function name
                        let name = match_.captures.iter()
                            .find(|c| c.name == "function.name")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a symbol for the function
                        let symbol = CodeSymbol {
                            name: name.clone(),
                            symbol_type: SymbolType::Function,
                            file_path: parsed_file.path.clone(),
                            start_line: capture.start_point.0,
                            end_line: capture.end_point.0,
                            start_col: capture.start_point.1,
                            end_col: capture.end_point.1,
                            parent: None,
                        };
                        
                        // Add the symbol to the map
                        self.symbols.insert(name, symbol);
                    }
                    "class.definition" => {
                        // Find the class name
                        let name = match_.captures.iter()
                            .find(|c| c.name == "class.name")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a symbol for the class
                        let symbol = CodeSymbol {
                            name: name.clone(),
                            symbol_type: SymbolType::Class,
                            file_path: parsed_file.path.clone(),
                            start_line: capture.start_point.0,
                            end_line: capture.end_point.0,
                            start_col: capture.start_point.1,
                            end_col: capture.end_point.1,
                            parent: None,
                        };
                        
                        // Add the symbol to the map
                        self.symbols.insert(name, symbol);
                    }
                    "struct.definition" => {
                        // Find the struct name
                        let name = match_.captures.iter()
                            .find(|c| c.name == "struct.name")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a symbol for the struct
                        let symbol = CodeSymbol {
                            name: name.clone(),
                            symbol_type: SymbolType::Struct,
                            file_path: parsed_file.path.clone(),
                            start_line: capture.start_point.0,
                            end_line: capture.end_point.0,
                            start_col: capture.start_point.1,
                            end_col: capture.end_point.1,
                            parent: None,
                        };
                        
                        // Add the symbol to the map
                        self.symbols.insert(name, symbol);
                    }
                    "call.expression" => {
                        // Find the function being called
                        let function_name = match_.captures.iter()
                            .find(|c| c.name == "call.function" || c.name == "call.method")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a reference to the function
                        let reference = SymbolReference {
                            symbol_name: function_name,
                            reference_file: parsed_file.path.clone(),
                            reference_line: capture.start_point.0,
                            reference_col: capture.start_point.1,
                            reference_type: ReferenceType::Call,
                        };
                        
                        // Add the reference
                        self.references.push(reference);
                    }
                    _ => {}
                }
            }
        }
        
        Ok(())
    }
    
    /// Extract symbols from a Java file
    fn extract_java_symbols(&mut self, parsed_file: &ParsedFile) -> Result<(), String> {
        // Execute the query to find methods, classes, etc.
        let matches = parsed_file.execute_predefined_query(QueryType::All)?;
        
        // Process the matches
        for match_ in matches {
            for capture in &match_.captures {
                match capture.name.as_str() {
                    "method.definition" => {
                        // Find the method name
                        let name = match_.captures.iter()
                            .find(|c| c.name == "method.name")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a symbol for the method
                        let symbol = CodeSymbol {
                            name: name.clone(),
                            symbol_type: SymbolType::Method,
                            file_path: parsed_file.path.clone(),
                            start_line: capture.start_point.0,
                            end_line: capture.end_point.0,
                            start_col: capture.start_point.1,
                            end_col: capture.end_point.1,
                            parent: None,
                        };
                        
                        // Add the symbol to the map
                        self.symbols.insert(name, symbol);
                    }
                    "class.definition" => {
                        // Find the class name
                        let name = match_.captures.iter()
                            .find(|c| c.name == "class.name")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a symbol for the class
                        let symbol = CodeSymbol {
                            name: name.clone(),
                            symbol_type: SymbolType::Class,
                            file_path: parsed_file.path.clone(),
                            start_line: capture.start_point.0,
                            end_line: capture.end_point.0,
                            start_col: capture.start_point.1,
                            end_col: capture.end_point.1,
                            parent: None,
                        };
                        
                        // Add the symbol to the map
                        self.symbols.insert(name, symbol);
                    }
                    "interface.definition" => {
                        // Find the interface name
                        let name = match_.captures.iter()
                            .find(|c| c.name == "interface.name")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a symbol for the interface
                        let symbol = CodeSymbol {
                            name: name.clone(),
                            symbol_type: SymbolType::Interface,
                            file_path: parsed_file.path.clone(),
                            start_line: capture.start_point.0,
                            end_line: capture.end_point.0,
                            start_col: capture.start_point.1,
                            end_col: capture.end_point.1,
                            parent: None,
                        };
                        
                        // Add the symbol to the map
                        self.symbols.insert(name, symbol);
                    }
                    "call.expression" => {
                        // Find the method being called
                        let method_name = match_.captures.iter()
                            .find(|c| c.name == "call.method")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a reference to the method
                        let reference = SymbolReference {
                            symbol_name: method_name,
                            reference_file: parsed_file.path.clone(),
                            reference_line: capture.start_point.0,
                            reference_col: capture.start_point.1,
                            reference_type: ReferenceType::Call,
                        };
                        
                        // Add the reference
                        self.references.push(reference);
                    }
                    _ => {}
                }
            }
        }
        
        Ok(())
    }
    
    /// Extract symbols from a C# file
    fn extract_csharp_symbols(&mut self, parsed_file: &ParsedFile) -> Result<(), String> {
        // Execute the query to find methods, classes, etc.
        let matches = parsed_file.execute_predefined_query(QueryType::All)?;
        
        // Process the matches
        for match_ in matches {
            for capture in &match_.captures {
                match capture.name.as_str() {
                    "method.definition" => {
                        // Find the method name
                        let name = match_.captures.iter()
                            .find(|c| c.name == "method.name")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a symbol for the method
                        let symbol = CodeSymbol {
                            name: name.clone(),
                            symbol_type: SymbolType::Method,
                            file_path: parsed_file.path.clone(),
                            start_line: capture.start_point.0,
                            end_line: capture.end_point.0,
                            start_col: capture.start_point.1,
                            end_col: capture.end_point.1,
                            parent: None,
                        };
                        
                        // Add the symbol to the map
                        self.symbols.insert(name, symbol);
                    }
                    "class.definition" => {
                        // Find the class name
                        let name = match_.captures.iter()
                            .find(|c| c.name == "class.name")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a symbol for the class
                        let symbol = CodeSymbol {
                            name: name.clone(),
                            symbol_type: SymbolType::Class,
                            file_path: parsed_file.path.clone(),
                            start_line: capture.start_point.0,
                            end_line: capture.end_point.0,
                            start_col: capture.start_point.1,
                            end_col: capture.end_point.1,
                            parent: None,
                        };
                        
                        // Add the symbol to the map
                        self.symbols.insert(name, symbol);
                    }
                    "interface.definition" => {
                        // Find the interface name
                        let name = match_.captures.iter()
                            .find(|c| c.name == "interface.name")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a symbol for the interface
                        let symbol = CodeSymbol {
                            name: name.clone(),
                            symbol_type: SymbolType::Interface,
                            file_path: parsed_file.path.clone(),
                            start_line: capture.start_point.0,
                            end_line: capture.end_point.0,
                            start_col: capture.start_point.1,
                            end_col: capture.end_point.1,
                            parent: None,
                        };
                        
                        // Add the symbol to the map
                        self.symbols.insert(name, symbol);
                    }
                    "call.expression" => {
                        // Find the method being called
                        let method_name = match_.captures.iter()
                            .find(|c| c.name == "call.function" || c.name == "call.method")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a reference to the method
                        let reference = SymbolReference {
                            symbol_name: method_name,
                            reference_file: parsed_file.path.clone(),
                            reference_line: capture.start_point.0,
                            reference_col: capture.start_point.1,
                            reference_type: ReferenceType::Call,
                        };
                        
                        // Add the reference
                        self.references.push(reference);
                    }
                    _ => {}
                }
            }
        }
        
        Ok(())
    }

    /// Extract symbols from a Go file
    fn extract_go_symbols(&mut self, parsed_file: &ParsedFile) -> Result<(), String> {
        // Execute the query to find functions, methods, etc.
        let matches = parsed_file.execute_predefined_query(QueryType::All)?;
        
        // Process the matches
        for match_ in matches {
            for capture in &match_.captures {
                match capture.name.as_str() {
                    "function.definition" => {
                        // Find the function name
                        let name = match_.captures.iter()
                            .find(|c| c.name == "function.name")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a symbol for the function
                        let symbol = CodeSymbol {
                            name: name.clone(),
                            symbol_type: SymbolType::Function,
                            file_path: parsed_file.path.clone(),
                            start_line: capture.start_point.0,
                            end_line: capture.end_point.0,
                            start_col: capture.start_point.1,
                            end_col: capture.end_point.1,
                            parent: None,
                        };
                        
                        // Add the symbol to the map
                        self.symbols.insert(name, symbol);
                    }
                    "method.definition" => {
                        // Find the method name
                        let name = match_.captures.iter()
                            .find(|c| c.name == "method.name")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a symbol for the method
                        let symbol = CodeSymbol {
                            name: name.clone(),
                            symbol_type: SymbolType::Method,
                            file_path: parsed_file.path.clone(),
                            start_line: capture.start_point.0,
                            end_line: capture.end_point.0,
                            start_col: capture.start_point.1,
                            end_col: capture.end_point.1,
                            parent: None,
                        };
                        
                        // Add the symbol to the map
                        self.symbols.insert(name, symbol);
                    }
                    "struct.definition" => {
                        // Find the struct name
                        let name = match_.captures.iter()
                            .find(|c| c.name == "struct.name")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a symbol for the struct
                        let symbol = CodeSymbol {
                            name: name.clone(),
                            symbol_type: SymbolType::Struct,
                            file_path: parsed_file.path.clone(),
                            start_line: capture.start_point.0,
                            end_line: capture.end_point.0,
                            start_col: capture.start_point.1,
                            end_col: capture.end_point.1,
                            parent: None,
                        };
                        
                        // Add the symbol to the map
                        self.symbols.insert(name, symbol);
                    }
                    "interface.definition" => {
                        // Find the interface name
                        let name = match_.captures.iter()
                            .find(|c| c.name == "interface.name")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a symbol for the interface
                        let symbol = CodeSymbol {
                            name: name.clone(),
                            symbol_type: SymbolType::Interface,
                            file_path: parsed_file.path.clone(),
                            start_line: capture.start_point.0,
                            end_line: capture.end_point.0,
                            start_col: capture.start_point.1,
                            end_col: capture.end_point.1,
                            parent: None,
                        };
                        
                        // Add the symbol to the map
                        self.symbols.insert(name, symbol);
                    }
                    "call.expression" => {
                        // Find the function being called
                        let function_name = match_.captures.iter()
                            .find(|c| c.name == "call.function" || c.name == "call.method")
                            .map(|c| c.text.clone())
                            .unwrap_or_default();
                        
                        // Create a reference to the function
                        let reference = SymbolReference {
                            symbol_name: function_name,
                            reference_file: parsed_file.path.clone(),
                            reference_line: capture.start_point.0,
                            reference_col: capture.start_point.1,
                            reference_type: ReferenceType::Call,
                        };
                        
                        // Add the reference
                        self.references.push(reference);
                    }
                    _ => {}
                }
            }
        }
        
        Ok(())
    }

    /// Get all symbols
    pub fn get_symbols(&self) -> &HashMap<String, CodeSymbol> {
        &self.symbols
    }

    /// Get all references
    pub fn get_references(&self) -> &[SymbolReference] {
        &self.references
    }

    /// Find symbol by name
    pub fn find_symbol(&self, name: &str) -> Option<&CodeSymbol> {
        self.symbols.get(name)
    }

    /// Find references to a symbol
    pub fn find_references(&self, symbol_name: &str) -> Vec<&SymbolReference> {
        self.references
            .iter()
            .filter(|r| r.symbol_name == symbol_name)
            .collect()
    }
}

/// Create a new context extractor
pub fn create_context_extractor() -> ContextExtractor {
    ContextExtractor::new()
}