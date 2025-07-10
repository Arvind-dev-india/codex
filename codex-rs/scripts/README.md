# Test Scripts

This directory contains test scripts for running language-specific test suites for the code analysis functionality.

## Available Scripts

### 🔧 C++ Test Suite
```bash
./run_cpp_tests.sh [--verbose]
```
Runs all C++ related tests including:
- Comprehensive C++ language features
- Code analysis tools
- Multi-file analysis
- Advanced C++ patterns
- Performance and edge cases

### 🔷 C# Test Suite
```bash
./run_csharp_tests.sh [--verbose]
```
Runs all C# related tests including:
- Basic C# class parsing
- Skeleton generation with line numbers
- Inheritance and interfaces
- Generics and advanced features
- Inter-file references
- Multi-file analysis

### 🐍 Python Test Suite
```bash
./run_python_tests.sh [--verbose]
```
Runs all Python related tests including:
- Basic Python class parsing
- Skeleton generation with line numbers
- Inheritance and decorators
- Import and module analysis
- Cross-file references
- Line number validation

## Usage

### Basic Usage
```bash
# Run C++ tests (quiet mode)
./run_cpp_tests.sh

# Run C# tests (quiet mode)
./run_csharp_tests.sh

# Run Python tests (quiet mode)
./run_python_tests.sh
```

### Verbose Mode
```bash
# Run with detailed output
./run_cpp_tests.sh --verbose
./run_csharp_tests.sh -v
./run_python_tests.sh --verbose
```

## Features

### 📊 Detailed Test Statistics
Each script provides comprehensive statistics at two levels:

**Test Suites Level:**
- ✅ **Suites Passed**: Number of successful test suites
- ❌ **Suites Failed**: Number of failed test suites  
- ⚠️ **Suites Skipped**: Number of skipped/ignored test suites
- 📈 **Total Suites**: Total number of test suites executed

**Individual Tests Level:**
- ✅ **Tests Passed**: Number of successful individual test functions
- ❌ **Tests Failed**: Number of failed individual test functions
- ⚠️ **Tests Skipped**: Number of skipped/ignored individual test functions
- 📈 **Total Tests**: Total number of individual test functions executed

### 🎯 Smart Error Handling
- Scripts continue running even if individual tests fail
- Error output is captured and displayed for failed tests
- Final exit code reflects overall success/failure
- Colored output for easy visual parsing

### 🔍 Detailed Test Coverage
Each script runs:
- **Comprehensive test suites** for language-specific features
- **Individual test functions** for specific functionality
- **Cross-language tests** that include the target language
- **Edge case and performance tests**

## Example Output

```bash
$ ./run_csharp_tests.sh

=== Running C# Code Analysis Test Suite ===
This will run all C# related tests to verify code analysis functionality

Running test: csharp_analysis_comprehensive
Comprehensive tests for C# language features and code analysis tools
[PASS] Test suite passed (5 tests passed, 0 ignored)

Running test function: test_csharp_skeleton_generation
Testing C# skeleton generation with line numbers
[PASS] Test function passed

...

=== C# Test Suite Summary ===
Test Suites:
  [PASS] Suites Passed: 12
  [FAIL] Suites Failed: 0
  [SKIP] Suites Skipped: 1
  [TOTAL] Total Suites: 13

Individual Tests:
  [PASS] Tests Passed: 47
  [FAIL] Tests Failed: 0
  [SKIP] Tests Skipped: 3
  [TOTAL] Total Tests: 50

=== All C# tests completed successfully! ===
```

## Integration

These scripts are designed to be used in:
- **CI/CD pipelines** for automated testing
- **Development workflows** for local validation
- **Release processes** for quality assurance
- **Debugging sessions** for isolating language-specific issues

## Requirements

- Rust toolchain with Cargo
- Access to the codex-rs workspace
- Test files in the appropriate test_files directories

Run from the `codex-rs` directory or its parent directory.