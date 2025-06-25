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

// Helper function to find a symbol in the analysis result by name
fn find_symbol_by_name<'a>(analysis: &'a serde_json::Value, name: &str) -> Option<&'a serde_json::Value> {
    let symbols = analysis.get("symbols")?.as_array()?;
    symbols.iter().find(|s| {
        s.get("name").and_then(|n| n.as_str()) == Some(name)
    })
}

// Helper function to get line numbers from a symbol
fn get_symbol_lines(symbol: &serde_json::Value) -> (Option<u64>, Option<u64>) {
    let start_line = symbol.get("start_line").and_then(|l| l.as_u64());
    let end_line = symbol.get("end_line").and_then(|l| l.as_u64());
    (start_line, end_line)
}

#[test]
fn test_csharp_class_with_methods_line_numbers() {
    let dir = tempdir().unwrap();
    
    // Create a C# file with a class containing methods, with and without comments
    let csharp_content = r#"using System;

namespace TestNamespace
{
    /// <summary>
    /// A test class for demonstrating line number detection
    /// </summary>
    public class Calculator
    {
        private int _value;

        /// <summary>
        /// Constructor for Calculator
        /// </summary>
        /// <param name="initialValue">The initial value</param>
        public Calculator(int initialValue)
        {
            _value = initialValue;
        }

        /// <summary>
        /// Adds two numbers together
        /// </summary>
        /// <param name="a">First number</param>
        /// <param name="b">Second number</param>
        /// <returns>The sum of a and b</returns>
        public int Add(int a, int b)
        {
            return a + b;
        }

        // Simple method without XML comments
        public int Multiply(int a, int b)
        {
            return a * b;
        }

        /// <summary>
        /// A more complex method with multiple statements
        /// </summary>
        public void ComplexMethod()
        {
            var temp = _value;
            for (int i = 0; i < 10; i++)
            {
                temp += i;
                Console.WriteLine($"Iteration {i}: {temp}");
            }
            _value = temp;
        }

        // Property with getter and setter
        public int Value
        {
            get { return _value; }
            set { _value = value; }
        }
    }

    // Another class to test multiple classes
    public class MathHelper
    {
        public static double Pi = 3.14159;

        public static double CalculateCircleArea(double radius)
        {
            return Pi * radius * radius;
        }
    }
}
"#;
    
    let csharp_file_path = create_temp_file(&dir, "Calculator.cs", csharp_content);
    
    // Analyze the C# code
    let input = AnalyzeCodeInput {
        file_path: csharp_file_path.to_str().unwrap().to_string(),
    };
    
    let result = analyze_code_handler(input);
    assert!(result.is_ok(), "Failed to analyze C# code: {:?}", result.err());
    
    let analysis = result.unwrap();
    println!("Analysis result: {}", serde_json::to_string_pretty(&analysis).unwrap());
    
    // Test Calculator class
    let calculator_class = find_symbol_by_name(&analysis, "Calculator");
    assert!(calculator_class.is_some(), "Calculator class not found");
    
    let calculator_class = calculator_class.unwrap();
    let (start_line, end_line) = get_symbol_lines(calculator_class);
    
    // Calculator class should start around line 8 and end around line 54
    assert!(start_line.is_some(), "Calculator class start_line is missing");
    assert!(end_line.is_some(), "Calculator class end_line is missing");
    
    let start_line = start_line.unwrap();
    let end_line = end_line.unwrap();
    
    println!("Calculator class: lines {} to {}", start_line, end_line);
    
    // The class declaration starts at line 8 (1-based)
    assert_eq!(start_line, 8, "Calculator class should start at line 8");
    // The class should end at the closing brace around line 58 (accounting for all methods and properties)
    assert!(end_line >= 56 && end_line <= 60, "Calculator class should end around line 56-60, but got {}", end_line);
    
    // Test Add method
    let add_method = find_symbol_by_name(&analysis, "Add");
    assert!(add_method.is_some(), "Add method not found");
    
    let add_method = add_method.unwrap();
    let (start_line, end_line) = get_symbol_lines(add_method);
    
    assert!(start_line.is_some(), "Add method start_line is missing");
    assert!(end_line.is_some(), "Add method end_line is missing");
    
    let start_line = start_line.unwrap();
    let end_line = end_line.unwrap();
    
    println!("Add method: lines {} to {}", start_line, end_line);
    
    // Add method should start around line 27 and end around line 30
    assert!(start_line >= 26 && start_line <= 28, "Add method should start around line 26-28, but got {}", start_line);
    assert!(end_line >= 29 && end_line <= 31, "Add method should end around line 29-31, but got {}", end_line);
    
    // Test Multiply method (without XML comments)
    let multiply_method = find_symbol_by_name(&analysis, "Multiply");
    assert!(multiply_method.is_some(), "Multiply method not found");
    
    let multiply_method = multiply_method.unwrap();
    let (start_line, end_line) = get_symbol_lines(multiply_method);
    
    assert!(start_line.is_some(), "Multiply method start_line is missing");
    assert!(end_line.is_some(), "Multiply method end_line is missing");
    
    let start_line = start_line.unwrap();
    let end_line = end_line.unwrap();
    
    println!("Multiply method: lines {} to {}", start_line, end_line);
    
    // Multiply method should start around line 33 and end around line 36
    assert!(start_line >= 32 && start_line <= 34, "Multiply method should start around line 32-34, but got {}", start_line);
    assert!(end_line >= 35 && end_line <= 37, "Multiply method should end around line 35-37, but got {}", end_line);
    
    // Test ComplexMethod
    let complex_method = find_symbol_by_name(&analysis, "ComplexMethod");
    assert!(complex_method.is_some(), "ComplexMethod not found");
    
    let complex_method = complex_method.unwrap();
    let (start_line, end_line) = get_symbol_lines(complex_method);
    
    assert!(start_line.is_some(), "ComplexMethod start_line is missing");
    assert!(end_line.is_some(), "ComplexMethod end_line is missing");
    
    let start_line = start_line.unwrap();
    let end_line = end_line.unwrap();
    
    println!("ComplexMethod: lines {} to {}", start_line, end_line);
    
    // ComplexMethod should start around line 41 and end around line 50
    assert!(start_line >= 40 && start_line <= 42, "ComplexMethod should start around line 40-42, but got {}", start_line);
    assert!(end_line >= 49 && end_line <= 51, "ComplexMethod should end around line 49-51, but got {}", end_line);
    
    // Test MathHelper class
    let math_helper_class = find_symbol_by_name(&analysis, "MathHelper");
    assert!(math_helper_class.is_some(), "MathHelper class not found");
    
    let math_helper_class = math_helper_class.unwrap();
    let (start_line, end_line) = get_symbol_lines(math_helper_class);
    
    assert!(start_line.is_some(), "MathHelper class start_line is missing");
    assert!(end_line.is_some(), "MathHelper class end_line is missing");
    
    let start_line = start_line.unwrap();
    let end_line = end_line.unwrap();
    
    println!("MathHelper class: lines {} to {}", start_line, end_line);
    
    // MathHelper class should start around line 61 and end around line 69
    assert!(start_line >= 60 && start_line <= 62, "MathHelper class should start around line 60-62, but got {}", start_line);
    assert!(end_line >= 68 && end_line <= 70, "MathHelper class should end around line 68-70, but got {}", end_line);
}

