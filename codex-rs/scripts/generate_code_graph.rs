use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::json;

use codex_core::code_analysis::context_extractor::{ContextExtractor, CodeSymbol, SymbolReference};

#[derive(Serialize, Deserialize)]
struct GraphNode {
    id: String,
    name: String,
    symbol_type: String,
    file_path: String,
    start_line: usize,
    end_line: usize,
    parent: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct GraphEdge {
    source: String,
    target: String,
    edge_type: String,
}

#[derive(Serialize, Deserialize)]
struct CodeGraph {
    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,
}

fn main() {
    // Get the source directory path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <source_directory>", args[0]);
        std::process::exit(1);
    }
    
    let source_dir = &args[1];
    let output_path = if args.len() > 2 {
        args[2].clone()
    } else {
        "code_graph.json".to_string()
    };
    
    // Create a context extractor
    let mut extractor = ContextExtractor::new();
    
    // Process all files in the directory recursively
    process_directory(&mut extractor, Path::new(source_dir)).unwrap_or_else(|e| {
        eprintln!("Error processing directory: {}", e);
        std::process::exit(1);
    });
    
    // Get all symbols and references
    let symbols = extractor.get_symbols();
    let references = extractor.get_references();
    
    // Create graph nodes from symbols
    let mut nodes = Vec::new();
    for (fqn, symbol) in symbols {
        nodes.push(GraphNode {
            id: fqn.clone(),
            name: symbol.name.clone(),
            symbol_type: format!("{:?}", symbol.symbol_type),
            file_path: symbol.file_path.clone(),
            start_line: symbol.start_line,
            end_line: symbol.end_line,
            parent: symbol.parent.clone(),
        });
    }
    
    // Create graph edges from references
    let mut edges = Vec::new();
    for reference in references {
        if !reference.symbol_fqn.is_empty() {
            edges.push(GraphEdge {
                source: reference.reference_file.clone() + "::" + &reference.reference_line.to_string(),
                target: reference.symbol_fqn.clone(),
                edge_type: format!("{:?}", reference.reference_type),
            });
        }
    }
    
    // Create the graph
    let graph = CodeGraph {
        nodes,
        edges,
    };
    
    // Write the graph to a JSON file
    let json = serde_json::to_string_pretty(&graph).unwrap();
    fs::write(&output_path, json).unwrap_or_else(|e| {
        eprintln!("Error writing output file: {}", e);
        std::process::exit(1);
    });
    
    println!("Graph data written to {}", output_path);
}

fn process_directory(extractor: &mut ContextExtractor, dir_path: &Path) -> Result<(), String> {
    if !dir_path.is_dir() {
        return Err(format!("{} is not a directory", dir_path.display()));
    }
    
    for entry in fs::read_dir(dir_path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        
        if path.is_dir() {
            // Skip hidden directories and target directories
            let dir_name = path.file_name().unwrap().to_string_lossy();
            if !dir_name.starts_with(".") && dir_name != "target" {
                process_directory(extractor, &path)?;
            }
        } else if path.is_file() {
            // Process only source code files
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if ["rs", "py", "js", "ts", "java", "cs", "cpp", "h", "hpp", "go"].contains(&ext_str.as_str()) {
                    // Extract symbols from the file
                    match extractor.extract_symbols_from_file(path.to_str().unwrap()) {
                        Ok(_) => println!("Processed {}", path.display()),
                        Err(e) => eprintln!("Error processing {}: {}", path.display(), e),
                    }
                }
            }
        }
    }
    
    Ok(())
}