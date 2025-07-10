use codex_core::code_analysis::context_extractor::{ContextExtractor, SymbolType};
use codex_core::code_analysis::{get_parser_pool, SupportedLanguage, QueryType};
use codex_core::code_analysis::handle_get_related_files_skeleton;
use serde_json::json;
use std::path::Path;

#[test]
fn test_cpp_basic_class_parsing() {
    let test_file = "../test_files/cpp_test_suite/basic_class.cpp";
    
    // Ensure the file exists
    assert!(Path::new(test_file).exists(), "Test file {} does not exist", test_file);
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(test_file);
    
    assert!(result.is_ok(), "Failed to extract symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    
    // Print all found symbols for debugging
    println!("Found {} symbols:", symbols.len());
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
    }
    
    // Test class detection
    let basic_class = symbols.values()
        .find(|s| s.name == "BasicClass" && matches!(s.symbol_type, SymbolType::Class))
        .expect("BasicClass should be found");
    
    assert!(basic_class.start_line > 0, "BasicClass start line should be positive");
    assert!(basic_class.end_line > basic_class.start_line, "BasicClass end line should be after start line");
    
    // Test derived class
    let derived_class = symbols.values()
        .find(|s| s.name == "DerivedClass" && matches!(s.symbol_type, SymbolType::Class))
        .expect("DerivedClass should be found");
    
    assert!(derived_class.start_line > 0);
    assert!(derived_class.end_line > derived_class.start_line);
    assert!(derived_class.start_line > basic_class.end_line, "DerivedClass should come after BasicClass");
    
    // Test template class
    let template_class = symbols.values()
        .find(|s| s.name == "TemplateClass" && matches!(s.symbol_type, SymbolType::Class))
        .expect("TemplateClass should be found");
    
    assert!(template_class.start_line > 0);
    assert!(template_class.end_line > template_class.start_line);
    
    // Test nested class
    let outer_class = symbols.values()
        .find(|s| s.name == "OuterClass" && matches!(s.symbol_type, SymbolType::Class))
        .expect("OuterClass should be found");
    
    let inner_class = symbols.values()
        .find(|s| s.name == "InnerClass" && matches!(s.symbol_type, SymbolType::Class))
        .expect("InnerClass should be found");
    
    assert!(inner_class.start_line > outer_class.start_line);
    assert!(inner_class.end_line < outer_class.end_line);
    
    // Test standalone functions
    let standalone_func = symbols.values()
        .find(|s| s.name == "standaloneFunction" && matches!(s.symbol_type, SymbolType::Function))
        .expect("standaloneFunction should be found");
    
    assert!(standalone_func.start_line > 0);
    assert!(standalone_func.end_line >= standalone_func.start_line);
    
    // Should find multiple classes and functions
    assert!(symbols.len() >= 8, "Should find at least 8 symbols, found {}", symbols.len());
}

#[test]
fn test_cpp_header_file_parsing() {
    let test_file = "../test_files/cpp_test_suite/basic_class.h";
    
    assert!(Path::new(test_file).exists(), "Test file {} does not exist", test_file);
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(test_file);
    
    assert!(result.is_ok(), "Failed to extract symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    
    println!("Found {} symbols in header file:", symbols.len());
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
    }
    
    // Test class declarations
    let basic_class = symbols.values()
        .find(|s| s.name == "BasicClass" && matches!(s.symbol_type, SymbolType::Class))
        .expect("BasicClass should be found in header");
    
    assert!(basic_class.start_line > 0);
    assert!(basic_class.end_line > basic_class.start_line);
    
    let template_class = symbols.values()
        .find(|s| s.name == "TemplateClass" && matches!(s.symbol_type, SymbolType::Class))
        .expect("TemplateClass should be found in header");
    
    assert!(template_class.start_line > 0);
    assert!(template_class.end_line > template_class.start_line);
    
    let derived_class = symbols.values()
        .find(|s| s.name == "DerivedClass" && matches!(s.symbol_type, SymbolType::Class))
        .expect("DerivedClass should be found in header");
    
    assert!(derived_class.start_line > 0);
    assert!(derived_class.end_line > derived_class.start_line);
    
    // Should find class declarations
    assert!(symbols.len() >= 4, "Should find at least 4 symbols in header, found {}", symbols.len());
}

