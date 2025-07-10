#!/bin/bash
# Script to run all C++ code analysis tests
# Usage: ./run_cpp_tests.sh [--verbose]
#
# This script runs all C++ related tests to verify the code analysis functionality.
# It includes comprehensive tests for C++ language features, code analysis tools,
# and performance/edge cases.

set -e  # Exit on error

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default verbosity
VERBOSE=0

# Parse arguments
for arg in "$@"; do
  if [ "$arg" == "--verbose" ] || [ "$arg" == "-v" ]; then
    VERBOSE=1
  fi
done

# Function to run a test and report results
run_test() {
  local test_name=$1
  local test_description=$2
  
  echo -e "${BLUE}Running test: ${test_name}${NC}"
  echo -e "${YELLOW}${test_description}${NC}"
  
  if [ $VERBOSE -eq 1 ]; then
    # Run with nocapture to show test output
    cargo test --test "$test_name" -- --nocapture
  else
    # Run without showing test output
    cargo test --test "$test_name" > /dev/null 2>&1
    if [ $? -eq 0 ]; then
      echo -e "${GREEN}✓ Test passed${NC}"
    else
      echo -e "${RED}✗ Test failed${NC}"
      exit 1
    fi
  fi
  echo ""
}

# Function to run a specific test function
run_test_function() {
  local test_file=$1
  local test_function=$2
  local test_description=$3
  
  echo -e "${BLUE}Running test function: ${test_function}${NC}"
  echo -e "${YELLOW}${test_description}${NC}"
  
  if [ $VERBOSE -eq 1 ]; then
    # Run with nocapture to show test output
    cargo test --test "$test_file" "$test_function" -- --nocapture
  else
    # Run without showing test output
    cargo test --test "$test_file" "$test_function" > /dev/null 2>&1
    if [ $? -eq 0 ]; then
      echo -e "${GREEN}✓ Test passed${NC}"
    else
      echo -e "${RED}✗ Test failed${NC}"
      exit 1
    fi
  fi
  echo ""
}

# Check if we're in the right directory
if [ ! -d "core/tests" ]; then
  if [ -d "codex-rs" ]; then
    cd codex-rs
  else
    echo -e "${RED}Error: Please run this script from the codex-rs directory or its parent directory${NC}"
    exit 1
  fi
fi

echo -e "${BLUE}=== Running C++ Code Analysis Test Suite ===${NC}"
echo -e "${YELLOW}This will run all C++ related tests to verify code analysis functionality${NC}"
echo ""

# Run the comprehensive detailed tests
run_test "cpp_detailed_comprehensive_test" "Comprehensive tests for C++ language features and code analysis tools"

# Run specific test functions from the detailed test suite
run_test_function "cpp_detailed_comprehensive_test" "test_cpp_symbol_extraction_accuracy" "Testing C++ symbol extraction accuracy"
run_test_function "cpp_detailed_comprehensive_test" "test_cpp_line_number_accuracy" "Testing C++ line number accuracy"
run_test_function "cpp_detailed_comprehensive_test" "test_cpp_comprehensive_language_features" "Testing comprehensive C++ language features"
run_test_function "cpp_detailed_comprehensive_test" "test_cpp_code_analysis_tools" "Testing C++ code analysis tools"
run_test_function "cpp_detailed_comprehensive_test" "test_cpp_performance_and_edge_cases" "Testing C++ performance and edge cases"

# Run other C++ related tests
run_test "cpp_simple_test" "Simple C++ parsing tests"
run_test "cpp_analysis_comprehensive" "Comprehensive C++ analysis tests"
run_test "cpp_comprehensive_multifile_test" "Multi-file C++ analysis tests"
run_test "cpp_intra_file_calls" "C++ intra-file call analysis"
run_test "code_analysis_cpp_csharp_java" "Cross-language tests including C++"

echo -e "${GREEN}=== All C++ tests completed successfully! ===${NC}"