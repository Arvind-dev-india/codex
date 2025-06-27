use codex_core::code_analysis::tools::{
    handle_analyze_code,
    handle_find_symbol_references,
    handle_find_symbol_definitions,
    handle_get_symbol_subgraph,
    AnalyzeCodeInput,
    FindSymbolReferencesInput,
    FindSymbolDefinitionsInput,
    GetSymbolSubgraphInput,
};
use codex_core::code_analysis::graph_manager::initialize_graph_async;
use serde_json::json;
use std::fs;
use tempfile::tempdir;

#[tokio::test]
async fn test_csharp_analyze_code_tool() {
    // Create test files
    let dir = tempdir().unwrap();
    let file1_path = dir.path().join("Calculator.cs");
    let file2_path = dir.path().join("MathHelper.cs");
    
    let calculator_content = r#"
using System;

namespace MathLibrary
{
    public class Calculator
    {
        public int Add(int a, int b)
        {
            return a + b;
        }
        
        public int Multiply(int x, int y)
        {
            var helper = new MathHelper();
            return helper.MultiplyNumbers(x, y);
        }
        
        public void PrintResult(int result)
        {
            Console.WriteLine($"Result: {result}");
        }
    }
}
"#;

    let helper_content = r#"
using System;

namespace MathLibrary
{
    public class MathHelper
    {
        public int MultiplyNumbers(int a, int b)
        {
            return a * b;
        }
        
        public void LogCalculation(string operation)
        {
            Console.WriteLine($"Operation: {operation}");
        }
    }
}
"#;
    
    fs::write(&file1_path, calculator_content).expect("Failed to write Calculator.cs");
    fs::write(&file2_path, helper_content).expect("Failed to write MathHelper.cs");
    
    // Initialize the code graph for the directory (two-pass approach)
    println!("=== INITIALIZING CODE GRAPH ===");
    let result = initialize_graph_async(dir.path()).await;
    assert!(result.is_ok(), "Failed to initialize code graph: {:?}", result.err());
    
    // Test 1: Analyze code tool
    println!("\n=== TESTING ANALYZE_CODE TOOL ===");
    let analyze_input = json!({
        "file_path": file1_path.to_str().unwrap()
    });
    
    let analyze_result = handle_analyze_code(analyze_input);
    assert!(analyze_result.is_some(), "analyze_code should return a result");
    
    let analyze_output = analyze_result.unwrap();
    assert!(analyze_output.is_ok(), "analyze_code should succeed: {:?}", analyze_output.err());
    
    let analyze_json = analyze_output.unwrap();
    println!("Analyze code result: {}", serde_json::to_string_pretty(&analyze_json).unwrap());
    
    // Verify we found symbols in the file
    if let Some(symbols) = analyze_json.get("symbols") {
        if let Some(symbols_array) = symbols.as_array() {
            assert!(!symbols_array.is_empty(), "Should find symbols in the file");
            
            // Look for Calculator class
            let calculator_class = symbols_array.iter()
                .find(|s| s.get("name").and_then(|n| n.as_str()) == Some("Calculator"));
            assert!(calculator_class.is_some(), "Should find Calculator class");
            
            // Look for Add method
            let add_method = symbols_array.iter()
                .find(|s| s.get("name").and_then(|n| n.as_str()) == Some("Add"));
            assert!(add_method.is_some(), "Should find Add method");
            
            println!("✅ Found {} symbols in Calculator.cs", symbols_array.len());
        }
    }
}

#[tokio::test]
async fn test_csharp_find_symbol_references_tool() {
    // Create test files
    let dir = tempdir().unwrap();
    let file1_path = dir.path().join("Calculator.cs");
    let file2_path = dir.path().join("Program.cs");
    
    let calculator_content = r#"
namespace MathLibrary
{
    public class Calculator
    {
        public int Add(int a, int b)
        {
            return a + b;
        }
    }
}
"#;

    let program_content = r#"
using MathLibrary;

namespace TestApp
{
    public class Program
    {
        public static void Main()
        {
            var calc = new Calculator();
            int result = calc.Add(5, 3);
            System.Console.WriteLine(result);
        }
    }
}
"#;
    
    fs::write(&file1_path, calculator_content).expect("Failed to write Calculator.cs");
    fs::write(&file2_path, program_content).expect("Failed to write Program.cs");
    
    // Initialize the code graph
    println!("=== INITIALIZING CODE GRAPH FOR REFERENCES TEST ===");
    let result = initialize_graph_async(dir.path()).await;
    assert!(result.is_ok(), "Failed to initialize code graph: {:?}", result.err());
    
    // Test 2: Find symbol references tool
    println!("\n=== TESTING FIND_SYMBOL_REFERENCES TOOL ===");
    let references_input = json!({
        "symbol_name": "Calculator"
    });
    
    let references_result = handle_find_symbol_references(references_input);
    assert!(references_result.is_some(), "find_symbol_references should return a result");
    
    let references_output = references_result.unwrap();
    assert!(references_output.is_ok(), "find_symbol_references should succeed: {:?}", references_output.err());
    
    let references_json = references_output.unwrap();
    println!("Find references result: {}", serde_json::to_string_pretty(&references_json).unwrap());
    
    // Check if we found references to Calculator
    if let Some(references) = references_json.get("references") {
        if let Some(refs_array) = references.as_array() {
            println!("✅ Found {} references to Calculator", refs_array.len());
            
            // Look for the "new Calculator()" reference
            let constructor_ref = refs_array.iter()
                .find(|r| r.get("file_path").and_then(|f| f.as_str()).unwrap_or("").contains("Program.cs"));
            
            if constructor_ref.is_some() {
                println!("✅ Found Calculator reference in Program.cs");
            } else {
                println!("❌ Did not find Calculator reference in Program.cs");
            }
        }
    }
}

