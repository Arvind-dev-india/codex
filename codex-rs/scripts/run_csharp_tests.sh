#!/bin/bash
# Script to run all C# code analysis tests
# Usage: ./run_csharp_tests.sh [--verbose]
#
# This script runs all C# related tests to verify the code analysis functionality.
# It includes comprehensive tests for C# language features, code analysis tools,
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

# Test suite counters
SUITES_PASSED=0
SUITES_FAILED=0
SUITES_SKIPPED=0
TOTAL_SUITES=0

# Individual test function counters
INDIVIDUAL_TESTS_PASSED=0
INDIVIDUAL_TESTS_FAILED=0
INDIVIDUAL_TESTS_SKIPPED=0
TOTAL_INDIVIDUAL_TESTS=0

# Parse arguments
for arg in "$@"; do
  if [ "$arg" == "--verbose" ] || [ "$arg" == "-v" ]; then
    VERBOSE=1
  fi
done

# Function to parse test results from cargo output
parse_test_results() {
  local output="$1"
  
  # Extract numbers from cargo test output
  local passed=$(echo "$output" | grep -o "[0-9]\+ passed" | grep -o "[0-9]\+" | head -1 || echo "0")
  local failed=$(echo "$output" | grep -o "[0-9]\+ failed" | grep -o "[0-9]\+" | head -1 || echo "0")
  local ignored=$(echo "$output" | grep -o "[0-9]\+ ignored" | grep -o "[0-9]\+" | head -1 || echo "0")
  
  # Default to 0 if empty
  passed=${passed:-0}
  failed=${failed:-0}
  ignored=${ignored:-0}
  
  echo "$passed $failed $ignored"
}

