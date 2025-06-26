#[cfg(test)]
mod tests {
    use serde_json::json;
    use codex_core::code_analysis::tools::{
        update_code_graph_handler,
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

}