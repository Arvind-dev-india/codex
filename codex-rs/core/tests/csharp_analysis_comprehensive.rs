use codex_core::code_analysis::context_extractor::{ContextExtractor, SymbolType};
use codex_core::code_analysis::{get_parser_pool, SupportedLanguage, QueryType};
use std::path::Path;

#[test]
fn test_csharp_basic_class_parsing() {
    let test_file = "test_files/csharp_test_suite/BasicClass.cs";
    
    // Ensure the file exists
    assert!(Path::new(test_file).exists(), "Test file {} does not exist", test_file);
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(test_file);
    
    assert!(result.is_ok(), "Failed to extract symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    
    // Print all found symbols for debugging
    println!("Found {} symbols:", symbols.len());
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
    }
    
    // Test class detection
    let basic_class = symbols.values()
        .find(|s| s.name == "BasicClass" && matches!(s.symbol_type, SymbolType::Class))
        .expect("BasicClass should be found");
    
    assert_eq!(basic_class.file_path, test_file);
    assert!(basic_class.start_line > 0, "Start line should be positive");
    assert!(basic_class.end_line > basic_class.start_line, "End line should be after start line");
    
    // Test method detection
    let add_method = symbols.values()
        .find(|s| s.name == "Add" && matches!(s.symbol_type, SymbolType::Method))
        .expect("Add method should be found");
    
    assert_eq!(add_method.file_path, test_file);
    assert!(add_method.start_line > 0);
    assert!(add_method.end_line > add_method.start_line);
    
    // Test constructor detection
    let constructor = symbols.values()
        .find(|s| s.name == "BasicClass" && matches!(s.symbol_type, SymbolType::Method))
        .expect("Constructor should be found as a method");
    
    // Test static method detection
    let static_method = symbols.values()
        .find(|s| s.name == "StaticMethod" && matches!(s.symbol_type, SymbolType::Method))
        .expect("StaticMethod should be found");
    
    // Verify we found expected number of methods (constructors + methods)
    let method_count = symbols.values()
        .filter(|s| matches!(s.symbol_type, SymbolType::Method))
        .count();
    
    // Expected: 2 constructors + Add + PrintInfo + IsValid + StaticMethod = 6 methods
    assert!(method_count >= 4, "Should find at least 4 methods, found {}", method_count);
}

#[test]
fn test_csharp_inheritance_parsing() {
    let test_file = "test_files/csharp_test_suite/InheritanceExample.cs";
    
    assert!(Path::new(test_file).exists(), "Test file {} does not exist", test_file);
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(test_file);
    
    assert!(result.is_ok(), "Failed to extract symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    
    // Print all found symbols for debugging
    println!("Found {} symbols in inheritance test:", symbols.len());
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
    }
    
    // Test interface detection
    let interface = symbols.values()
        .find(|s| s.name == "ICalculator" && matches!(s.symbol_type, SymbolType::Interface))
        .expect("ICalculator interface should be found");
    
    // Test abstract class detection
    let base_class = symbols.values()
        .find(|s| s.name == "BaseCalculator" && matches!(s.symbol_type, SymbolType::Class))
        .expect("BaseCalculator class should be found");
    
    // Test derived classes
    let add_calc = symbols.values()
        .find(|s| s.name == "AddCalculator" && matches!(s.symbol_type, SymbolType::Class))
        .expect("AddCalculator class should be found");
    
    let multiply_calc = symbols.values()
        .find(|s| s.name == "MultiplyCalculator" && matches!(s.symbol_type, SymbolType::Class))
        .expect("MultiplyCalculator class should be found");
    
    // Test method overrides
    let calculate_methods: Vec<_> = symbols.values()
        .filter(|s| s.name == "Calculate" && matches!(s.symbol_type, SymbolType::Method))
        .collect();
    
    assert!(calculate_methods.len() >= 2, "Should find at least 2 Calculate methods (abstract + implementations)");
}

#[test]
fn test_csharp_inter_file_references() {
    // First, parse all files to build the complete symbol table
    let files = [
        "test_files/csharp_test_suite/BasicClass.cs",
        "test_files/csharp_test_suite/InheritanceExample.cs",
        "test_files/csharp_test_suite/InterFileReferences.cs",
    ];
    
    for file in &files {
        assert!(Path::new(file).exists(), "Test file {} does not exist", file);
    }
    
    let mut extractor = ContextExtractor::new();
    
    // Extract symbols from all files
    for file in &files {
        let result = extractor.extract_symbols_from_file(file);
        assert!(result.is_ok(), "Failed to extract symbols from {}: {:?}", file, result.err());
    }
    
    let symbols = extractor.get_symbols();
    let references = extractor.get_references();
    
    println!("Found {} symbols across all files:", symbols.len());
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) in {}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.file_path);
    }
    
    println!("Found {} references:", references.len());
    for reference in references {
        println!("  {} ({:?}) at {}:{}", 
                 reference.symbol_name, reference.reference_type, 
                 reference.reference_file, reference.reference_line);
    }
    
    // Test that we can find references to BasicClass
    let basic_class_refs: Vec<_> = references.iter()
        .filter(|r| r.symbol_name == "BasicClass")
        .collect();
    
    assert!(!basic_class_refs.is_empty(), "Should find references to BasicClass");
    
    // Test that we can find references to interface methods
    let calculate_refs: Vec<_> = references.iter()
        .filter(|r| r.symbol_name == "Calculate")
        .collect();
    
    // Should find calls to Calculate method
    assert!(!calculate_refs.is_empty(), "Should find references to Calculate method");
}

