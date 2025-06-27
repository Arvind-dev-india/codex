use codex_core::code_analysis::{get_parser_pool, SupportedLanguage};
use std::fs;
use tempfile::tempdir;

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
fn debug_csharp_method_call_ast() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("DebugCalls.cs");
    
    let content = r#"
public class Test
{
    public void MethodA()
    {
        MethodB();
        this.MethodC();
        Console.WriteLine("test");
    }
    
    public void MethodB() { }
    public void MethodC() { }
}
"#;
    
    fs::write(&file_path, content).expect("Failed to write test file");
    
    let parser_pool = get_parser_pool();
    let parsed_file = parser_pool.parse_file_from_disk(file_path.to_str().unwrap()).unwrap();
    
    println!("=== FULL AST STRUCTURE ===");
    print_ast_node(parsed_file.tree.root_node(), &parsed_file.source, 0);
}