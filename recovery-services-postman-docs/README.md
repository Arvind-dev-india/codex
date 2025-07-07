# Recovery Services Postman Documentation Project

## Overview

This project provides comprehensive documentation and tooling for the Azure Recovery Services Vault API based on the official Microsoft Postman collection. It's designed to be LLM-friendly and supports automated code generation, validation, and integration workflows.

## Project Structure

```
recovery-services-postman-docs/
├── README.md                                     # This file
├── RECOVERY_SERVICES_POSTMAN_DOCUMENTATION.md   # Main API documentation
├── RECOVERY_SERVICES_POSTMAN_INTEGRATION_GUIDE.md # Integration guide
├── postman-collection/
│   ├── collection.json                          # Raw Postman collection
│   ├── environment.json                         # Environment variables
│   └── README.md                                # Collection documentation
├── scripts/
│   ├── extract_postman_docs.py                 # Documentation extractor
│   ├── validate_endpoints.py                   # Endpoint validation
│   ├── generate_code_samples.py                # Code sample generator
│   ├── sync_with_postman.py                    # Sync with latest collection
│   └── README.md                               # Scripts documentation
├── templates/
│   ├── curl_templates.md                       # cURL command templates
│   ├── powershell_templates.md                 # PowerShell templates
│   ├── python_templates.md                     # Python SDK templates
│   ├── csharp_templates.md                     # C# SDK templates
│   └── README.md                               # Templates documentation
└── examples/
    ├── complete_workflows/                      # End-to-end examples
    ├── error_handling/                          # Error handling examples
    ├── authentication/                          # Auth examples
    └── README.md                               # Examples documentation
```

## Features

### 🤖 LLM-Optimized Documentation
- Structured markdown format for easy parsing
- Complete API reference with examples
- Comprehensive error handling documentation
- Step-by-step workflow guides

### 🔧 Automation Tools
- Postman collection extraction and parsing
- Endpoint validation against live APIs
- Automated code sample generation
- Documentation synchronization scripts

### 📚 Code Templates
- Multi-language support (Python, PowerShell, C#, cURL)
- Consistent error handling patterns
- Authentication integration examples
- Best practices implementation

### 🧪 Examples and Workflows
- Complete backup and restore workflows
- Real-world use case implementations
- Error handling and troubleshooting guides
- Integration patterns for different scenarios

## Quick Start

### 1. Extract Latest Documentation

```bash
cd scripts
python extract_postman_docs.py --collection-id 198900 --output ../postman-collection/
```

### 2. Generate Code Samples

```bash
python generate_code_samples.py --language python --output ../examples/python/
python generate_code_samples.py --language powershell --output ../examples/powershell/
```

### 3. Validate Endpoints

```bash
python validate_endpoints.py --collection ../postman-collection/collection.json
```

## Usage with LLMs

### Context Injection

Use the documentation as context for LLM interactions:

```python
# Load Recovery Services API context
with open('RECOVERY_SERVICES_POSTMAN_DOCUMENTATION.md', 'r') as f:
    api_docs = f.read()

prompt = f"""
{api_docs}

Based on the above Azure Recovery Services API documentation, help me:
[Your specific request here]
"""
```

### Code Generation

Generate implementation code using the templates:

```bash
# Generate Python function for listing vaults
codex "Using the Recovery Services API documentation, create a Python function that lists all Recovery Services vaults with proper error handling and type hints."

# Generate PowerShell script for VM backup
codex "Create a PowerShell script that enables backup protection for an Azure VM using the exact API endpoints from the documentation."
```

### Troubleshooting

Get help with API issues:

```bash
codex "I'm getting a 403 Forbidden error when calling the Recovery Services API. Based on the documentation, what permissions do I need?"
```

## Integration with Existing Projects

### Codex Integration

This project complements the existing Codex Recovery Services implementation:

1. **Enhanced Documentation**: Provides Postman-verified API reference
2. **Code Generation**: Templates for generating new tool implementations
3. **Validation**: Scripts to validate existing implementations
4. **Examples**: Real-world usage patterns and workflows

### MCP Server Enhancement

Use the documentation to enhance MCP server implementations:

```rust
// Example: Enhanced tool with Postman-verified parameters
pub async fn recovery_services_list_vaults_enhanced(
    subscription_id: Option<String>,
    resource_group: Option<String>,
    filter: Option<String>,
    top: Option<i32>,
) -> Result<serde_json::Value, RecoveryServicesError> {
    // Implementation based on Postman collection structure
}
```

## Documentation Standards

### API Endpoint Documentation

Each endpoint includes:
- HTTP method and URL
- Path variables with descriptions
- Query parameters with examples
- Request headers
- Request body schema
- Response examples
- Error codes and handling
- Complete cURL examples

### Code Examples

All code examples include:
- Proper error handling
- Type annotations (where applicable)
- Authentication setup
- Parameter validation
- Response processing

### Workflow Documentation

Complete workflows include:
- Prerequisites and setup
- Step-by-step instructions
- Error handling at each step
- Validation and verification
- Cleanup procedures

## Contributing

### Adding New Endpoints

1. Update the Postman collection
2. Run the extraction script
3. Validate the new endpoints
4. Generate code samples
5. Update documentation

### Improving Templates

1. Identify common patterns
2. Create reusable templates
3. Test with multiple scenarios
4. Document template usage
5. Update generation scripts

### Enhancing Examples

1. Identify real-world use cases
2. Create complete implementations
3. Test thoroughly
4. Document prerequisites
5. Add troubleshooting guides

## Maintenance

### Regular Updates

- **Weekly**: Check for Postman collection updates
- **Monthly**: Validate all endpoints against live APIs
- **Quarterly**: Review and update code templates
- **As needed**: Add new examples and use cases

### Quality Assurance

- Automated endpoint validation
- Code sample testing
- Documentation consistency checks
- LLM compatibility verification

## Support and Resources

### Official Documentation
- [Azure Backup REST API](https://docs.microsoft.com/en-us/rest/api/backup/)
- [Recovery Services REST API](https://docs.microsoft.com/en-us/rest/api/recoveryservices/)
- [Postman Collection](https://documenter.getpostman.com/view/198900/RztoMoMS)

### Related Projects
- [Codex Recovery Services Implementation](../codex-rs/recovery-services-server/)
- [Azure DevOps Integration](../AZURE_DEVOPS_INTEGRATION.md)
- [Kusto Integration](../KUSTO_INTEGRATION.md)

### Community
- Report issues and suggestions
- Contribute improvements and examples
- Share use cases and workflows

---

*This project is designed to make Azure Recovery Services API integration easier, more reliable, and more accessible for both human developers and AI assistants.*