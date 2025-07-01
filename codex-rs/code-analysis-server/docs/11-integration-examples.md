# Integration Examples

This guide provides examples of integrating the Code Analysis Server with various tools and systems.

## Integration with VS Code Extension

Here's how to integrate the Code Analysis Server with a Visual Studio Code extension:

```javascript
// Extension activation function
function activate(context) {
  // Register commands
  context.subscriptions.push(
    vscode.commands.registerCommand('codeAnalysis.startServer', startServer),
    vscode.commands.registerCommand('codeAnalysis.analyzeFile', analyzeCurrentFile),
    vscode.commands.registerCommand('codeAnalysis.findReferences', findReferences)
  );
  
  // Status bar item
  const statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left);
  statusBarItem.text = "$(search) Code Analysis";
  statusBarItem.command = 'codeAnalysis.startServer';
  statusBarItem.show();
  context.subscriptions.push(statusBarItem);
}

// Start the server
async function startServer() {
  const serverProcess = spawn('./code-analysis-server', [
    '--project-dir', vscode.workspace.rootPath
  ]);
  
  // Initialize communication
  // ...
}
```

## Integration with CI/CD Pipeline

### GitHub Actions Example

```yaml
name: Code Analysis

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  analyze:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Download Code Analysis Server
        run: |
          curl -L -o code-analysis-server.tar.gz https://github.com/your-repo/releases/download/v0.1.0/code-analysis-server-linux-x64.tar.gz
          tar -xzf code-analysis-server.tar.gz
      
      - name: Run Analysis
        run: |
          ./code-analysis-server --project-dir . > /dev/null &
          SERVER_PID=$!
          sleep 2  # Wait for server to start
          
          # Use NPX MCP Tool to run analysis
          npx @modelcontextprotocol/inspector --server-pid=$SERVER_PID \
            --script <(echo 'tools/call analyze_code {"file_path": "src/main.py"}') \
            --output analysis.json
          
          kill $SERVER_PID
      
      - name: Upload Results
        uses: actions/upload-artifact@v3
        with:
          name: code-analysis
          path: analysis.json
```

## Integration with Web Application

### React Component Example

```jsx
import React, { useState, useEffect } from 'react';

function CodeAnalysisComponent({ filePath }) {
  const [analysis, setAnalysis] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    async function fetchAnalysis() {
      try {
        // Call your backend API that interfaces with the Code Analysis Server
        const response = await fetch(`/api/code-analysis?file=${encodeURIComponent(filePath)}`);
        const data = await response.json();
        setAnalysis(data);
      } catch (err) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    }
    
    fetchAnalysis();
  }, [filePath]);

  if (loading) return <div>Loading analysis...</div>;
  if (error) return <div>Error: {error}</div>;
  
  return (
    <div className="code-analysis">
      <h2>Code Analysis for {filePath}</h2>
      <div className="symbols">
        <h3>Symbols</h3>
        <ul>
          {analysis.symbols.map(symbol => (
            <li key={symbol.id}>
              {symbol.name} ({symbol.type}) - Line {symbol.line}
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}
```

### Backend API Example (Node.js/Express)

```javascript
const express = require('express');
const { spawn } = require('child_process');
const app = express();

// Start the Code Analysis Server when the API server starts
let serverProcess;
function startServer() {
  serverProcess = spawn('./code-analysis-server', ['--project-dir', './project']);
  // Handle server output...
}
startServer();

// API endpoint to analyze a file
app.get('/api/code-analysis', async (req, res) => {
  const filePath = req.query.file;
  if (!filePath) {
    return res.status(400).json({ error: 'File path is required' });
  }
  
  try {
    // Send request to the server
    const result = await sendRequest('tools/call', {
      name: 'analyze_code',
      arguments: { file_path: filePath }
    });
    
    res.json(result);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Function to send requests to the Code Analysis Server
function sendRequest(method, params) {
  // Implementation...
}

// Clean up when the API server shuts down
process.on('exit', () => {
  if (serverProcess) {
    serverProcess.kill();
  }
});

app.listen(3000, () => {
  console.log('API server listening on port 3000');
});
```

## Integration with Language Server Protocol (LSP)

You can wrap the Code Analysis Server in an LSP server to integrate with any editor that supports LSP:

```typescript
import {
  createConnection,
  TextDocuments,
  ProposedFeatures,
  InitializeParams,
  TextDocumentSyncKind,
  InitializeResult
} from 'vscode-languageserver/node';

import { TextDocument } from 'vscode-languageserver-textdocument';
import { spawn } from 'child_process';

// Create a connection for the server
const connection = createConnection(ProposedFeatures.all);
const documents = new TextDocuments(TextDocument);

// Start the Code Analysis Server
const analysisServer = spawn('./code-analysis-server', ['--project-dir', '.']);

connection.onInitialize((params: InitializeParams): InitializeResult => {
  return {
    capabilities: {
      textDocumentSync: TextDocumentSyncKind.Incremental,
      // Add other capabilities as needed
      referencesProvider: true,
      definitionProvider: true
    }
  };
});

// Handle reference requests
connection.onReferences(async (params) => {
  const document = documents.get(params.textDocument.uri);
  if (!document) return null;
  
  // Get the word at the position
  const position = params.position;
  const text = document.getText();
  const lines = text.split('\n');
  const line = lines[position.line];
  const wordMatch = line.slice(0, position.character).match(/[a-zA-Z0-9_]+$/);
  if (!wordMatch) return null;
  
  const word = wordMatch[0];
  
  // Call the Code Analysis Server
  // Implementation...
  
  return null;
});

// Listen for document changes
documents.listen(connection);
connection.listen();
```

## Integration with JetBrains IDEs

For JetBrains IDEs (IntelliJ, PyCharm, etc.), you can create a plugin:

```kotlin
class CodeAnalysisAction : AnAction("Analyze Code") {
    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return
        val editor = e.getData(CommonDataKeys.EDITOR) ?: return
        val file = e.getData(CommonDataKeys.VIRTUAL_FILE) ?: return
        
        // Start the Code Analysis Server if not already running
        val serverProcess = ProcessBuilder("./code-analysis-server", "--project-dir", project.basePath!!)
            .redirectErrorStream(true)
            .start()
        
        // Send request to analyze the current file
        // Implementation...
        
        // Show results in a tool window
        ApplicationManager.getApplication().invokeLater {
            val toolWindow = ToolWindowManager.getInstance(project)
                .getToolWindow("Code Analysis") ?: return@invokeLater
            
            val content = toolWindow.contentManager.getFactory().createContent(
                JPanel(), "Analysis Results", false)
            toolWindow.contentManager.addContent(content)
            toolWindow.show()
        }
    }
}