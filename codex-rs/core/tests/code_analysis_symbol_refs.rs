use std::path::PathBuf;
use std::fs;
use std::io::Write;
use tempfile::tempdir;

use codex_core::code_analysis::tools::{
    find_symbol_references_handler, FindSymbolReferencesInput,
    find_symbol_definitions_handler, FindSymbolDefinitionsInput,
    get_symbol_subgraph_handler, GetSymbolSubgraphInput,
};

// Helper function to create a temporary file with content
fn create_temp_file(dir: &tempfile::TempDir, filename: &str, content: &str) -> PathBuf {
    let file_path = dir.path().join(filename);
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file_path
}

#[test]
fn test_find_symbol_references() {
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
    let _utils_mod_path = create_temp_file(&dir, "src/utils/mod.rs", utils_mod_rs);
    let _helper_path = create_temp_file(&dir, "src/utils/helper.rs", helper_rs);
    let _person_path = create_temp_file(&dir, "src/utils/person.rs", person_rs);
    
    // Find references to the Person struct
    let input = FindSymbolReferencesInput {
        symbol_name: "Person".to_string(),
        directory: dir.path().to_str().unwrap().to_string(),
    };
    
    let result = find_symbol_references_handler(input);
    
    // Verify that the references were found
    assert!(result.is_ok(), "Failed to find symbol references: {:?}", result.err());
    
    let references = result.unwrap();
    let references_array = references.as_array().expect("References is not an array");
    
    // Check that we found references in both main.rs and person.rs
    let has_main_reference = references_array.iter().any(|r| {
        r.get("file_path").map_or(false, |path| path.as_str().unwrap_or("").contains("main.rs"))
    });
    
    let has_person_reference = references_array.iter().any(|r| {
        r.get("file_path").map_or(false, |path| path.as_str().unwrap_or("").contains("person.rs"))
    });
    
    assert!(has_main_reference, "Did not find reference to Person in main.rs");
    assert!(has_person_reference, "Did not find reference to Person in person.rs");
    
    // There should be at least 3 references (1 definition + at least 2 usages)
    assert!(references_array.len() >= 3, "Expected at least 3 references to Person, found {}", references_array.len());
}

#[test]
fn test_find_symbol_definitions() {
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
    let _utils_mod_path = create_temp_file(&dir, "src/utils/mod.rs", utils_mod_rs);
    let _helper_path = create_temp_file(&dir, "src/utils/helper.rs", helper_rs);
    let _person_path = create_temp_file(&dir, "src/utils/person.rs", person_rs);
    
    // Find definitions of the Person struct
    let input = FindSymbolDefinitionsInput {
        symbol_name: "Person".to_string(),
        directory: dir.path().to_str().unwrap().to_string(),
    };
    
    let result = find_symbol_definitions_handler(input);
    
    // Verify that the definitions were found
    assert!(result.is_ok(), "Failed to find symbol definitions: {:?}", result.err());
    
    let definitions = result.unwrap();
    let definitions_array = definitions.as_array().expect("Definitions is not an array");
    
    // Check that we found the definition in person.rs
    let has_person_definition = definitions_array.iter().any(|d| {
        d.get("file_path").map_or(false, |path| path.as_str().unwrap_or("").contains("person.rs"))
    });
    
    assert!(has_person_definition, "Did not find definition of Person in person.rs");
    
    // There should be exactly 1 definition
    assert_eq!(definitions_array.len(), 1, "Expected 1 definition of Person, found {}", definitions_array.len());
}

#[test]
fn test_get_symbol_subgraph() {
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
    let _utils_mod_path = create_temp_file(&dir, "src/utils/mod.rs", utils_mod_rs);
    let _helper_path = create_temp_file(&dir, "src/utils/helper.rs", helper_rs);
    let _person_path = create_temp_file(&dir, "src/utils/person.rs", person_rs);
    
    // Get the subgraph for the Person symbol
    let input = GetSymbolSubgraphInput {
        symbol_name: "Person".to_string(),
        directory: dir.path().to_str().unwrap().to_string(),
        depth: Some(2),
        max_depth: 2,
    };
    
    let result = get_symbol_subgraph_handler(input);
    
    // Verify that the subgraph was created successfully
    assert!(result.is_ok(), "Failed to get symbol subgraph: {:?}", result.err());
    
    let subgraph = result.unwrap();
    
    // Check that the subgraph contains nodes and edges
    let nodes = subgraph.get("nodes").expect("No nodes found in subgraph");
    let edges = subgraph.get("edges").expect("No edges found in subgraph");
    
    let nodes_array = nodes.as_array().expect("Nodes is not an array");
    let edges_array = edges.as_array().expect("Edges is not an array");
    
    // Check that we have the Person node and its related nodes (new and greet methods)
    let has_person_node = nodes_array.iter().any(|n| {
        n.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "Person")
    });
    
    let has_new_method = nodes_array.iter().any(|n| {
        n.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "new")
    });
    
    let has_greet_method = nodes_array.iter().any(|n| {
        n.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "greet")
    });
    
    assert!(has_person_node, "Did not find Person node in subgraph");
    assert!(has_new_method, "Did not find new method node in subgraph");
    assert!(has_greet_method, "Did not find greet method node in subgraph");
    
    // Check that we have edges connecting Person to its methods
    assert!(!edges_array.is_empty(), "No edges found in the subgraph");
}