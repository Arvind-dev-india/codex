#[cfg(test)]
mod tests {
    use serde_json::json;
    use codex_core::code_analysis::tools::{
        get_code_graph_handler,
        update_code_graph_handler,
        GetCodeGraphInput,
        UpdateCodeGraphInput,
    };

    #[test]
    fn test_update_code_graph() {
        // Create a simple input
        let input = UpdateCodeGraphInput {
            root_path: Some(".".to_string()),
        };
        
        // Call the handler
        let result = update_code_graph_handler(input);
        
        // Check that the result is successful
        assert!(result.is_ok());
        
        // Parse the result
        let value = result.unwrap();
        let obj = value.as_object().unwrap();
        
        // Check that the status is success
        assert_eq!(obj.get("status").unwrap().as_str().unwrap(), "success");
        
        // Check that the root_path is present
        assert!(obj.contains_key("root_path"));
    }

    #[test]
    fn test_get_code_graph() {
        // Create a simple input
        let input = GetCodeGraphInput {
            root_path: ".".to_string(),
            include_files: None,
            exclude_patterns: None,
        };
        
        // Call the handler
        let result = get_code_graph_handler(input);
        
        // Check that the result is successful
        assert!(result.is_ok());
        
        // Parse the result
        let value = result.unwrap();
        let obj = value.as_object().unwrap();
        
        // Check that the graph is present
        assert!(obj.contains_key("graph"));
        let graph = obj.get("graph").unwrap().as_object().unwrap();
        assert!(graph.contains_key("nodes"));
        assert!(graph.contains_key("edges"));
        
        // Check that there are nodes and edges
        let nodes = graph.get("nodes").unwrap().as_array().unwrap();
        let edges = graph.get("edges").unwrap().as_array().unwrap();
        assert!(!nodes.is_empty());
    }
}