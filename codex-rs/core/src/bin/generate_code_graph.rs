use std::env;
use std::fs;
use std::path::Path;
use std::time::Instant;
use serde::{Serialize, Deserialize};

use codex_core::code_analysis::repo_mapper::RepoMapper;

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
    
    // Create a repository mapper and map the repository
    let mut repo_mapper = RepoMapper::new(Path::new(source_dir));
    
    println!("Starting repository analysis...");
    let start_time = Instant::now();
    
    match repo_mapper.map_repository() {
        Ok(()) => {
            let duration = start_time.elapsed();
            println!("Successfully mapped repository: {} in {:.2?}", source_dir, duration);
            
            // Get and display parsing statistics
            let (total, successful, failed, _failed_files) = repo_mapper.get_parsing_statistics();
            println!("Parsing summary: {}/{} files processed successfully ({} failed)", 
                     successful, total, failed);
            
            if total > 0 {
                let files_per_second = total as f64 / duration.as_secs_f64();
                println!("Processing rate: {:.1} files/second", files_per_second);
            }
        },
        Err(e) => {
            eprintln!("Error mapping repository: {}", e);
            std::process::exit(1);
        }
    }
    
    // Get the code reference graph
    let graph = repo_mapper.get_graph();
    
    // Convert nodes to the expected format
    let mut nodes = Vec::new();
    for node in &graph.nodes {
        let symbol_type = match node.node_type {
            codex_core::code_analysis::repo_mapper::CodeNodeType::File => "File",
            codex_core::code_analysis::repo_mapper::CodeNodeType::Function => "Function",
            codex_core::code_analysis::repo_mapper::CodeNodeType::Method => "Method",
            codex_core::code_analysis::repo_mapper::CodeNodeType::Class => "Class",
            codex_core::code_analysis::repo_mapper::CodeNodeType::Struct => "Struct",
            codex_core::code_analysis::repo_mapper::CodeNodeType::Module => "Module",
        };
        
        nodes.push(GraphNode {
            id: node.id.clone(),
            name: node.name.clone(),
            symbol_type: symbol_type.to_string(),
            file_path: node.file_path.clone(),
            start_line: node.start_line,
            end_line: node.end_line,
            parent: None, // TODO: Extract parent information if needed
        });
    }
    
    // Convert edges to the expected format
    let mut edges = Vec::new();
    for edge in &graph.edges {
        let edge_type = match edge.edge_type {
            codex_core::code_analysis::repo_mapper::CodeEdgeType::Calls => "Call",
            codex_core::code_analysis::repo_mapper::CodeEdgeType::Imports => "Import",
            codex_core::code_analysis::repo_mapper::CodeEdgeType::Inherits => "Inheritance",
            codex_core::code_analysis::repo_mapper::CodeEdgeType::Contains => "Contains",
            codex_core::code_analysis::repo_mapper::CodeEdgeType::References => "Usage",
        };
        
        edges.push(GraphEdge {
            source: edge.source.clone(),
            target: edge.target.clone(),
            edge_type: edge_type.to_string(),
        });
    }
    
    // Create the final graph structure
    let final_graph = CodeGraph {
        nodes,
        edges,
    };
    
    // Write the graph to a JSON file
    let json = serde_json::to_string_pretty(&final_graph).unwrap();
    fs::write(&output_path, json).unwrap_or_else(|e| {
        eprintln!("Error writing output file: {}", e);
        std::process::exit(1);
    });
    
    println!("Graph data written to {}", output_path);
    println!("Total nodes: {}", final_graph.nodes.len());
    println!("Total edges: {}", final_graph.edges.len());
}

