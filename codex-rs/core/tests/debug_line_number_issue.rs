use codex_core::code_analysis::context_extractor::{ContextExtractor, SymbolType};
use tempfile::tempdir;
use std::fs;

#[test]
fn debug_line_number_issue() {
    let dir = tempdir().unwrap();
    
    // Test Python
    let python_file = dir.path().join("test.py");
    let python_content = r#"# Line 1: Comment
def simple_function():  # Line 2: Function definition
    """Docstring"""     # Line 3: Docstring
    return 42           # Line 4: Return statement
                        # Line 5: End of function

class TestClass:        # Line 7: Class definition
    def method(self):   # Line 8: Method definition
        return "test"   # Line 9: Return statement
                        # Line 10: End of method and class
"#;
    
    fs::write(&python_file, python_content).expect("Failed to write Python test file");
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(python_file.to_str().unwrap());
    
    assert!(result.is_ok(), "Failed to extract Python symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    
    println!("Python file content:");
    for (i, line) in python_content.lines().enumerate() {
        println!("  Line {}: {}", i + 1, line);
    }
    
    println!("\nFound {} Python symbols:", symbols.len());
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
        
        // Check if line numbers are correct
        match symbol.symbol_type {
            SymbolType::Function if symbol.name == "simple_function" => {
                println!("    Expected: Function 'simple_function' should start at line 2");
                println!("    Actual: starts at line {}", symbol.start_line);
                if symbol.start_line != 2 {
                    println!("    ❌ OFF BY {}", symbol.start_line - 2);
                } else {
                    println!("    ✅ CORRECT");
                }
            },
            SymbolType::Class if symbol.name == "TestClass" => {
                println!("    Expected: Class 'TestClass' should start at line 7");
                println!("    Actual: starts at line {}", symbol.start_line);
                if symbol.start_line != 7 {
                    println!("    ❌ OFF BY {}", symbol.start_line - 7);
                } else {
                    println!("    ✅ CORRECT");
                }
            },
            SymbolType::Method if symbol.name == "method" => {
                println!("    Expected: Method 'method' should start at line 8");
                println!("    Actual: starts at line {}", symbol.start_line);
                if symbol.start_line != 8 {
                    println!("    ❌ OFF BY {}", symbol.start_line - 8);
                } else {
                    println!("    ✅ CORRECT");
                }
            },
            _ => {}
        }
    }
    
    // Test C#
    println!("\n{}", "=".repeat(50));
    
    let csharp_file = dir.path().join("test.cs");
    let csharp_content = r#"using System;

// Line 3: Comment
public class TestClass  // Line 4: Class definition
{
    // Line 6: Method definition
    public int Add(int a, int b)
    {
        return a + b;   // Line 9: Return statement
    }                   // Line 10: End of method
}                       // Line 11: End of class
"#;
    
    fs::write(&csharp_file, csharp_content).expect("Failed to write C# test file");
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(csharp_file.to_str().unwrap());
    
    assert!(result.is_ok(), "Failed to extract C# symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    
    println!("C# file content:");
    for (i, line) in csharp_content.lines().enumerate() {
        println!("  Line {}: {}", i + 1, line);
    }
    
    println!("\nFound {} C# symbols:", symbols.len());
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
        
        // Check if line numbers are correct
        match symbol.symbol_type {
            SymbolType::Class if symbol.name == "TestClass" => {
                println!("    Expected: Class 'TestClass' should start at line 4");
                println!("    Actual: starts at line {}", symbol.start_line);
                if symbol.start_line != 4 {
                    println!("    ❌ OFF BY {}", symbol.start_line - 4);
                } else {
                    println!("    ✅ CORRECT");
                }
            },
            SymbolType::Method if symbol.name == "Add" => {
                println!("    Expected: Method 'Add' should start at line 7");
                println!("    Actual: starts at line {}", symbol.start_line);
                if symbol.start_line != 7 {
                    println!("    ❌ OFF BY {}", symbol.start_line - 7);
                } else {
                    println!("    ✅ CORRECT");
                }
            },
            _ => {}
        }
    }
}