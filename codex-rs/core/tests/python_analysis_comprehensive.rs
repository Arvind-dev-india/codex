use codex_core::code_analysis::context_extractor::{ContextExtractor, SymbolType};
use codex_core::code_analysis::{get_parser_pool, SupportedLanguage, QueryType};
use std::path::Path;

#[test]
fn test_python_basic_class_parsing() {
    let test_file = "../test_files/python_test_suite/basic_class.py";
    
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
    
    // Test method detection (in Python, methods are detected as methods)
    let init_method = symbols.values()
        .find(|s| s.name == "__init__" && matches!(s.symbol_type, SymbolType::Method))
        .expect("__init__ method should be found");
    
    assert!(init_method.start_line > 0);
    assert!(init_method.end_line > init_method.start_line);
    
    let add_method = symbols.values()
        .find(|s| s.name == "add" && matches!(s.symbol_type, SymbolType::Method))
        .expect("add method should be found");
    
    assert!(add_method.start_line > 0);
    assert!(add_method.end_line > add_method.start_line);
    
    // Test function detection
    let standalone_func = symbols.values()
        .find(|s| s.name == "standalone_function" && matches!(s.symbol_type, SymbolType::Function))
        .expect("standalone_function should be found");
    
    assert!(standalone_func.start_line > 0);
    assert!(standalone_func.end_line > standalone_func.start_line);
    
    // Test nested class detection
    let nested_class = symbols.values()
        .find(|s| s.name == "NestedClass" && matches!(s.symbol_type, SymbolType::Class))
        .expect("NestedClass should be found");
    
    assert!(nested_class.start_line > 0);
    assert!(nested_class.end_line > nested_class.start_line);
    
    // Should find at least: BasicClass, NestedClass, InnerClass, multiple methods, and functions
    assert!(symbols.len() >= 10, "Should find at least 10 symbols, found {}", symbols.len());
}

