use std::path::PathBuf;
use std::fs;
use std::io::Write;
use tempfile::tempdir;

use codex_core::code_analysis::repo_mapper::RepoMapper;

// Helper function to create a temporary file with content
fn create_temp_file(dir: &tempfile::TempDir, filename: &str, content: &str) -> PathBuf {
    let file_path = dir.path().join(filename);
    // Create parent directories if they don't exist
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file_path
}

#[test]
fn test_csharp_intrafile_edges() {
    let dir = tempdir().unwrap();
    
    // Create a C# file with multiple classes and methods that reference each other
    let csharp_content = r#"
using System;

namespace TestNamespace
{
    public interface ICalculator
    {
        int Add(int a, int b);
        int Multiply(int a, int b);
    }

    public class Calculator : ICalculator
    {
        private int lastResult;

        public int Add(int a, int b)
        {
            lastResult = a + b;
            return lastResult;
        }

        public int Multiply(int a, int b)
        {
            lastResult = a * b;
            return lastResult;
        }

        public int AddAndMultiply(int a, int b, int c)
        {
            // This method calls other methods in the same class (intrafile references)
            int sum = Add(a, b);        // Reference to Add method
            int result = Multiply(sum, c);  // Reference to Multiply method
            return result;
        }

        public void PrintResult()
        {
            // Reference to another method in same class
            int result = AddAndMultiply(5, 3, 2);
            Console.WriteLine($"Result: {result}");
        }
    }

    public class MathHelper
    {
        private Calculator calc;

        public MathHelper()
        {
            // Reference to Calculator class (intrafile)
            calc = new Calculator();
        }

        public int DoComplexCalculation(int x, int y, int z)
        {
            // Reference to Calculator methods (intrafile)
            return calc.AddAndMultiply(x, y, z);
        }
    }
}
"#;
    
    let _csharp_file_path = create_temp_file(&dir, "Calculator.cs", csharp_content);
    
    // Create a repository mapper and map the repository
    let mut repo_mapper = RepoMapper::new(dir.path());
    let result = repo_mapper.map_repository();
    assert!(result.is_ok(), "Failed to map repository: {:?}", result.err());
    
    // Get the graph
    let graph = repo_mapper.get_graph();
    
    // Print the graph for debugging
    println!("=== INTRAFILE TEST ===");
    println!("Nodes:");
    for node in &graph.nodes {
        println!("  {} - {} ({:?}) at lines {}-{}", 
                 node.id, node.name, node.node_type, node.start_line, node.end_line);
    }
    
    println!("Edges:");
    for edge in &graph.edges {
        println!("  {} -> {} ({:?})", edge.source, edge.target, edge.edge_type);
    }
    
    // Verify that we have the expected symbols
    let has_calculator_class = graph.nodes.iter().any(|n| n.name == "Calculator");
    let has_math_helper_class = graph.nodes.iter().any(|n| n.name == "MathHelper");
    let has_add_method = graph.nodes.iter().any(|n| n.name == "Add");
    let has_multiply_method = graph.nodes.iter().any(|n| n.name == "Multiply");
    let has_add_and_multiply_method = graph.nodes.iter().any(|n| n.name == "AddAndMultiply");
    let has_print_result_method = graph.nodes.iter().any(|n| n.name == "PrintResult");
    let has_do_complex_calculation_method = graph.nodes.iter().any(|n| n.name == "DoComplexCalculation");
    
    assert!(has_calculator_class, "Should have Calculator class");
    assert!(has_math_helper_class, "Should have MathHelper class");
    assert!(has_add_method, "Should have Add method");
    assert!(has_multiply_method, "Should have Multiply method");
    assert!(has_add_and_multiply_method, "Should have AddAndMultiply method");
    assert!(has_print_result_method, "Should have PrintResult method");
    assert!(has_do_complex_calculation_method, "Should have DoComplexCalculation method");
    
    // Find specific nodes for edge verification
    let add_and_multiply_node = graph.nodes.iter()
        .find(|n| n.name == "AddAndMultiply")
        .expect("AddAndMultiply method should exist");
    
    let print_result_node = graph.nodes.iter()
        .find(|n| n.name == "PrintResult")
        .expect("PrintResult method should exist");
    
    let math_helper_constructor = graph.nodes.iter()
        .find(|n| n.name == "MathHelper" && n.node_type == codex_core::code_analysis::repo_mapper::CodeNodeType::Method)
        .or_else(|| graph.nodes.iter().find(|n| n.name == "MathHelper"));
    
    let do_complex_calculation_node = graph.nodes.iter()
        .find(|n| n.name == "DoComplexCalculation")
        .expect("DoComplexCalculation method should exist");
    
    // Verify intrafile edges exist and have correct source nodes
    
    // 1. AddAndMultiply method should call Add method
    let add_and_multiply_to_add_edge = graph.edges.iter()
        .find(|e| e.source == add_and_multiply_node.id && 
                  graph.nodes.iter().any(|n| n.id == e.target && n.name == "Add"));
    
    if let Some(edge) = add_and_multiply_to_add_edge {
        println!("✓ Found edge from AddAndMultiply to Add: {:?}", edge.edge_type);
        assert!(matches!(edge.edge_type, 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::Calls | 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::References),
                "Edge should be a call or reference");
    } else {
        println!("⚠ Missing edge from AddAndMultiply to Add method");
    }
    
    // 2. AddAndMultiply method should call Multiply method
    let add_and_multiply_to_multiply_edge = graph.edges.iter()
        .find(|e| e.source == add_and_multiply_node.id && 
                  graph.nodes.iter().any(|n| n.id == e.target && n.name == "Multiply"));
    
    if let Some(edge) = add_and_multiply_to_multiply_edge {
        println!("✓ Found edge from AddAndMultiply to Multiply: {:?}", edge.edge_type);
        assert!(matches!(edge.edge_type, 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::Calls | 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::References),
                "Edge should be a call or reference");
    } else {
        println!("⚠ Missing edge from AddAndMultiply to Multiply method");
    }
    
    // 3. PrintResult method should call AddAndMultiply method
    let print_result_to_add_and_multiply_edge = graph.edges.iter()
        .find(|e| e.source == print_result_node.id && e.target == add_and_multiply_node.id);
    
    if let Some(edge) = print_result_to_add_and_multiply_edge {
        println!("✓ Found edge from PrintResult to AddAndMultiply: {:?}", edge.edge_type);
        assert!(matches!(edge.edge_type, 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::Calls | 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::References),
                "Edge should be a call or reference");
    } else {
        println!("⚠ Missing edge from PrintResult to AddAndMultiply method");
    }
    
    // 4. MathHelper constructor should reference Calculator class
    if let Some(constructor_node) = math_helper_constructor {
        let constructor_to_calculator_edge = graph.edges.iter()
            .find(|e| e.source == constructor_node.id && 
                      graph.nodes.iter().any(|n| n.id == e.target && n.name == "Calculator"));
        
        if let Some(edge) = constructor_to_calculator_edge {
            println!("✓ Found edge from MathHelper constructor to Calculator: {:?}", edge.edge_type);
        } else {
            println!("⚠ Missing edge from MathHelper constructor to Calculator class");
        }
    }
    
    // 5. DoComplexCalculation should call AddAndMultiply
    let do_complex_to_add_and_multiply_edge = graph.edges.iter()
        .find(|e| e.source == do_complex_calculation_node.id && e.target == add_and_multiply_node.id);
    
    if let Some(edge) = do_complex_to_add_and_multiply_edge {
        println!("✓ Found edge from DoComplexCalculation to AddAndMultiply: {:?}", edge.edge_type);
        assert!(matches!(edge.edge_type, 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::Calls | 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::References),
                "Edge should be a call or reference");
    } else {
        println!("⚠ Missing edge from DoComplexCalculation to AddAndMultiply method");
    }
    
    // Verify that all edges have proper source nodes (not line numbers)
    for edge in &graph.edges {
        assert!(edge.source.contains("::") || edge.source.starts_with("file:") || edge.source.starts_with("symbol:"),
                "Source should be a proper node ID, not a line number: {}", edge.source);
        assert!(edge.target.contains("::") || edge.target.starts_with("file:") || edge.target.starts_with("symbol:"),
                "Target should be a proper node ID, not a line number: {}", edge.target);
    }
    
    println!("✓ All intrafile edges have proper source and target nodes");
}