#[test]
fn test_rust_functions_and_structs_line_numbers() {
    let dir = tempdir().unwrap();
    
    // Create a Rust file with functions and structs, with and without comments
    let rust_content = r#"//! A test module for demonstrating line number detection

use std::fmt;

/// A simple struct representing a person
#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    /// Creates a new Person instance
    /// 
    /// # Arguments
    /// * `name` - The person's name
    /// * `age` - The person's age
    pub fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }

    // Simple method without doc comments
    pub fn greet(&self) {
        println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
    }

    /// A more complex method with multiple statements
    pub fn have_birthday(&mut self) {
        self.age += 1;
        println!("{} is now {} years old!", self.name, self.age);
        
        if self.age >= 18 {
            println!("You are an adult!");
        } else {
            println!("You are still a minor.");
        }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.age)
    }
}

// A simple function without doc comments
fn simple_function() {
    println!("This is a simple function");
}

/// A function with documentation
/// 
/// This function demonstrates proper documentation
/// and spans multiple lines
fn documented_function(x: i32, y: i32) -> i32 {
    let result = x + y;
    println!("Adding {} + {} = {}", x, y, result);
    result
}

// Another struct for testing
pub struct Calculator {
    value: f64,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { value: 0.0 }
    }

    pub fn add(&mut self, x: f64) -> &mut Self {
        self.value += x;
        self
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}
"#;
    
    let rust_file_path = create_temp_file(&dir, "test.rs", rust_content);
    
    // Analyze the Rust code
    let input = AnalyzeCodeInput {
        file_path: rust_file_path.to_str().unwrap().to_string(),
    };
    
    let result = analyze_code_handler(input);
    assert!(result.is_ok(), "Failed to analyze Rust code: {:?}", result.err());
    
    let analysis = result.unwrap();
    println!("Analysis result: {}", serde_json::to_string_pretty(&analysis).unwrap());
    
    // Test Person struct
    let person_struct = find_symbol_by_name(&analysis, "Person");
    assert!(person_struct.is_some(), "Person struct not found");
    
    let person_struct = person_struct.unwrap();
    let (start_line, end_line) = get_symbol_lines(person_struct);
    
    assert!(start_line.is_some(), "Person struct start_line is missing");
    assert!(end_line.is_some(), "Person struct end_line is missing");
    
    let start_line = start_line.unwrap();
    let end_line = end_line.unwrap();
    
    println!("Person struct: lines {} to {}", start_line, end_line);
    
    // Person struct should start around line 7 and end around line 10
    assert!(start_line >= 7 && start_line <= 8, "Person struct should start around line 7-8, but got {}", start_line);
    assert!(end_line >= 10 && end_line <= 11, "Person struct should end around line 10-11, but got {}", end_line);
    
    // Test new method
    let new_method = find_symbol_by_name(&analysis, "new");
    assert!(new_method.is_some(), "new method not found");
    
    let new_method = new_method.unwrap();
    let (start_line, end_line) = get_symbol_lines(new_method);
    
    assert!(start_line.is_some(), "new method start_line is missing");
    assert!(end_line.is_some(), "new method end_line is missing");
    
    let start_line = start_line.unwrap();
    let end_line = end_line.unwrap();
    
    println!("new method: lines {} to {}", start_line, end_line);
    
    // new method should start around line 17 and end around line 22
    assert!(start_line >= 17 && start_line <= 18, "new method should start around line 17-18, but got {}", start_line);
    assert!(end_line >= 22 && end_line <= 23, "new method should end around line 22-23, but got {}", end_line);
    
    // Test greet method (without doc comments)
    let greet_method = find_symbol_by_name(&analysis, "greet");
    assert!(greet_method.is_some(), "greet method not found");
    
    let greet_method = greet_method.unwrap();
    let (start_line, end_line) = get_symbol_lines(greet_method);
    
    assert!(start_line.is_some(), "greet method start_line is missing");
    assert!(end_line.is_some(), "greet method end_line is missing");
    
    let start_line = start_line.unwrap();
    let end_line = end_line.unwrap();
    
    println!("greet method: lines {} to {}", start_line, end_line);
    
    // greet method should start around line 25 and end around line 27
    assert!(start_line >= 25 && start_line <= 26, "greet method should start around line 25-26, but got {}", start_line);
    assert!(end_line >= 27 && end_line <= 28, "greet method should end around line 27-28, but got {}", end_line);
    
    // Test have_birthday method
    let birthday_method = find_symbol_by_name(&analysis, "have_birthday");
    assert!(birthday_method.is_some(), "have_birthday method not found");
    
    let birthday_method = birthday_method.unwrap();
    let (start_line, end_line) = get_symbol_lines(birthday_method);
    
    assert!(start_line.is_some(), "have_birthday method start_line is missing");
    assert!(end_line.is_some(), "have_birthday method end_line is missing");
    
    let start_line = start_line.unwrap();
    let end_line = end_line.unwrap();
    
    println!("have_birthday method: lines {} to {}", start_line, end_line);
    
    // have_birthday method should start around line 30 and end around line 38
    assert!(start_line >= 30 && start_line <= 31, "have_birthday method should start around line 30-31, but got {}", start_line);
    assert!(end_line >= 38 && end_line <= 39, "have_birthday method should end around line 38-39, but got {}", end_line);
    
    // Test documented_function
    let documented_function = find_symbol_by_name(&analysis, "documented_function");
    assert!(documented_function.is_some(), "documented_function not found");
    
    let documented_function = documented_function.unwrap();
    let (start_line, end_line) = get_symbol_lines(documented_function);
    
    assert!(start_line.is_some(), "documented_function start_line is missing");
    assert!(end_line.is_some(), "documented_function end_line is missing");
    
    let start_line = start_line.unwrap();
    let end_line = end_line.unwrap();
    
    println!("documented_function: lines {} to {}", start_line, end_line);
    
    // documented_function should start around line 55 and end around line 59
    assert!(start_line >= 55 && start_line <= 56, "documented_function should start around line 55-56, but got {}", start_line);
    assert!(end_line >= 59 && end_line <= 60, "documented_function should end around line 59-60, but got {}", end_line);
}

