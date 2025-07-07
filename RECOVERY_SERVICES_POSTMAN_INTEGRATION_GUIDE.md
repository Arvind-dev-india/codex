# Recovery Services Postman Integration Guide

## Overview

This guide explains how to integrate the Recovery Services Vault API Postman collection documentation into your development workflow and LLM-assisted coding projects.

## Project Structure

```
recovery-services-postman-docs/
├── RECOVERY_SERVICES_POSTMAN_DOCUMENTATION.md    # Main API documentation
├── RECOVERY_SERVICES_POSTMAN_INTEGRATION_GUIDE.md # This guide
├── postman-collection/
│   ├── collection.json                           # Raw Postman collection
│   ├── environment.json                          # Environment variables
│   └── examples/                                 # Request/response examples
├── scripts/
│   ├── extract_postman_docs.py                  # Documentation extractor
│   ├── validate_endpoints.py                    # Endpoint validation
│   └── generate_code_samples.py                 # Code sample generator
└── templates/
    ├── curl_templates.md                        # cURL command templates
    ├── powershell_templates.md                  # PowerShell templates
    └── python_templates.md                      # Python SDK templates
```

## Integration with Existing Codex Project

### 1. Merge with Existing Documentation

The new Postman-based documentation complements your existing Recovery Services documentation:

- **RECOVERY_SERVICES_API_REFERENCE.md** - Your comprehensive API reference
- **RECOVERY_SERVICES_POSTMAN_DOCUMENTATION.md** - Postman collection-based documentation
- **RECOVERY_SERVICES_IMPLEMENTATION_COMPLETE.md** - Implementation status

### 2. Update Tool Implementations

Use the Postman documentation to enhance your existing tools in `codex-rs/core/src/recovery_services/`:

```rust
// Example: Update tools_impl.rs with Postman-verified endpoints
pub async fn recovery_services_list_vaults(
    &self,
    subscription_id: Option<String>,
    resource_group: Option<String>,
    filter: Option<String>,
) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    // Use Postman-documented endpoint structure
    let url = format!(
        "https://management.azure.com/subscriptions/{}/providers/Microsoft.RecoveryServices/vaults",
        subscription_id.unwrap_or_else(|| self.config.subscription_id.clone())
    );
    
    let mut params = vec![("api-version", "2023-04-01")];
    if let Some(f) = filter {
        params.push(("$filter", &f));
    }
    
    // Implementation follows Postman collection patterns
    self.client.get(&url, &params).await
}
```

### 3. Code Generation Templates

Create templates based on the Postman documentation for automatic code generation:

#### cURL Template
```bash
#!/bin/bash
# Generated from Postman collection
# Operation: {{operation_name}}
# Description: {{description}}

curl -X {{method}} \
  '{{url}}' \
  {{#each headers}}
  -H '{{key}}: {{value}}' \
  {{/each}}
  {{#if body}}
  -d '{{body}}' \
  {{/if}}
```

#### Python Template
```python
# Generated from Postman collection
import requests
import json

def {{function_name}}({{parameters}}):
    """
    {{description}}
    
    Args:
        {{#each parameters}}
        {{name}} ({{type}}): {{description}}
        {{/each}}
    
    Returns:
        dict: API response
    """
    url = "{{url}}"
    headers = {
        {{#each headers}}
        "{{key}}": "{{value}}",
        {{/each}}
    }
    
    {{#if body}}
    data = {{body}}
    response = requests.{{method_lower}}(url, headers=headers, json=data)
    {{else}}
    response = requests.{{method_lower}}(url, headers=headers)
    {{/if}}
    
    response.raise_for_status()
    return response.json()
```

## LLM Integration Strategies

### 1. Context Injection

Use the Postman documentation as context for LLM interactions:

```python
def get_recovery_services_context():
    """Load Recovery Services API context for LLM."""
    with open('RECOVERY_SERVICES_POSTMAN_DOCUMENTATION.md', 'r') as f:
        postman_docs = f.read()
    
    with open('RECOVERY_SERVICES_API_REFERENCE.md', 'r') as f:
        api_reference = f.read()
    
    context = f"""
    Azure Recovery Services API Documentation:
    
    {postman_docs}
    
    Additional Reference:
    {api_reference}
    
    Use this documentation to generate accurate Azure Backup API calls.
    """
    return context
```

### 2. Prompt Templates

Create specialized prompts for different use cases:

#### API Call Generation
```
Given the Azure Recovery Services API documentation above, generate a {{language}} function that:
- {{operation_description}}
- Includes proper error handling
- Follows the exact API endpoint structure from the Postman collection
- Uses the correct HTTP method and parameters
- Includes authentication headers

Requirements:
- Function name: {{function_name}}
- Parameters: {{parameters}}
- Return type: {{return_type}}
```

#### Troubleshooting
```
Based on the Recovery Services API documentation, help troubleshoot this issue:
- Operation: {{operation}}
- Error: {{error_message}}
- Status Code: {{status_code}}

Provide:
1. Likely cause of the error
2. Steps to resolve
3. Corrected API call example
4. Prevention strategies
```

### 3. Automated Validation

Create scripts to validate API implementations against the Postman collection:

