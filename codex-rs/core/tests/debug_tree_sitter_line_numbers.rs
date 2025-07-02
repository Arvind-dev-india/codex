use codex_core::code_analysis::parser_pool::{get_parser_pool};
use tempfile::tempdir;
use std::fs;

#[test]
fn debug_tree_sitter_line_numbers() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.py");
    
    // Create a Python file with known line numbers for each construct
    let content = r#"# Line 1: Comment
def simple_function():  # Line 2: Function definition
    """Docstring"""     # Line 3: Docstring
    return 42           # Line 4: Return statement
                        # Line 5: End of function

class TestClass:        # Line 7: Class definition
    def method(self):   # Line 8: Method definition
        return "test"   # Line 9: Return statement
                        # Line 10: End of method and class

def another_function(): # Line 12: Another function
    pass                # Line 13: Pass statement
"#;
    
    fs::write(&file_path, content).expect("Failed to write test file");
    
    // Parse the file
    let parsed_file = get_parser_pool().parse_file_if_needed(file_path.to_str().unwrap())
        .expect("Failed to parse file");
    
    println!("File content with line numbers:");
    for (i, line) in content.lines().enumerate() {
        println!("  Line {}: {}", i + 1, line);
    }
    
    // Get query captures using the Python SCM query directly
    let python_query = include_str!("../src/code_analysis/queries/python.scm");
    let query_matches = parsed_file.execute_query(python_query)
        .expect("Failed to execute query");
    
    println!("\nTree-sitter query matches:");
    for (i, query_match) in query_matches.iter().enumerate() {
        println!("  Match {}: pattern_index={}", i, query_match.pattern_index);
        for capture in &query_match.captures {
            println!("    Capture: {} -> '{}' at 0-based ({}, {}) to ({}, {})", 
                     capture.name, 
                     capture.text.replace('\n', "\\n"),
                     capture.start_point.0, capture.start_point.1,
                     capture.end_point.0, capture.end_point.1);
            println!("      1-based: lines {}-{}", 
                     capture.start_point.0 + 1, capture.end_point.0 + 1);
            
            // Check what the actual lines contain
            let lines: Vec<&str> = content.lines().collect();
            if capture.start_point.0 < lines.len() {
                println!("      Actual start line {}: '{}'", 
                         capture.start_point.0 + 1, lines[capture.start_point.0]);
            }
            if capture.end_point.0 < lines.len() && capture.end_point.0 != capture.start_point.0 {
                println!("      Actual end line {}: '{}'", 
                         capture.end_point.0 + 1, lines[capture.end_point.0]);
            }
        }
        println!();
    }
}