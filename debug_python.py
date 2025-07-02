"""
A test module for demonstrating line number detection and tree-sitter parsing
"""

import math
import sys
import os
import subprocess
from typing import Optional

# Print the arguments
print(f"Arguments: {sys.argv}")

# If a file path is provided, read and print it
if len(sys.argv) > 1:
    try:
        file_path = sys.argv[1]
        with open(file_path, 'r') as f:
            content = f.read()
            print(f"File content of {file_path}:")
            print(content)
            
        # Try to use tree-sitter to parse the file
        print("\nAttempting to parse with tree-sitter...")
        
        # Create a simple Rust program to parse the file
        temp_rust_file = "temp_parser.rs"
        with open(temp_rust_file, "w") as f:
            f.write("""
use tree_sitter::{Parser, Language};
use std::fs;
use std::env;

extern "C" { fn tree_sitter_python() -> Language; }

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file_path>", args[0]);
        return;
    }
    
    let file_path = &args[1];
    let code = fs::read_to_string(file_path).expect("Could not read file");
    
    let language = unsafe { tree_sitter_python() };
    let mut parser = Parser::new();
    parser.set_language(language).expect("Error loading Python grammar");
    
    let tree = parser.parse(&code, None).unwrap();
    let root_node = tree.root_node();
    
    println!("Parsed successfully!");
    println!("Root node: {:?}", root_node);
    println!("Root node kind: {}", root_node.kind());
    println!("Root node start position: {:?}", root_node.start_position());
    println!("Root node end position: {:?}", root_node.end_position());
    
    // Print all child nodes
    println!("\\nChild nodes:");
    for i in 0..root_node.child_count() {
        let child = root_node.child(i).unwrap();
        println!("  Child {}: kind={}, text={:?}", 
                 i, 
                 child.kind(),
                 &code[child.start_byte()..child.end_byte()]);
    }
    
    // Find class definitions
    println!("\\nClass definitions:");
    find_nodes(root_node, "class_definition", &code);
    
    // Find function definitions
    println!("\\nFunction definitions:");
    find_nodes(root_node, "function_definition", &code);
}

fn find_nodes(node: tree_sitter::Node, kind: &str, source: &str) {
    if node.kind() == kind {
        println!("  Found {}: {:?}", kind, &source[node.start_byte()..node.end_byte()]);
        
        // For class and function definitions, find the name
        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            if child.kind() == "identifier" {
                println!("    Name: {:?}", &source[child.start_byte()..child.end_byte()]);
            }
        }
    }
    
    for i in 0..node.child_count() {
        find_nodes(node.child(i).unwrap(), kind, source);
    }
}
""")
        
        # Compile and run the Rust program
        print("Compiling Rust parser...")
        compile_result = subprocess.run(["rustc", temp_rust_file, "-o", "temp_parser"], 
                                       capture_output=True, text=True)
        
        if compile_result.returncode == 0:
            print("Running parser...")
            parse_result = subprocess.run(["./temp_parser", file_path], 
                                         capture_output=True, text=True)
            print(parse_result.stdout)
            if parse_result.stderr:
                print("Errors:", parse_result.stderr)
        else:
            print("Failed to compile parser:")
            print(compile_result.stderr)
            
        # Clean up
        try:
            os.remove(temp_rust_file)
            os.remove("temp_parser")
        except:
            pass
            
    except Exception as e:
        print(f"Error: {e}")

class Calculator:
    """A simple calculator class."""
    
    def __init__(self, initial_value: float = 0.0):
        """Initialize the calculator with an optional initial value."""
        self.value = initial_value
        self.history = []
    
    def add(self, x: float) -> float:
        """Add a number to the current value."""
        self.value += x
        self.history.append(f"Added {x}")
        return self.value

def simple_function():
    print("This is a simple function")
    return 42

class MathHelper:
    """Helper class for mathematical operations."""
    
    PI = 3.14159
    
    @staticmethod
    def calculate_circle_area(radius: float) -> float:
        """Calculate the area of a circle."""
        return MathHelper.PI * radius * radius
    
    @classmethod
    def from_diameter(cls, diameter: float):
        """Create a circle calculation from diameter."""
        return cls.calculate_circle_area(diameter / 2)