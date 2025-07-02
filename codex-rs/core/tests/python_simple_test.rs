use codex_core::code_analysis::context_extractor::{ContextExtractor, SymbolType};
use codex_core::code_analysis::{get_parser_pool, SupportedLanguage, QueryType};
use tempfile::tempdir;
use std::fs;

#[test]
fn test_python_simple_parsing() {
    // Create a temporary directory and file
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("SimpleClass.py");
    
    let content = r#"
"""
Simple Python module for testing basic parsing
"""

import math
from typing import List, Optional

def simple_function(x: int, y: int) -> int:
    """A simple function that adds two numbers"""
    return x + y

class SimpleClass:
    """A simple class for testing"""
    
    def __init__(self, name: str, value: int = 0):
        """Initialize the simple class"""
        self.name = name
        self.value = value
        self.items: List[str] = []
    
    def add_numbers(self, a: int, b: int) -> int:
        """Add two numbers together"""
        return a + b
    
    def print_message(self):
        """Print a message"""
        print(f"Hello from {self.name}, value: {self.value}")
    
    def get_info(self) -> dict:
        """Get information about this instance"""
        return {
            'name': self.name,
            'value': self.value,
            'item_count': len(self.items)
        }
    
    @property
    def display_name(self) -> str:
        """Property to get display name"""
        return f"{self.name} ({self.value})"
    
    @staticmethod
    def utility_method(text: str) -> str:
        """Static utility method"""
        return text.upper()
    
    @classmethod
    def from_string(cls, text: str):
        """Class method to create instance from string"""
        parts = text.split(':')
        name = parts[0] if parts else "default"
        value = int(parts[1]) if len(parts) > 1 and parts[1].isdigit() else 0
        return cls(name, value)

def another_function() -> float:
    """Another function using math"""
    return math.pi * 2.0

class AnotherClass:
    """Another simple class"""
    
    def __init__(self):
        self.data = {}
    
    def store_data(self, key: str, value):
        """Store data in the instance"""
        self.data[key] = value
    
    def get_data(self, key: str):
        """Get data from the instance"""
        return self.data.get(key)

if __name__ == "__main__":
    # Test the classes and functions
    obj = SimpleClass("test", 42)
    result = obj.add_numbers(10, 20)
    obj.print_message()
    
    another_obj = AnotherClass()
    another_obj.store_data("key", "value")
    
    func_result = simple_function(5, 10)
    math_result = another_function()
"#;
    
    fs::write(&file_path, content).expect("Failed to write test file");
    
    // Test language detection
    let language = SupportedLanguage::from_path(&file_path);
    assert_eq!(language, Some(SupportedLanguage::Python), "Should detect Python language from .py extension");
    
    // Test parser pool parsing
    let parser_pool = get_parser_pool();
    let result = parser_pool.parse_file_from_disk(file_path.to_str().unwrap());
    
    assert!(result.is_ok(), "Parser pool should successfully parse Python file: {:?}", result.err());
    
    let parsed_file = result.unwrap();
    assert_eq!(parsed_file.language, SupportedLanguage::Python);
    assert!(!parsed_file.source.is_empty(), "Parsed file should have source content");
    
    // Test query execution
    let query_result = parsed_file.execute_predefined_query(QueryType::All);
    assert!(query_result.is_ok(), "Should be able to execute Python queries: {:?}", query_result.err());
    
    let matches = query_result.unwrap();
    assert!(!matches.is_empty(), "Should find some matches in Python file");
    
    println!("Found {} query matches", matches.len());
    for (i, match_) in matches.iter().enumerate() {
        println!("Match {}: pattern {}", i, match_.pattern_index);
        for capture in &match_.captures {
            println!("  Capture '{}': '{}' at {}:{}-{}:{}", 
                     capture.name, capture.text, 
                     capture.start_point.0, capture.start_point.1,
                     capture.end_point.0, capture.end_point.1);
        }
    }
    
    // Test context extractor
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(file_path.to_str().unwrap());
    
    assert!(result.is_ok(), "Failed to extract symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    
    println!("Found {} symbols:", symbols.len());
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
    }
    
    // Test that we found the expected symbols
    
    // Test classes
    let simple_class = symbols.values()
        .find(|s| s.name == "SimpleClass" && matches!(s.symbol_type, SymbolType::Class))
        .expect("SimpleClass should be found");
    
    assert!(simple_class.start_line > 0, "Start line should be positive");
    assert!(simple_class.end_line > simple_class.start_line, "End line should be after start line");
    
    let another_class = symbols.values()
        .find(|s| s.name == "AnotherClass" && matches!(s.symbol_type, SymbolType::Class))
        .expect("AnotherClass should be found");
    
    assert!(another_class.start_line > 0);
    assert!(another_class.end_line > another_class.start_line);
    assert!(another_class.start_line > simple_class.end_line, "AnotherClass should come after SimpleClass");
    
    // Test functions
    let simple_function = symbols.values()
        .find(|s| s.name == "simple_function" && matches!(s.symbol_type, SymbolType::Function))
        .expect("simple_function should be found");
    
    assert!(simple_function.start_line > 0);
    assert!(simple_function.end_line > simple_function.start_line);
    
    let another_function = symbols.values()
        .find(|s| s.name == "another_function" && matches!(s.symbol_type, SymbolType::Function))
        .expect("another_function should be found");
    
    assert!(another_function.start_line > 0);
    assert!(another_function.end_line > another_function.start_line);
    
    // Test methods (in Python, methods are detected as methods)
    let init_method = symbols.values()
        .find(|s| s.name == "__init__" && matches!(s.symbol_type, SymbolType::Method))
        .expect("__init__ method should be found");
    
    assert!(init_method.start_line > 0);
    assert!(init_method.end_line > init_method.start_line);
    
    let add_numbers_method = symbols.values()
        .find(|s| s.name == "add_numbers" && matches!(s.symbol_type, SymbolType::Method))
        .expect("add_numbers method should be found");
    
    assert!(add_numbers_method.start_line > 0);
    assert!(add_numbers_method.end_line > add_numbers_method.start_line);
    
    // Test property and special methods
    let display_name_property = symbols.values()
        .find(|s| s.name == "display_name" && matches!(s.symbol_type, SymbolType::Method))
        .expect("display_name property should be found");
    
    assert!(display_name_property.start_line > 0);
    assert!(display_name_property.end_line > display_name_property.start_line);
    
    let utility_method = symbols.values()
        .find(|s| s.name == "utility_method" && matches!(s.symbol_type, SymbolType::Method))
        .expect("utility_method static method should be found");
    
    assert!(utility_method.start_line > 0);
    assert!(utility_method.end_line > utility_method.start_line);
    
    let from_string_method = symbols.values()
        .find(|s| s.name == "from_string" && matches!(s.symbol_type, SymbolType::Method))
        .expect("from_string class method should be found");
    
    assert!(from_string_method.start_line > 0);
    assert!(from_string_method.end_line > from_string_method.start_line);
    
    // Should find at least: 2 classes, 2 functions, multiple methods
    assert!(symbols.len() >= 10, "Should find at least 10 symbols (classes + functions + methods), found {}", symbols.len());
    
    // Test line number ordering
    assert!(simple_function.start_line < simple_class.start_line, "simple_function should come before SimpleClass");
    assert!(simple_class.end_line < another_function.start_line, "SimpleClass should end before another_function");
    assert!(another_function.end_line < another_class.start_line, "another_function should end before AnotherClass");
}