```python
#!/usr/bin/env python3
"""
Validate Recovery Services API implementations against Postman collection.
"""

import json
import requests
from typing import Dict, List

class PostmanValidator:
    def __init__(self, collection_path: str):
        with open(collection_path, 'r') as f:
            self.collection = json.load(f)
    
    def validate_endpoint(self, endpoint: str, method: str) -> Dict:
        """Validate an endpoint against the Postman collection."""
        # Find matching endpoint in collection
        for item in self.collection.get('item', []):
            if self._matches_endpoint(item, endpoint, method):
                return self._validate_structure(item)
        
        return {"valid": False, "error": "Endpoint not found in collection"}
    
    def _matches_endpoint(self, item: Dict, endpoint: str, method: str) -> bool:
        """Check if item matches the endpoint and method."""
        request = item.get('request', {})
        item_method = request.get('method', '').upper()
        item_url = request.get('url', {})
        
        if isinstance(item_url, dict):
            item_endpoint = item_url.get('raw', '')
        else:
            item_endpoint = str(item_url)
        
        return (item_method == method.upper() and 
                self._normalize_url(item_endpoint) == self._normalize_url(endpoint))
    
    def _normalize_url(self, url: str) -> str:
        """Normalize URL for comparison."""
        # Remove query parameters and normalize path variables
        base_url = url.split('?')[0]
        # Replace path variables with placeholders
        import re
        normalized = re.sub(r'\{[^}]+\}', '{param}', base_url)
        return normalized
    
    def _validate_structure(self, item: Dict) -> Dict:
        """Validate the structure of an API item."""
        request = item.get('request', {})
        
        validation = {
            "valid": True,
            "warnings": [],
            "errors": []
        }
        
        # Check required fields
        if not request.get('method'):
            validation["errors"].append("Missing HTTP method")
        
        if not request.get('url'):
            validation["errors"].append("Missing URL")
        
        # Check headers
        headers = request.get('header', [])
        auth_header = any(h.get('key', '').lower() == 'authorization' for h in headers)
        if not auth_header:
            validation["warnings"].append("Missing Authorization header")
        
        validation["valid"] = len(validation["errors"]) == 0
        return validation

def main():
    validator = PostmanValidator('postman-collection/collection.json')
    
    # Validate common endpoints
    endpoints_to_validate = [
        ("GET", "https://management.azure.com/subscriptions/{subscriptionId}/providers/Microsoft.RecoveryServices/vaults"),
        ("POST", "https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/{containerName}/protectedItems/{protectedItemName}/backup"),
    ]
    
    for method, endpoint in endpoints_to_validate:
        result = validator.validate_endpoint(endpoint, method)
        print(f"{method} {endpoint}: {'✓' if result['valid'] else '✗'}")
        if result.get('warnings'):
            for warning in result['warnings']:
                print(f"  Warning: {warning}")
        if result.get('errors'):
            for error in result['errors']:
                print(f"  Error: {error}")

if __name__ == "__main__":
    main()
```

## Best Practices

### 1. Documentation Maintenance

- **Regular Updates**: Sync with the latest Postman collection monthly
- **Version Control**: Track changes in API endpoints and parameters
- **Validation**: Automatically validate documentation against live APIs

### 2. Code Generation

- **Template-Based**: Use consistent templates for different languages
- **Error Handling**: Include comprehensive error handling in generated code
- **Testing**: Generate test cases alongside implementation code

### 3. LLM Optimization

- **Structured Format**: Keep documentation in a consistent, parseable format
- **Complete Examples**: Include full request/response examples
- **Context Separation**: Separate different types of operations clearly

## Usage Examples

### Generate Python SDK Function

```bash
# Using the documentation with an LLM
codex "Based on the Recovery Services Postman documentation, generate a Python function to list all backup jobs with filtering support. Include proper error handling and type hints."
```

### Create PowerShell Script

```bash
codex "Create a PowerShell script that uses the Recovery Services API to enable backup protection for a VM. Use the exact endpoint structure from the Postman collection."
```

### Troubleshoot API Issues

```bash
codex "I'm getting a 400 error when trying to trigger a backup using the Recovery Services API. Based on the Postman documentation, what could be wrong with my request?"
```

## Integration with Existing Tools

### Update MCP Server

Enhance your Recovery Services MCP server with Postman-verified endpoints:

```rust
// In codex-rs/recovery-services-server/src/tool_config.rs
pub fn get_postman_verified_tools() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "recovery_services_list_vaults_postman".to_string(),
            description: "List Recovery Services vaults using Postman-verified endpoint".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "subscription_id": {"type": "string"},
                    "filter": {"type": "string", "description": "OData filter expression"}
                }
            })
        },
        // Add more Postman-verified tools
    ]
}
```

### Enhance Documentation

Add cross-references between your existing documentation and the Postman collection:

```markdown
## See Also

- [Postman Collection Documentation](RECOVERY_SERVICES_POSTMAN_DOCUMENTATION.md) - Complete API reference from official Postman collection
- [Implementation Guide](RECOVERY_SERVICES_IMPLEMENTATION_COMPLETE.md) - Current implementation status
- [API Reference](RECOVERY_SERVICES_API_REFERENCE.md) - Comprehensive API documentation
```

## Conclusion

This integration guide provides a comprehensive approach to leveraging the Recovery Services Vault API Postman collection for enhanced development workflows, LLM-assisted coding, and automated documentation maintenance. The structured approach ensures consistency, accuracy, and maintainability of your Recovery Services integration.