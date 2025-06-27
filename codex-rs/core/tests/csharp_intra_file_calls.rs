use codex_core::code_analysis::context_extractor::{ContextExtractor, SymbolType, ReferenceType};
use codex_core::code_analysis::{get_parser_pool, SupportedLanguage, QueryType};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_csharp_intra_file_method_calls() {
    // Create a temporary directory and file
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("IntraFileCalls.cs");
    
    let content = r#"
using System;

namespace TestNamespace
{
    public class Calculator
    {
        private int _value = 0;
        
        public int Add(int a, int b)
        {
            int result = a + b;
            LogOperation("Addition", result);  // Method call within same class
            return result;
        }
        
        public int Multiply(int x, int y)
        {
            int result = x * y;
            LogOperation("Multiplication", result);  // Method call within same class
            return result;
        }
        
        public int Calculate(int a, int b)
        {
            int sum = Add(a, b);           // Method call within same class
            int product = Multiply(a, b);   // Method call within same class
            return sum + product;
        }
        
        private void LogOperation(string operation, int result)
        {
            Console.WriteLine($"{operation}: {result}");  // External method call
            PrintTimestamp();  // Method call within same class
        }
        
        private void PrintTimestamp()
        {
            Console.WriteLine(DateTime.Now);  // External method call
        }
        
        public void ChainedCalls()
        {
            int result = Calculate(5, 3);  // Method call within same class
            LogOperation("Final", result); // Method call within same class
        }
    }
}
"#;
    
    fs::write(&file_path, content).expect("Failed to write test file");
    
    // Test parser pool parsing first
    let parser_pool = get_parser_pool();
    let parsed_file = parser_pool.parse_file_from_disk(file_path.to_str().unwrap()).unwrap();
    
    // Test query execution to see what references are found
    let query_result = parsed_file.execute_predefined_query(QueryType::All);
    assert!(query_result.is_ok(), "Should be able to execute C# queries");
    
    let matches = query_result.unwrap();
    
    println!("=== RAW QUERY MATCHES ===");
    for (i, match_) in matches.iter().enumerate() {
        println!("Match {}: pattern {}", i, match_.pattern_index);
        for capture in &match_.captures {
            println!("  Capture '{}': '{}' at line {}", 
                     capture.name, capture.text, capture.start_point.0 + 1);
        }
    }
    
    // Test context extractor
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(file_path.to_str().unwrap());
    assert!(result.is_ok(), "Failed to extract symbols");
    
    let symbols = extractor.get_symbols();
    let references = extractor.get_references();
    
    println!("\n=== EXTRACTED SYMBOLS ===");
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
    }
    
    println!("\n=== EXTRACTED REFERENCES ===");
    for reference in references {
        println!("  {} ({:?}) at {}:{}", 
                 reference.symbol_name, reference.reference_type, 
                 reference.reference_file, reference.reference_line);
    }
    
    // Test specific method calls we expect to find
    let method_calls: Vec<_> = references.iter()
        .filter(|r| matches!(r.reference_type, ReferenceType::Call))
        .collect();
    
    println!("\n=== METHOD CALLS FOUND ===");
    for call in &method_calls {
        println!("  Call to '{}' at line {}", call.symbol_name, call.reference_line);
    }
    
    // Check if we found the expected intra-file method calls
    let expected_calls = [
        "LogOperation",  // Called from Add and Multiply
        "Add",          // Called from Calculate  
        "Multiply",     // Called from Calculate
        "PrintTimestamp", // Called from LogOperation
        "Calculate",    // Called from ChainedCalls
        "WriteLine",    // Called from LogOperation and PrintTimestamp
    ];
    
    for expected_call in &expected_calls {
        let found = method_calls.iter().any(|call| call.symbol_name == *expected_call);
        println!("Expected call to '{}': {}", expected_call, if found { "✅ FOUND" } else { "❌ MISSING" });
    }
    
    // Verify we found at least some method calls
    assert!(!method_calls.is_empty(), "Should find some method calls, but found none");
    
    // Check for specific calls we know should be there
    let log_operation_calls = method_calls.iter()
        .filter(|call| call.symbol_name == "LogOperation")
        .count();
    
    let add_calls = method_calls.iter()
        .filter(|call| call.symbol_name == "Add")
        .count();
    
    let multiply_calls = method_calls.iter()
        .filter(|call| call.symbol_name == "Multiply")
        .count();
    
    println!("\n=== CALL COUNTS ===");
    println!("LogOperation calls: {}", log_operation_calls);
    println!("Add calls: {}", add_calls);
    println!("Multiply calls: {}", multiply_calls);
    
    // We should find at least some of these calls
    let total_intra_file_calls = log_operation_calls + add_calls + multiply_calls;
    println!("Total intra-file method calls found: {}", total_intra_file_calls);
    
    if total_intra_file_calls == 0 {
        println!("❌ NO INTRA-FILE METHOD CALLS DETECTED - This needs to be fixed!");
    } else {
        println!("✅ Some intra-file method calls detected");
    }
}

#[test]
fn test_csharp_simple_method_call() {
    // Test with a very simple case to isolate the issue
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("SimpleCall.cs");
    
    let content = r#"
public class Simple
{
    public void MethodA()
    {
        MethodB();
    }
    
    public void MethodB()
    {
        System.Console.WriteLine("Hello");
    }
}
"#;
    
    fs::write(&file_path, content).expect("Failed to write test file");
    
    let parser_pool = get_parser_pool();
    let parsed_file = parser_pool.parse_file_from_disk(file_path.to_str().unwrap()).unwrap();
    
    let query_result = parsed_file.execute_predefined_query(QueryType::All).unwrap();
    
    println!("\n=== SIMPLE CALL TEST - RAW MATCHES ===");
    for (i, match_) in query_result.iter().enumerate() {
        println!("Match {}: pattern {}", i, match_.pattern_index);
        for capture in &match_.captures {
            println!("  Capture '{}': '{}' at line {}", 
                     capture.name, capture.text, capture.start_point.0 + 1);
        }
    }
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(file_path.to_str().unwrap());
    assert!(result.is_ok());
    
    let references = extractor.get_references();
    
    println!("\n=== SIMPLE CALL TEST - REFERENCES ===");
    for reference in references {
        println!("  {} ({:?}) at line {}", 
                 reference.symbol_name, reference.reference_type, reference.reference_line);
    }
    
    // Check if MethodB call is detected
    let method_b_calls = references.iter()
        .filter(|r| r.symbol_name == "MethodB" && matches!(r.reference_type, ReferenceType::Call))
        .count();
    
    println!("MethodB calls found: {}", method_b_calls);
    
    if method_b_calls == 0 {
        println!("❌ Simple method call MethodB() not detected!");
    } else {
        println!("✅ Simple method call MethodB() detected");
    }
}