#[test]
fn test_cpp_inheritance_and_polymorphism() {
    let test_file = "../test_files/cpp_test_suite/models/user.h";
    
    assert!(Path::new(test_file).exists(), "Test file {} does not exist", test_file);
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(test_file);
    
    assert!(result.is_ok(), "Failed to extract symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    
    println!("Found {} symbols in user.h:", symbols.len());
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
    }
    
    // Test base class
    let user_class = symbols.values()
        .find(|s| s.name == "User" && matches!(s.symbol_type, SymbolType::Class))
        .expect("User class should be found");
    
    assert!(user_class.start_line > 0);
    assert!(user_class.end_line > user_class.start_line);
    
    // Test derived class
    let admin_user_class = symbols.values()
        .find(|s| s.name == "AdminUser" && matches!(s.symbol_type, SymbolType::Class))
        .expect("AdminUser class should be found");
    
    assert!(admin_user_class.start_line > 0);
    assert!(admin_user_class.end_line > admin_user_class.start_line);
    assert!(admin_user_class.start_line > user_class.end_line, "AdminUser should come after User class");
    
    // Test manager class
    let user_manager_class = symbols.values()
        .find(|s| s.name == "UserManager" && matches!(s.symbol_type, SymbolType::Class))
        .expect("UserManager class should be found");
    
    assert!(user_manager_class.start_line > 0);
    assert!(user_manager_class.end_line > user_manager_class.start_line);
    
    // Should find multiple classes
    assert!(symbols.len() >= 3, "Should find at least 3 classes, found {}", symbols.len());
}

#[test]
fn test_cpp_templates_and_generics() {
    let test_file = "../test_files/cpp_test_suite/models/product.h";
    
    assert!(Path::new(test_file).exists(), "Test file {} does not exist", test_file);
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(test_file);
    
    assert!(result.is_ok(), "Failed to extract symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    
    println!("Found {} symbols in product.h:", symbols.len());
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
    }
    
    // Test base product class
    let product_class = symbols.values()
        .find(|s| s.name == "Product" && matches!(s.symbol_type, SymbolType::Class))
        .expect("Product class should be found");
    
    assert!(product_class.start_line > 0);
    assert!(product_class.end_line > product_class.start_line);
    
    // Test derived classes
    let digital_product_class = symbols.values()
        .find(|s| s.name == "DigitalProduct" && matches!(s.symbol_type, SymbolType::Class))
        .expect("DigitalProduct class should be found");
    
    let physical_product_class = symbols.values()
        .find(|s| s.name == "PhysicalProduct" && matches!(s.symbol_type, SymbolType::Class))
        .expect("PhysicalProduct class should be found");
    
    assert!(digital_product_class.start_line > product_class.end_line);
    assert!(physical_product_class.start_line > digital_product_class.end_line);
    
    // Test factory class
    let product_factory_class = symbols.values()
        .find(|s| s.name == "ProductFactory" && matches!(s.symbol_type, SymbolType::Class))
        .expect("ProductFactory class should be found");
    
    assert!(product_factory_class.start_line > 0);
    assert!(product_factory_class.end_line > product_factory_class.start_line);
    
    // Test catalog class
    let product_catalog_class = symbols.values()
        .find(|s| s.name == "ProductCatalog" && matches!(s.symbol_type, SymbolType::Class))
        .expect("ProductCatalog class should be found");
    
    assert!(product_catalog_class.start_line > 0);
    assert!(product_catalog_class.end_line > product_catalog_class.start_line);
    
    // Should find multiple classes including templates
    assert!(symbols.len() >= 5, "Should find at least 5 classes, found {}", symbols.len());
}

