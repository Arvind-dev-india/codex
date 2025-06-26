use std::path::PathBuf;
use std::fs;
use std::io::Write;
use tempfile::tempdir;

use codex_core::code_analysis::tools::{
    analyze_code_handler, AnalyzeCodeInput,
};

// Helper function to create a temporary file with content
fn create_temp_file(dir: &tempfile::TempDir, filename: &str, content: &str) -> PathBuf {
    let file_path = dir.path().join(filename);
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file_path
}

#[test]
fn test_rust_code_analysis() {
    let dir = tempdir().unwrap();
    
    // Create a simple Rust file
    let rust_content = r#"
fn hello_world() {
    println!("Hello, world!");
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }
    
    fn greet(&self) {
        println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
    }
}
"#;
    
    let rust_file_path = create_temp_file(&dir, "test.rs", rust_content);
    
    // Analyze the Rust code
    let input = AnalyzeCodeInput {
        file_path: rust_file_path.to_str().unwrap().to_string(),
    };
    
    let result = analyze_code_handler(input);
    
    // Verify that the analysis found the expected symbols
    assert!(result.is_ok(), "Failed to analyze Rust code: {:?}", result.err());
    
    let analysis = result.unwrap();
    let symbols = analysis.get("symbols").expect("No symbols found in analysis");
    
    // Check that we found the function and struct
    let symbols_array = symbols.as_array().expect("Symbols is not an array");
    
    let has_hello_world = symbols_array.iter().any(|s| {
        s.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "hello_world")
    });
    
    let has_person = symbols_array.iter().any(|s| {
        s.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "Person")
    });
    
    let has_greet_method = symbols_array.iter().any(|s| {
        s.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "greet")
    });
    
    assert!(has_hello_world, "Did not find hello_world function");
    assert!(has_person, "Did not find Person struct");
    assert!(has_greet_method, "Did not find greet method");
}

#[test]
fn test_python_code_analysis() {
    let dir = tempdir().unwrap();
    
    // Create a simple Python file
    let python_content = r#"
def hello_world():
    print("Hello, world!")

class Person:
    def __init__(self, name, age):
        self.name = name
        self.age = age
    
    def greet(self):
        print(f"Hello, my name is {self.name} and I am {self.age} years old.")
"#;
    
    let python_file_path = create_temp_file(&dir, "test.py", python_content);
    
    // Analyze the Python code
    let input = AnalyzeCodeInput {
        file_path: python_file_path.to_str().unwrap().to_string(),
    };
    
    let result = analyze_code_handler(input);
    
    // Verify that the analysis found the expected symbols
    assert!(result.is_ok(), "Failed to analyze Python code: {:?}", result.err());
    
    let analysis = result.unwrap();
    let symbols = analysis.get("symbols").expect("No symbols found in analysis");
    
    // Check that we found the function and class
    let symbols_array = symbols.as_array().expect("Symbols is not an array");
    
    let has_hello_world = symbols_array.iter().any(|s| {
        s.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "hello_world")
    });
    
    let has_person = symbols_array.iter().any(|s| {
        s.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "Person")
    });
    
    let has_greet_method = symbols_array.iter().any(|s| {
        s.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "greet")
    });
    
    assert!(has_hello_world, "Did not find hello_world function");
    assert!(has_person, "Did not find Person class");
    assert!(has_greet_method, "Did not find greet method");
}