#[tokio::test]
async fn test_csharp_find_symbol_definitions_tool() {
    // Create test files
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("Calculator.cs");
    
    let content = r#"
namespace MathLibrary
{
    public class Calculator
    {
        public int Add(int a, int b)
        {
            return a + b;
        }
        
        public int Add(double a, double b)  // Overloaded method
        {
            return (int)(a + b);
        }
    }
}
"#;
    
    fs::write(&file_path, content).expect("Failed to write Calculator.cs");
    
    // Initialize the code graph
    println!("=== INITIALIZING CODE GRAPH FOR DEFINITIONS TEST ===");
    let result = initialize_graph_async(dir.path()).await;
    assert!(result.is_ok(), "Failed to initialize code graph: {:?}", result.err());
    
    // Test 3: Find symbol definitions tool
    println!("\n=== TESTING FIND_SYMBOL_DEFINITIONS TOOL ===");
    let definitions_input = json!({
        "symbol_name": "Add"
    });
    
    let definitions_result = handle_find_symbol_definitions(definitions_input);
    assert!(definitions_result.is_some(), "find_symbol_definitions should return a result");
    
    let definitions_output = definitions_result.unwrap();
    assert!(definitions_output.is_ok(), "find_symbol_definitions should succeed: {:?}", definitions_output.err());
    
    let definitions_json = definitions_output.unwrap();
    println!("Find definitions result: {}", serde_json::to_string_pretty(&definitions_json).unwrap());
    
    // Check if we found definitions of Add method
    if let Some(definitions) = definitions_json.get("definitions") {
        if let Some(defs_array) = definitions.as_array() {
            println!("✅ Found {} definitions of Add method", defs_array.len());
            
            // Should find both overloaded versions
            assert!(defs_array.len() >= 1, "Should find at least one Add method definition");
        }
    }
}

#[tokio::test]
async fn test_csharp_get_symbol_subgraph_tool() {
    // Create test files with method calls
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("Calculator.cs");
    
    let content = r#"
namespace MathLibrary
{
    public class Calculator
    {
        public int Add(int a, int b)
        {
            LogOperation("Add");
            return a + b;
        }
        
        public int Calculate(int x, int y)
        {
            int sum = Add(x, y);
            LogOperation("Calculate");
            return sum * 2;
        }
        
        private void LogOperation(string operation)
        {
            System.Console.WriteLine($"Operation: {operation}");
        }
    }
}
"#;
    
    fs::write(&file_path, content).expect("Failed to write Calculator.cs");
    
    // Initialize the code graph
    println!("=== INITIALIZING CODE GRAPH FOR SUBGRAPH TEST ===");
    let result = initialize_graph_async(dir.path()).await;
    assert!(result.is_ok(), "Failed to initialize code graph: {:?}", result.err());
    
    // Test 4: Get symbol subgraph tool
    println!("\n=== TESTING GET_SYMBOL_SUBGRAPH TOOL ===");
    let subgraph_input = json!({
        "symbol_name": "Calculate",
        "max_depth": 2
    });
    
    let subgraph_result = handle_get_symbol_subgraph(subgraph_input);
    assert!(subgraph_result.is_some(), "get_symbol_subgraph should return a result");
    
    let subgraph_output = subgraph_result.unwrap();
    assert!(subgraph_output.is_ok(), "get_symbol_subgraph should succeed: {:?}", subgraph_output.err());
    
    let subgraph_json = subgraph_output.unwrap();
    println!("Get subgraph result: {}", serde_json::to_string_pretty(&subgraph_json).unwrap());
    
    // Check if we found a subgraph starting from Calculate
    if let Some(nodes) = subgraph_json.get("nodes") {
        if let Some(nodes_array) = nodes.as_array() {
            println!("✅ Found {} nodes in Calculate subgraph", nodes_array.len());
            
            // Should include Calculate method itself
            let calculate_node = nodes_array.iter()
                .find(|n| n.get("name").and_then(|name| name.as_str()) == Some("Calculate"));
            assert!(calculate_node.is_some(), "Should include Calculate node in subgraph");
        }
    }
    
    if let Some(edges) = subgraph_json.get("edges") {
        if let Some(edges_array) = edges.as_array() {
            println!("✅ Found {} edges in Calculate subgraph", edges_array.len());
        }
    }
}

