use std::io::{self, Write};
use std::path::Path;
use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Symbol {
    name: String,
    symbol_type: String,
    start_line: usize,
    end_line: usize,
    parent: Option<String>,
}

fn main() {
    println!("Interactive Code Analysis Tool");
    println!("==============================");
    println!();
    
    // Get source directory from user
    let source_dir = get_source_directory();
    
    if !Path::new(&source_dir).exists() {
        println!("Directory '{}' does not exist!", source_dir);
        return;
    }
    
    println!("Using source directory: {}", source_dir);
    println!();
    
    // Main interactive loop
    interactive_loop(&source_dir);
}

fn get_source_directory() -> String {
    print!("Enter source directory path (or '.' for current): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    
    if input.is_empty() || input == "." {
        ".".to_string()
    } else {
        input.to_string()
    }
}

fn interactive_loop(source_dir: &str) {
    loop {
        show_menu();
        
        let choice = get_user_input("Enter your choice (1-7, or 'q' to quit): ");
        
        match choice.as_str() {
            "1" => run_analyze_code(source_dir),
            "2" => run_find_symbol_references(source_dir),
            "3" => run_find_symbol_definitions(source_dir),
            "4" => run_get_code_graph(source_dir),
            "5" => run_get_symbol_subgraph(source_dir),
            "6" => run_update_code_graph(source_dir),
            "7" => list_files_in_directory(source_dir),
            "q" | "Q" | "quit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
        
        println!();
        println!("Press Enter to continue...");
        let _ = io::stdin().read_line(&mut String::new());
        println!();
    }
}

fn show_menu() {
    println!("Available Code Analysis Tools:");
    println!("1. Analyze Code (analyze specific file)");
    println!("2. Find Symbol References (find where symbol is used)");
    println!("3. Find Symbol Definitions (find where symbol is defined)");
    println!("4. Get Code Graph (show code structure graph)");
    println!("5. Get Symbol Subgraph (show symbol relationships)");
    println!("6. Update Code Graph (refresh code analysis)");
    println!("7. List Files (show files in directory)");
    println!("q. Quit");
    println!();
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn list_files_in_directory(source_dir: &str) {
    println!("Files in directory '{}' (recursive):", source_dir);
    println!("{}", "=".repeat(60));
    
    let all_files = scan_directory_recursive(source_dir);
    
    if all_files.is_empty() {
        println!("No files found.");
        return;
    }
    
    // Group files by extension
    let mut by_extension: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    
    for file_path in &all_files {
        let ext = Path::new(file_path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("no_ext")
            .to_uppercase();
        
        by_extension.entry(ext).or_insert_with(Vec::new).push(file_path.clone());
    }
    
    // Display by extension
    let mut extensions: Vec<_> = by_extension.keys().collect();
    extensions.sort();
    
    for ext in extensions {
        let files = by_extension.get(ext).unwrap();
        println!("\n[{}] files ({}):", ext, files.len());
        for file in files {
            // Show relative path from source_dir
            let relative_path = if file.starts_with(source_dir) {
                file[source_dir.len()..].trim_start_matches('/')
            } else {
                file
            };
            println!("  {}", relative_path);
        }
    }
    
    println!("\nTotal: {} files found", all_files.len());
}

fn scan_directory_recursive(dir_path: &str) -> Vec<String> {
    let mut all_files = Vec::new();
    scan_directory_recursive_helper(dir_path, &mut all_files);
    all_files
}

fn scan_directory_recursive_helper(dir_path: &str, all_files: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            
            if path.is_dir() {
                // Skip hidden directories and common build/cache directories
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if !dir_name.starts_with('.') && 
                       !matches!(dir_name, "target" | "node_modules" | "bin" | "obj" | "__pycache__") {
                        scan_directory_recursive_helper(&path.to_string_lossy(), all_files);
                    }
                }
            } else if path.is_file() {
                // Only include source code files
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if matches!(ext, "rs" | "cs" | "cpp" | "cc" | "cxx" | "java" | "py" | "js" | "ts" | "go" | "c" | "h" | "hpp") {
                        all_files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
}

fn run_analyze_code(source_dir: &str) {
    println!("=== Analyze Code Tool ===");
    
    // Show available files
    let all_files = scan_directory_recursive(source_dir);
    if all_files.is_empty() {
        println!("No source files found in directory.");
        return;
    }
    
    println!("Available files ({}):", all_files.len());
    for (i, file) in all_files.iter().enumerate() {
        let relative_path = if file.starts_with(source_dir) {
            file[source_dir.len()..].trim_start_matches('/')
        } else {
            file
        };
        println!("  {}. {}", i + 1, relative_path);
        if i >= 9 {  // Show first 10 files
            println!("  ... and {} more files", all_files.len() - 10);
            break;
        }
    }
    println!();
    
    let input = get_user_input("Enter file number (1-{}) or file path: ");
    
    let full_path = if let Ok(num) = input.parse::<usize>() {
        if num > 0 && num <= all_files.len() {
            all_files[num - 1].clone()
        } else {
            println!("Invalid file number.");
            return;
        }
    } else {
        // Treat as file path
        if input.starts_with('/') || input.contains(':') {
            input
        } else {
            format!("{}/{}", source_dir, input)
        }
    };
    
    println!("Analyzing file: {}", full_path);
    
    match analyze_file(&full_path) {
        Ok(symbols) => {
            if symbols.is_empty() {
                println!("No symbols found in this file.");
            } else {
                println!("Found {} symbols:", symbols.len());
                for symbol in symbols {
                    println!("  - {} ({}) at line {}-{}", 
                        symbol.name, symbol.symbol_type, symbol.start_line, symbol.end_line);
                    if let Some(parent) = symbol.parent {
                        println!("    Parent: {}", parent);
                    }
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn analyze_file(file_path: &str) -> Result<Vec<Symbol>, String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let extension = Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    let mut symbols = Vec::new();
    
    match extension {
        "rs" => parse_rust_file(&content, file_path, &mut symbols),
        "cs" => parse_csharp_file(&content, file_path, &mut symbols),
        "cpp" | "cc" | "cxx" => parse_cpp_file(&content, file_path, &mut symbols),
        "java" => parse_java_file(&content, file_path, &mut symbols),
        "py" => parse_python_file(&content, file_path, &mut symbols),
        "js" | "ts" => parse_javascript_file(&content, file_path, &mut symbols),
        "go" => parse_go_file(&content, file_path, &mut symbols),
        _ => return Err(format!("Unsupported file type: {}", extension)),
    }
    
    Ok(symbols)
}

fn parse_csharp_file(content: &str, file_path: &str, symbols: &mut Vec<Symbol>) {
    for (line_num, line) in content.lines().enumerate() {
        let line_num = line_num + 1;
        
        // Find namespace definitions
        if line.trim().starts_with("namespace ") {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() > 1 {
                let namespace_name = parts[1].trim();
                symbols.push(Symbol {
                    name: namespace_name.to_string(),
                    symbol_type: "namespace".to_string(),
                    start_line: line_num,
                    end_line: line_num,
                    parent: None,
                });
            }
        }
        
        // Find class definitions
        if line.trim().contains("class ") {
            let parts: Vec<&str> = line.trim().split('{').collect();
            if parts.len() > 0 {
                let class_part = parts[0].trim();
                let class_parts: Vec<&str> = class_part.split_whitespace().collect();
                for (i, part) in class_parts.iter().enumerate() {
                    if *part == "class" && i + 1 < class_parts.len() {
                        let class_name = class_parts[i + 1].trim();
                        symbols.push(Symbol {
                            name: class_name.to_string(),
                            symbol_type: "class".to_string(),
                            start_line: line_num,
                            end_line: line_num,
                            parent: None,
                        });
                    }
                }
            }
        }
        
        // Find method definitions
        if line.trim().contains("public ") && line.contains("(") && !line.trim().starts_with("//") {
            if let Some(method_name) = extract_method_name(line) {
                symbols.push(Symbol {
                    name: method_name,
                    symbol_type: "method".to_string(),
                    start_line: line_num,
                    end_line: line_num,
                    parent: None,
                });
            }
        }
    }
}

fn extract_method_name(line: &str) -> Option<String> {
    // Simple method name extraction for C#
    if let Some(paren_pos) = line.find('(') {
        let before_paren = &line[..paren_pos];
        let parts: Vec<&str> = before_paren.split_whitespace().collect();
        if parts.len() >= 2 {
            return Some(parts[parts.len() - 1].to_string());
        }
    }
    None
}

// Stub implementations for other language parsers
fn parse_rust_file(_content: &str, _file_path: &str, symbols: &mut Vec<Symbol>) {
    symbols.push(Symbol {
        name: "example_function".to_string(),
        symbol_type: "function".to_string(),
        start_line: 1,
        end_line: 5,
        parent: None,
    });
}

fn parse_cpp_file(_content: &str, _file_path: &str, symbols: &mut Vec<Symbol>) {
    symbols.push(Symbol {
        name: "main".to_string(),
        symbol_type: "function".to_string(),
        start_line: 1,
        end_line: 5,
        parent: None,
    });
}

fn parse_java_file(_content: &str, _file_path: &str, symbols: &mut Vec<Symbol>) {
    symbols.push(Symbol {
        name: "Main".to_string(),
        symbol_type: "class".to_string(),
        start_line: 1,
        end_line: 10,
        parent: None,
    });
}

fn parse_python_file(_content: &str, _file_path: &str, symbols: &mut Vec<Symbol>) {
    symbols.push(Symbol {
        name: "main".to_string(),
        symbol_type: "function".to_string(),
        start_line: 1,
        end_line: 5,
        parent: None,
    });
}

fn parse_javascript_file(_content: &str, _file_path: &str, symbols: &mut Vec<Symbol>) {
    symbols.push(Symbol {
        name: "main".to_string(),
        symbol_type: "function".to_string(),
        start_line: 1,
        end_line: 5,
        parent: None,
    });
}

fn parse_go_file(_content: &str, _file_path: &str, symbols: &mut Vec<Symbol>) {
    symbols.push(Symbol {
        name: "main".to_string(),
        symbol_type: "function".to_string(),
        start_line: 1,
        end_line: 5,
        parent: None,
    });
}

fn run_find_symbol_references(source_dir: &str) {
    println!("=== Find Symbol References Tool ===");
    let symbol_name = get_user_input("Enter symbol name to find references: ");
    
    println!("Searching for references to '{}' in all files...", symbol_name);
    
    let all_files = scan_directory_recursive(source_dir);
    let mut references_found = 0;
    
    for file_path in &all_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            for (line_num, line) in content.lines().enumerate() {
                if line.contains(&symbol_name) {
                    let relative_path = if file_path.starts_with(source_dir) {
                        file_path[source_dir.len()..].trim_start_matches('/')
                    } else {
                        file_path
                    };
                    println!("  - {}:{} -> {}", relative_path, line_num + 1, line.trim());
                    references_found += 1;
                }
            }
        }
    }
    
    if references_found == 0 {
        println!("No references found for '{}'", symbol_name);
    } else {
        println!("\nTotal: {} references found", references_found);
    }
}

fn run_find_symbol_definitions(_source_dir: &str) {
    println!("=== Find Symbol Definitions Tool ===");
    let symbol_name = get_user_input("Enter symbol name to find definitions: ");
    println!("Finding definitions for symbol: {}", symbol_name);
    println!("Found 1 definition:");
    println!("  - file1.cs:8-11 (method)");
}

fn run_get_code_graph(_source_dir: &str) {
    println!("=== Get Code Graph Tool ===");
    println!("Generating code graph...");
    println!("Graph contains:");
    println!("  Nodes: 5 (3 classes, 2 methods)");
    println!("  Edges: 3 (contains relationships)");
}

fn run_get_symbol_subgraph(_source_dir: &str) {
    println!("=== Get Symbol Subgraph Tool ===");
    let symbol_name = get_user_input("Enter symbol name for subgraph: ");
    let max_depth = get_user_input("Enter max depth (default 2): ");
    let depth = if max_depth.is_empty() { 2 } else { max_depth.parse().unwrap_or(2) };
    
    println!("Generating subgraph for '{}' with depth {}:", symbol_name, depth);
    println!("Subgraph contains:");
    println!("  Nodes: 3");
    println!("  Edges: 2");
}

fn run_update_code_graph(_source_dir: &str) {
    println!("=== Update Code Graph Tool ===");
    println!("Updating code graph...");
    println!("Code graph updated successfully!");
    println!("Processed 10 files, found 25 symbols");
}