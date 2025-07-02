# Test Scripts for C# and Python Code Analysis

This directory contains two comprehensive test scripts for running language-specific tests in the codex-rs project.

## Scripts Overview

### 🔍 `run_csharp_tests.sh`
Runs all C# related tests including:
- Core C# parsing and analysis tests
- Multi-language tests that include C#
- Line number validation for C# constructs
- Advanced C# features (generics, inheritance, etc.)

### 🐍 `run_python_tests.sh`
Runs all Python related tests including:
- Core Python parsing and analysis tests
- Multi-language tests that include Python
- Line number validation for Python constructs
- Inter-file and intra-file analysis
- Debug and development tests

## Usage

### Prerequisites
- Ensure you're in the root directory of the project
- Make sure Rust and Cargo are installed
- The scripts must be executable (they are set as executable by default)

### Running the Scripts

```bash
# Run all C# tests
./run_csharp_tests.sh

# Run all Python tests
./run_python_tests.sh
```

### Script Features

Both scripts provide:
- **Colored output** for easy reading (green for pass, red for fail, blue for info)
- **Detailed test results** with pass/fail status for each test
- **Summary statistics** showing total tests, passed, and failed counts
- **Exit codes** (0 for success, 1 for failures) for CI/CD integration
- **Error suppression** for cleaner output while preserving test results

## Test Categories

### C# Tests (`run_csharp_tests.sh`)

#### Core C# Tests
- `csharp_simple_test` - Basic C# parsing functionality
- `csharp_analysis_comprehensive` - Comprehensive C# code analysis
- `csharp_comprehensive_multifile_test` - Multi-file C# analysis
- `csharp_intra_file_calls` - Intra-file method calls and references
- `csharp_advanced_features_test` - Advanced C# features (generics, inheritance)
- `debug_csharp_ast` - C# AST debugging
- `debug_csharp_parser_capabilities` - Parser capability testing
- `code_analysis_csharp_edges` - Edge case handling
- `code_analysis_csharp_integration` - Integration testing

#### Multi-Language Tests (including C#)
- `code_analysis_cpp_csharp_java` - Multi-language parsing
- `code_analysis_line_numbers` - Line number accuracy across languages

#### Specific Test Functions
- Line number validation tests
- Class and method detection tests

### Python Tests (`run_python_tests.sh`)

#### Core Python Tests
- `python_simple_test` - Basic Python parsing functionality
- `python_analysis_comprehensive` - Comprehensive Python code analysis
- `python_inter_file_analysis` - Inter-file dependency analysis
- `python_intra_file_calls` - Intra-file function calls and references
- `python_line_number_validation` - Line number accuracy validation
- `debug_python_line_numbers` - Python line number debugging

#### Debug and Development Tests
- `debug_tree_sitter_line_numbers` - Tree-sitter line number debugging
- `verify_actual_line_numbers` - Line number verification

#### Multi-Language Tests (including Python)
- `code_analysis_line_numbers` - Line number accuracy across languages
- `code_analysis` - General code analysis functionality

#### Related Tests
- `code_analysis_js_ts` - JavaScript/TypeScript (similar parsing logic)
- `code_analysis_typescript_edges` - TypeScript edge cases

## Sample Output

### Successful Run
```
🔍 Running C# Tests for Codex-RS
=================================
📋 C# Specific Tests
-------------------
Running test: csharp_simple_test
✅ PASSED: csharp_simple_test

Running test: csharp_analysis_comprehensive
✅ PASSED: csharp_analysis_comprehensive

...

=======================================
📊 C# Test Results Summary
=======================================
Total tests run: 13
Passed: 13
Failed: 0
🎉 All C# tests passed!
```

### Failed Run
```
🔍 Running C# Tests for Codex-RS
=================================
📋 C# Specific Tests
-------------------
Running test: csharp_simple_test
✅ PASSED: csharp_simple_test

Running test: csharp_analysis_comprehensive
❌ FAILED: csharp_analysis_comprehensive

...

=======================================
📊 C# Test Results Summary
=======================================
Total tests run: 13
Passed: 10
Failed: 3
💥 Some C# tests failed!
```

## Integration with CI/CD

The scripts return appropriate exit codes:
- **Exit code 0**: All tests passed
- **Exit code 1**: Some tests failed

This makes them suitable for use in CI/CD pipelines:

```bash
# In your CI script
./run_csharp_tests.sh && ./run_python_tests.sh
```

## Troubleshooting

### Common Issues

1. **Permission Denied**
   ```bash
   chmod +x run_csharp_tests.sh run_python_tests.sh
   ```

2. **Tests Not Found**
   - Ensure you're running from the project root directory
   - Check that `codex-rs/` directory exists

3. **Compilation Errors**
   - Run `cd codex-rs && cargo build` first
   - Check for any missing dependencies

### Verbose Output

To see full test output including compilation errors:
```bash
# Modify the scripts to remove `2>/dev/null` from cargo test commands
# Or run individual tests manually:
cd codex-rs && cargo test --test python_simple_test -- --nocapture
```

## Customization

### Adding New Tests

To add new tests to the scripts:

1. **For C# tests**: Add the test name to the `csharp_tests` array in `run_csharp_tests.sh`
2. **For Python tests**: Add the test name to the `python_tests` array in `run_python_tests.sh`

### Modifying Test Categories

You can create custom test categories by:
1. Creating new arrays for different test groups
2. Adding corresponding loops to run those test groups
3. Updating the summary statistics

## Related Files

- `codex-rs/core/tests/` - Contains all the test files
- `codex-rs/core/src/code_analysis/` - Core analysis implementation
- `codex-rs/core/src/code_analysis/queries/` - Tree-sitter query files

## Recent Fixes

- ✅ Fixed Python line number validation test expectations
- ✅ Corrected off-by-one errors in test assertions
- ✅ Updated test expectations to match actual Tree-sitter output
- ✅ Verified line number parsing accuracy for both C# and Python