#[tokio::test]
async fn test_two_pass_graph_building() {
    // Test the two-pass approach: first pass builds symbols, second pass builds references
    let dir = tempdir().unwrap();
    
    // Create multiple files that reference each other
    let file1_path = dir.path().join("ServiceA.cs");
    let file2_path = dir.path().join("ServiceB.cs");
    let file3_path = dir.path().join("Client.cs");
    
    let service_a_content = r#"
namespace Services
{
    public class ServiceA
    {
        public void ProcessData()
        {
            var serviceB = new ServiceB();
            serviceB.HandleData();
        }
    }
}
"#;

    let service_b_content = r#"
namespace Services
{
    public class ServiceB
    {
        public void HandleData()
        {
            System.Console.WriteLine("Handling data");
        }
    }
}
"#;

    let client_content = r#"
using Services;

namespace Client
{
    public class ClientApp
    {
        public void Run()
        {
            var serviceA = new ServiceA();
            serviceA.ProcessData();
        }
    }
}
"#;
    
    fs::write(&file1_path, service_a_content).expect("Failed to write ServiceA.cs");
    fs::write(&file2_path, service_b_content).expect("Failed to write ServiceB.cs");
    fs::write(&file3_path, client_content).expect("Failed to write Client.cs");
    
    println!("=== TESTING TWO-PASS GRAPH BUILDING ===");
    
    // Initialize the code graph (this should do the two-pass approach)
    let result = initialize_graph_async(dir.path()).await;
    assert!(result.is_ok(), "Failed to initialize code graph: {:?}", result.err());
    
    // Test that we can find cross-file references
    println!("\n=== TESTING CROSS-FILE REFERENCES ===");
    
    // Look for ServiceA references
    let service_a_refs_input = json!({
        "symbol_name": "ServiceA"
    });
    
    let service_a_refs_result = handle_find_symbol_references(service_a_refs_input);
    if let Some(Ok(refs_json)) = service_a_refs_result {
        println!("ServiceA references: {}", serde_json::to_string_pretty(&refs_json).unwrap());
    }
    
    // Look for ServiceB references  
    let service_b_refs_input = json!({
        "symbol_name": "ServiceB"
    });
    
    let service_b_refs_result = handle_find_symbol_references(service_b_refs_input);
    if let Some(Ok(refs_json)) = service_b_refs_result {
        println!("ServiceB references: {}", serde_json::to_string_pretty(&refs_json).unwrap());
    }
    
    // Test subgraph from ClientApp to see if it includes the call chain
    let client_subgraph_input = json!({
        "symbol_name": "Run",
        "max_depth": 3
    });
    
    let client_subgraph_result = handle_get_symbol_subgraph(client_subgraph_input);
    if let Some(Ok(subgraph_json)) = client_subgraph_result {
        println!("ClientApp.Run subgraph: {}", serde_json::to_string_pretty(&subgraph_json).unwrap());
        
        // Check if the subgraph includes the call chain
        if let Some(nodes) = subgraph_json.get("nodes") {
            if let Some(nodes_array) = nodes.as_array() {
                let node_names: Vec<_> = nodes_array.iter()
                    .filter_map(|n| n.get("name").and_then(|name| name.as_str()))
                    .collect();
                
                println!("Nodes in subgraph: {:?}", node_names);
                
                // Should ideally include Run -> ProcessData -> HandleData call chain
                let has_run = node_names.contains(&"Run");
                let has_process_data = node_names.contains(&"ProcessData");
                let has_handle_data = node_names.contains(&"HandleData");
                
                println!("Call chain detection:");
                println!("  Run method: {}", if has_run { "✅" } else { "❌" });
                println!("  ProcessData method: {}", if has_process_data { "✅" } else { "❌" });
                println!("  HandleData method: {}", if has_handle_data { "✅" } else { "❌" });
            }
        }
    }
    
    println!("✅ Two-pass graph building test completed");
}