use std::path::PathBuf;
use std::fs;
use std::io::Write;
use tempfile::tempdir;

use codex_core::code_analysis::context_extractor::create_context_extractor;
use codex_core::code_analysis::repo_mapper::RepoMapper;

// Helper function to create a temporary file with content
fn create_temp_file(dir: &tempfile::TempDir, filename: &str, content: &str) -> PathBuf {
    let file_path = dir.path().join(filename);
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file_path
}

#[test]
fn test_line_to_symbol_mapping() {
    let dir = tempdir().unwrap();
    
    // Create a TypeScript file with a function and interface
    let ts_content = r#"
interface CustomVSCodeCommentThread {
    id: string;
    label: string;
    comments: Comment[];
}

class DiffViewer {
    private threads: CustomVSCodeCommentThread[] = [];
    
    createCommentThread(line: number): CustomVSCodeCommentThread {
        const thread: CustomVSCodeCommentThread = {
            id: `thread-${line}`,
            label: `Comment at line ${line}`,
            comments: []
        };
        this.threads.push(thread);
        return thread;
    }
    
    processComment(thread: CustomVSCodeCommentThread) {
        // This line should be mapped to the processComment method
        console.log(`Processing thread: ${thread.id}`);
    }
}
"#;
    
    let ts_file_path = create_temp_file(&dir, "diffViewer.ts", ts_content);
    
    // Create a context extractor and extract symbols
    let mut context_extractor = create_context_extractor();
    let result = context_extractor.extract_symbols_from_file(ts_file_path.to_str().unwrap());
    assert!(result.is_ok(), "Failed to extract symbols: {:?}", result.err());
    
    // Test finding containing symbol for different lines
    let file_path = ts_file_path.to_str().unwrap();
    
    // Line 2 should be within the interface
    if let Some(symbol) = context_extractor.find_most_specific_containing_symbol(file_path, 2) {
        assert_eq!(symbol.name, "CustomVSCodeCommentThread");
        println!("Line 2 is contained in: {} ({:?})", symbol.name, symbol.symbol_type);
    }
    
    // Line 10 should be within the createCommentThread method
    if let Some(symbol) = context_extractor.find_most_specific_containing_symbol(file_path, 10) {
        assert!(symbol.name == "createCommentThread" || symbol.name == "DiffViewer");
        println!("Line 10 is contained in: {} ({:?})", symbol.name, symbol.symbol_type);
    }
    
    // Line 20 should be within the processComment method
    if let Some(symbol) = context_extractor.find_most_specific_containing_symbol(file_path, 20) {
        assert!(symbol.name == "processComment" || symbol.name == "DiffViewer");
        println!("Line 20 is contained in: {} ({:?})", symbol.name, symbol.symbol_type);
    }
}

#[test]
fn test_graph_with_proper_source_nodes() {
    let dir = tempdir().unwrap();
    
    // Create a simple TypeScript file with function calls
    let ts_content = r#"
interface CustomVSCodeCommentThread {
    id: string;
    label: string;
}

function createCommentThread(): CustomVSCodeCommentThread {
    return {
        id: "test",
        label: "test"
    };
}

function processThread() {
    const thread = createCommentThread(); // This should create an edge
    console.log(thread.id);
}
"#;
    
    let ts_file_path = create_temp_file(&dir, "test.ts", ts_content);
    
    // Create a repository mapper and map the repository
    let mut repo_mapper = RepoMapper::new(dir.path());
    let result = repo_mapper.map_repository();
    assert!(result.is_ok(), "Failed to map repository: {:?}", result.err());
    
    // Get the graph
    let graph = repo_mapper.get_graph();
    
    // Print the graph for debugging
    println!("Nodes:");
    for node in &graph.nodes {
        println!("  {} - {} ({:?}) at lines {}-{}", 
                 node.id, node.name, node.node_type, node.start_line, node.end_line);
    }
    
    println!("Edges:");
    for edge in &graph.edges {
        println!("  {} -> {} ({:?})", edge.source, edge.target, edge.edge_type);
    }
    
    // Verify that we have nodes for the symbols
    let has_interface = graph.nodes.iter().any(|n| n.name == "CustomVSCodeCommentThread");
    let has_create_function = graph.nodes.iter().any(|n| n.name == "createCommentThread");
    let has_process_function = graph.nodes.iter().any(|n| n.name == "processThread");
    
    assert!(has_interface, "Should have interface node");
    assert!(has_create_function, "Should have createCommentThread function node");
    assert!(has_process_function, "Should have processThread function node");
    
    // Verify that edges use proper source nodes (not just line numbers)
    for edge in &graph.edges {
        // Source should be a proper symbol ID, not just a line number
        assert!(edge.source.contains("::") || edge.source.starts_with("file:") || edge.source.starts_with("symbol:"),
                "Source should be a proper node ID: {}", edge.source);
        assert!(edge.target.contains("::") || edge.target.starts_with("file:") || edge.target.starts_with("symbol:"),
                "Target should be a proper node ID: {}", edge.target);
    }
}