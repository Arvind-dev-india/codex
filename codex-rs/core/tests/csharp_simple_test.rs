use codex_core::code_analysis::context_extractor::{ContextExtractor, SymbolType};
use codex_core::code_analysis::{get_parser_pool, SupportedLanguage, QueryType};
use std::path::Path;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_csharp_simple_parsing() {
    // Create a temporary directory and file
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("SimpleClass.cs");
    
    let content = r#"
using System;

namespace TestNamespace
{
    public class SimpleClass
    {
        public int Add(int a, int b)
        {
            return a + b;
        }
        
        public void PrintMessage()
        {
            Console.WriteLine("Hello World");
        }
    }
}
"#;
    
    fs::write(&file_path, content).expect("Failed to write test file");
    
    // Test language detection
    let language = SupportedLanguage::from_path(&file_path);
    assert_eq!(language, Some(SupportedLanguage::CSharp), "Should detect C# language from .cs extension");
    
    // Test parser pool parsing
    let parser_pool = get_parser_pool();
    let result = parser_pool.parse_file_from_disk(file_path.to_str().unwrap());
    
    assert!(result.is_ok(), "Parser pool should successfully parse C# file: {:?}", result.err());
    
    let parsed_file = result.unwrap();
    assert_eq!(parsed_file.language, SupportedLanguage::CSharp);
    assert!(!parsed_file.source.is_empty(), "Parsed file should have source content");
    
    // Test query execution
    let query_result = parsed_file.execute_predefined_query(QueryType::All);
    assert!(query_result.is_ok(), "Should be able to execute C# queries: {:?}", query_result.err());
    
    let matches = query_result.unwrap();
    assert!(!matches.is_empty(), "Should find some matches in C# file");
    
    println!("Found {} query matches", matches.len());
    for (i, match_) in matches.iter().enumerate() {
        println!("Match {}: pattern {}", i, match_.pattern_index);
        for capture in &match_.captures {
            println!("  Capture '{}': '{}' at {}:{}-{}:{}", 
                     capture.name, capture.text, 
                     capture.start_point.0, capture.start_point.1,
                     capture.end_point.0, capture.end_point.1);
        }
    }
    
    // Test context extractor
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(file_path.to_str().unwrap());
    
    assert!(result.is_ok(), "Failed to extract symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    
    println!("Found {} symbols:", symbols.len());
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
    }
    
    // Test that we found the expected symbols
    let simple_class = symbols.values()
        .find(|s| s.name == "SimpleClass" && matches!(s.symbol_type, SymbolType::Class))
        .expect("SimpleClass should be found");
    
    assert!(simple_class.start_line > 0, "Start line should be positive");
    assert!(simple_class.end_line > simple_class.start_line, "End line should be after start line");
    
    let add_method = symbols.values()
        .find(|s| s.name == "Add" && matches!(s.symbol_type, SymbolType::Method))
        .expect("Add method should be found");
    
    assert!(add_method.start_line > 0);
    assert!(add_method.end_line > add_method.start_line);
    
    // Should find at least class + 2 methods
    assert!(symbols.len() >= 3, "Should find at least 3 symbols (class + methods), found {}", symbols.len());
}