#[test]
fn test_python_inheritance_parsing() {
    let test_file = "../test_files/python_test_suite/models/user.py";
    
    assert!(Path::new(test_file).exists(), "Test file {} does not exist", test_file);
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(test_file);
    
    assert!(result.is_ok(), "Failed to extract symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    
    println!("Found {} symbols in user.py:", symbols.len());
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
    
    // Test inherited class
    let admin_user_class = symbols.values()
        .find(|s| s.name == "AdminUser" && matches!(s.symbol_type, SymbolType::Class))
        .expect("AdminUser class should be found");
    
    assert!(admin_user_class.start_line > 0);
    assert!(admin_user_class.end_line > admin_user_class.start_line);
    assert!(admin_user_class.start_line > user_class.end_line, "AdminUser should come after User class");
    
    // Test methods in both classes
    let user_methods = ["__init__", "add_order", "get_order", "get_total_order_value", "_log_activity", "__str__", "__repr__"];
    for method_name in &user_methods {
        let method = symbols.values()
            .find(|s| s.name == *method_name && matches!(s.symbol_type, SymbolType::Function))
            .expect(&format!("{} method should be found", method_name));
        
        assert!(method.start_line > 0);
        assert!(method.end_line > method.start_line);
    }
    
    let admin_methods = ["add_permission", "has_permission", "promote_user"];
    for method_name in &admin_methods {
        let method = symbols.values()
            .find(|s| s.name == *method_name && matches!(s.symbol_type, SymbolType::Function))
            .expect(&format!("{} method should be found", method_name));
        
        assert!(method.start_line > 0);
        assert!(method.end_line > method.start_line);
    }
}

#[test]
fn test_python_enum_and_complex_types() {
    let test_file = "../test_files/python_test_suite/models/order.py";
    
    assert!(Path::new(test_file).exists(), "Test file {} does not exist", test_file);
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(test_file);
    
    assert!(result.is_ok(), "Failed to extract symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    
    println!("Found {} symbols in order.py:", symbols.len());
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
    }
    
    // Test enum class
    let order_status_enum = symbols.values()
        .find(|s| s.name == "OrderStatus" && matches!(s.symbol_type, SymbolType::Class))
        .expect("OrderStatus enum should be found");
    
    assert!(order_status_enum.start_line > 0);
    assert!(order_status_enum.end_line > order_status_enum.start_line);
    
    // Test main classes
    let order_class = symbols.values()
        .find(|s| s.name == "Order" && matches!(s.symbol_type, SymbolType::Class))
        .expect("Order class should be found");
    
    let order_item_class = symbols.values()
        .find(|s| s.name == "OrderItem" && matches!(s.symbol_type, SymbolType::Class))
        .expect("OrderItem class should be found");
    
    assert!(order_class.start_line > 0);
    assert!(order_item_class.start_line > 0);
    assert!(order_item_class.start_line > order_class.end_line, "OrderItem should come after Order class");
    
    // Test complex methods with calculations
    let calculation_methods = ["calculate_subtotal", "calculate_discount", "calculate_tax", "calculate_total"];
    for method_name in &calculation_methods {
        let method = symbols.values()
            .find(|s| s.name == *method_name && matches!(s.symbol_type, SymbolType::Function))
            .expect(&format!("{} method should be found", method_name));
        
        assert!(method.start_line > 0);
        assert!(method.end_line > method.start_line);
    }
}

#[test]
fn test_python_generic_and_abstract_classes() {
    let test_file = "../test_files/python_test_suite/data/repository.py";
    
    assert!(Path::new(test_file).exists(), "Test file {} does not exist", test_file);
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(test_file);
    
    assert!(result.is_ok(), "Failed to extract symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    
    println!("Found {} symbols in repository.py:", symbols.len());
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
    }
    
    // Test abstract base class
    let repository_class = symbols.values()
        .find(|s| s.name == "Repository" && matches!(s.symbol_type, SymbolType::Class))
        .expect("Repository abstract class should be found");
    
    assert!(repository_class.start_line > 0);
    assert!(repository_class.end_line > repository_class.start_line);
    
    // Test generic implementation
    let in_memory_repo = symbols.values()
        .find(|s| s.name == "InMemoryRepository" && matches!(s.symbol_type, SymbolType::Class))
        .expect("InMemoryRepository class should be found");
    
    assert!(in_memory_repo.start_line > 0);
    assert!(in_memory_repo.end_line > in_memory_repo.start_line);
    assert!(in_memory_repo.start_line > repository_class.end_line, "InMemoryRepository should come after Repository");
    
    // Test specialized repositories
    let user_repo = symbols.values()
        .find(|s| s.name == "UserRepository" && matches!(s.symbol_type, SymbolType::Class))
        .expect("UserRepository class should be found");
    
    let order_repo = symbols.values()
        .find(|s| s.name == "OrderRepository" && matches!(s.symbol_type, SymbolType::Class))
        .expect("OrderRepository class should be found");
    
    assert!(user_repo.start_line > in_memory_repo.end_line);
    assert!(order_repo.start_line > user_repo.end_line);
    
    // Test abstract methods
    let abstract_methods = ["add", "get_by_id", "get_all", "update", "delete"];
    for method_name in &abstract_methods {
        let method = symbols.values()
            .find(|s| s.name == *method_name && matches!(s.symbol_type, SymbolType::Function))
            .expect(&format!("{} abstract method should be found", method_name));
        
        assert!(method.start_line > 0);
        assert!(method.end_line > method.start_line);
    }
}

#[test]
fn test_python_utility_functions_and_decorators() {
    let test_file = "../test_files/python_test_suite/utils/helpers.py";
    
    assert!(Path::new(test_file).exists(), "Test file {} does not exist", test_file);
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(test_file);
    
    assert!(result.is_ok(), "Failed to extract symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    
    println!("Found {} symbols in helpers.py:", symbols.len());
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
    }
    
    // Test utility functions
    let utility_functions = ["validate_email", "generate_hash", "format_currency", "calculate_percentage"];
    for func_name in &utility_functions {
        let function = symbols.values()
            .find(|s| s.name == *func_name && matches!(s.symbol_type, SymbolType::Function))
            .expect(&format!("{} function should be found", func_name));
        
        assert!(function.start_line > 0);
        assert!(function.end_line > function.start_line);
    }
    
    // Test decorator function
    let decorator = symbols.values()
        .find(|s| s.name == "retry_on_failure" && matches!(s.symbol_type, SymbolType::Function))
        .expect("retry_on_failure decorator should be found");
    
    assert!(decorator.start_line > 0);
    assert!(decorator.end_line > decorator.start_line);
    
    // Test classes
    let utility_classes = ["Timer", "DataValidator", "ConfigManager", "Singleton", "Logger"];
    for class_name in &utility_classes {
        let class = symbols.values()
            .find(|s| s.name == *class_name && matches!(s.symbol_type, SymbolType::Class))
            .expect(&format!("{} class should be found", class_name));
        
        assert!(class.start_line > 0);
        assert!(class.end_line > class.start_line);
    }
    
    // Test context manager methods
    let timer_methods = ["__enter__", "__exit__", "elapsed"];
    for method_name in &timer_methods {
        let method = symbols.values()
            .find(|s| s.name == *method_name && matches!(s.symbol_type, SymbolType::Function))
            .expect(&format!("{} method should be found", method_name));
        
        assert!(method.start_line > 0);
        assert!(method.end_line > method.start_line);
    }
}

#[test]
fn test_python_line_number_accuracy() {
    let test_file = "../test_files/python_test_suite/basic_class.py";
    
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
    
    // Test method line numbers
    let add_method = symbols.values()
        .find(|s| s.name == "add" && matches!(s.symbol_type, SymbolType::Function))
        .expect("add method should be found");
    
    let method_line = lines.get(add_method.start_line as usize - 1)
        .expect("Method start line should exist");
    assert!(method_line.contains("def add"), 
           "Line {} should contain 'def add', but contains: '{}'", 
           add_method.start_line, method_line);
    
    // Test function line numbers
    let standalone_func = symbols.values()
        .find(|s| s.name == "standalone_function" && matches!(s.symbol_type, SymbolType::Function))
        .expect("standalone_function should be found");
    
    let func_line = lines.get(standalone_func.start_line as usize - 1)
        .expect("Function start line should exist");
    assert!(func_line.contains("def standalone_function"), 
           "Line {} should contain 'def standalone_function', but contains: '{}'", 
           standalone_func.start_line, func_line);
    
    // Verify line ordering
    assert!(basic_class.start_line < add_method.start_line, 
           "BasicClass should start before add method");
    assert!(add_method.end_line < standalone_func.start_line, 
           "add method should end before standalone_function starts");
}