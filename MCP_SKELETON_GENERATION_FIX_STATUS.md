# MCP Skeleton Generation Fix - Current Status

## Problem Statement
The MCP server's skeleton generation tool (`get_multiple_files_skeleton`) is using fallback regex-based parsing instead of our improved tree-sitter parsing, even though the direct `analyze_code` tool works perfectly with tree-sitter.

## Root Cause Analysis
**Path Mismatch Issue**: The skeleton generation looks for symbols in the graph manager's cache using the exact file path provided, but there's a mismatch between:
- **Paths stored during initialization**: Absolute paths (e.g., `/home/arvkum/projects/codex/codex-rs/test_files/csharp_test_suite/BasicClass.cs`)
- **Paths used in queries**: Relative paths (e.g., `codex-rs/test_files/csharp_test_suite/BasicClass.cs`)

## Current Evidence

### ✅ Working: Direct API Analysis
```bash
curl -X POST http://localhost:3000/mcp -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"analyze_code","arguments":{"file_path":"codex-rs/test_files/csharp_test_suite/BasicClass.cs"}}}'
```
**Result**: ✅ Returns 7 symbols including `TestNamespace`, `BasicClass`, and 6 methods

### ❌ Not Working: Skeleton Generation
```bash
curl -X POST http://localhost:3000/mcp -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"get_multiple_files_skeleton","arguments":{"file_paths":["codex-rs/test_files/csharp_test_suite/BasicClass.cs"],"max_tokens":4000}}}'
```
**Result**: ❌ Returns "Fallback skeleton generation (no symbols detected by parser)"

## Code Flow Analysis

### Working Path (analyze_code):
1. `handle_analyze_code()` → `ContextExtractor::extract_symbols_from_file()` 
2. **Direct tree-sitter parsing** → Returns symbols

### Broken Path (skeleton generation):
1. `handle_get_multiple_files_skeleton()` → `generate_single_file_skeleton()`
2. `repo_mapper.get_symbols_for_file()` → `ContextExtractor::get_symbols_for_file()`
3. **Cache lookup fails** → Falls back to regex parsing

## Fixes Implemented

### 1. Enhanced Path Normalization (✅ DONE)
**File**: `codex-rs/core/src/code_analysis/context_extractor.rs`
**Lines**: 507-550
**Fix**: Added comprehensive path matching in `get_symbols_for_file()`:
- Exact path matching
- Canonical path matching (absolute paths)
- Relative path matching (ends_with)
- File name matching

### 2. Debug Logging Added (✅ DONE)
**Purpose**: To see what paths are stored vs queried
**Location**: Same file, added tracing::debug statements

### 3. Fixed Comprehensive Test (✅ DONE)
**File**: `codex-rs/scripts/comprehensive_skeleton_test.py`
**Changes**:
- Use existing MCP server instead of starting new one
- Use relative paths instead of absolute paths
- Fixed server cleanup logic

## How to Test

### 1. Start MCP Server
```bash
cd /home/arvkum/projects/codex/codex-rs/target/release
./code-analysis-server --sse --project-dir /home/arvkum/projects/codex/
```

### 2. Test Direct API (Should Work)
```bash
curl -X POST http://localhost:3000/mcp -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"analyze_code","arguments":{"file_path":"codex-rs/test_files/csharp_test_suite/BasicClass.cs"}}}' | grep -o '"name":"[^"]*"'
```
**Expected**: Should show symbols like `TestNamespace`, `BasicClass`, etc.

### 3. Test Skeleton Generation (Currently Broken)
```bash
curl -X POST http://localhost:3000/mcp -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"get_multiple_files_skeleton","arguments":{"file_paths":["codex-rs/test_files/csharp_test_suite/BasicClass.cs"],"max_tokens":4000}}}' | grep -o "Fallback skeleton\|Generated from Tree-sitter"
```
**Current Result**: "Fallback skeleton"
**Expected Result**: "Generated from Tree-sitter" or similar

### 4. Run Comprehensive Test
```bash
cd /home/arvkum/projects/codex
python3 codex-rs/scripts/comprehensive_skeleton_test.py
```
**Current Result**: 0% tree-sitter usage for C# and C++
**Expected Result**: >50% tree-sitter usage

### 5. Check Debug Logs
After restarting server, the debug logs will show:
- What file path is being queried
- What paths are actually stored in the cache
- Whether path matching is working

## Next Steps

1. **Restart Server** to see debug logs
2. **Analyze Debug Output** to understand path storage vs query mismatch
3. **Fix Path Normalization** if the current approach isn't working
4. **Verify Tree-sitter Usage** in comprehensive test

## Key Files Modified

- `codex-rs/core/src/code_analysis/context_extractor.rs` - Path normalization fix
- `codex-rs/scripts/comprehensive_skeleton_test.py` - Use existing server + relative paths
- `codex-rs/core/src/code_analysis/tools.rs` - Removed duplicate functions

## Success Criteria

- ✅ Direct API analysis working (ACHIEVED)
- ❌ Skeleton generation using tree-sitter (IN PROGRESS)
- ❌ Comprehensive test showing >50% tree-sitter usage (PENDING)
- ✅ Production test suite 100% pass rate (ACHIEVED)

## Current Status: 🔧 DEBUGGING PHASE
The path normalization fix has been implemented and debug logging added. Need to restart server and analyze debug output to understand why the cache lookup is still failing.