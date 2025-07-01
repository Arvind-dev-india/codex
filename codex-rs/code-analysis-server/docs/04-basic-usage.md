# Basic Usage

This guide shows how to use the Code Analysis Server in standalone mode.

## Command Line Options

The server accepts the following options:

- `--project-dir` or `-p`: Directory to analyze (required)
- `--verbose` or `-v`: Enable detailed logging
- `--port`: Network port (0 = stdin/stdout mode, default)

## Running the Server

### Linux/macOS
```bash
# Basic usage
./code-analysis-server --project-dir /path/to/your/project

# With verbose logging
./code-analysis-server --project-dir /path/to/your/project --verbose

# Analyze current directory
./code-analysis-server --project-dir .
```

### Windows
```cmd
# Basic usage
code-analysis-server.exe --project-dir C:\path\to\your\project

# With verbose logging
code-analysis-server.exe --project-dir C:\path\to\your\project --verbose

# Analyze current directory
code-analysis-server.exe --project-dir .
```

## What Happens When You Run It

1. **Initialization**: The server scans the project directory
2. **Parsing**: It analyzes supported code files using Tree-sitter
3. **Graph Building**: Creates a symbol reference graph
4. **Ready**: Waits for MCP commands via stdin

## Supported File Types

The server automatically detects and analyzes:
- Python (`.py`)
- JavaScript/TypeScript (`.js`, `.ts`, `.jsx`, `.tsx`)
- Rust (`.rs`)
- C/C++ (`.c`, `.cpp`, `.h`, `.hpp`)
- C# (`.cs`)
- Java (`.java`)
- Go (`.go`)

## Example Output

When you run the server, you'll see something like:
```
INFO code_analysis_server: Starting Code Analysis MCP Server
INFO code_analysis_server: Initializing code graph for: /path/to/project
INFO code_analysis_server: Code analysis complete: 150 nodes, 89 edges, 95% parsed (45/47 files)
```

The server is now ready to receive MCP commands.

**Next:** [Available Tools](05-available-tools.md)