#[test]
fn test_csharp_generic_parsing() {
    let test_file = "test_files/csharp_test_suite/GenericAndAdvanced.cs";
    
    assert!(Path::new(test_file).exists(), "Test file {} does not exist", test_file);
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(test_file);
    
    assert!(result.is_ok(), "Failed to extract symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    
    println!("Found {} symbols in generic test:", symbols.len());
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
    }
    
    // Test generic class detection
    let generic_repo = symbols.values()
        .find(|s| s.name == "GenericRepository" && matches!(s.symbol_type, SymbolType::Class))
        .expect("GenericRepository class should be found");
    
    // Test data processor class
    let data_processor = symbols.values()
        .find(|s| s.name == "DataProcessor" && matches!(s.symbol_type, SymbolType::Class))
        .expect("DataProcessor class should be found");
    
    // Test generic methods
    let add_method = symbols.values()
        .find(|s| s.name == "Add" && matches!(s.symbol_type, SymbolType::Method))
        .expect("Add method should be found in generic class");
    
    let get_method = symbols.values()
        .find(|s| s.name == "Get" && matches!(s.symbol_type, SymbolType::Method))
        .expect("Get method should be found in generic class");
}

#[test]
fn test_csharp_line_numbers_accuracy() {
    let test_file = "test_files/csharp_test_suite/BasicClass.cs";
    
    assert!(Path::new(test_file).exists(), "Test file {} does not exist", test_file);
    
    // Read the file content to verify line numbers
    let content = std::fs::read_to_string(test_file).expect("Failed to read test file");
    let lines: Vec<&str> = content.lines().collect();
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(test_file);
    assert!(result.is_ok());
    
    let symbols = extractor.get_symbols();
    
    // Find the BasicClass class symbol
    let basic_class = symbols.values()
        .find(|s| s.name == "BasicClass" && matches!(s.symbol_type, SymbolType::Class))
        .expect("BasicClass should be found");
    
    // Verify the class starts where we expect (should be around line with "public class BasicClass")
    let class_line_content = lines.get(basic_class.start_line - 1)
        .expect("Class start line should be valid");
    
    println!("BasicClass found at lines {}-{}", basic_class.start_line, basic_class.end_line);
    println!("Line {} content: '{}'", basic_class.start_line, class_line_content);
    
    // The line should contain "class BasicClass" or similar
    assert!(class_line_content.contains("BasicClass"), 
            "Line {} should contain 'BasicClass', but contains: '{}'", 
            basic_class.start_line, class_line_content);
    
    // Find the Add method
    let add_method = symbols.values()
        .find(|s| s.name == "Add" && matches!(s.symbol_type, SymbolType::Method))
        .expect("Add method should be found");
    
    println!("Add method found at lines {}-{}", add_method.start_line, add_method.end_line);
    
    // Verify the method line contains the method signature
    let method_line_content = lines.get(add_method.start_line - 1)
        .expect("Method start line should be valid");
    
    println!("Line {} content: '{}'", add_method.start_line, method_line_content);
    
    // Should contain method signature
    assert!(method_line_content.contains("Add") || 
            lines.iter().skip(add_method.start_line - 1)
                .take(3)
                .any(|line| line.contains("Add")),
            "Method lines should contain 'Add' method signature");
}

#[test]
fn test_csharp_namespace_detection() {
    let test_file = "test_files/csharp_test_suite/BasicClass.cs";
    
    assert!(Path::new(test_file).exists(), "Test file {} does not exist", test_file);
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(test_file);
    assert!(result.is_ok());
    
    let symbols = extractor.get_symbols();
    
    // Look for namespace symbol
    let namespace = symbols.values()
        .find(|s| s.name == "TestNamespace" && matches!(s.symbol_type, SymbolType::Module))
        .expect("TestNamespace should be found");
    
    println!("Namespace found: {} at lines {}-{}", 
             namespace.name, namespace.start_line, namespace.end_line);
    
    assert_eq!(namespace.name, "TestNamespace");
}

#[test]
fn test_csharp_parser_pool_integration() {
    let test_file = "test_files/csharp_test_suite/BasicClass.cs";
    
    assert!(Path::new(test_file).exists(), "Test file {} does not exist", test_file);
    
    // Test that the parser pool can correctly identify C# files
    let path = Path::new(test_file);
    let language = SupportedLanguage::from_path(path);
    
    assert_eq!(language, Some(SupportedLanguage::CSharp), "Should detect C# language from .cs extension");
    
    // Test parsing through parser pool
    let parser_pool = get_parser_pool();
    let result = parser_pool.parse_file_from_disk(test_file);
    
    assert!(result.is_ok(), "Parser pool should successfully parse C# file: {:?}", result.err());
    
    let parsed_file = result.unwrap();
    assert_eq!(parsed_file.language, SupportedLanguage::CSharp);
    assert_eq!(parsed_file.path, test_file);
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
}