# Function to run a test and report results
run_test() {
  local test_name=$1
  local test_description=$2
  
  echo -e "${BLUE}Running test: ${test_name}${NC}"
  echo -e "${YELLOW}${test_description}${NC}"
  
  TOTAL_SUITES=$((TOTAL_SUITES + 1))
  
  if [ $VERBOSE -eq 1 ]; then
    # Run with nocapture to show test output
    local output=$(cargo test --test "$test_name" -- --nocapture 2>&1)
    echo "$output"
    local exit_code=$?
  else
    # Run without showing test output and capture output for analysis
    local output=$(cargo test --test "$test_name" 2>&1)
    local exit_code=$?
  fi
  
  # Parse individual test results from cargo output
  local results=$(parse_test_results "$output")
  local individual_passed=$(echo $results | cut -d' ' -f1)
  local individual_failed=$(echo $results | cut -d' ' -f2)
  local individual_ignored=$(echo $results | cut -d' ' -f3)
  
  # Update individual test counters
  INDIVIDUAL_TESTS_PASSED=$((INDIVIDUAL_TESTS_PASSED + individual_passed))
  INDIVIDUAL_TESTS_FAILED=$((INDIVIDUAL_TESTS_FAILED + individual_failed))
  INDIVIDUAL_TESTS_SKIPPED=$((INDIVIDUAL_TESTS_SKIPPED + individual_ignored))
  TOTAL_INDIVIDUAL_TESTS=$((TOTAL_INDIVIDUAL_TESTS + individual_passed + individual_failed + individual_ignored))
  
  if [ $exit_code -eq 0 ]; then
    echo -e "${GREEN}[PASS] Test suite passed (${individual_passed} tests passed, ${individual_ignored} ignored)${NC}"
    SUITES_PASSED=$((SUITES_PASSED + 1))
  else
    # Check if test was skipped or failed
    if echo "$output" | grep -q "ignored\|skipped"; then
      echo -e "${YELLOW}[SKIP] Test suite skipped${NC}"
      SUITES_SKIPPED=$((SUITES_SKIPPED + 1))
    else
      echo -e "${RED}[FAIL] Test suite failed (${individual_failed} tests failed, ${individual_passed} passed, ${individual_ignored} ignored)${NC}"
      SUITES_FAILED=$((SUITES_FAILED + 1))
      if [ $VERBOSE -eq 0 ]; then
        echo -e "${RED}Error output:${NC}"
        echo "$output"
      fi
      # Don't exit immediately, continue with other tests
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
  
  TOTAL_SUITES=$((TOTAL_SUITES + 1))
  
  if [ $VERBOSE -eq 1 ]; then
    # Run with nocapture to show test output
    local output=$(cargo test --test "$test_file" "$test_function" -- --nocapture 2>&1)
    echo "$output"
    local exit_code=$?
  else
    # Run without showing test output and capture output for analysis
    local output=$(cargo test --test "$test_file" "$test_function" 2>&1)
    local exit_code=$?
  fi
  
  # Parse individual test results from cargo output
  local results=$(parse_test_results "$output")
  local individual_passed=$(echo $results | cut -d' ' -f1)
  local individual_failed=$(echo $results | cut -d' ' -f2)
  local individual_ignored=$(echo $results | cut -d' ' -f3)
  
  # Update individual test counters
  INDIVIDUAL_TESTS_PASSED=$((INDIVIDUAL_TESTS_PASSED + individual_passed))
  INDIVIDUAL_TESTS_FAILED=$((INDIVIDUAL_TESTS_FAILED + individual_failed))
  INDIVIDUAL_TESTS_SKIPPED=$((INDIVIDUAL_TESTS_SKIPPED + individual_ignored))
  TOTAL_INDIVIDUAL_TESTS=$((TOTAL_INDIVIDUAL_TESTS + individual_passed + individual_failed + individual_ignored))
  
  if [ $exit_code -eq 0 ]; then
    echo -e "${GREEN}[PASS] Test function passed${NC}"
    SUITES_PASSED=$((SUITES_PASSED + 1))
  else
    # Check if test was skipped or failed
    if echo "$output" | grep -q "ignored\|skipped"; then
      echo -e "${YELLOW}[SKIP] Test function skipped${NC}"
      SUITES_SKIPPED=$((SUITES_SKIPPED + 1))
    else
      echo -e "${RED}[FAIL] Test function failed${NC}"
      SUITES_FAILED=$((SUITES_FAILED + 1))
      if [ $VERBOSE -eq 0 ]; then
        echo -e "${RED}Error output:${NC}"
        echo "$output"
      fi
      # Don't exit immediately, continue with other tests
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

echo -e "${BLUE}=== Running C# Code Analysis Test Suite ===${NC}"
echo -e "${YELLOW}This will run all C# related tests to verify code analysis functionality${NC}"
echo ""

# Run the comprehensive analysis tests
run_test "csharp_analysis_comprehensive" "Comprehensive tests for C# language features and code analysis tools"

# Run specific test functions from the comprehensive test suite
run_test_function "csharp_analysis_comprehensive" "test_csharp_basic_class_parsing" "Testing C# basic class parsing"
run_test_function "csharp_analysis_comprehensive" "test_csharp_skeleton_generation" "Testing C# skeleton generation with line numbers"
run_test_function "csharp_analysis_comprehensive" "test_csharp_skeleton_with_token_limit" "Testing C# skeleton generation with token limits"
run_test_function "csharp_analysis_comprehensive" "test_csharp_skeleton_bfs_depth" "Testing C# skeleton BFS depth functionality"
run_test_function "csharp_analysis_comprehensive" "test_csharp_skeleton_edge_weight_priority" "Testing C# skeleton edge-weight priority"

# Run other C# related tests
run_test "csharp_simple_test" "Simple C# parsing tests"
run_test "csharp_comprehensive_multifile_test" "Multi-file C# analysis tests"
run_test "csharp_intra_file_calls" "C# intra-file call analysis"
run_test "csharp_advanced_features_test" "Advanced C# features and patterns"

# Run cross-language tests that include C#
run_test "code_analysis_cpp_csharp_java" "Cross-language tests including C#"

# Run C# specific edge case tests
run_test_function "csharp_analysis_comprehensive" "test_csharp_inheritance_and_interfaces" "Testing C# inheritance and interface parsing"
run_test_function "csharp_analysis_comprehensive" "test_csharp_generics_and_advanced_features" "Testing C# generics and advanced features"
run_test_function "csharp_analysis_comprehensive" "test_csharp_inter_file_references" "Testing C# inter-file reference analysis"

echo -e "${BLUE}=== C# Test Suite Summary ===${NC}"
echo -e "${BLUE}Test Suites:${NC}"
echo -e "${GREEN}  [PASS] Suites Passed: ${SUITES_PASSED}${NC}"
echo -e "${RED}  [FAIL] Suites Failed: ${SUITES_FAILED}${NC}"
echo -e "${YELLOW}  [SKIP] Suites Skipped: ${SUITES_SKIPPED}${NC}"
echo -e "${BLUE}  [TOTAL] Total Suites: ${TOTAL_SUITES}${NC}"
echo ""
echo -e "${BLUE}Individual Tests:${NC}"
echo -e "${GREEN}  [PASS] Tests Passed: ${INDIVIDUAL_TESTS_PASSED}${NC}"
echo -e "${RED}  [FAIL] Tests Failed: ${INDIVIDUAL_TESTS_FAILED}${NC}"
echo -e "${YELLOW}  [SKIP] Tests Skipped: ${INDIVIDUAL_TESTS_SKIPPED}${NC}"
echo -e "${BLUE}  [TOTAL] Total Tests: ${TOTAL_INDIVIDUAL_TESTS}${NC}"
echo ""

if [ $SUITES_FAILED -eq 0 ] && [ $INDIVIDUAL_TESTS_FAILED -eq 0 ]; then
  echo -e "${GREEN}=== All C# tests completed successfully! ===${NC}"
  exit 0
else
  echo -e "${RED}=== Some C# tests failed! ===${NC}"
  echo -e "${RED}Failed suites: ${SUITES_FAILED}, Failed individual tests: ${INDIVIDUAL_TESTS_FAILED}${NC}"
  exit 1
fi