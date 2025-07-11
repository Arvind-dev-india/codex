//! Context extractor for extracting code context from parsed files.

use std::collections::{HashMap, HashSet};
use std::fs;
use tracing;

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
    Property,
    Import,
    Module,
    Package,
    // Add more symbol types as needed
}

impl SymbolType {
    /// Get string representation of symbol type
    pub fn as_str(&self) -> &'static str {
        match self {
            SymbolType::Function => "function",
            SymbolType::Method => "method",
            SymbolType::Class => "class",
            SymbolType::Struct => "struct",
            SymbolType::Enum => "enum",
            SymbolType::Interface => "interface",
            SymbolType::Variable => "variable",
            SymbolType::Constant => "constant",
            SymbolType::Property => "property",
            SymbolType::Import => "import",
            SymbolType::Module => "module",
            SymbolType::Package => "package",
        }
    }
}

/// Code symbol with its location and type
#[derive(Debug, Clone, PartialEq)]
pub struct CodeSymbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub parent: Option<String>,
    pub fqn: String,
}

/// Reference to a code symbol
#[derive(Debug, Clone)]
pub struct SymbolReference {
    pub symbol_name: String,
    pub symbol_fqn: String,
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
    // Map from symbol name to its FQNs (to handle name collisions)
    name_to_fqns: HashMap<String, Vec<String>>,
    references: Vec<SymbolReference>,
    // Map from file path to symbol FQNs defined in that file
    file_symbols: HashMap<String, HashSet<String>>,
}

