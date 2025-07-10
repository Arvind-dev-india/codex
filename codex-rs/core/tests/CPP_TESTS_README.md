# C++ Code Analysis Test Suite

This directory contains comprehensive tests for the C++ code analysis functionality in the Codex project.

## Overview

The C++ code analysis tests verify the ability to parse, analyze, and extract information from C++ source code. These tests cover a wide range of C++ language features and ensure that the code analysis tools work correctly with C++ codebases.

## Test Files

### Main Test Files

1. **`cpp_detailed_comprehensive_test.rs`**
   - Comprehensive tests for all C++ language features
   - Tests for all code analysis tools with C++ code
   - Performance and edge case tests

2. **`cpp_simple_test.rs`**
   - Basic C++ parsing tests
   - Simple C++ language feature tests

3. **`cpp_analysis_comprehensive.rs`**
   - Comprehensive tests for C++ symbol extraction
   - Tests for class hierarchies, templates, etc.

4. **`cpp_comprehensive_multifile_test.rs`**
   - Tests for analyzing C++ code across multiple files
   - Tests for cross-file dependencies

5. **`cpp_intra_file_calls.rs`**
   - Tests for detecting function calls within C++ files

6. **`code_analysis_cpp_csharp_java.rs`**
   - Cross-language tests including C++

## Test Data

The tests use sample C++ code located in `codex-rs/test_files/cpp_test_suite/`:

- `basic_class.h/cpp` - Core C++ features
- `main.cpp` - Main program
- `models/user.h` - Models namespace with inheritance
- `models/product.h` - Template classes
- `models/order.h` - Order classes
- `services/user_service.h` - Service layer
- `data/repository.h` - Repository pattern
- `utils/helpers.h` - Utility functions

## Running the Tests

### Using the Test Script

We provide a bash script to run all C++ tests:

```bash
# From the codex-rs directory
./scripts/run_cpp_tests.sh

# For verbose output
./scripts/run_cpp_tests.sh --verbose
```

### Running Individual Tests

You can run individual tests using cargo:

```bash
# Run all tests in a specific test file
cargo test --test cpp_detailed_comprehensive_test

# Run a specific test function
cargo test --test cpp_detailed_comprehensive_test test_cpp_symbol_extraction_accuracy

# Run with output
cargo test --test cpp_detailed_comprehensive_test -- --nocapture
```

## Test Categories

### 1. Language Feature Tests

Tests for parsing and analyzing various C++ language features:

- Classes and inheritance
- Templates and generics
- Namespaces and scope
- Operator overloading
- Memory management (smart pointers)
- Function overloading
- Enums and structs
- Preprocessor directives
- Cross-file dependencies
- Advanced features (lambdas, auto keyword)

### 2. Code Analysis Tool Tests

Tests for the code analysis tools with C++ code:

- `analyze_code` - Analyzes C++ files and extracts symbols
- `find_symbol_references` - Finds references to C++ symbols
- `find_symbol_definitions` - Finds definitions of C++ symbols
- `get_symbol_subgraph` - Gets the subgraph of related symbols
- `get_related_files_skeleton` - Gets skeleton views of related files

### 3. Performance and Edge Case Tests

Tests for handling special cases:

- Large file parsing
- Malformed code handling
- Unicode character support
- Deep nesting structures

## Adding New Tests

When adding new C++ tests:

1. Create a new test file or add tests to existing files
2. Add test data to `codex-rs/test_files/cpp_test_suite/` if needed
3. Update the `run_cpp_tests.sh` script to include your new tests
4. Document the new tests in this README

## Common Issues

- **Symbol Type Classification**: The parser may classify some C++ constructs differently than expected (e.g., enums as classes)
- **Line Number Accuracy**: Line numbers may vary slightly depending on the parser implementation
- **Template Parsing**: Complex template syntax may be parsed differently than expected

## Related Documentation

- [C++ Parser Limitations](../src/code_analysis/CSHARP_PARSER_LIMITATIONS.md)
- [Code Analysis README](../src/code_analysis/README.md)