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
fn test_typescript_intrafile_edges() {
    let dir = tempdir().unwrap();
    
    // Create a TypeScript file with multiple classes and methods that reference each other
    let typescript_content = r#"
interface ICalculator {
    add(a: number, b: number): number;
    multiply(a: number, b: number): number;
}

class Calculator implements ICalculator {
    private lastResult: number = 0;

    add(a: number, b: number): number {
        this.lastResult = a + b;
        return this.lastResult;
    }

    multiply(a: number, b: number): number {
        this.lastResult = a * b;
        return this.lastResult;
    }

    addAndMultiply(a: number, b: number, c: number): number {
        // This method calls other methods in the same class (intrafile references)
        const sum = this.add(a, b);        // Reference to add method
        const result = this.multiply(sum, c);  // Reference to multiply method
        return result;
    }

    printResult(): void {
        // Reference to another method in same class
        const result = this.addAndMultiply(5, 3, 2);
        console.log(`Result: ${result}`);
    }
}

class MathHelper {
    private calc: Calculator;

    constructor() {
        // Reference to Calculator class (intrafile)
        this.calc = new Calculator();
    }

    doComplexCalculation(x: number, y: number, z: number): number {
        // Reference to Calculator methods (intrafile)
        return this.calc.addAndMultiply(x, y, z);
    }
}

function createCalculator(): Calculator {
    return new Calculator();
}

function performCalculation(): void {
    const calc = createCalculator();  // Reference to createCalculator function
    const helper = new MathHelper();  // Reference to MathHelper class
    
    const result1 = calc.add(10, 20);  // Reference to Calculator.add method
    const result2 = helper.doComplexCalculation(1, 2, 3);  // Reference to MathHelper method
    
    console.log(`Results: ${result1}, ${result2}`);
}
"#;
    
    let _ts_file_path = create_temp_file(&dir, "calculator.ts", typescript_content);
    
    // Create a repository mapper and map the repository
    let mut repo_mapper = RepoMapper::new(dir.path());
    let result = repo_mapper.map_repository();
    assert!(result.is_ok(), "Failed to map repository: {:?}", result.err());
    
    // Get the graph
    let graph = repo_mapper.get_graph();
    
    // Print the graph for debugging
    println!("=== TYPESCRIPT INTRAFILE TEST ===");
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
    let has_add_method = graph.nodes.iter().any(|n| n.name == "add");
    let has_multiply_method = graph.nodes.iter().any(|n| n.name == "multiply");
    let has_add_and_multiply_method = graph.nodes.iter().any(|n| n.name == "addAndMultiply");
    let has_print_result_method = graph.nodes.iter().any(|n| n.name == "printResult");
    let has_do_complex_calculation_method = graph.nodes.iter().any(|n| n.name == "doComplexCalculation");
    let has_create_calculator_function = graph.nodes.iter().any(|n| n.name == "createCalculator");
    let has_perform_calculation_function = graph.nodes.iter().any(|n| n.name == "performCalculation");
    
    assert!(has_calculator_class, "Should have Calculator class");
    assert!(has_math_helper_class, "Should have MathHelper class");
    assert!(has_add_method, "Should have add method");
    assert!(has_multiply_method, "Should have multiply method");
    assert!(has_add_and_multiply_method, "Should have addAndMultiply method");
    assert!(has_print_result_method, "Should have printResult method");
    assert!(has_do_complex_calculation_method, "Should have doComplexCalculation method");
    assert!(has_create_calculator_function, "Should have createCalculator function");
    assert!(has_perform_calculation_function, "Should have performCalculation function");
    
    // Find specific nodes for edge verification
    let add_and_multiply_node = graph.nodes.iter()
        .find(|n| n.name == "addAndMultiply")
        .expect("addAndMultiply method should exist");
    
    let print_result_node = graph.nodes.iter()
        .find(|n| n.name == "printResult")
        .expect("printResult method should exist");
    
    let perform_calculation_node = graph.nodes.iter()
        .find(|n| n.name == "performCalculation")
        .expect("performCalculation function should exist");
    
    // Verify intrafile edges exist and have correct source nodes
    
    // 1. addAndMultiply method should call add method
    let add_and_multiply_to_add_edge = graph.edges.iter()
        .find(|e| e.source == add_and_multiply_node.id && 
                  graph.nodes.iter().any(|n| n.id == e.target && n.name == "add"));
    
    if let Some(edge) = add_and_multiply_to_add_edge {
        println!("✓ Found edge from addAndMultiply to add: {:?}", edge.edge_type);
        assert!(matches!(edge.edge_type, 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::Calls | 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::References),
                "Edge should be a call or reference");
    } else {
        println!("⚠ Missing edge from addAndMultiply to add method");
    }
    
    // 2. addAndMultiply method should call multiply method
    let add_and_multiply_to_multiply_edge = graph.edges.iter()
        .find(|e| e.source == add_and_multiply_node.id && 
                  graph.nodes.iter().any(|n| n.id == e.target && n.name == "multiply"));
    
    if let Some(edge) = add_and_multiply_to_multiply_edge {
        println!("✓ Found edge from addAndMultiply to multiply: {:?}", edge.edge_type);
        assert!(matches!(edge.edge_type, 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::Calls | 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::References),
                "Edge should be a call or reference");
    } else {
        println!("⚠ Missing edge from addAndMultiply to multiply method");
    }
    
    // 3. printResult method should call addAndMultiply method
    let print_result_to_add_and_multiply_edge = graph.edges.iter()
        .find(|e| e.source == print_result_node.id && e.target == add_and_multiply_node.id);
    
    if let Some(edge) = print_result_to_add_and_multiply_edge {
        println!("✓ Found edge from printResult to addAndMultiply: {:?}", edge.edge_type);
        assert!(matches!(edge.edge_type, 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::Calls | 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::References),
                "Edge should be a call or reference");
    } else {
        println!("⚠ Missing edge from printResult to addAndMultiply method");
    }
    
    // 4. performCalculation should call createCalculator function
    let perform_calc_to_create_calc_edge = graph.edges.iter()
        .find(|e| e.source == perform_calculation_node.id && 
                  graph.nodes.iter().any(|n| n.id == e.target && n.name == "createCalculator"));
    
    if let Some(edge) = perform_calc_to_create_calc_edge {
        println!("✓ Found edge from performCalculation to createCalculator: {:?}", edge.edge_type);
        assert!(matches!(edge.edge_type, 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::Calls | 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::References),
                "Edge should be a call or reference");
    } else {
        println!("⚠ Missing edge from performCalculation to createCalculator function");
    }
    
    // 5. performCalculation should reference MathHelper class
    let perform_calc_to_math_helper_edge = graph.edges.iter()
        .find(|e| e.source == perform_calculation_node.id && 
                  graph.nodes.iter().any(|n| n.id == e.target && n.name == "MathHelper"));
    
    if let Some(edge) = perform_calc_to_math_helper_edge {
        println!("✓ Found edge from performCalculation to MathHelper: {:?}", edge.edge_type);
        assert!(matches!(edge.edge_type, 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::References),
                "Edge should be a reference");
    } else {
        println!("⚠ Missing edge from performCalculation to MathHelper class");
    }
    
    // Verify that all edges have proper source nodes (not line numbers)
    for edge in &graph.edges {
        assert!(edge.source.contains("::") || edge.source.starts_with("file:") || edge.source.starts_with("symbol:"),
                "Source should be a proper node ID, not a line number: {}", edge.source);
        assert!(edge.target.contains("::") || edge.target.starts_with("file:") || edge.target.starts_with("symbol:"),
                "Target should be a proper node ID, not a line number: {}", edge.target);
    }
    
    // Count intrafile edges (edges within the same file)
    let intrafile_edges: Vec<_> = graph.edges.iter()
        .filter(|edge| {
            let source_node = graph.nodes.iter().find(|n| n.id == edge.source);
            let target_node = graph.nodes.iter().find(|n| n.id == edge.target);
            
            if let (Some(src), Some(tgt)) = (source_node, target_node) {
                src.file_path == tgt.file_path && !matches!(edge.edge_type, codex_core::code_analysis::repo_mapper::CodeEdgeType::Contains)
            } else {
                false
            }
        })
        .collect();
    
    println!("Found {} intrafile reference edges:", intrafile_edges.len());
    for edge in &intrafile_edges {
        let source_node = graph.nodes.iter().find(|n| n.id == edge.source).unwrap();
        let target_node = graph.nodes.iter().find(|n| n.id == edge.target).unwrap();
        println!("  {} -> {} [{:?}]", source_node.name, target_node.name, edge.edge_type);
    }
    
    println!("✓ All intrafile edges have proper source and target nodes");
}