#[test]
fn test_python_minimal_parsing() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("Minimal.py");
    
    let content = r#"
def hello():
    print("Hello, World!")

class Person:
    def __init__(self, name):
        self.name = name
    
    def greet(self):
        print(f"Hello, I'm {self.name}")

hello()
person = Person("Alice")
person.greet()
"#;
    
    fs::write(&file_path, content).expect("Failed to write test file");
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(file_path.to_str().unwrap());
    
    assert!(result.is_ok(), "Failed to extract symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    
    println!("Found {} symbols in minimal test:", symbols.len());
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
    }
    
    // Should find: hello function, Person class, __init__ method, greet method
    assert!(symbols.len() >= 4, "Should find at least 4 symbols, found {}", symbols.len());
    
    let hello_func = symbols.values()
        .find(|s| s.name == "hello" && matches!(s.symbol_type, SymbolType::Function))
        .expect("hello function should be found");
    
    let person_class = symbols.values()
        .find(|s| s.name == "Person" && matches!(s.symbol_type, SymbolType::Class))
        .expect("Person class should be found");
    
    let init_method = symbols.values()
        .find(|s| s.name == "__init__" && matches!(s.symbol_type, SymbolType::Method))
        .expect("__init__ method should be found");
    
    let greet_method = symbols.values()
        .find(|s| s.name == "greet" && matches!(s.symbol_type, SymbolType::Method))
        .expect("greet method should be found");
    
    // Test line number relationships
    assert!(hello_func.start_line < person_class.start_line, "hello function should come before Person class");
    assert!(person_class.start_line < init_method.start_line, "Person class should start before __init__ method");
    assert!(init_method.end_line < greet_method.start_line, "__init__ method should end before greet method");
    assert!(greet_method.end_line <= person_class.end_line, "greet method should end within Person class");
}

