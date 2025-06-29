use codex_core::code_analysis::tools::{
    handle_analyze_code,
    handle_find_symbol_references,
    handle_find_symbol_definitions,
    handle_get_symbol_subgraph,
};
use codex_core::code_analysis::graph_manager::initialize_graph_async;
use serde_json::json;
use std::path::Path;

#[tokio::test]
async fn test_cpp_comprehensive_multifile_analysis() {
    let test_suite_path = Path::new("../test_files/cpp_test_suite");
    
    // Verify all test files exist
    let expected_files = [
        "basic_class.h",
        "basic_class.cpp",
        "models/user.h",
        "models/order.h", 
        "models/product.h",
        "services/user_service.h",
        "data/repository.h",
        "utils/helpers.h",
        "main.cpp",
    ];
    
    for file in &expected_files {
        let file_path = test_suite_path.join(file);
        assert!(file_path.exists(), "Test file {} does not exist", file_path.display());
    }
    
    println!("=== INITIALIZING C++ CODE GRAPH FOR COMPREHENSIVE TEST ===");
    let result = initialize_graph_async(test_suite_path).await;
    assert!(result.is_ok(), "Failed to initialize code graph: {:?}", result.err());
    
    // Run comprehensive tests
    test_analyze_code_tool(test_suite_path).await;
    test_find_symbol_references_tool(test_suite_path).await;
    test_find_symbol_definitions_tool(test_suite_path).await;
    test_get_symbol_subgraph_tool(test_suite_path).await;
    test_complex_cross_file_relationships(test_suite_path).await;
    test_cpp_include_analysis(test_suite_path).await;
    test_cpp_inheritance_analysis(test_suite_path).await;
    test_cpp_template_analysis(test_suite_path).await;
}

async fn test_analyze_code_tool(base_path: &Path) {
    println!("\n=== TESTING ANALYZE CODE TOOL ===");
    
    // Test analyzing the main application file
    let main_file = base_path.join("main.cpp");
    let input = json!({
        "file_path": main_file.to_str().unwrap()
    });
    
    let result = handle_analyze_code(input);
    assert!(result.is_ok(), "Failed to analyze main.cpp: {:?}", result.err());
    
    let analysis = result.unwrap();
    let symbols = analysis.get("symbols").expect("No symbols found").as_array().unwrap();
    
    println!("Found {} symbols in main.cpp", symbols.len());
    
    // Check for expected symbols
    let expected_symbols = [
        ("createSampleData", "function"),
        ("demonstrateFunctionality", "function"),
        ("ApplicationManager", "class"),
        ("main", "function")
    ];
    
    for (symbol_name, symbol_type) in &expected_symbols {
        let found = symbols.iter().any(|s| {
            s.get("name").and_then(|n| n.as_str()) == Some(*symbol_name) &&
            s.get("symbol_type").and_then(|t| t.as_str()).map(|t| t.to_lowercase().contains(symbol_type)).unwrap_or(false)
        });
        assert!(found, "Expected {} {} not found in main.cpp", symbol_type, symbol_name);
    }
    
    // Test analyzing a header file
    let user_header_file = base_path.join("models/user.h");
    let input = json!({
        "file_path": user_header_file.to_str().unwrap()
    });
    
    let result = handle_analyze_code(input);
    assert!(result.is_ok(), "Failed to analyze user.h: {:?}", result.err());
    
    let analysis = result.unwrap();
    let symbols = analysis.get("symbols").expect("No symbols found").as_array().unwrap();
    
    println!("Found {} symbols in models/user.h", symbols.len());
    
    // Check for User and AdminUser classes
    let has_user_class = symbols.iter().any(|s| {
        s.get("name").and_then(|n| n.as_str()) == Some("User") &&
        s.get("symbol_type").and_then(|t| t.as_str()).map(|t| t.to_lowercase().contains("class")).unwrap_or(false)
    });
    
    let has_admin_user_class = symbols.iter().any(|s| {
        s.get("name").and_then(|n| n.as_str()) == Some("AdminUser") &&
        s.get("symbol_type").and_then(|t| t.as_str()).map(|t| t.to_lowercase().contains("class")).unwrap_or(false)
    });
    
    assert!(has_user_class, "User class not found in user.h");
    assert!(has_admin_user_class, "AdminUser class not found in user.h");
}

