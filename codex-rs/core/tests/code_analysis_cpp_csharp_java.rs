use std::path::PathBuf;
use std::fs;
use std::io::Write;
use tempfile::tempdir;

use codex_core::code_analysis::tools::{
    analyze_code_handler, AnalyzeCodeInput,
    get_code_graph_handler, GetCodeGraphInput,
};

// Helper function to create a temporary file with content
fn create_temp_file(dir: &tempfile::TempDir, filename: &str, content: &str) -> PathBuf {
    let file_path = dir.path().join(filename);
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file_path
}

#[test]
fn test_cpp_code_analysis() {
    let dir = tempdir().unwrap();
    
    // Create a simple C++ file
    let cpp_content = r#"
#include <iostream>
#include <string>

void helloWorld() {
    std::cout << "Hello, world!" << std::endl;
}

class Person {
private:
    std::string name;
    int age;

public:
    Person(const std::string& name, int age) : name(name), age(age) {}
    
    void greet() {
        std::cout << "Hello, my name is " << name << " and I am " << age << " years old." << std::endl;
    }
};
"#;
    
    let cpp_file_path = create_temp_file(&dir, "test.cpp", cpp_content);
    
    // Analyze the C++ code
    let input = AnalyzeCodeInput {
        file_path: cpp_file_path.to_str().unwrap().to_string(),
    };
    
    let result = analyze_code_handler(input);
    
    // Verify that the analysis found the expected symbols
    assert!(result.is_ok(), "Failed to analyze C++ code: {:?}", result.err());
    
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
    
    assert!(has_hello_world, "Did not find helloWorld function");
    assert!(has_person, "Did not find Person class");
}

#[test]
fn test_csharp_code_analysis() {
    let dir = tempdir().unwrap();
    
    // Create a simple C# file
    let csharp_content = r#"
using System;

namespace TestNamespace
{
    public class Person
    {
        public string Name { get; set; }
        public int Age { get; set; }
        
        public Person(string name, int age)
        {
            Name = name;
            Age = age;
        }
        
        public void Greet()
        {
            Console.WriteLine($"Hello, my name is {Name} and I am {Age} years old.");
        }
    }
    
    public static class Program
    {
        public static void HelloWorld()
        {
            Console.WriteLine("Hello, world!");
        }
        
        public static void Main(string[] args)
        {
            HelloWorld();
            var person = new Person("John", 30);
            person.Greet();
        }
    }
}
"#;
    
    let csharp_file_path = create_temp_file(&dir, "test.cs", csharp_content);
    
    // Analyze the C# code
    let input = AnalyzeCodeInput {
        file_path: csharp_file_path.to_str().unwrap().to_string(),
    };
    
    let result = analyze_code_handler(input);
    
    // Verify that the analysis found the expected symbols
    assert!(result.is_ok(), "Failed to analyze C# code: {:?}", result.err());
    
    let analysis = result.unwrap();
    let symbols = analysis.get("symbols").expect("No symbols found in analysis");
    
    // Check that we found the namespace, class, and methods
    let symbols_array = symbols.as_array().expect("Symbols is not an array");
    
    let has_namespace = symbols_array.iter().any(|s| {
        s.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "TestNamespace")
    });
    
    let has_person = symbols_array.iter().any(|s| {
        s.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "Person")
    });
    
    let has_program = symbols_array.iter().any(|s| {
        s.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "Program")
    });
    
    assert!(has_namespace, "Did not find TestNamespace namespace");
    assert!(has_person, "Did not find Person class");
    assert!(has_program, "Did not find Program class");
}

#[test]
fn test_java_code_analysis() {
    let dir = tempdir().unwrap();
    
    // Create a simple Java file
    let java_content = r#"
package test;

public class Person {
    private String name;
    private int age;
    
    public Person(String name, int age) {
        this.name = name;
        this.age = age;
    }
    
    public void greet() {
        System.out.println("Hello, my name is " + name + " and I am " + age + " years old.");
    }
    
    public static void helloWorld() {
        System.out.println("Hello, world!");
    }
    
    public static void main(String[] args) {
        helloWorld();
        Person person = new Person("John", 30);
        person.greet();
    }
}
"#;
    
    let java_file_path = create_temp_file(&dir, "Person.java", java_content);
    
    // Analyze the Java code
    let input = AnalyzeCodeInput {
        file_path: java_file_path.to_str().unwrap().to_string(),
    };
    
    let result = analyze_code_handler(input);
    
    // Verify that the analysis found the expected symbols
    assert!(result.is_ok(), "Failed to analyze Java code: {:?}", result.err());
    
    let analysis = result.unwrap();
    let symbols = analysis.get("symbols").expect("No symbols found in analysis");
    
    // Check that we found the package, class, and methods
    let symbols_array = symbols.as_array().expect("Symbols is not an array");
    
    let has_package = symbols_array.iter().any(|s| {
        s.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "test")
    });
    
    let has_person = symbols_array.iter().any(|s| {
        s.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "Person")
    });
    
    let has_hello_world = symbols_array.iter().any(|s| {
        s.get("name").map_or(false, |name| name.as_str().unwrap_or("") == "helloWorld")
    });
    
    assert!(has_package, "Did not find test package");
    assert!(has_person, "Did not find Person class");
    assert!(has_hello_world, "Did not find helloWorld method");
}