#[test]
fn test_python_edge_cases() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("EdgeCases.py");
    
    let content = r#"
# Test file with various Python edge cases

# Empty class
class EmptyClass:
    pass

# Class with only docstring
class DocstringOnlyClass:
    """This class only has a docstring"""
    pass

# Function with nested function
def outer_function():
    def inner_function():
        return "inner"
    return inner_function()

# Class with nested class
class OuterClass:
    class InnerClass:
        def inner_method(self):
            return "inner method"
    
    def outer_method(self):
        inner = self.InnerClass()
        return inner.inner_method()

# Lambda functions (should not be detected as regular functions)
lambda_func = lambda x: x * 2

# Decorator
def my_decorator(func):
    def wrapper(*args, **kwargs):
        return func(*args, **kwargs)
    return wrapper

@my_decorator
def decorated_function():
    return "decorated"

# Class with properties and special methods
class SpecialMethodsClass:
    def __init__(self, value):
        self._value = value
    
    @property
    def value(self):
        return self._value
    
    @value.setter
    def value(self, new_value):
        self._value = new_value
    
    def __str__(self):
        return f"SpecialMethodsClass({self._value})"
    
    def __repr__(self):
        return f"SpecialMethodsClass(value={self._value})"
    
    def __len__(self):
        return len(str(self._value))

# Multiple inheritance
class Base1:
    def method1(self):
        return "base1"

class Base2:
    def method2(self):
        return "base2"

class MultipleInheritance(Base1, Base2):
    def combined_method(self):
        return self.method1() + " " + self.method2()
"#;
    
    fs::write(&file_path, content).expect("Failed to write test file");
    
    let mut extractor = ContextExtractor::new();
    let result = extractor.extract_symbols_from_file(file_path.to_str().unwrap());
    
    assert!(result.is_ok(), "Failed to extract symbols: {:?}", result.err());
    
    let symbols = extractor.get_symbols();
    
    println!("Found {} symbols in edge cases test:", symbols.len());
    for (fqn, symbol) in symbols {
        println!("  {} -> {} ({:?}) at lines {}-{}", 
                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
    }
    
    // Test empty class
    let empty_class = symbols.values()
        .find(|s| s.name == "EmptyClass" && matches!(s.symbol_type, SymbolType::Class))
        .expect("EmptyClass should be found");
    
    assert!(empty_class.start_line > 0);
    assert!(empty_class.end_line > empty_class.start_line);
    
    // Test nested class
    let outer_class = symbols.values()
        .find(|s| s.name == "OuterClass" && matches!(s.symbol_type, SymbolType::Class))
        .expect("OuterClass should be found");
    
    let inner_class = symbols.values()
        .find(|s| s.name == "InnerClass" && matches!(s.symbol_type, SymbolType::Class))
        .expect("InnerClass should be found");
    
    assert!(inner_class.start_line > outer_class.start_line);
    assert!(inner_class.end_line < outer_class.end_line);
    
    // Test nested function
    let outer_function = symbols.values()
        .find(|s| s.name == "outer_function" && matches!(s.symbol_type, SymbolType::Function))
        .expect("outer_function should be found");
    
    let inner_function = symbols.values()
        .find(|s| s.name == "inner_function" && matches!(s.symbol_type, SymbolType::Function))
        .expect("inner_function should be found");
    
    assert!(inner_function.start_line > outer_function.start_line);
    assert!(inner_function.end_line < outer_function.end_line);
    
    // Test special methods
    let special_methods = ["__init__", "__str__", "__repr__", "__len__"];
    for method_name in &special_methods {
        let method = symbols.values()
            .find(|s| s.name == *method_name && matches!(s.symbol_type, SymbolType::Method))
            .expect(&format!("{} method should be found", method_name));
        
        assert!(method.start_line > 0);
        assert!(method.end_line > method.start_line);
    }
    
    // Test property methods
    let value_property = symbols.values()
        .find(|s| s.name == "value" && matches!(s.symbol_type, SymbolType::Method))
        .expect("value property should be found");
    
    assert!(value_property.start_line > 0);
    assert!(value_property.end_line > value_property.start_line);
    
    // Test multiple inheritance
    let multiple_inheritance_class = symbols.values()
        .find(|s| s.name == "MultipleInheritance" && matches!(s.symbol_type, SymbolType::Class))
        .expect("MultipleInheritance class should be found");
    
    assert!(multiple_inheritance_class.start_line > 0);
    assert!(multiple_inheritance_class.end_line > multiple_inheritance_class.start_line);
    
    // Should find a reasonable number of symbols
    assert!(symbols.len() >= 15, "Should find at least 15 symbols in edge cases test, found {}", symbols.len());
}