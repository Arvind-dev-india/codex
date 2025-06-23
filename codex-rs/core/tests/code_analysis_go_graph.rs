use std::path::PathBuf;
use std::fs;
use std::io::Write;
use tempfile::tempdir;

use codex_core::code_analysis::tools::{
    analyze_code_handler, AnalyzeCodeInput,
    get_code_graph_handler, GetCodeGraphInput,
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

#[test]
fn test_code_graph_creation() {
    let dir = tempdir().unwrap();
    
    // Create a simple project with multiple files
    let main_rs = r#"
mod utils;

fn main() {
    println!("Hello, world!");
    utils::helper::print_message("Hello from main");
    let person = utils::person::Person::new("John", 30);
    person.greet();
}
"#;
    
    let utils_mod_rs = r#"
pub mod helper;
pub mod person;
"#;
    
    let helper_rs = r#"
pub fn print_message(msg: &str) {
    println!("Message: {}", msg);
}
"#;
    
    let person_rs = r#"
pub struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }
    
    pub fn greet(&self) {
        println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
    }
}
"#;
    
    // Create the directory structure
    let src_dir = dir.path().join("src");
    fs::create_dir_all(&src_dir).unwrap();
    let utils_dir = src_dir.join("utils");
    fs::create_dir_all(&utils_dir).unwrap();
    
    // Create the files
    let main_path = create_temp_file(&dir, "src/main.rs", main_rs);
    let utils_mod_path = create_temp_file(&dir, "src/utils/mod.rs", utils_mod_rs);
    let helper_path = create_temp_file(&dir, "src/utils/helper.rs", helper_rs);
    let person_path = create_temp_file(&dir, "src/utils/person.rs", person_rs);
    
    // Get the code graph for the project
    let input = GetCodeGraphInput {
        directory: dir.path().to_str().unwrap().to_string(),
    };
    
    let result = get_code_graph_handler(input);
    
    // Verify that the graph was created successfully
    assert!(result.is_ok(), "Failed to create code graph: {:?}", result.err());
    
    let graph = result.unwrap();
    
    // Check that the graph contains nodes for all files and symbols
    let nodes = graph.get("nodes").expect("No nodes found in graph");
    let edges = graph.get("edges").expect("No edges found in graph");
    
    let nodes_array = nodes.as_array().expect("Nodes is not an array");
    let edges_array = edges.as_array().expect("Edges is not an array");
    
    // Check that we have nodes for all files and key symbols
    let has_main_file = nodes_array.iter().any(|n| {
        n.get("file_path").map_or(false, |path| path.as_str().unwrap_or("").contains("main.rs"))
    });
    
    let has_person_struct = nodes_array.iter().any(|n| {
        n.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "Person")
    });
    
    let has_print_message_function = nodes_array.iter().any(|n| {
        n.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "print_message")
    });
    
    assert!(has_main_file, "Did not find main.rs file in graph nodes");
    assert!(has_person_struct, "Did not find Person struct in graph nodes");
    assert!(has_print_message_function, "Did not find print_message function in graph nodes");
    
    // Check that we have edges representing relationships
    assert!(!edges_array.is_empty(), "No edges found in the graph");
    
    // Update the graph with a new file
    let new_file_content = r#"
pub fn calculate_age(birth_year: u32, current_year: u32) -> u32 {
    current_year - birth_year
}
"#;
    
    let new_file_path = create_temp_file(&dir, "src/utils/age_calculator.rs", new_file_content);
    
    // Update the utils/mod.rs file to include the new module
    let updated_utils_mod_rs = r#"
pub mod helper;
pub mod person;
pub mod age_calculator;
"#;
    
    fs::write(utils_mod_path, updated_utils_mod_rs).unwrap();
    
    // Update the code graph
    let update_input = UpdateCodeGraphInput {
        directory: dir.path().to_str().unwrap().to_string(),
    };
    
    let update_result = update_code_graph_handler(update_input);
    
    // Verify that the graph was updated successfully
    assert!(update_result.is_ok(), "Failed to update code graph: {:?}", update_result.err());
    
    let updated_graph = update_result.unwrap();
    
    // Check that the updated graph contains the new file and function
    let updated_nodes = updated_graph.get("nodes").expect("No nodes found in updated graph");
    let updated_nodes_array = updated_nodes.as_array().expect("Updated nodes is not an array");
    
    let has_age_calculator_file = updated_nodes_array.iter().any(|n| {
        n.get("file_path").map_or(false, |path| path.as_str().unwrap_or("").contains("age_calculator.rs"))
    });
    
    let has_calculate_age_function = updated_nodes_array.iter().any(|n| {
        n.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "calculate_age")
    });
    
    assert!(has_age_calculator_file, "Did not find age_calculator.rs file in updated graph nodes");
    assert!(has_calculate_age_function, "Did not find calculate_age function in updated graph nodes");
}