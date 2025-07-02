use std::fs;
use codex_core::code_analysis::context_extractor::{ContextExtractor, SymbolType};

fn main() {
    println!("=== Testing Python Line Numbers ===");
    test_python_line_numbers();
    
    println!("\n=== Testing C# Line Numbers ===");
    test_csharp_line_numbers();
}

fn test_python_line_numbers() {
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file("test_line_numbers.py");
    
    if let Err(e) = result {
        println!("Error extracting Python symbols: {}", e);
        return;
    }
    
    let symbols = extractor.get_symbols();
    
    println!("Python file content:");
    if let Ok(content) = fs::read_to_string("test_line_numbers.py") {
        for (i, line) in content.lines().enumerate() {
            println!("  Line {}: {}", i + 1, line);
        }
    }
    
    println!("\nFound {} Python symbols:", symbols.len());
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
    }
}

fn test_csharp_line_numbers() {
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file("test_line_numbers.cs");
    
    if let Err(e) = result {
        println!("Error extracting C# symbols: {}", e);
        return;
    }
    
    let symbols = extractor.get_symbols();
    
    println!("C# file content:");
    if let Ok(content) = fs::read_to_string("test_line_numbers.cs") {
        for (i, line) in content.lines().enumerate() {
            println!("  Line {}: {}", i + 1, line);
        }
    }
    
    println!("\nFound {} C# symbols:", symbols.len());
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
    }
}