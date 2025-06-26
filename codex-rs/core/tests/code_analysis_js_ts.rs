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
fn test_javascript_code_analysis() {
    let dir = tempdir().unwrap();
    
    // Create a simple JavaScript file
    let js_content = r#"
function helloWorld() {
    console.log("Hello, world!");
}

class Person {
    constructor(name, age) {
        this.name = name;
        this.age = age;
    }
    
    greet() {
        console.log(`Hello, my name is ${this.name} and I am ${this.age} years old.`);
    }
}

const createPerson = (name, age) => {
    return new Person(name, age);
};
"#;
    
    let js_file_path = create_temp_file(&dir, "test.js", js_content);
    
    // Analyze the JavaScript code
    let input = AnalyzeCodeInput {
        file_path: js_file_path.to_str().unwrap().to_string(),
    };
    
    let result = analyze_code_handler(input);
    
    // Verify that the analysis found the expected symbols
    assert!(result.is_ok(), "Failed to analyze JavaScript code: {:?}", result.err());
    
    let analysis = result.unwrap();
    let symbols = analysis.get("symbols").expect("No symbols found in analysis");
    
    // Check that we found the function and class
    let symbols_array = symbols.as_array().expect("Symbols is not an array");
    
    let has_hello_world = symbols_array.iter().any(|s| {
        s.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "helloWorld")
    });
    
    let has_person = symbols_array.iter().any(|s| {
        s.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "Person")
    });
    
    let has_greet_method = symbols_array.iter().any(|s| {
        s.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "greet")
    });
    
    assert!(has_hello_world, "Did not find helloWorld function");
    assert!(has_person, "Did not find Person class");
    assert!(has_greet_method, "Did not find greet method");
}

#[test]
fn test_typescript_code_analysis() {
    let dir = tempdir().unwrap();
    
    // Create a simple TypeScript file
    let ts_content = r#"
function helloWorld(): void {
    console.log("Hello, world!");
}

interface PersonInterface {
    name: string;
    age: number;
    greet(): void;
}

class Person implements PersonInterface {
    name: string;
    age: number;
    
    constructor(name: string, age: number) {
        this.name = name;
        this.age = age;
    }
    
    greet(): void {
        console.log(`Hello, my name is ${this.name} and I am ${this.age} years old.`);
    }
}

const createPerson = (name: string, age: number): Person => {
    return new Person(name, age);
};
"#;
    
    let ts_file_path = create_temp_file(&dir, "test.ts", ts_content);
    
    // Analyze the TypeScript code
    let input = AnalyzeCodeInput {
        file_path: ts_file_path.to_str().unwrap().to_string(),
    };
    
    let result = analyze_code_handler(input);
    
    // Verify that the analysis found the expected symbols
    assert!(result.is_ok(), "Failed to analyze TypeScript code: {:?}", result.err());
    
    let analysis = result.unwrap();
    let symbols = analysis.get("symbols").expect("No symbols found in analysis");
    
    // Check that we found the function, interface, and class
    let symbols_array = symbols.as_array().expect("Symbols is not an array");
    
    let has_hello_world = symbols_array.iter().any(|s| {
        s.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "helloWorld")
    });
    
    let has_person = symbols_array.iter().any(|s| {
        s.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "Person")
    });
    
    let has_person_interface = symbols_array.iter().any(|s| {
        s.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "PersonInterface")
    });
    
    assert!(has_hello_world, "Did not find helloWorld function");
    assert!(has_person, "Did not find Person class");
    assert!(has_person_interface, "Did not find PersonInterface interface");
}