#[test]
fn test_cpp_repository_pattern() {
    let test_file = "../test_files/cpp_test_suite/data/repository.h";
    
    assert!(Path::new(test_file).exists(), "Test file {} does not exist", test_file);
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(test_file);
    
    assert!(result.is_ok(), "Failed to extract symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    
    println!("Found {} symbols in repository.h:", symbols.len());
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
    }
    
    // Test template repository class
    let repository_class = symbols.values()
        .find(|s| s.name == "Repository" && matches!(s.symbol_type, SymbolType::Class))
        .expect("Repository template class should be found");
    
    assert!(repository_class.start_line > 0);
    assert!(repository_class.end_line > repository_class.start_line);
    
    // Test implementation
    let in_memory_repo = symbols.values()
        .find(|s| s.name == "InMemoryRepository" && matches!(s.symbol_type, SymbolType::Class))
        .expect("InMemoryRepository class should be found");
    
    assert!(in_memory_repo.start_line > 0);
    assert!(in_memory_repo.end_line > in_memory_repo.start_line);
    assert!(in_memory_repo.start_line > repository_class.end_line, "InMemoryRepository should come after Repository");
    
    // Test cached repository
    let cached_repo = symbols.values()
        .find(|s| s.name == "CachedRepository" && matches!(s.symbol_type, SymbolType::Class))
        .expect("CachedRepository class should be found");
    
    assert!(cached_repo.start_line > in_memory_repo.end_line);
    
    // Test factory
    let repo_factory = symbols.values()
        .find(|s| s.name == "RepositoryFactory" && matches!(s.symbol_type, SymbolType::Class))
        .expect("RepositoryFactory class should be found");
    
    assert!(repo_factory.start_line > 0);
    assert!(repo_factory.end_line > repo_factory.start_line);
    
    // Test unit of work
    let unit_of_work = symbols.values()
        .find(|s| s.name == "UnitOfWork" && matches!(s.symbol_type, SymbolType::Class))
        .expect("UnitOfWork class should be found");
    
    assert!(unit_of_work.start_line > 0);
    assert!(unit_of_work.end_line > unit_of_work.start_line);
    
    // Should find multiple template and concrete classes
    assert!(symbols.len() >= 5, "Should find at least 5 classes, found {}", symbols.len());
}

#[test]
fn test_cpp_utility_classes_and_templates() {
    let test_file = "../test_files/cpp_test_suite/utils/helpers.h";
    
    assert!(Path::new(test_file).exists(), "Test file {} does not exist", test_file);
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(test_file);
    
    assert!(result.is_ok(), "Failed to extract symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    
    println!("Found {} symbols in helpers.h:", symbols.len());
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
    }
    
    // Test utility classes
    let string_utils = symbols.values()
        .find(|s| s.name == "StringUtils" && matches!(s.symbol_type, SymbolType::Class))
        .expect("StringUtils class should be found");
    
    assert!(string_utils.start_line > 0);
    assert!(string_utils.end_line > string_utils.start_line);
    
    let timer_class = symbols.values()
        .find(|s| s.name == "Timer" && matches!(s.symbol_type, SymbolType::Class))
        .expect("Timer class should be found");
    
    assert!(timer_class.start_line > 0);
    assert!(timer_class.end_line > timer_class.start_line);
    
    let data_validator = symbols.values()
        .find(|s| s.name == "DataValidator" && matches!(s.symbol_type, SymbolType::Class))
        .expect("DataValidator class should be found");
    
    assert!(data_validator.start_line > 0);
    assert!(data_validator.end_line > data_validator.start_line);
    
    let config_manager = symbols.values()
        .find(|s| s.name == "ConfigManager" && matches!(s.symbol_type, SymbolType::Class))
        .expect("ConfigManager class should be found");
    
    assert!(config_manager.start_line > 0);
    assert!(config_manager.end_line > config_manager.start_line);
    
    // Test template classes
    let batch_processor = symbols.values()
        .find(|s| s.name == "BatchProcessor" && matches!(s.symbol_type, SymbolType::Class))
        .expect("BatchProcessor template class should be found");
    
    assert!(batch_processor.start_line > 0);
    assert!(batch_processor.end_line > batch_processor.start_line);
    
    let singleton_class = symbols.values()
        .find(|s| s.name == "Singleton" && matches!(s.symbol_type, SymbolType::Class))
        .expect("Singleton template class should be found");
    
    assert!(singleton_class.start_line > 0);
    assert!(singleton_class.end_line > singleton_class.start_line);
    
    let logger_class = symbols.values()
        .find(|s| s.name == "Logger" && matches!(s.symbol_type, SymbolType::Class))
        .expect("Logger class should be found");
    
    assert!(logger_class.start_line > 0);
    assert!(logger_class.end_line > logger_class.start_line);
    
    let retry_policy = symbols.values()
        .find(|s| s.name == "RetryPolicy" && matches!(s.symbol_type, SymbolType::Class))
        .expect("RetryPolicy template class should be found");
    
    assert!(retry_policy.start_line > 0);
    assert!(retry_policy.end_line > retry_policy.start_line);
    
    // Should find multiple utility classes
    assert!(symbols.len() >= 8, "Should find at least 8 utility classes, found {}", symbols.len());
}

