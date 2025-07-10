# Interactive C# Skeleton Generation Test Tool

This tool provides an interactive interface for testing the C# skeleton generation functionality of the code analysis server.

## Features

- **Interactive Testing**: Start a server and interactively test skeleton generation for any C# file
- **Quality Analysis**: Automatic verification of skeleton quality with detailed scoring
- **File Discovery**: List and browse available C# files in your project
- **Flexible Configuration**: Customize project directory, server port, and token limits
- **Real-time Results**: See skeleton output immediately with formatting and analysis

## Usage

### Basic Usage

```bash
# Test files in current directory
cd codex-rs/scripts
python3 interactive_skeleton_test.py

# Test files in a specific directory
python3 interactive_skeleton_test.py --project-dir ../test_files/csharp_test_suite

# Use a custom port
python3 interactive_skeleton_test.py --port 3001

# Just list available files
python3 interactive_skeleton_test.py --list-only
```

### Interactive Commands

Once the tool starts, you can use these commands:

- **`search`** - **🔍 Fuzzy search and select files** (Primary method)
  ```
  search                      # Start interactive fuzzy search
  # Then type: "basic" to find BasicClass.cs
  # Or type: "user" to find User.cs, UserService.cs, etc.
  # Or type: "5" to select file #5 from the list
  ```

- **`skeleton [file]`** - Generate skeleton for a file
  ```
  skeleton BasicClass.cs      # Generate for specific file
  skeleton                    # Start fuzzy search if no file specified
  ```

- **`analyze [file]`** - Generate skeleton and show quality analysis
  ```
  analyze SkeletonTestExample.cs  # Analyze specific file
  analyze                         # Start fuzzy search if no file specified
  ```

- **`show [file] [lines]`** - Generate and display skeleton content
  ```
  show BasicClass.cs           # Show first 50 lines
  show BasicClass.cs 100       # Show first 100 lines
  show                         # Start fuzzy search if no file specified
  ```

- **`tokens <number>`** - Set maximum tokens for skeleton generation
  ```
  tokens 8000                  # Set max tokens to 8000
  tokens                       # Show current token limit
  ```

- **`list [pattern]`** - List all files (fallback method)
  ```
  list                    # List all .cs files
  list *.cs              # List C# files
  list Models/*.cs       # List files in Models directory
  ```

- **`help`** - Show available commands
- **`quit`** - Exit the tool

## Example Session

```bash
$ python3 interactive_skeleton_test.py --project-dir ../test_files/csharp_test_suite

🚀 Starting code analysis server...
   Project directory: /home/user/codex/test_files/csharp_test_suite
   Server port: 3000
✅ Server started successfully!
🔧 Initializing MCP server...
✅ MCP server initialized

🎮 Interactive Skeleton Testing Mode
Commands:
  search             - Fuzzy search and select file
  skeleton <file>    - Generate skeleton for file
  analyze <file>     - Generate and analyze skeleton
  show <file> [lines] - Generate and display skeleton
  tokens <number>    - Set max tokens (default: 6000)
  list [pattern]     - List all files (fallback)
  help              - Show this help
  quit              - Exit

[skeleton-test] search

🔍 Fuzzy File Search (19 files available)
Type to search, press Enter to select first match, or type number to select:
Commands: 'list' to show all files, 'quit' to cancel

🔍 Search: basic

📋 Search results for 'basic':
   1. BasicClass.cs (1066 bytes)

✅ Auto-selected: BasicClass.cs
📄 Generating skeleton for: BasicClass.cs
   Max tokens: 6000

=== SKELETON CONTENT ===
  1: using System;
  2: using System.Collections.Generic;
  3: 
  4: // Lines 4-46
  5: // symbol: TestNamespace {
  6:     // Lines 9-45
  7:     public class BasicClass {
  8:         // ...
  9:     }
 10: 
 11:     // Lines 12-12
 12:     public string PublicProperty { get; set; } {
 13:         // ...
 14:     }
... (18 more lines)
==================================================

=== SKELETON ANALYSIS ===
File: BasicClass.cs
Original: 1066 chars, 46 lines
Skeleton: 425 chars, 28 lines
Tokens used: 177
Compression: 39.9%

📋 Quality Checks:
   ✅ Line numbers
   ✅ Using statements
   ✅ Namespace/symbols
   ✅ Class definitions
   ✅ Method signatures
   ✅ Property signatures
   ✅ Ellipsis usage
   ✅ Code structure

🎯 Quality Score: 8/8
🎉 EXCELLENT skeleton quality!

[skeleton-test] search

🔍 Search: user

📋 Search results for 'user':
   1. Models/User.cs (1544 bytes)
   2. Services/UserService.cs (4677 bytes)
   3. Services/IUserService.cs (1076 bytes)
   4. Controllers/UserController.cs (6566 bytes)

Select file (1-4) or press Enter for #1: 2

✅ Selected: Services/UserService.cs
📄 Generating skeleton for: Services/UserService.cs
   Max tokens: 6000
... (skeleton output)

[skeleton-test] quit
👋 Goodbye!
```

## Quality Analysis

The tool automatically analyzes skeleton quality based on these criteria:

- **Line numbers**: Presence of `// Lines X-Y` comments
- **Using statements**: Preservation of import statements
- **Namespace/symbols**: Correct namespace or symbol structure
- **Class definitions**: Preservation of class signatures
- **Method signatures**: Preservation of method declarations
- **Property signatures**: Preservation of property declarations
- **Ellipsis usage**: Replacement of implementation with `...`
- **Code structure**: Maintenance of code structure with braces

### Quality Scores

- **8/8**: 🎉 EXCELLENT skeleton quality
- **6-7/8**: ✅ GOOD skeleton quality
- **<6/8**: ⚠️ Skeleton quality could be improved

## Requirements

- Python 3.6+
- `requests` library (`pip install requests`)
- Built `code-analysis-server` binary (run `cargo build --release -p code-analysis-server`)
- **Optional**: `fuzzywuzzy` for better search (`pip install fuzzywuzzy python-levenshtein`)

## Troubleshooting

### Server Won't Start
- Ensure the `code-analysis-server` binary is built: `cd codex-rs && cargo build --release -p code-analysis-server`
- Check if the port is already in use: `lsof -i :3000`
- Try a different port: `--port 3001`

### No Files Found
- Check the project directory path: `--project-dir /correct/path`
- Verify C# files exist: `find /path -name "*.cs"`

### Skeleton Generation Fails
- Check server logs for errors
- Ensure files are valid C# files
- Try increasing token limit: `tokens 10000`

## Integration with Development Workflow

This tool is perfect for:

- **Testing skeleton quality** during development
- **Verifying C# language feature support** 
- **Debugging skeleton generation issues**
- **Demonstrating skeleton functionality** to users
- **Regression testing** after code changes

## Advanced Usage

### Testing Multiple Files
```bash
[skeleton-test] list Models/*.cs
[skeleton-test] analyze Models/User.cs
[skeleton-test] analyze Models/Order.cs
[skeleton-test] analyze Models/Product.cs
```

### Performance Testing
```bash
[skeleton-test] tokens 2000    # Test with low token limit
[skeleton-test] analyze LargeFile.cs
[skeleton-test] tokens 10000   # Test with high token limit
[skeleton-test] analyze LargeFile.cs
```

### Quality Comparison
```bash
[skeleton-test] analyze SimpleClass.cs     # Compare simple vs complex files
[skeleton-test] analyze ComplexClass.cs
```

This tool provides a comprehensive way to test and verify the C# skeleton generation functionality interactively!