async fn test_find_symbol_references_tool(base_path: &Path) {
    println!("\n=== TESTING FIND SYMBOL REFERENCES TOOL ===");
    
    // Test finding references to User class
    let input = json!({
        "symbol_name": "User",
        "file_path": base_path.join("models/user.h").to_str().unwrap()
    });
    
    let result = handle_find_symbol_references(input);
    assert!(result.is_ok(), "Failed to find User references: {:?}", result.err());
    
    let references = result.unwrap();
    let refs_array = references.get("references").expect("No references found").as_array().unwrap();
    
    println!("Found {} references to User", refs_array.len());
    
    // Should find references in multiple files (main.cpp, services/user_service.h, etc.)
    assert!(!refs_array.is_empty(), "Should find at least one reference to User class");
    
    // Test finding references to a template class
    let input = json!({
        "symbol_name": "Repository",
        "file_path": base_path.join("data/repository.h").to_str().unwrap()
    });
    
    let result = handle_find_symbol_references(input);
    assert!(result.is_ok(), "Failed to find Repository references: {:?}", result.err());
    
    let references = result.unwrap();
    println!("Found references to Repository template: {:?}", references);
}

async fn test_find_symbol_definitions_tool(_base_path: &Path) {
    println!("\n=== TESTING FIND SYMBOL DEFINITIONS TOOL ===");
    
    // Test finding definition of Product class
    let input = json!({
        "symbol_name": "Product"
    });
    
    let result = handle_find_symbol_definitions(input);
    assert!(result.is_ok(), "Failed to find Product definitions: {:?}", result.err());
    
    let definitions = result.unwrap();
    let defs_array = definitions.get("definitions").expect("No definitions found").as_array().unwrap();
    
    println!("Found {} definitions for Product", defs_array.len());
    
    // Should find Product class definition in models/product.h
    assert!(!defs_array.is_empty(), "Should find at least one definition for Product");
    
    let product_def = &defs_array[0];
    let file_path = product_def.get("file_path").and_then(|p| p.as_str()).unwrap();
    assert!(file_path.contains("models/product.h"), "Product should be defined in models/product.h");
}

async fn test_get_symbol_subgraph_tool(_base_path: &Path) {
    println!("\n=== TESTING GET SYMBOL SUBGRAPH TOOL ===");
    
    // Test getting subgraph for UserService class
    let input = json!({
        "symbol_name": "UserService",
        "max_depth": 2
    });
    
    let result = handle_get_symbol_subgraph(input);
    assert!(result.is_ok(), "Failed to get UserService subgraph: {:?}", result.err());
    
    let subgraph = result.unwrap();
    let nodes = subgraph.get("nodes").expect("No nodes found").as_array().unwrap();
    let edges = subgraph.get("edges").expect("No edges found").as_array().unwrap();
    
    println!("UserService subgraph: {} nodes, {} edges", nodes.len(), edges.len());
    
    // Should find connections to User, Repository classes
    assert!(!nodes.is_empty(), "Should find nodes in UserService subgraph");
    assert!(!edges.is_empty(), "Should find edges in UserService subgraph");
    
    // Check for expected related symbols
    let node_names: Vec<String> = nodes.iter()
        .filter_map(|n| n.get("name").and_then(|name| name.as_str()).map(|s| s.to_string()))
        .collect();
    
    println!("Subgraph nodes: {:?}", node_names);
    
    // Should include related classes
    let expected_related = ["User", "Repository"];
    for expected in &expected_related {
        let found = node_names.iter().any(|name| name.contains(expected));
        if !found {
            println!("Warning: Expected related symbol '{}' not found in subgraph", expected);
        }
    }
}