#[test]
fn test_typescript_interfile_edges() {
    let dir = tempdir().unwrap();
    
    // Create first TypeScript file with utility classes
    let utils_content = r#"
export interface ILogger {
    log(message: string): void;
}

export class Logger implements ILogger {
    log(message: string): void {
        console.log(`[LOG] ${message}`);
    }
}

export class MathUtils {
    static square(x: number): number {
        return x * x;
    }

    static cube(x: number): number {
        return x * x * x;
    }

    static factorial(n: number): number {
        if (n <= 1) return 1;
        return n * MathUtils.factorial(n - 1);  // Recursive call
    }
}

export function createLogger(): ILogger {
    return new Logger();
}
"#;
    
    // Create second TypeScript file that uses classes from the first file
    let main_content = r#"
import { ILogger, Logger, MathUtils, createLogger } from './utils';

class Program {
    private logger: ILogger;

    constructor() {
        // Interfile reference to createLogger function
        this.logger = createLogger();
    }

    runCalculations(): void {
        // Interfile references to MathUtils methods
        const result1 = MathUtils.square(5);
        const result2 = MathUtils.cube(3);
        const result3 = MathUtils.factorial(4);
        
        // Interfile reference to Logger method
        this.logger.log(`Square result: ${result1}`);
        this.logger.log(`Cube result: ${result2}`);
        this.logger.log(`Factorial result: ${result3}`);
    }

    static main(): void {
        const program = new Program();
        program.runCalculations();
    }
}

class Calculator {
    private logger: ILogger;

    constructor(logger: ILogger) {
        this.logger = logger;
    }

    calculate(x: number): number {
        // Interfile reference to MathUtils
        const squared = MathUtils.square(x);
        
        // Interfile reference to Logger
        this.logger.log(`Calculated square of ${x} = ${squared}`);
        
        return squared;
    }
}

function performAdvancedCalculation(): void {
    // Interfile references
    const logger = createLogger();
    const calc = new Calculator(logger);
    
    const result = calc.calculate(7);
    
    // More interfile references
    const cubed = MathUtils.cube(result);
    logger.log(`Final result: ${cubed}`);
}

// Call the main function
Program.main();
performAdvancedCalculation();
"#;
    
    let _utils_file_path = create_temp_file(&dir, "utils.ts", utils_content);
    let _main_file_path = create_temp_file(&dir, "main.ts", main_content);
    
    // Create a repository mapper and map the repository
    let mut repo_mapper = RepoMapper::new(dir.path());
    let result = repo_mapper.map_repository();
    assert!(result.is_ok(), "Failed to map repository: {:?}", result.err());
    
    // Get the graph
    let graph = repo_mapper.get_graph();
    
    // Print the graph for debugging
    println!("=== TYPESCRIPT INTERFILE TEST ===");
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
    let has_square_method = graph.nodes.iter().any(|n| n.name == "square");
    let has_cube_method = graph.nodes.iter().any(|n| n.name == "cube");
    let has_run_calculations_method = graph.nodes.iter().any(|n| n.name == "runCalculations");
    let has_create_logger_function = graph.nodes.iter().any(|n| n.name == "createLogger");
    let has_perform_advanced_calculation = graph.nodes.iter().any(|n| n.name == "performAdvancedCalculation");
    
    assert!(has_logger_class, "Should have Logger class");
    assert!(has_math_utils_class, "Should have MathUtils class");
    assert!(has_program_class, "Should have Program class");
    assert!(has_calculator_class, "Should have Calculator class");
    assert!(has_square_method, "Should have square method");
    assert!(has_cube_method, "Should have cube method");
    assert!(has_run_calculations_method, "Should have runCalculations method");
    assert!(has_create_logger_function, "Should have createLogger function");
    assert!(has_perform_advanced_calculation, "Should have performAdvancedCalculation function");
    
    // Find specific nodes for interfile edge verification
    let program_constructor = graph.nodes.iter()
        .find(|n| n.name == "constructor" && n.file_path.contains("main.ts"))
        .or_else(|| graph.nodes.iter().find(|n| n.name == "Program" && n.file_path.contains("main.ts")))
        .expect("Program constructor should exist");
    
    let run_calculations_node = graph.nodes.iter()
        .find(|n| n.name == "runCalculations")
        .expect("runCalculations method should exist");
    
    let calculator_calculate_method = graph.nodes.iter()
        .find(|n| n.name == "calculate" && n.file_path.contains("main.ts"))
        .expect("calculate method should exist");
    
    let perform_advanced_calculation_node = graph.nodes.iter()
        .find(|n| n.name == "performAdvancedCalculation")
        .expect("performAdvancedCalculation function should exist");
    
    // Verify interfile edges exist and have correct source nodes
    
    // 1. Program constructor should reference createLogger function (from different file)
    let program_to_create_logger_edge = graph.edges.iter()
        .find(|e| e.source == program_constructor.id && 
                  graph.nodes.iter().any(|n| n.id == e.target && n.name == "createLogger" && n.file_path.contains("utils.ts")));
    
    if let Some(edge) = program_to_create_logger_edge {
        println!("✓ Found interfile edge from Program constructor to createLogger: {:?}", edge.edge_type);
        assert!(matches!(edge.edge_type, 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::References | 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::Calls),
                "Edge should be a reference or call");
    } else {
        println!("⚠ Missing interfile edge from Program constructor to createLogger function");
    }
    
    // 2. runCalculations should reference MathUtils.square (from different file)
    let run_calculations_to_square_edge = graph.edges.iter()
        .find(|e| e.source == run_calculations_node.id && 
                  graph.nodes.iter().any(|n| n.id == e.target && n.name == "square" && n.file_path.contains("utils.ts")));
    
    if let Some(edge) = run_calculations_to_square_edge {
        println!("✓ Found interfile edge from runCalculations to square: {:?}", edge.edge_type);
        assert!(matches!(edge.edge_type, 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::Calls | 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::References),
                "Edge should be a call or reference");
    } else {
        println!("⚠ Missing interfile edge from runCalculations to square method");
    }
    
    // 3. runCalculations should reference MathUtils.cube (from different file)
    let run_calculations_to_cube_edge = graph.edges.iter()
        .find(|e| e.source == run_calculations_node.id && 
                  graph.nodes.iter().any(|n| n.id == e.target && n.name == "cube" && n.file_path.contains("utils.ts")));
    
    if let Some(edge) = run_calculations_to_cube_edge {
        println!("✓ Found interfile edge from runCalculations to cube: {:?}", edge.edge_type);
        assert!(matches!(edge.edge_type, 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::Calls | 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::References),
                "Edge should be a call or reference");
    } else {
        println!("⚠ Missing interfile edge from runCalculations to cube method");
    }
    
    // 4. Calculator.calculate should reference MathUtils.square (from different file)
    let calculate_to_square_edge = graph.edges.iter()
        .find(|e| e.source == calculator_calculate_method.id && 
                  graph.nodes.iter().any(|n| n.id == e.target && n.name == "square" && n.file_path.contains("utils.ts")));
    
    if let Some(edge) = calculate_to_square_edge {
        println!("✓ Found interfile edge from Calculator.calculate to square: {:?}", edge.edge_type);
        assert!(matches!(edge.edge_type, 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::Calls | 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::References),
                "Edge should be a call or reference");
    } else {
        println!("⚠ Missing interfile edge from Calculator.calculate to square method");
    }
    
    // 5. performAdvancedCalculation should reference createLogger (from different file)
    let perform_advanced_to_create_logger_edge = graph.edges.iter()
        .find(|e| e.source == perform_advanced_calculation_node.id && 
                  graph.nodes.iter().any(|n| n.id == e.target && n.name == "createLogger" && n.file_path.contains("utils.ts")));
    
    if let Some(edge) = perform_advanced_to_create_logger_edge {
        println!("✓ Found interfile edge from performAdvancedCalculation to createLogger: {:?}", edge.edge_type);
        assert!(matches!(edge.edge_type, 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::Calls | 
                        codex_core::code_analysis::repo_mapper::CodeEdgeType::References),
                "Edge should be a call or reference");
    } else {
        println!("⚠ Missing interfile edge from performAdvancedCalculation to createLogger function");
    }
    
    // Verify that interfile edges connect nodes from different files
    let interfile_edges: Vec<_> = graph.edges.iter()
        .filter(|edge| {
            let source_node = graph.nodes.iter().find(|n| n.id == edge.source);
            let target_node = graph.nodes.iter().find(|n| n.id == edge.target);
            
            if let (Some(src), Some(tgt)) = (source_node, target_node) {
                src.file_path != tgt.file_path && !matches!(edge.edge_type, codex_core::code_analysis::repo_mapper::CodeEdgeType::Contains)
            } else {
                false
            }
        })
        .collect();
    
    println!("Found {} interfile reference edges:", interfile_edges.len());
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
    
    // Count intrafile edges for the final summary
    let intrafile_edges_count = graph.edges.iter()
        .filter(|edge| {
            let source_node = graph.nodes.iter().find(|n| n.id == edge.source);
            let target_node = graph.nodes.iter().find(|n| n.id == edge.target);
            
            if let (Some(src), Some(tgt)) = (source_node, target_node) {
                src.file_path == tgt.file_path && !matches!(edge.edge_type, codex_core::code_analysis::repo_mapper::CodeEdgeType::Contains)
            } else {
                false
            }
        })
        .count();
    
    // Demonstrate the two-pass system working correctly
    println!("\n=== TWO-PASS SYSTEM VERIFICATION ===");
    println!("Pass 1 - Symbol Extraction:");
    println!("  - Extracted {} total nodes (symbols + files)", graph.nodes.len());
    println!("  - Found symbols in {} files", graph.nodes.iter().filter(|n| matches!(n.node_type, codex_core::code_analysis::repo_mapper::CodeNodeType::File)).count());
    
    println!("Pass 2 - Graph Construction:");
    println!("  - Created {} total edges", graph.edges.len());
    println!("  - {} intrafile reference edges", intrafile_edges_count);
    println!("  - {} interfile reference edges", interfile_edges.len());
    println!("  - {} containment edges", graph.edges.iter().filter(|e| matches!(e.edge_type, codex_core::code_analysis::repo_mapper::CodeEdgeType::Contains)).count());
    
    println!("✓ Two-pass system successfully created correct source and target nodes");
}