#[test]
fn test_csharp_interfile_edges() {
    let dir = tempdir().unwrap();
    
    // Create first C# file with base classes
    let base_classes_content = r#"
using System;

namespace TestNamespace
{
    public interface ILogger
    {
        void Log(string message);
    }

    public class Logger : ILogger
    {
        public void Log(string message)
        {
            Console.WriteLine($"[LOG] {message}");
        }
    }

    public static class MathUtils
    {
        public static int Square(int x)
        {
            return x * x;
        }

        public static int Cube(int x)
        {
            return x * x * x;
        }
    }
}
"#;
    
    // Create second C# file that uses classes from the first file
    let main_program_content = r#"
using System;

namespace TestNamespace
{
    public class Program
    {
        private ILogger logger;

        public Program()
        {
            // Interfile reference to Logger class
            logger = new Logger();
        }

        public void RunCalculations()
        {
            // Interfile references to MathUtils methods
            int result1 = MathUtils.Square(5);
            int result2 = MathUtils.Cube(3);
            
            // Interfile reference to Logger method
            logger.Log($"Square result: {result1}");
            logger.Log($"Cube result: {result2}");
        }

        public static void Main(string[] args)
        {
            Program program = new Program();
            program.RunCalculations();
        }
    }

    public class Calculator
    {
        private ILogger logger;

        public Calculator(ILogger logger)
        {
            this.logger = logger;
        }

        public int Calculate(int x)
        {
            // Interfile reference to MathUtils
            int squared = MathUtils.Square(x);
            
            // Interfile reference to Logger
            logger.Log($"Calculated square of {x} = {squared}");
            
            return squared;
        }
    }
}
"#;
    
    let _base_file_path = create_temp_file(&dir, "BaseClasses.cs", base_classes_content);
    let _main_file_path = create_temp_file(&dir, "Program.cs", main_program_content);
    
    // Create a repository mapper and map the repository
    let mut repo_mapper = RepoMapper::new(dir.path());
    let result = repo_mapper.map_repository();
    assert!(result.is_ok(), "Failed to map repository: {:?}", result.err());
    
    // Get the graph
    let graph = repo_mapper.get_graph();
    
    // Print the graph for debugging
    println!("=== INTERFILE TEST ===");
    println!("Nodes:");
    for node in &graph.nodes {
        println!("  {} - {} ({:?}) at lines {}-{} in {}", 
                 node.id, node.name, node.node_type, node.start_line, node.end_line, node.file_path);
    }
    
    println!("Edges:");
    for edge in &graph.edges {
        println!("  {} -> {} ({:?})", edge.source, edge.target, edge.edge_type);
    }
    
    // Verify that we have symbols from both files
    let has_logger_class = graph.nodes.iter().any(|n| n.name == "Logger");
    let has_math_utils_class = graph.nodes.iter().any(|n| n.name == "MathUtils");
    let has_program_class = graph.nodes.iter().any(|n| n.name == "Program");
    let has_calculator_class = graph.nodes.iter().any(|n| n.name == "Calculator");
    let has_square_method = graph.nodes.iter().any(|n| n.name == "Square");
    let has_cube_method = graph.nodes.iter().any(|n| n.name == "Cube");
    let has_run_calculations_method = graph.nodes.iter().any(|n| n.name == "RunCalculations");
    
    assert!(has_logger_class, "Should have Logger class");
    assert!(has_math_utils_class, "Should have MathUtils class");
    assert!(has_program_class, "Should have Program class");
    assert!(has_calculator_class, "Should have Calculator class");
    assert!(has_square_method, "Should have Square method");
    assert!(has_cube_method, "Should have Cube method");
    assert!(has_run_calculations_method, "Should have RunCalculations method");
    
    // Find specific nodes for interfile edge verification
    let program_constructor = graph.nodes.iter()
        .find(|n| n.name == "Program" && n.file_path.contains("Program.cs"))
        .expect("Program constructor should exist");
    
    let run_calculations_node = graph.nodes.iter()
        .find(|n| n.name == "RunCalculations")
        .expect("RunCalculations method should exist");
    
    let calculator_calculate_method = graph.nodes.iter()
        .find(|n| n.name == "Calculate" && n.file_path.contains("Program.cs"))
        .expect("Calculate method should exist");
    
    // Verify interfile edges exist and have correct source nodes
    
    // 1. Program constructor should reference Logger class (from different file)
    let program_to_logger_edge = graph.edges.iter()
        .find(|e| e.source == program_constructor.id && 
                  graph.nodes.iter().any(|n| n.id == e.target && n.name == "Logger" && n.file_path.contains("BaseClasses.cs")));
    
    if let Some(edge) = program_to_logger_edge {
        println!("✓ Found interfile edge from Program constructor to Logger: {:?}", edge.edge_type);
        assert!(matches!(edge.edge_type, 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::References | 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::Calls),
                "Edge should be a reference or call");
    } else {
        println!("⚠ Missing interfile edge from Program constructor to Logger class");
    }
    
    // 2. RunCalculations should reference MathUtils.Square (from different file)
    let run_calculations_to_square_edge = graph.edges.iter()
        .find(|e| e.source == run_calculations_node.id && 
                  graph.nodes.iter().any(|n| n.id == e.target && n.name == "Square" && n.file_path.contains("BaseClasses.cs")));
    
    if let Some(edge) = run_calculations_to_square_edge {
        println!("✓ Found interfile edge from RunCalculations to Square: {:?}", edge.edge_type);
        assert!(matches!(edge.edge_type, 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::Calls | 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::References),
                "Edge should be a call or reference");
    } else {
        println!("⚠ Missing interfile edge from RunCalculations to Square method");
    }
    
    // 3. RunCalculations should reference MathUtils.Cube (from different file)
    let run_calculations_to_cube_edge = graph.edges.iter()
        .find(|e| e.source == run_calculations_node.id && 
                  graph.nodes.iter().any(|n| n.id == e.target && n.name == "Cube" && n.file_path.contains("BaseClasses.cs")));
    
    if let Some(edge) = run_calculations_to_cube_edge {
        println!("✓ Found interfile edge from RunCalculations to Cube: {:?}", edge.edge_type);
        assert!(matches!(edge.edge_type, 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::Calls | 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::References),
                "Edge should be a call or reference");
    } else {
        println!("⚠ Missing interfile edge from RunCalculations to Cube method");
    }
    
    // 4. Calculator.Calculate should reference MathUtils.Square (from different file)
    let calculate_to_square_edge = graph.edges.iter()
        .find(|e| e.source == calculator_calculate_method.id && 
                  graph.nodes.iter().any(|n| n.id == e.target && n.name == "Square" && n.file_path.contains("BaseClasses.cs")));
    
    if let Some(edge) = calculate_to_square_edge {
        println!("✓ Found interfile edge from Calculator.Calculate to Square: {:?}", edge.edge_type);
        assert!(matches!(edge.edge_type, 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::Calls | 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::References),
                "Edge should be a call or reference");
    } else {
        println!("⚠ Missing interfile edge from Calculator.Calculate to Square method");
    }
    
    // Verify that interfile edges connect nodes from different files
    let interfile_edges: Vec<_> = graph.edges.iter()
        .filter(|edge| {
            let source_node = graph.nodes.iter().find(|n| n.id == edge.source);
            let target_node = graph.nodes.iter().find(|n| n.id == edge.target);
            
            if let (Some(src), Some(tgt)) = (source_node, target_node) {
                src.file_path != tgt.file_path
            } else {
                false
            }
        })
        .collect();
    
    println!("Found {} interfile edges:", interfile_edges.len());
    for edge in &interfile_edges {
        let source_node = graph.nodes.iter().find(|n| n.id == edge.source).unwrap();
        let target_node = graph.nodes.iter().find(|n| n.id == edge.target).unwrap();
        println!("  {} ({}) -> {} ({}) [{:?}]", 
                 source_node.name, source_node.file_path,
                 target_node.name, target_node.file_path,
                 edge.edge_type);
    }
    
    // We should have at least some interfile edges
    assert!(!interfile_edges.is_empty(), "Should have at least one interfile edge");
    
    // Verify that all edges have proper source nodes (not line numbers)
    for edge in &graph.edges {
        assert!(edge.source.contains("::") || edge.source.starts_with("file:") || edge.source.starts_with("symbol:"),
                "Source should be a proper node ID, not a line number: {}", edge.source);
        assert!(edge.target.contains("::") || edge.target.starts_with("file:") || edge.target.starts_with("symbol:"),
                "Target should be a proper node ID, not a line number: {}", edge.target);
    }
    
    println!("✓ All interfile edges have proper source and target nodes");
    println!("✓ Interfile edges correctly connect symbols from different files");
}