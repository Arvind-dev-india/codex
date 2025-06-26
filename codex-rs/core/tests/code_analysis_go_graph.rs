use std::path::PathBuf;
use std::fs;
use std::io::Write;
use tempfile::tempdir;

use codex_core::code_analysis::tools::{
    analyze_code_handler, AnalyzeCodeInput,
    update_code_graph_handler, UpdateCodeGraphInput,
};

// Helper function to create a temporary file with content
fn create_temp_file(dir: &tempfile::TempDir, filename: &str, content: &str) -> PathBuf {
    let file_path = dir.path().join(filename);
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file_path
}

#[test]
fn test_go_code_analysis() {
    let dir = tempdir().unwrap();
    
    // Create a simple Go file
    let go_content = r#"
package main

import "fmt"

func helloWorld() {
    fmt.Println("Hello, world!")
}

type Person struct {
    Name string
    Age  int
}

func (p *Person) Greet() {
    fmt.Printf("Hello, my name is %s and I am %d years old.\n", p.Name, p.Age)
}

func NewPerson(name string, age int) *Person {
    return &Person{
        Name: name,
        Age:  age,
    }
}

func main() {
    helloWorld()
    person := NewPerson("John", 30)
    person.Greet()
}
"#;
    
    let go_file_path = create_temp_file(&dir, "main.go", go_content);
    
    // Analyze the Go code
    let input = AnalyzeCodeInput {
        file_path: go_file_path.to_str().unwrap().to_string(),
    };
    
    let result = analyze_code_handler(input);
    
    // Verify that the analysis found the expected symbols
    assert!(result.is_ok(), "Failed to analyze Go code: {:?}", result.err());
    
    let analysis = result.unwrap();
    let symbols = analysis.get("symbols").expect("No symbols found in analysis");
    
    // Check that we found the package, struct, and functions
    let symbols_array = symbols.as_array().expect("Symbols is not an array");
    
    let has_main_package = symbols_array.iter().any(|s| {
        s.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "main")
    });
    
    let has_person = symbols_array.iter().any(|s| {
        s.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "Person")
    });
    
    let has_hello_world = symbols_array.iter().any(|s| {
        s.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "helloWorld")
    });
    
    assert!(has_main_package, "Did not find main package");
    assert!(has_person, "Did not find Person struct");
    assert!(has_hello_world, "Did not find helloWorld function");
}

