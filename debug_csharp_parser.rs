use codex_core::code_analysis::context_extractor::{ContextExtractor, SymbolType};
use codex_core::code_analysis::{get_parser_pool, SupportedLanguage, QueryType};
use std::fs;
use tempfile::tempdir;

fn main() {
    println!("=== DEBUGGING C# PARSER CAPABILITIES ===");
    
    // Test 1: Basic class detection
    test_basic_class();
    
    // Test 2: Static class detection
    test_static_class();
    
    // Test 3: Interface detection
    test_interface();
    
    // Test 4: Record detection
    test_record();
    
    // Test 5: Top-level program
    test_top_level_program();
}

fn test_basic_class() {
    println!("\n--- Testing Basic Class ---");
    let content = r#"
namespace TestApp.Models
{
    public class User
    {
        public int Id { get; set; }
        public string Name { get; set; }
        
        public User() { }
        
        public User(int id, string name)
        {
            Id = id;
            Name = name;
        }
        
        public void DoSomething() { }
    }
}
"#;
    
    test_content("BasicClass.cs", content);
}

fn test_static_class() {
    println!("\n--- Testing Static Class ---");
    let content = r#"
namespace TestApp
{
    public static class StringExtensions
    {
        public static bool IsValidEmail(this string email)
        {
            return !string.IsNullOrEmpty(email) && email.Contains('@');
        }
    }
}
"#;
    
    test_content("StaticClass.cs", content);
}

fn test_interface() {
    println!("\n--- Testing Interface ---");
    let content = r#"
namespace TestApp
{
    public interface IRepository<T>
    {
        T GetById(int id);
        void Save(T entity);
    }
}
"#;
    
    test_content("Interface.cs", content);
}

fn test_record() {
    println!("\n--- Testing Record ---");
    let content = r#"
namespace TestApp
{
    public record UserRecord(int Id, string Name, string Email);
    
    public record struct Point(double X, double Y);
}
"#;
    
    test_content("Record.cs", content);
}

fn test_top_level_program() {
    println!("\n--- Testing Top-Level Program ---");
    let content = r#"
using System;

Console.WriteLine("Hello World");

static void LocalFunction()
{
    Console.WriteLine("Local function");
}

LocalFunction();
"#;
    
    test_content("TopLevel.cs", content);
}

fn test_content(filename: &str, content: &str) {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join(filename);
    
    fs::write(&file_path, content).expect("Failed to write test file");
    
    // Test language detection
    let language = SupportedLanguage::from_path(&file_path);
    println!("Language detected: {:?}", language);
    
    // Test parser pool parsing
    let parser_pool = get_parser_pool();
    let result = parser_pool.parse_file_from_disk(file_path.to_str().unwrap());
    
    match result {
        Ok(parsed_file) => {
            println!("✅ Parsing successful");
            
            // Test query execution
            let query_result = parsed_file.execute_predefined_query(QueryType::All);
            match query_result {
                Ok(matches) => {
                    println!("Found {} query matches", matches.len());
                    for (i, match_) in matches.iter().enumerate() {
                        println!("  Match {}: pattern {}", i, match_.pattern_index);
                        for capture in &match_.captures {
                            println!("    Capture '{}': '{}' at {}:{}-{}:{}", 
                                     capture.name, capture.text, 
                                     capture.start_point.0, capture.start_point.1,
                                     capture.end_point.0, capture.end_point.1);
                        }
                    }
                }
                Err(e) => println!("❌ Query execution failed: {:?}", e),
            }
            
            // Test context extractor
            let mut extractor = ContextExtractor::new();
            let result = extractor.extract_symbols_from_file(file_path.to_str().unwrap());
            
            match result {
                Ok(_) => {
                    let symbols = extractor.get_symbols();
                    println!("Found {} symbols:", symbols.len());
                    for (fqn, symbol) in symbols {
                        println!("  {} -> {} ({:?}) at lines {}-{}", 
                                 fqn, symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
                    }
                }
                Err(e) => println!("❌ Symbol extraction failed: {:?}", e),
            }
        }
        Err(e) => println!("❌ Parsing failed: {:?}", e),
    }
}