#[test]
fn test_python_classes_and_functions_line_numbers() {
    let dir = tempdir().unwrap();
    
    // Create a Python file with classes and functions, with and without comments
    let python_content = r#"""
A test module for demonstrating line number detection
"""

import math
from typing import Optional

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
    
    # Simple method without docstring
    def multiply(self, x: float) -> float:
        self.value *= x
        self.history.append(f"Multiplied by {x}")
        return self.value
    
    def complex_calculation(self, x: float, y: float) -> float:
        """
        Perform a complex calculation involving multiple operations.
        
        Args:
            x: First operand
            y: Second operand
            
        Returns:
            The result of the calculation
        """
        temp = self.value
        for i in range(int(x)):
            temp += y * math.sin(i)
            if temp > 100:
                temp = temp / 2
        
        self.value = temp
        self.history.append(f"Complex calculation with {x}, {y}")
        return self.value
    
    def get_history(self) -> list:
        return self.history.copy()

# Simple function without docstring
def simple_function():
    print("This is a simple function")
    return 42

def documented_function(a: int, b: int) -> int:
    """
    A function with proper documentation.
    
    This function demonstrates proper documentation
    and spans multiple lines.
    
    Args:
        a: First integer
        b: Second integer
        
    Returns:
        The sum of a and b
    """
    result = a + b
    print(f"Adding {a} + {b} = {result}")
    return result

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
"#;
    
    let python_file_path = create_temp_file(&dir, "test.py", python_content);
    
    // Analyze the Python code
    let input = AnalyzeCodeInput {
        file_path: python_file_path.to_str().unwrap().to_string(),
    };
    
    let result = analyze_code_handler(input);
    assert!(result.is_ok(), "Failed to analyze Python code: {:?}", result.err());
    
    let analysis = result.unwrap();
    println!("Analysis result: {}", serde_json::to_string_pretty(&analysis).unwrap());
    
    // Test Calculator class
    let calculator_class = find_symbol_by_name(&analysis, "Calculator");
    assert!(calculator_class.is_some(), "Calculator class not found");
    
    let calculator_class = calculator_class.unwrap();
    let (start_line, end_line) = get_symbol_lines(calculator_class);
    
    assert!(start_line.is_some(), "Calculator class start_line is missing");
    assert!(end_line.is_some(), "Calculator class end_line is missing");
    
    let start_line = start_line.unwrap();
    let end_line = end_line.unwrap();
    
    println!("Calculator class: lines {} to {}", start_line, end_line);
    
    // Calculator class should start around line 8 and end around line 48
    assert!(start_line >= 8 && start_line <= 9, "Calculator class should start around line 8-9, but got {}", start_line);
    assert!(end_line >= 48 && end_line <= 50, "Calculator class should end around line 48-50, but got {}", end_line);
    
    // Test __init__ method
    let init_method = find_symbol_by_name(&analysis, "__init__");
    assert!(init_method.is_some(), "__init__ method not found");
    
    let init_method = init_method.unwrap();
    let (start_line, end_line) = get_symbol_lines(init_method);
    
    assert!(start_line.is_some(), "__init__ method start_line is missing");
    assert!(end_line.is_some(), "__init__ method end_line is missing");
    
    let start_line = start_line.unwrap();
    let end_line = end_line.unwrap();
    
    println!("__init__ method: lines {} to {}", start_line, end_line);
    
    // __init__ method should start around line 11 and end around line 14
    assert!(start_line >= 11 && start_line <= 12, "__init__ method should start around line 11-12, but got {}", start_line);
    assert!(end_line >= 14 && end_line <= 15, "__init__ method should end around line 14-15, but got {}", end_line);
    
    // Test add method
    let add_method = find_symbol_by_name(&analysis, "add");
    assert!(add_method.is_some(), "add method not found");
    
    let add_method = add_method.unwrap();
    let (start_line, end_line) = get_symbol_lines(add_method);
    
    assert!(start_line.is_some(), "add method start_line is missing");
    assert!(end_line.is_some(), "add method end_line is missing");
    
    let start_line = start_line.unwrap();
    let end_line = end_line.unwrap();
    
    println!("add method: lines {} to {}", start_line, end_line);
    
    // add method should start around line 16 and end around line 20
    assert!(start_line >= 16 && start_line <= 17, "add method should start around line 16-17, but got {}", start_line);
    assert!(end_line >= 20 && end_line <= 21, "add method should end around line 20-21, but got {}", end_line);
    
    // Test multiply method (without docstring)
    let multiply_method = find_symbol_by_name(&analysis, "multiply");
    assert!(multiply_method.is_some(), "multiply method not found");
    
    let multiply_method = multiply_method.unwrap();
    let (start_line, end_line) = get_symbol_lines(multiply_method);
    
    assert!(start_line.is_some(), "multiply method start_line is missing");
    assert!(end_line.is_some(), "multiply method end_line is missing");
    
    let start_line = start_line.unwrap();
    let end_line = end_line.unwrap();
    
    println!("multiply method: lines {} to {}", start_line, end_line);
    
    // multiply method should start around line 23 and end around line 26
    assert!(start_line >= 23 && start_line <= 24, "multiply method should start around line 23-24, but got {}", start_line);
    assert!(end_line >= 26 && end_line <= 27, "multiply method should end around line 26-27, but got {}", end_line);
}

#[test]
fn test_other_languages_basic_line_numbers() {
    let dir = tempdir().unwrap();
    
    // Test JavaScript
    let js_content = r#"// JavaScript test file

/**
 * A simple class for testing
 */
class Calculator {
    constructor(initialValue = 0) {
        this.value = initialValue;
    }
    
    /**
     * Add a number to the current value
     */
    add(x) {
        this.value += x;
        return this.value;
    }
    
    // Simple method without JSDoc
    multiply(x) {
        this.value *= x;
        return this.value;
    }
}

// Simple function
function simpleFunction() {
    console.log("Hello from JavaScript");
}

/**
 * A documented function
 */
function documentedFunction(a, b) {
    const result = a + b;
    console.log(`${a} + ${b} = ${result}`);
    return result;
}
"#;
    
    let js_file_path = create_temp_file(&dir, "test.js", js_content);
    
    let input = AnalyzeCodeInput {
        file_path: js_file_path.to_str().unwrap().to_string(),
    };
    
    let result = analyze_code_handler(input);
    assert!(result.is_ok(), "Failed to analyze JavaScript code: {:?}", result.err());
    
    let analysis = result.unwrap();
    println!("JavaScript analysis result: {}", serde_json::to_string_pretty(&analysis).unwrap());
    
    // Test Calculator class
    let calculator_class = find_symbol_by_name(&analysis, "Calculator");
    if let Some(calculator_class) = calculator_class {
        let (start_line, end_line) = get_symbol_lines(calculator_class);
        println!("JavaScript Calculator class: lines {:?} to {:?}", start_line, end_line);
        
        if let (Some(start), Some(end)) = (start_line, end_line) {
            assert!(start >= 6 && start <= 7, "Calculator class should start around line 6-7, but got {}", start);
            assert!(end >= 24 && end <= 26, "Calculator class should end around line 24-26, but got {}", end);
        }
    }
    
    // Test Go
    let go_content = r#"package main

import "fmt"

// Calculator represents a simple calculator
type Calculator struct {
    value float64
}

// NewCalculator creates a new calculator instance
func NewCalculator(initialValue float64) *Calculator {
    return &Calculator{
        value: initialValue,
    }
}

// Add adds a number to the current value
func (c *Calculator) Add(x float64) float64 {
    c.value += x
    return c.value
}

// Simple method without comments
func (c *Calculator) Multiply(x float64) float64 {
    c.value *= x
    return c.value
}

// SimpleFunction is a simple function
func SimpleFunction() {
    fmt.Println("Hello from Go")
}

// DocumentedFunction demonstrates a documented function
func DocumentedFunction(a, b int) int {
    result := a + b
    fmt.Printf("%d + %d = %d\n", a, b, result)
    return result
}
"#;
    
    let go_file_path = create_temp_file(&dir, "test.go", go_content);
    
    let input = AnalyzeCodeInput {
        file_path: go_file_path.to_str().unwrap().to_string(),
    };
    
    let result = analyze_code_handler(input);
    assert!(result.is_ok(), "Failed to analyze Go code: {:?}", result.err());
    
    let analysis = result.unwrap();
    println!("Go analysis result: {}", serde_json::to_string_pretty(&analysis).unwrap());
    
    // Test Calculator struct
    let calculator_struct = find_symbol_by_name(&analysis, "Calculator");
    if let Some(calculator_struct) = calculator_struct {
        let (start_line, end_line) = get_symbol_lines(calculator_struct);
        println!("Go Calculator struct: lines {:?} to {:?}", start_line, end_line);
        
        if let (Some(start), Some(end)) = (start_line, end_line) {
            assert!(start >= 6 && start <= 7, "Calculator struct should start around line 6-7, but got {}", start);
            assert!(end >= 8 && end <= 9, "Calculator struct should end around line 8-9, but got {}", end);
        }
    }
}