# Cross-Project Analysis Test Suite

This directory contains a comprehensive test for the **Enhanced BFS with Cross-Project Skeleton Generation** feature.

## 🎯 What This Tests

- **Enhanced BFS Traversal**: Finds related files across project boundaries
- **Cross-Project Symbol Resolution**: Uses supplementary registry for cross-project symbols
- **Skeleton Generation**: Generates proper parsed skeletons from cross-project files
- **Project Boundary Respect**: BFS stops at cross-project files as intended

## 📁 Test Structure

```
test_cross_project_mcp/
├── MainProject/
│   └── UserService.cs          # Main project file that references cross-project symbols
├── SkeletonProject/            # Supplementary project (external dependency)
│   ├── User.cs                 # Base class used by MainProject
│   ├── IUserRepository.cs      # Interface implemented by MainProject
│   └── ValidationHelper.cs     # Utility class used by MainProject
└── README.md                   # This file
```

## 🚀 How to Run

### 1. Start MCP Server
```bash
cd codex-rs
./target/release/code-analysis-server \
  --sse \
  --project-dir /home/arvkum/project/codex/codex-rs/test_cross_project_mcp/MainProject \
  --supplementary SkeletonProject:/home/arvkum/project/codex/codex-rs/test_cross_project_mcp/SkeletonProject \
  --verbose
```

**Note**: Use absolute paths to avoid path resolution issues. Replace `/home/arvkum/project/codex/codex-rs` with your actual codex-rs directory path.

### 2. Run Test Suite
```bash
cd codex-rs
python3 test_cross_project_mcp_suite.py http://localhost:3000
```

## ✅ Expected Results

When working correctly, the test should show:

1. **UserService.cs Analysis**: 4+ symbols found
2. **Cross-Project Skeleton Generation**: 
   - 3 files found from SkeletonProject
   - All with ✅ PARSED skeletons (not fallback)
   - Proper C# structure with classes, methods, interfaces
3. **Symbol References**: Cross-project references detected
4. **Symbol Definitions**: ValidationHelper found in SkeletonProject

## 🔧 Implementation Details

### Enhanced BFS Approach
1. **Main Project**: Parse once, build full graph with symbols and references
2. **Supplementary Project**: Parse once, extract symbols lightweight (FQN only), store in registry
3. **BFS Strategy**: 
   - Traverse main project graph normally
   - When hitting unresolved symbols, check supplementary registry for definitions
   - If found in supplementary, add as special boundary symbol (don't traverse further)
   - Generate skeleton for supplementary files on-demand using cached symbols

### Key Features
- **Performance Optimized**: Supplementary files parsed only once during initialization
- **Memory Efficient**: Lightweight registry instead of full graph for supplementary projects
- **Boundary Respect**: BFS doesn't traverse into supplementary projects
- **Rich Context**: Generates proper skeletons with parsed symbols, not fallback text

## 🐛 Troubleshooting

If the test fails:

1. **Check server logs** for initialization errors
2. **Verify file paths** are correct relative to project directories
3. **Ensure C# parsing** is working (check for tree-sitter errors)
4. **Check supplementary registry** population in server logs

## 📝 Test Output Example

```
=== Cross-Project Analysis Test Suite ===

--- Analyze UserService.cs ---
✅ Analyze UserService.cs SUCCESS
   Found 4 symbols in UserService.cs

--- Get Related Files Skeleton ---
✅ Get Related Files Skeleton SUCCESS
   Total related files: 3
   Cross-project files: 3
   Cross-project boundaries detected: True
   ✅ CROSS-PROJECT ANALYSIS WORKING!
   - IUserRepository.cs (CROSS-PROJECT) - ✅ PARSED skeleton (835 chars)
   - ValidationHelper.cs (CROSS-PROJECT) - ✅ PARSED skeleton (989 chars)
   - User.cs (CROSS-PROJECT) - ✅ PARSED skeleton (943 chars)
```

This test validates that the enhanced BFS approach is working correctly for cross-project analysis! 🎉