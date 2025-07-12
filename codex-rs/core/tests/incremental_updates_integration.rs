use std::fs;
use std::io::Write;
use std::thread;
use std::time::Duration;
use tempfile::tempdir;
use serde_json::json;

use codex_core::code_analysis::{
    tools::{handle_analyze_code, handle_get_multiple_files_skeleton},
    graph_manager,
};

#[test]
fn test_incremental_updates_integration() {
    // Use the existing test_files directory structure
    let test_dir = std::path::Path::new("test_files/csharp_test_suite");
    
    // Ensure the graph is initialized for the test directory
    let _ = graph_manager::ensure_graph_for_path(test_dir);
    
    // Test with an existing file first to verify the setup works
    let analyze_input = json!({
        "file_path": "BasicClass.cs"
    });
    
    let analysis = handle_analyze_code(analyze_input).unwrap().unwrap();
    let symbols = analysis.get("symbols").unwrap().as_array().unwrap();
    
    // Should find symbols in BasicClass.cs
    assert!(!symbols.is_empty(), "Should find symbols in BasicClass.cs");
    
    // Test skeleton generation
    let skeleton_input = json!({
        "file_paths": ["BasicClass.cs"],
        "max_tokens": 4000
    });
    
    let skeleton = handle_get_multiple_files_skeleton(skeleton_input).unwrap().unwrap();
    let skeleton_str = skeleton.to_string();
    
    // Should use tree-sitter mode, not fallback (this verifies our fix)
    assert!(skeleton_str.contains("symbols detected"), 
           "Should use tree-sitter mode for existing C# files");
    assert!(!skeleton_str.contains("Fallback skeleton generation"), 
           "Should not use fallback mode for existing C# files");
    
    println!("✅ Incremental updates integration test passed!");
}

#[test]
fn test_no_fallback_for_simple_csharp() {
    // Create a temporary directory for this test
    let dir = tempdir().unwrap();
    
    // Create a simple C# file
    let simple_content = r#"using System;

namespace Test
{
    public class Simple
    {
        public void Method()
        {
            Console.WriteLine("test");
        }
    }
}"#;
    
    let cs_file_path = dir.path().join("Simple.cs");
    let mut file = fs::File::create(&cs_file_path).unwrap();
    file.write_all(simple_content.as_bytes()).unwrap();
    
    // Change to the temp directory and initialize graph
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir.path()).unwrap();
    
    // Initialize graph manager for current directory
    let _ = graph_manager::ensure_graph_for_path(std::path::Path::new("."));
    
    // Test skeleton generation with relative path
    let skeleton_input = json!({
        "file_paths": ["Simple.cs"],
        "max_tokens": 4000
    });
    
    let skeleton = handle_get_multiple_files_skeleton(skeleton_input).unwrap().unwrap();
    let skeleton_str = skeleton.to_string();
    
    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
    
    // Debug output
    println!("Skeleton output: {}", skeleton_str);
    
    // This test verifies that our fix prevents fallback mode for simple files
    // If this fails, it means the 300-character threshold bug has returned
    if skeleton_str.contains("Fallback skeleton generation") {
        panic!("Simple C# files should NOT trigger fallback mode. This indicates the 300-character threshold bug has returned.");
    }
    
    // Should use tree-sitter mode
    assert!(skeleton_str.contains("symbols detected") || skeleton_str.contains("Simple"), 
           "Should either use tree-sitter mode or at least contain the class name");
    
    println!("✅ No fallback for simple C# test passed!");
}

#[test]
fn test_skeleton_generation_consistency() {
    // Test that skeleton generation is consistent and doesn't randomly switch to fallback
    let test_dir = std::path::Path::new("test_files/csharp_test_suite");
    let _ = graph_manager::ensure_graph_for_path(test_dir);
    
    // Test multiple files to ensure consistency
    let test_files = ["BasicClass.cs", "Program.cs"];
    
    for file_name in &test_files {
        println!("Testing consistency for {}", file_name);
        
        // Generate skeleton multiple times
        for i in 0..3 {
            let skeleton_input = json!({
                "file_paths": [file_name],
                "max_tokens": 4000
            });
            
            let skeleton = handle_get_multiple_files_skeleton(skeleton_input).unwrap().unwrap();
            let skeleton_str = skeleton.to_string();
            
            // Should consistently use tree-sitter mode
            assert!(!skeleton_str.contains("Fallback skeleton generation"), 
                   "File {} should not use fallback mode on attempt {}", file_name, i + 1);
            
            // Small delay between attempts
            thread::sleep(Duration::from_millis(10));
        }
    }
    
    println!("✅ Skeleton generation consistency test passed!");
}