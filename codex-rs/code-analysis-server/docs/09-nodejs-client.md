# Node.js Client Example

This guide shows how to create a simple Node.js client for the Code Analysis Server.

## Basic Node.js Client

Save this as `code-analysis-client.js`:

```javascript
#!/usr/bin/env node

const { spawn } = require('child_process');
const readline = require('readline');
const path = require('path');
const fs = require('fs');

class CodeAnalysisClient {
  constructor(serverPath, projectDir) {
    this.serverProcess = spawn(serverPath, ['--project-dir', projectDir], {
      stdio: ['pipe', 'pipe', 'pipe']
    });
    
    this.rl = readline.createInterface({
      input: this.serverProcess.stdout,
      crlfDelay: Infinity
    });
    
    this.pendingRequests = new Map();
    this.nextRequestId = 1;
    
    // Set up response handler
    this.rl.on('line', (line) => {
      try {
        const response = JSON.parse(line);
        const { id } = response;
        
        if (this.pendingRequests.has(id)) {
          const { resolve, reject } = this.pendingRequests.get(id);
          this.pendingRequests.delete(id);
          resolve(response);
        }
      } catch (error) {
        console.error('Error parsing response:', error);
      }
    });
    
    // Handle server errors
    this.serverProcess.stderr.on('data', (data) => {
      console.error(`Server error: ${data}`);
    });
    
    // Initialize the server
    this.initialize();
  }
  
  async sendRequest(method, params) {
    return new Promise((resolve, reject) => {
      const id = `req-${this.nextRequestId++}`;
      
      const request = {
        jsonrpc: '2.0',
        id,
        method,
        params
      };
      
      this.pendingRequests.set(id, { resolve, reject });
      
      this.serverProcess.stdin.write(JSON.stringify(request) + '\n');
    });
  }
  
  async initialize() {
    return this.sendRequest('initialize', {
      protocol_version: '2025-03-26'
    });
  }
  
  async listTools() {
    return this.sendRequest('tools/list', {});
  }
  
  async analyzeCode(filePath) {
    return this.sendRequest('tools/call', {
      name: 'analyze_code',
      arguments: {
        file_path: filePath
      }
    });
  }
  
  async findReferences(symbolName, symbolType = null) {
    const arguments = { symbol_name: symbolName };
    if (symbolType) {
      arguments.symbol_type = symbolType;
    }
    
    return this.sendRequest('tools/call', {
      name: 'find_symbol_references',
      arguments
    });
  }
  
  async findDefinitions(symbolName, symbolType = null) {
    const arguments = { symbol_name: symbolName };
    if (symbolType) {
      arguments.symbol_type = symbolType;
    }
    
    return this.sendRequest('tools/call', {
      name: 'find_symbol_definitions',
      arguments
    });
  }
  
  async getSymbolSubgraph(symbolName, depth = 2) {
    return this.sendRequest('tools/call', {
      name: 'get_symbol_subgraph',
      arguments: {
        symbol_name: symbolName,
        depth
      }
    });
  }
  
  close() {
    this.serverProcess.stdin.end();
    this.serverProcess.kill();
  }
}

// Command line interface
async function main() {
  const args = process.argv.slice(2);
  
  // Parse command line arguments
  const serverPath = args.find((_, i) => args[i-1] === '--server' || args[i-1] === '-s');
  const projectDir = args.find((_, i) => args[i-1] === '--project' || args[i-1] === '-p');
  const action = args.find((_, i) => args[i-1] === '--action' || args[i-1] === '-a');
  const filePath = args.find((_, i) => args[i-1] === '--file' || args[i-1] === '-f');
  const symbolName = args.find((_, i) => args[i-1] === '--symbol' || args[i-1] === '-n');
  const symbolType = args.find((_, i) => args[i-1] === '--type' || args[i-1] === '-t');
  const depthArg = args.find((_, i) => args[i-1] === '--depth' || args[i-1] === '-d');
  const depth = depthArg ? parseInt(depthArg, 10) : 2;
  
  if (!serverPath || !projectDir || !action) {
    console.error('Required arguments: --server, --project, --action');
    process.exit(1);
  }
  
  const client = new CodeAnalysisClient(serverPath, projectDir);
  
  try {
    let result;
    
    switch (action) {
      case 'list-tools':
        result = await client.listTools();
        break;
        
      case 'analyze':
        if (!filePath) {
          console.error('--file is required for analyze action');
          process.exit(1);
        }
        result = await client.analyzeCode(filePath);
        break;
        
      case 'find-refs':
        if (!symbolName) {
          console.error('--symbol is required for find-refs action');
          process.exit(1);
        }
        result = await client.findReferences(symbolName, symbolType);
        break;
        
      case 'find-defs':
        if (!symbolName) {
          console.error('--symbol is required for find-defs action');
          process.exit(1);
        }
        result = await client.findDefinitions(symbolName, symbolType);
        break;
        
      case 'subgraph':
        if (!symbolName) {
          console.error('--symbol is required for subgraph action');
          process.exit(1);
        }
        result = await client.getSymbolSubgraph(symbolName, depth);
        break;
        
      default:
        console.error(`Unknown action: ${action}`);
        process.exit(1);
    }
    
    console.log(JSON.stringify(result, null, 2));
  } finally {
    client.close();
  }
}

// Run the main function
if (require.main === module) {
  main().catch(error => {
    console.error('Error:', error);
    process.exit(1);
  });
}

module.exports = CodeAnalysisClient;
```

## Usage Examples

```bash
# Make the script executable
chmod +x code-analysis-client.js

# List available tools
./code-analysis-client.js --server ./code-analysis-server --project /path/to/project --action list-tools

# Analyze a file
./code-analysis-client.js --server ./code-analysis-server --project /path/to/project --action analyze --file src/main.js

# Find references to a symbol
./code-analysis-client.js --server ./code-analysis-server --project /path/to/project --action find-refs --symbol UserService --type class

# Find definitions of a symbol
./code-analysis-client.js --server ./code-analysis-server --project /path/to/project --action find-defs --symbol processData

# Get symbol subgraph
./code-analysis-client.js --server ./code-analysis-server --project /path/to/project --action subgraph --symbol DatabaseManager --depth 3
```

## Using as a Module

You can also use the client as a module in your Node.js applications:

```javascript
const CodeAnalysisClient = require('./code-analysis-client');

async function analyzeProject() {
  const client = new CodeAnalysisClient('./code-analysis-server', '/path/to/project');
  
  try {
    // List tools
    const tools = await client.listTools();
    console.log('Available tools:', tools.result.tools.map(t => t.name));
    
    // Analyze a file
    const analysis = await client.analyzeCode('src/main.js');
    console.log('Analysis result:', analysis);
    
    // Find references
    const refs = await client.findReferences('UserService');
    console.log('References:', refs);
  } finally {
    client.close();
  }
}

analyzeProject().catch(console.error);
```

**Next:** [Troubleshooting](10-troubleshooting.md)