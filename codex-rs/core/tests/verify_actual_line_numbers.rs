use tempfile::tempdir;
use std::fs;

#[test]
fn verify_actual_line_numbers() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("LineNumberTest.py");
    
    // This is the exact content from the failing test
    let content = r#"# Line 1: Comment
"""
Line 2-4: Module docstring
"""

# Line 6: Import
import math
from typing import List, Optional

# Line 10: Function definition starts
def simple_function(x: int, y: int) -> int:
    """A simple function that adds two numbers"""
    return x + y
# Line 13: Function ends

# Line 15: Class definition starts
class TestClass:
    """A test class for line number validation"""
    
    # Line 19: Method definition starts
    def __init__(self, name: str, value: int = 0):
        """Initialize the test class"""
        self.name = name
        self.value = value
    # Line 23: Method ends
    
    # Line 25: Method definition starts
    def add_numbers(self, a: int, b: int) -> int:
        """Add two numbers together"""
        result = a + b
        return result
    # Line 29: Method ends
    
    # Line 31: Property definition starts
    @property
    def display_name(self) -> str:
        """Property to get display name"""
        return f"{self.name} ({self.value})"
    # Line 35: Property ends
    
    # Line 37: Static method definition starts
    @staticmethod
    def utility_method(text: str) -> str:
        """Static utility method"""
        return text.upper()
    # Line 41: Static method ends
    
    # Line 43: Class method definition starts
    @classmethod
    def from_string(cls, text: str):
        """Class method to create instance from string"""
        parts = text.split(':')
        name = parts[0] if parts else "default"
        value = int(parts[1]) if len(parts) > 1 and parts[1].isdigit() else 0
        return cls(name, value)
    # Line 49: Class method ends
# Line 50: Class ends

# Line 52: Another function definition starts
def another_function() -> float:
    """Another function using math"""
    return math.pi * 2.0
# Line 55: Function ends

# Line 57: Nested function test starts
def outer_function():
    """Function with nested function"""
    # Line 60: Nested function starts
    def inner_function():
        """Nested function"""
        return "inner"
    # Line 63: Nested function ends
    return inner_function()
# Line 65: Outer function ends

# Line 67: Class with nested class starts
class OuterClass:
    """Class with nested class"""
    
    # Line 71: Nested class starts
    class InnerClass:
        """Nested class"""
        # Line 74: Nested class method starts
        def inner_method(self):
            """Method in nested class"""
            return "inner method"
        # Line 77: Nested class method ends
    # Line 78: Nested class ends
    
    # Line 80: Outer class method starts
    def outer_method(self):
        """Method in outer class"""
        inner = self.InnerClass()
        return inner.inner_method()
    # Line 84: Outer class method ends
# Line 85: Outer class ends
"#;
    
    fs::write(&file_path, content).expect("Failed to write test file");
    
    println!("Actual line numbers for key constructs:");
    for (i, line) in content.lines().enumerate() {
        let line_num = i + 1;
        if line.contains("def simple_function") {
            println!("  simple_function starts at line {}: {}", line_num, line.trim());
        }
        if line.contains("def another_function") {
            println!("  another_function starts at line {}: {}", line_num, line.trim());
        }
        if line.contains("def outer_function") {
            println!("  outer_function starts at line {}: {}", line_num, line.trim());
        }
        if line.contains("def inner_function") {
            println!("  inner_function starts at line {}: {}", line_num, line.trim());
        }
        if line.contains("class TestClass") {
            println!("  TestClass starts at line {}: {}", line_num, line.trim());
        }
        if line.contains("class InnerClass") {
            println!("  InnerClass starts at line {}: {}", line_num, line.trim());
        }
        if line.contains("def __init__") {
            println!("  __init__ starts at line {}: {}", line_num, line.trim());
        }
        if line.contains("def add_numbers") {
            println!("  add_numbers starts at line {}: {}", line_num, line.trim());
        }
    }
}