impl ContextExtractor {
    /// Create a new context extractor
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            name_to_fqns: HashMap::new(),
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
        self.extract_symbols_from_parsed_file(&parsed_file)
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
        self.extract_symbols_from_parsed_file(&parsed_file)
    }
    
    /// Remove symbols defined in a specific file
    pub fn remove_symbols_for_file(&mut self, file_path: &str) {
        // Get the set of symbol FQNs defined in this file
        if let Some(symbol_fqns) = self.file_symbols.get(file_path) {
            // Remove each symbol from the symbols map and update name_to_fqns
            for fqn in symbol_fqns {
                if let Some(symbol) = self.symbols.remove(fqn) {
                    // Remove the FQN from the name_to_fqns map
                    if let Some(fqns) = self.name_to_fqns.get_mut(&symbol.name) {
                        fqns.retain(|f| f != fqn);
                        if fqns.is_empty() {
                            self.name_to_fqns.remove(&symbol.name);
                        }
                    }
                }
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
        let matches = match parsed_file.execute_predefined_query(QueryType::All) {
            Ok(matches) => matches,
            Err(e) => {
                return Err(format!("Failed to execute Rust query for file {}: {}", parsed_file.path, e));
            }
        };
        
        // Process the matches to extract symbols
        self.process_matches(&matches, parsed_file)?;
        
        Ok(())
    }
    
    /// Process Tree-sitter query matches to extract symbols and references
    fn process_matches(&mut self, matches: &[super::parser_pool::QueryMatch], parsed_file: &ParsedFile) -> Result<(), String> {
        for match_ in matches {
            // Group captures by their base type (definition, reference)
            let mut definition_captures: HashMap<&str, &super::parser_pool::QueryCapture> = HashMap::new();
            let mut name_definition_captures: HashMap<&str, &super::parser_pool::QueryCapture> = HashMap::new();
            let mut reference_captures: HashMap<&str, &super::parser_pool::QueryCapture> = HashMap::new();
            let mut name_reference_captures: HashMap<&str, &super::parser_pool::QueryCapture> = HashMap::new();
            
            // First pass: collect all captures by type
            for capture in &match_.captures {
                // eprintln!("Processing capture: {} = '{}'", capture.name, capture.text);
                
                if capture.name.starts_with("definition.") {
                    let symbol_type = &capture.name[11..]; // Remove "definition." prefix
                    definition_captures.insert(symbol_type, capture);
                } else if capture.name.starts_with("name.definition.") {
                    let symbol_type = &capture.name[16..]; // Remove "name.definition." prefix
                    name_definition_captures.insert(symbol_type, capture);
                } else if capture.name.starts_with("reference.") {
                    let ref_type = &capture.name[10..]; // Remove "reference." prefix
                    reference_captures.insert(ref_type, capture);
                } else if capture.name.starts_with("name.reference.") {
                    let ref_type = &capture.name[15..]; // Remove "name.reference." prefix
                    name_reference_captures.insert(ref_type, capture);
                }
            }
            
            // Second pass: create symbols when we have both definition and name captures
            for (symbol_type, def_capture) in definition_captures.iter() {
                if let Some(name_capture) = name_definition_captures.get(symbol_type) {
                    // Create symbol with both definition and name captures
                    self.create_symbol_from_captures(def_capture, name_capture, symbol_type, parsed_file)?;
                }
            }
            
            // Third pass: create references when we have both reference and name captures
            for (ref_type, ref_capture) in reference_captures.iter() {
                if let Some(name_capture) = name_reference_captures.get(ref_type) {
                    // Create reference with both reference and name captures
                    self.create_reference_from_captures(ref_capture, name_capture, ref_type, parsed_file)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Create a symbol from definition and name captures
    fn create_symbol_from_captures(
        &mut self, 
        def_capture: &super::parser_pool::QueryCapture, 
        name_capture: &super::parser_pool::QueryCapture,
        symbol_type_str: &str,
        parsed_file: &ParsedFile
    ) -> Result<(), String> {
        let symbol_type = match symbol_type_str {
            "function" => SymbolType::Function,
            "method" => SymbolType::Method,
            "class" => SymbolType::Class,
            "struct" => SymbolType::Struct,
            "enum" => SymbolType::Enum,
            "interface" => SymbolType::Interface,
            "variable" => SymbolType::Variable,
            "constant" => SymbolType::Constant,
            "property" => SymbolType::Property,
            "module" => SymbolType::Module,
            "package" => SymbolType::Package,
            "import" => SymbolType::Import,
            "type" => SymbolType::Class, // Map type definitions to class for now
            _ => {
                tracing::debug!("Unknown symbol type: {}", symbol_type_str);
                return Ok(());
            }
        };
        
        let name = name_capture.text.clone();
        let parent = None; // TODO: Extract parent information from context
        let mut fqn = self.generate_fqn(&name, &symbol_type, &parsed_file.path, &parent);
        
        // For methods, add line number to ensure uniqueness (handles interface/abstract/override methods)
        if matches!(symbol_type, SymbolType::Method) {
            fqn = format!("{}::{}", fqn, def_capture.start_point.0 + 1);
        }
        
        let symbol = CodeSymbol {
            name: name.clone(),
            symbol_type,
            file_path: parsed_file.path.clone(),
            // Convert from 0-based to 1-based line numbers for consistency
            start_line: def_capture.start_point.0 + 1,
            end_line: def_capture.end_point.0 + 1,
            start_col: def_capture.start_point.1,
            end_col: def_capture.end_point.1,
            parent,
            fqn: fqn.clone(),
        };
        
        // Add the symbol to the map using FQN as key
        self.symbols.insert(fqn.clone(), symbol);
        
        // Update the name_to_fqns map
        self.name_to_fqns.entry(name.clone())
            .or_insert_with(Vec::new)
            .push(fqn.clone());
            
        // Update file_symbols map
        self.file_symbols.entry(parsed_file.path.clone())
            .or_insert_with(HashSet::new)
            .insert(fqn);
            
        Ok(())
    }
    
    /// Create a reference from reference and name captures
    fn create_reference_from_captures(
        &mut self,
        ref_capture: &super::parser_pool::QueryCapture,
        name_capture: &super::parser_pool::QueryCapture,
        ref_type_str: &str,
        parsed_file: &ParsedFile
    ) -> Result<(), String> {
        let reference_type = match ref_type_str {
            "call" => ReferenceType::Call,
            "class" => ReferenceType::Usage,
            "interface" => ReferenceType::Usage,
            "implementation" => ReferenceType::Implementation,
            "type" => ReferenceType::Usage,
            "send" => ReferenceType::Call,
            "import" => ReferenceType::Import,
            "usage" => ReferenceType::Usage,
            _ => {
                tracing::debug!("Unknown reference type: {}", ref_type_str);
                return Ok(());
            }
        };
        
        let symbol_name = name_capture.text.clone();
        
        // Try to find the FQN of the symbol being referenced
        let symbol_fqn = if let Some(fqns) = self.name_to_fqns.get(&symbol_name) {
            if !fqns.is_empty() {
                fqns[0].clone()
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        
        // Create a reference to the symbol
        let reference = SymbolReference {
            symbol_name,
            symbol_fqn,
            reference_file: parsed_file.path.clone(),
            // Convert from 0-based to 1-based line numbers for consistency
            reference_line: ref_capture.start_point.0 + 1,
            reference_col: ref_capture.start_point.1,
            reference_type,
        };
        
        // Add the reference
        self.references.push(reference);
        
        Ok(())
    }

    /// Extract symbols from a JavaScript/TypeScript file
    fn extract_js_ts_symbols(&mut self, parsed_file: &ParsedFile) -> Result<(), String> {
        // Execute the query to find functions, classes, etc.
        let matches = match parsed_file.execute_predefined_query(QueryType::All) {
            Ok(matches) => matches,
            Err(e) => {
                return Err(format!("Failed to execute JS/TS query for file {}: {}", parsed_file.path, e));
            }
        };
        
        // Process the matches to extract symbols
        self.process_matches(&matches, parsed_file)?;
        
        Ok(())
    }

    /// Extract symbols from a Python file
    fn extract_python_symbols(&mut self, parsed_file: &ParsedFile) -> Result<(), String> {
        // Debug: Print the first 500 characters of the file content
        // let content_preview = if parsed_file.source.len() > 500 {
        //     format!("{}...", &parsed_file.source[..500])
        // } else {
        //     parsed_file.source.clone()
        // };
        // eprintln!("Python file content preview: {}", content_preview);
        
        // Execute the query to find functions, classes, etc.
        let matches = match parsed_file.execute_predefined_query(QueryType::All) {
            Ok(matches) => matches,
            Err(e) => {
                // eprintln!("Python query execution failed: {}", e);
                return Err(format!("Failed to execute Python query for file {}: {}", parsed_file.path, e));
            }
        };
        
        // eprintln!("Python query found {} matches", matches.len());
        
        // Process the matches to extract symbols
        self.process_matches(&matches, parsed_file)?;
        
        Ok(())
    }
    
    /// Extract symbols from a C++ file
    fn extract_cpp_symbols(&mut self, parsed_file: &ParsedFile) -> Result<(), String> {
        // Execute the query to find functions, classes, etc.
        let matches = match parsed_file.execute_predefined_query(QueryType::All) {
            Ok(matches) => matches,
            Err(e) => {
                return Err(format!("Failed to execute C++ query for file {}: {}", parsed_file.path, e));
            }
        };
        
        // Process the matches to extract symbols
        self.process_matches(&matches, parsed_file)?;
        
        Ok(())
    }
    
    /// Extract symbols from a Java file
    fn extract_java_symbols(&mut self, parsed_file: &ParsedFile) -> Result<(), String> {
        // Execute the query to find methods, classes, etc.
        let matches = match parsed_file.execute_predefined_query(QueryType::All) {
            Ok(matches) => matches,
            Err(e) => {
                // eprintln!("Java query execution failed: {}", e);
                return Err(format!("Failed to execute Java query for file {}: {}", parsed_file.path, e));
            }
        };
        
        // eprintln!("Java query found {} matches", matches.len());
        
        // Process the matches to extract symbols
        self.process_matches(&matches, parsed_file)?;
        
        Ok(())
    }
    
    /// Extract symbols from a C# file
    fn extract_csharp_symbols(&mut self, parsed_file: &ParsedFile) -> Result<(), String> {
        // Execute the query to find methods, classes, etc.
        let matches = match parsed_file.execute_predefined_query(QueryType::All) {
            Ok(matches) => matches,
            Err(e) => {
                return Err(format!("Failed to execute C# query for file {}: {}", parsed_file.path, e));
            }
        };
        
        // Process the matches to extract symbols
        self.process_matches(&matches, parsed_file)?;
        
        Ok(())
    }

    /// Extract symbols from a Go file
    fn extract_go_symbols(&mut self, parsed_file: &ParsedFile) -> Result<(), String> {
        // Execute the query to find functions, methods, etc.
        let matches = match parsed_file.execute_predefined_query(QueryType::All) {
            Ok(matches) => matches,
            Err(e) => {
                return Err(format!("Failed to execute Go query for file {}: {}", parsed_file.path, e));
            }
        };
        
        // Process the matches to extract symbols
        self.process_matches(&matches, parsed_file)?;
        
        Ok(())
    }

    /// Get all symbols
    pub fn get_symbols(&self) -> &HashMap<String, CodeSymbol> {
        &self.symbols
    }
    
    /// Get symbols for a specific file - O(1) lookup using cached file_symbols index
    pub fn get_symbols_for_file(&self, file_path: &str) -> Vec<&CodeSymbol> {
        if let Some(symbol_fqns) = self.file_symbols.get(file_path) {
            symbol_fqns.iter()
                .filter_map(|fqn| self.symbols.get(fqn))
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get mapping from symbol names to their FQNs
    pub fn get_name_to_fqns(&self) -> &HashMap<String, Vec<String>> {
        &self.name_to_fqns
    }
    
    /// Add a symbol to the context extractor (for parallel processing)
    pub fn add_symbol(&mut self, fqn: String, symbol: CodeSymbol) {
        // Update the name_to_fqns map
        self.name_to_fqns.entry(symbol.name.clone())
            .or_insert_with(Vec::new)
            .push(fqn.clone());
            
        // Update file_symbols map
        self.file_symbols.entry(symbol.file_path.clone())
            .or_insert_with(HashSet::new)
            .insert(fqn.clone());
            
        // Add the symbol
        self.symbols.insert(fqn, symbol);
    }
    
    /// Add a reference to the context extractor (for parallel processing)
    pub fn add_reference(&mut self, reference: SymbolReference) {
        self.references.push(reference);
    }

    /// Get all references
    pub fn get_references(&self) -> &[SymbolReference] {
        &self.references
    }

    /// Find symbol by name (returns the first match if multiple exist)
    pub fn find_symbol(&self, name: &str) -> Option<&CodeSymbol> {
        if let Some(fqns) = self.name_to_fqns.get(name) {
            if !fqns.is_empty() {
                return self.symbols.get(&fqns[0]);
            }
        }
        None
    }

    /// Find references to a symbol by name
    pub fn find_references(&self, symbol_name: &str) -> Vec<&SymbolReference> {
        self.references
            .iter()
            .filter(|r| r.symbol_name == symbol_name)
            .collect()
    }
    
    /// Find references to a symbol by FQN
    pub fn find_references_by_fqn(&self, fqn: &str) -> Vec<&SymbolReference> {
        self.references
            .iter()
            .filter(|r| r.symbol_fqn == fqn)
            .collect()
    }
    
    /// Generate a fully qualified name for a symbol
    fn generate_fqn(&self, name: &str, symbol_type: &SymbolType, file_path: &str, parent: &Option<String>) -> String {
        let mut fqn_parts = Vec::new();
        
        // Add file path (relative to project root if possible)
        fqn_parts.push(file_path.to_string());
        
        // Add parent if available
        if let Some(parent_name) = parent {
            fqn_parts.push(parent_name.clone());
        }
        
        // Add symbol type
        let type_str = match symbol_type {
            SymbolType::Function => "function",
            SymbolType::Method => "method",
            SymbolType::Class => "class",
            SymbolType::Struct => "struct",
            SymbolType::Enum => "enum",
            SymbolType::Interface => "interface",
            SymbolType::Variable => "variable",
            SymbolType::Constant => "constant",
            SymbolType::Property => "property",
            SymbolType::Import => "import",
            SymbolType::Module => "module",
            SymbolType::Package => "package",
        };
        fqn_parts.push(type_str.to_string());
        
        // Add symbol name
        fqn_parts.push(name.to_string());
        
        // For methods with the same name, add line number to make them unique
        // This handles cases like interface methods, abstract methods, and overrides
        if matches!(symbol_type, SymbolType::Method) {
            // We don't have access to line numbers here, so we'll need to modify the caller
            // For now, just use the basic FQN and handle uniqueness in the caller
        }
        
        // Join with double colons to form FQN
        fqn_parts.join("::")
    }
    
    /// Find a symbol by its fully qualified name
    pub fn find_symbol_by_fqn(&self, fqn: &str) -> Option<&CodeSymbol> {
        self.symbols.get(fqn)
    }
    
    /// Find all symbols with a given name
    pub fn find_symbols_by_name(&self, name: &str) -> Vec<&CodeSymbol> {
        if let Some(fqns) = self.name_to_fqns.get(name) {
            fqns.iter()
                .filter_map(|fqn| self.symbols.get(fqn))
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Find the symbol that contains a specific line in a file
    pub fn find_containing_symbol(&self, file_path: &str, line: usize) -> Option<&CodeSymbol> {
        // Find all symbols in the file
        if let Some(symbol_fqns) = self.file_symbols.get(file_path) {
            // Find the symbol that contains this line (line should be within start_line and end_line)
            for fqn in symbol_fqns {
                if let Some(symbol) = self.symbols.get(fqn) {
                    // Check if the line is within the symbol's range
                    // Note: Both line and symbol line numbers are now 1-based
                    if line >= symbol.start_line && line <= symbol.end_line {
                        return Some(symbol);
                    }
                }
            }
        }
        None
    }
    
    /// Find the most specific symbol that contains a specific line in a file
    /// (e.g., prefer method over class if both contain the line)
    pub fn find_most_specific_containing_symbol(&self, file_path: &str, line: usize) -> Option<&CodeSymbol> {
        // Find all symbols in the file that contain this line
        let mut containing_symbols = Vec::new();
        
        if let Some(symbol_fqns) = self.file_symbols.get(file_path) {
            for fqn in symbol_fqns {
                if let Some(symbol) = self.symbols.get(fqn) {
                    // Check if the line is within the symbol's range
                    if line >= symbol.start_line && line <= symbol.end_line {
                        containing_symbols.push(symbol);
                    }
                }
            }
        }
        
        if containing_symbols.is_empty() {
            return None;
        }
        
        // Sort by specificity: smaller range (end_line - start_line) is more specific
        containing_symbols.sort_by(|a, b| {
            let a_range = a.end_line - a.start_line;
            let b_range = b.end_line - b.start_line;
            a_range.cmp(&b_range)
        });
        
        // Return the most specific (smallest range) symbol
        Some(containing_symbols[0])
    }
}

/// Create a new context extractor
pub fn create_context_extractor() -> ContextExtractor {
    ContextExtractor::new()
}