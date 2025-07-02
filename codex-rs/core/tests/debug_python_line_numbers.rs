use codex_core::code_analysis::parser_pool::{get_parser_pool, QueryType};
use tempfile::tempdir;
use std::fs;

#[test]
fn debug_python_line_numbers() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("debug.py");
    
    // Create a simple Python file with known line numbers
    let content = r#"# Line 1: Comment
def simple_function():  # Line 2: Function definition
    """Docstring"""     # Line 3: Docstring
    return 42           # Line 4: Return statement
                        # Line 5: End of function

class TestClass:        # Line 6: Class definition
    def method(self):   # Line 7: Method definition
        return "test"   # Line 8: Return statement
                        # Line 9: End of method and class
"#;
    
    fs::write(&file_path, content).expect("Failed to write test file");
    
    // Parse the file
    let parsed_file = get_parser_pool().parse_file_if_needed(file_path.to_str().unwrap())
        .expect("Failed to parse file");
    
    println!("File content:");
    println!("{}", content);
    println!("\nParsed tree:");
    println!("{}", parsed_file.tree.root_node().to_sexp());
    
    // Get query captures using the Python SCM query directly
    let python_query = include_str!("../src/code_analysis/queries/python.scm");
    let query_matches = parsed_file.execute_query(python_query)
        .expect("Failed to execute query");
    
    println!("\nQuery matches:");
    for (i, query_match) in query_matches.iter().enumerate() {
        println!("  Match {}: pattern_index={}", i, query_match.pattern_index);
        for capture in &query_match.captures {
            println!("    Capture: {} -> '{}' at ({}, {}) to ({}, {})", 
                     capture.name, 
                     capture.text,
                     capture.start_point.0 + 1, capture.start_point.1,
                     capture.end_point.0 + 1, capture.end_point.1);
            
            // Get the actual text from the source for this node
            let start_byte = capture.start_byte;
            let end_byte = capture.end_byte;
            let node_text = &content.as_bytes()[start_byte..end_byte];
            let node_text_str = String::from_utf8_lossy(node_text);
            println!("      Full node text: {:?}", node_text_str);
        }
        println!();
    }
    
    // Also check what lines each capture spans
    let lines: Vec<&str> = content.lines().collect();
    println!("Line-by-line content:");
    for (i, line) in lines.iter().enumerate() {
        println!("  Line {}: {}", i + 1, line);
    }
}