async fn test_complex_cross_file_relationships(_base_path: &Path) {
    println!("\n=== TESTING COMPLEX CROSS-FILE RELATIONSHIPS ===");
    
    // Test finding all references to the BasicClass across files
    let input = json!({
        "symbol_name": "BasicClass"
    });
    
    let result = handle_find_symbol_references(input);
    assert!(result.is_ok(), "Failed to find BasicClass cross-file references: {:?}", result.err());
    
    let references = result.unwrap();
    let refs_array = references.get("references").expect("No references found").as_array().unwrap();
    
    println!("Found {} cross-file references to BasicClass", refs_array.len());
    
    // Collect unique file paths that reference BasicClass
    let mut referenced_files = std::collections::HashSet::new();
    for reference in refs_array {
        if let Some(file_path) = reference.get("file_path").and_then(|p| p.as_str()) {
            referenced_files.insert(file_path);
        }
    }
    
    println!("BasicClass is referenced in {} different files", referenced_files.len());
    
    // BasicClass should be referenced in multiple files:
    // - basic_class.h (declaration)
    // - basic_class.cpp (implementation)
    // - main.cpp (usage)
    assert!(referenced_files.len() >= 1, "BasicClass should be referenced in at least 1 file");
    
    // Test template relationships
    let input = json!({
        "symbol_name": "TemplateClass"
    });
    
    let result = handle_find_symbol_definitions(input);
    assert!(result.is_ok(), "Failed to find TemplateClass definition: {:?}", result.err());
    
    let definitions = result.unwrap();
    println!("TemplateClass template analysis: {:?}", definitions);
}

async fn test_cpp_include_analysis(_base_path: &Path) {
    println!("\n=== TESTING C++ INCLUDE ANALYSIS ===");
    
    // Test analyzing includes in main.cpp
    let input = json!({
        "symbol_name": "ApplicationManager"
    });
    
    let result = handle_find_symbol_references(input);
    assert!(result.is_ok(), "Failed to analyze ApplicationManager includes: {:?}", result.err());
    
    let references = result.unwrap();
    let refs_array = references.get("references").expect("No references found").as_array().unwrap();
    
    println!("Found {} references to ApplicationManager", refs_array.len());
    
    // Should find definition and usage in main.cpp
    let main_references = refs_array.iter().filter(|r| {
        r.get("file_path")
            .and_then(|p| p.as_str())
            .map(|path| path.contains("main.cpp"))
            .unwrap_or(false)
    }).count();
    
    println!("ApplicationManager references in main.cpp: {}", main_references);
    assert!(main_references > 0, "Should find ApplicationManager references in main.cpp");
}

async fn test_cpp_inheritance_analysis(_base_path: &Path) {
    println!("\n=== TESTING C++ INHERITANCE ANALYSIS ===");
    
    // Test analyzing inheritance relationship between User and AdminUser
    let input = json!({
        "symbol_name": "AdminUser",
        "max_depth": 3
    });
    
    let result = handle_get_symbol_subgraph(input);
    assert!(result.is_ok(), "Failed to get AdminUser inheritance subgraph: {:?}", result.err());
    
    let subgraph = result.unwrap();
    let nodes = subgraph.get("nodes").expect("No nodes found").as_array().unwrap();
    let edges = subgraph.get("edges").expect("No edges found").as_array().unwrap();
    
    println!("AdminUser inheritance subgraph: {} nodes, {} edges", nodes.len(), edges.len());
    
    // Should show relationship to User class
    let node_names: Vec<String> = nodes.iter()
        .filter_map(|n| n.get("name").and_then(|name| name.as_str()).map(|s| s.to_string()))
        .collect();
    
    println!("Inheritance subgraph nodes: {:?}", node_names);
    
    // Should include User class as base class
    let has_user = node_names.iter().any(|name| name == "User");
    if !has_user {
        println!("Warning: User base class not found in AdminUser subgraph");
    }
    
    // Test Product inheritance (Product -> DigitalProduct, PhysicalProduct)
    let input = json!({
        "symbol_name": "DigitalProduct",
        "max_depth": 2
    });
    
    let result = handle_get_symbol_subgraph(input);
    assert!(result.is_ok(), "Failed to get DigitalProduct inheritance subgraph: {:?}", result.err());
    
    let subgraph = result.unwrap();
    let nodes = subgraph.get("nodes").expect("No nodes found").as_array().unwrap();
    
    println!("DigitalProduct inheritance subgraph: {} nodes", nodes.len());
    
    let node_names: Vec<String> = nodes.iter()
        .filter_map(|n| n.get("name").and_then(|name| name.as_str()).map(|s| s.to_string()))
        .collect();
    
    println!("DigitalProduct subgraph nodes: {:?}", node_names);
}