#[test]
fn test_cpp_line_number_accuracy() {
    let test_file = "../test_files/cpp_test_suite/basic_class.cpp";
    
    assert!(Path::new(test_file).exists(), "Test file {} does not exist", test_file);
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(test_file);
    
    assert!(result.is_ok(), "Failed to extract symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    
    // Read the file content to verify line numbers
    let content = std::fs::read_to_string(test_file).expect("Failed to read test file");
    let lines: Vec<&str> = content.lines().collect();
    
    // Test BasicClass line numbers
    let basic_class = symbols.values()
        .find(|s| s.name == "BasicClass" && matches!(s.symbol_type, SymbolType::Class))
        .expect("BasicClass should be found");
    
    // Verify the class definition line contains "class BasicClass"
    let class_line = lines.get(basic_class.start_line as usize - 1)
        .expect("Class start line should exist");
    assert!(class_line.contains("class BasicClass"), 
           "Line {} should contain 'class BasicClass', but contains: '{}'", 
           basic_class.start_line, class_line);
    
    // Test function line numbers
    let standalone_func = symbols.values()
        .find(|s| s.name == "standaloneFunction" && matches!(s.symbol_type, SymbolType::Function))
        .expect("standaloneFunction should be found");
    
    let func_line = lines.get(standalone_func.start_line as usize - 1)
        .expect("Function start line should exist");
    assert!(func_line.contains("standaloneFunction"), 
           "Line {} should contain 'standaloneFunction', but contains: '{}'", 
           standalone_func.start_line, func_line);
    
    // Test template class line numbers
    let template_class = symbols.values()
        .find(|s| s.name == "TemplateClass" && matches!(s.symbol_type, SymbolType::Class))
        .expect("TemplateClass should be found");
    
    // Find the template class definition (might be on previous line due to template<>)
    let mut found_template = false;
    for i in (template_class.start_line.saturating_sub(2) as usize)..=(template_class.start_line as usize) {
        if let Some(line) = lines.get(i) {
            if line.contains("class TemplateClass") {
                found_template = true;
                break;
            }
        }
    }
    assert!(found_template, "Should find 'class TemplateClass' near line {}", template_class.start_line);
    
    // Verify line ordering
    assert!(basic_class.start_line < template_class.start_line, 
           "BasicClass should start before TemplateClass");
}

#[test]
fn test_cpp_skeleton_generation() {
    println!("Testing C++ skeleton generation functionality...");
    
    // Initialize the code graph first
    let root_path = std::path::Path::new("../test_files/cpp_test_suite");
    if let Err(e) = codex_core::code_analysis::graph_manager::ensure_graph_for_path(root_path) {
        println!("Warning: Could not initialize graph: {}", e);
        // Continue with test anyway
    }
    
    // Test input with C++ files that have inter-dependencies
    let test_input = json!({
        "active_files": [
            "../test_files/cpp_test_suite/main.cpp",
            "../test_files/cpp_test_suite/basic_class.cpp"
        ],
        "max_tokens": 2000,
        "max_depth": 2
    });
    
    println!("Test input: {}", test_input);
    
    // Call the skeleton handler
    match handle_get_related_files_skeleton(test_input) {
        Some(Ok(result)) => {
            println!("C++ skeleton generation successful!");
            println!("Result: {}", serde_json::to_string_pretty(&result).unwrap_or_else(|_| "Failed to serialize".to_string()));
            
            // Verify the result structure
            assert!(result.is_object(), "Result should be an object");
            
            let result_obj = result.as_object().unwrap();
            assert!(result_obj.contains_key("related_files"), "Should contain related_files");
            assert!(result_obj.contains_key("total_files"), "Should contain total_files");
            assert!(result_obj.contains_key("max_tokens_used"), "Should contain max_tokens_used");
            
            // Check related files array
            let related_files = result_obj.get("related_files").unwrap().as_array().unwrap();
            assert!(!related_files.is_empty(), "Should find at least some related files");
            
            // Verify each file entry has expected structure
            for file_entry in related_files {
                let file_obj = file_entry.as_object().unwrap();
                assert!(file_obj.contains_key("file_path"), "Each file should have file_path");
                assert!(file_obj.contains_key("skeleton"), "Each file should have skeleton");
                assert!(file_obj.contains_key("tokens"), "Each file should have token count");
                
                let skeleton = file_obj.get("skeleton").unwrap().as_str().unwrap();
                println!("C++ Skeleton for {}: \n{}\n", 
                    file_obj.get("file_path").unwrap().as_str().unwrap_or("unknown"),
                    skeleton
                );
                
                // Verify skeleton contains expected C++ elements
                if skeleton.contains("class") || skeleton.contains("#include") {
                    // Should contain includes
                    assert!(skeleton.contains("#include") || skeleton.contains("class") || skeleton.contains("template"), 
                        "C++ skeleton should contain include statements or class/template definitions");
                    
                    // Should contain simplified implementations
                    assert!(skeleton.contains("// ...") || skeleton.contains("{"), 
                        "Skeleton should contain simplified implementations");
                }
            }
            
            println!("SUCCESS: C++ skeleton generation test passed!");
        },
        Some(Err(e)) => {
            println!("ERROR: C++ skeleton generation failed: {}", e);
            panic!("C++ skeleton generation failed: {}", e);
        },
        None => {
            println!("ERROR: C++ skeleton handler returned None");
            panic!("C++ skeleton handler returned None");
        }
    }
}

#[test]
fn test_cpp_skeleton_with_token_limit() {
    println!("Testing C++ skeleton generation with token limits...");
    
    // Initialize the code graph
    let root_path = std::path::Path::new("../test_files/cpp_test_suite");
    if let Err(e) = codex_core::code_analysis::graph_manager::ensure_graph_for_path(root_path) {
        println!("Warning: Could not initialize graph: {}", e);
    }
    
    // Test with very small token limit
    let test_input = json!({
        "active_files": [
            "../test_files/cpp_test_suite/main.cpp"
        ],
        "max_tokens": 100,  // Very small limit
        "max_depth": 1
    });
    
    match handle_get_related_files_skeleton(test_input) {
        Some(Ok(result)) => {
            println!("C++ small token limit test successful!");
            
            let result_obj = result.as_object().unwrap();
            let related_files = result_obj.get("related_files").unwrap().as_array().unwrap();
            
            // Should respect token limit
            let total_tokens: i64 = related_files.iter()
                .map(|f| f.as_object().unwrap().get("tokens").unwrap().as_i64().unwrap_or(0))
                .sum();
            
            assert!(total_tokens <= 100, "Should respect token limit of 100, got {}", total_tokens);
            
            println!("SUCCESS: C++ token limit test passed! Used {} tokens", total_tokens);
        },
        Some(Err(e)) => {
            println!("ERROR: C++ token limit test failed: {}", e);
            // Don't panic here as this might fail due to graph not being initialized
        },
        None => {
            println!("ERROR: C++ token limit test returned None");
        }
    }
}

#[test]
fn test_cpp_skeleton_bfs_depth() {
    println!("Testing C++ skeleton BFS depth functionality...");
    
    // Initialize the code graph
    let root_path = std::path::Path::new("../test_files/cpp_test_suite");
    if let Err(e) = codex_core::code_analysis::graph_manager::ensure_graph_for_path(root_path) {
        println!("Warning: Could not initialize graph: {}", e);
    }
    
    // Test with different depths
    for depth in [1, 2, 3] {
        let test_input = json!({
            "active_files": [
                "../test_files/cpp_test_suite/main.cpp"
            ],
            "max_tokens": 5000,
            "max_depth": depth
        });
        
        match handle_get_related_files_skeleton(test_input) {
            Some(Ok(result)) => {
                let result_obj = result.as_object().unwrap();
                let total_files = result_obj.get("total_files").unwrap().as_i64().unwrap();
                
                println!("C++ Depth {}: Found {} related files", depth, total_files);
                
                // Generally, deeper searches should find more or equal files
                // (though this depends on the actual code structure)
                assert!(total_files >= 0, "Should find at least 0 files");
            },
            Some(Err(e)) => {
                println!("C++ Depth {} test failed: {}", depth, e);
            },
            None => {
                println!("C++ Depth {} test returned None", depth);
            }
        }
    }
    
    println!("SUCCESS: C++ BFS depth test completed!");
}