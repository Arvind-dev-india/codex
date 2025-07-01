use codex_core::code_analysis::get_parser_pool;
use std::fs;
use tempfile::tempdir;
use tree_sitter::StreamingIterator;

fn print_ast_node(node: tree_sitter::Node, source: &str, depth: usize) {
    let indent = "  ".repeat(depth);
    let node_text = node.utf8_text(source.as_bytes()).unwrap_or("<error>");
    let preview = if node_text.len() > 50 {
        format!("{}...", &node_text[..47])
    } else {
        node_text.to_string()
    };
    
    println!("{}{}[{}:{}] {}: '{}'", 
             indent, 
             node.start_position().row + 1,
             node.start_position().column,
             node.end_position().column,
             node.kind(), 
             preview.replace('\n', "\\n"));
    
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            print_ast_node(child, source, depth + 1);
        }
    }
}

#[test]
fn debug_csharp_interface_abstract_ast() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("DebugInterfaceAbstract.cs");
    
    let content = r#"
interface ICalculator
{
    int Calculate(int x, int y);
}

abstract class BaseCalculator
{
    public abstract int Calculate(int x, int y);
}

class ConcreteCalculator : BaseCalculator
{
    public override int Calculate(int x, int y)
    {
        return x + y;
    }
}
"#;
    
    fs::write(&file_path, content).expect("Failed to write test file");
    
    let parser_pool = get_parser_pool();
    let parsed_file = parser_pool.parse_file_from_disk(file_path.to_str().unwrap()).unwrap();
    
    println!("=== FULL AST STRUCTURE ===");
    print_ast_node(parsed_file.tree.root_node(), &parsed_file.source, 0);
    
    // Test current query
    let query_source = std::fs::read_to_string("src/code_analysis/queries/csharp.scm").unwrap();
    let language = &parsed_file.tree.language();
    let query = tree_sitter::Query::new(language, &query_source).unwrap();
    let mut cursor = tree_sitter::QueryCursor::new();
    let mut matches = cursor.matches(&query, parsed_file.tree.root_node(), parsed_file.source.as_bytes());
    
    println!("\n=== QUERY MATCHES ===");
    while let Some(m) = matches.next() {
        for capture in m.captures {
            let node = capture.node;
            let text = &parsed_file.source[node.byte_range()];
            println!("  {}: {} ({}:{})", 
                query.capture_names()[capture.index as usize],
                text,
                node.start_position().row + 1,
                node.start_position().column + 1
            );
        }
    }
}