async fn test_cpp_template_analysis(_base_path: &Path) {
    println!("\n=== TESTING C++ TEMPLATE ANALYSIS ===");
    
    // Test analyzing template classes
    let input = json!({
        "symbol_name": "Repository",
        "max_depth": 2
    });
    
    let result = handle_get_symbol_subgraph(input);
    assert!(result.is_ok(), "Failed to get Repository template subgraph: {:?}", result.err());
    
    let subgraph = result.unwrap();
    let nodes = subgraph.get("nodes").expect("No nodes found").as_array().unwrap();
    let edges = subgraph.get("edges").expect("No edges found").as_array().unwrap();
    
    println!("Repository template subgraph: {} nodes, {} edges", nodes.len(), edges.len());
    
    // Should show relationships to template implementations
    let node_names: Vec<String> = nodes.iter()
        .filter_map(|n| n.get("name").and_then(|name| name.as_str()).map(|s| s.to_string()))
        .collect();
    
    println!("Template subgraph nodes: {:?}", node_names);
    
    // Should include template implementations
    let expected_related = ["InMemoryRepository", "CachedRepository"];
    for expected in &expected_related {
        let found = node_names.iter().any(|name| name.contains(expected));
        if !found {
            println!("Warning: Expected template implementation '{}' not found in subgraph", expected);
        }
    }
    
    // Test template function analysis
    let input = json!({
        "symbol_name": "maxValue"
    });
    
    let result = handle_find_symbol_definitions(input);
    if result.is_ok() {
        let definitions = result.unwrap();
        println!("Found template function definitions: {:?}", definitions);
    }
}

#[tokio::test]
async fn test_cpp_namespace_analysis() {
    let test_suite_path = Path::new("../test_files/cpp_test_suite");
    
    println!("=== TESTING C++ NAMESPACE ANALYSIS ===");
    
    // Initialize graph for namespace analysis
    let result = initialize_graph_async(test_suite_path).await;
    assert!(result.is_ok(), "Failed to initialize code graph for namespace analysis: {:?}", result.err());
    
    // Test analyzing namespace usage
    let input = json!({
        "symbol_name": "TestNamespace"
    });
    
    let result = handle_find_symbol_references(input);
    if result.is_ok() {
        let references = result.unwrap();
        println!("Found references to 'TestNamespace': {:?}", references);
    }
    
    // Test Models namespace
    let input = json!({
        "symbol_name": "Models"
    });
    
    let result = handle_find_symbol_references(input);
    if result.is_ok() {
        let references = result.unwrap();
        println!("Found references to 'Models' namespace: {:?}", references);
    }
}

#[tokio::test]
async fn test_cpp_operator_overloading_analysis() {
    let test_suite_path = Path::new("../test_files/cpp_test_suite");
    
    println!("=== TESTING C++ OPERATOR OVERLOADING ANALYSIS ===");
    
    // Initialize graph
    let result = initialize_graph_async(test_suite_path).await;
    assert!(result.is_ok(), "Failed to initialize code graph: {:?}", result.err());
    
    // Test analyzing operator overloads
    let input = json!({
        "symbol_name": "operator="
    });
    
    let result = handle_find_symbol_definitions(input);
    if result.is_ok() {
        let definitions = result.unwrap();
        println!("Found operator= definitions: {:?}", definitions);
    }
    
    // Test other operators
    let operators = ["operator+", "operator==", "operator!="];
    for op in &operators {
        let input = json!({
            "symbol_name": op
        });
        
        let result = handle_find_symbol_definitions(input);
        if result.is_ok() {
            let definitions = result.unwrap();
            println!("Found {} definitions: {:?}", op, definitions);
        }
    }
}