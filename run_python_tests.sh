#!/bin/bash

# Script to run all Python related tests in codex-rs
# This script runs tests specifically for Python code analysis and parsing

set -e  # Exit on any error

echo "🐍 Running Python Tests for Codex-RS"
echo "===================================="

cd codex-rs

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to run a test and report results
run_test() {
    local test_name=$1
    echo -e "${BLUE}Running test: ${test_name}${NC}"
    
    if cargo test --test "$test_name" -- --nocapture 2>/dev/null; then
        echo -e "${GREEN}✅ PASSED: ${test_name}${NC}"
        return 0
    else
        echo -e "${RED}❌ FAILED: ${test_name}${NC}"
        return 1
    fi
}

# Function to run a specific test function within a test file
run_test_function() {
    local test_file=$1
    local test_function=$2
    echo -e "${BLUE}Running test function: ${test_file}::${test_function}${NC}"
    
    if cargo test --test "$test_file" "$test_function" -- --nocapture 2>/dev/null; then
        echo -e "${GREEN}✅ PASSED: ${test_file}::${test_function}${NC}"
        return 0
    else
        echo -e "${RED}❌ FAILED: ${test_file}::${test_function}${NC}"
        return 1
    fi
}

# Counter for test results
total_tests=0
passed_tests=0
failed_tests=0

echo -e "${YELLOW}📋 Python Specific Tests${NC}"
echo "------------------------"

# Core Python tests
python_tests=(
    "python_simple_test"
    "python_analysis_comprehensive"
    "python_inter_file_analysis"
    "python_intra_file_calls"
    "python_line_number_validation"
    "debug_python_line_numbers"
)

for test in "${python_tests[@]}"; do
    total_tests=$((total_tests + 1))
    if run_test "$test"; then
        passed_tests=$((passed_tests + 1))
    else
        failed_tests=$((failed_tests + 1))
    fi
    echo ""
done

echo -e "${YELLOW}📋 Debug and Development Tests${NC}"
echo "-----------------------------"

# Debug tests for Python
debug_tests=(
    "debug_tree_sitter_line_numbers"
    "verify_actual_line_numbers"
)

for test in "${debug_tests[@]}"; do
    total_tests=$((total_tests + 1))
    if run_test "$test"; then
        passed_tests=$((passed_tests + 1))
    else
        failed_tests=$((failed_tests + 1))
    fi
    echo ""
done

echo -e "${YELLOW}📋 Multi-Language Tests (including Python)${NC}"
echo "------------------------------------------"

# Multi-language tests that include Python
multi_lang_tests=(
    "code_analysis_line_numbers"
    "code_analysis"
)

for test in "${multi_lang_tests[@]}"; do
    total_tests=$((total_tests + 1))
    if run_test "$test"; then
        passed_tests=$((passed_tests + 1))
    else
        failed_tests=$((failed_tests + 1))
    fi
    echo ""
done

echo -e "${YELLOW}📋 Python Specific Test Functions${NC}"
echo "--------------------------------"

# Specific Python test functions within general test files
total_tests=$((total_tests + 1))
if run_test_function "debug_line_number_issue" "debug_line_number_issue"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
fi
echo ""

# Test Python functionality in general code analysis tests
total_tests=$((total_tests + 1))
if run_test_function "test_code_analysis_tools" "test_python_classes_and_functions_line_numbers"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
fi
echo ""

# Test Python line mapping
total_tests=$((total_tests + 1))
if run_test_function "code_analysis_line_mapping" "test_python_line_mapping"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
fi
echo ""

# Test Python symbol references
total_tests=$((total_tests + 1))
if run_test_function "code_analysis_symbol_refs" "test_python_symbol_references"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
fi
echo ""

echo -e "${YELLOW}📋 JavaScript/TypeScript Tests (Tree-sitter related)${NC}"
echo "--------------------------------------------------"

# JS/TS tests that share similar parsing logic with Python
js_ts_tests=(
    "code_analysis_js_ts"
    "code_analysis_typescript_edges"
)

for test in "${js_ts_tests[@]}"; do
    total_tests=$((total_tests + 1))
    if run_test "$test"; then
        passed_tests=$((passed_tests + 1))
    else
        failed_tests=$((failed_tests + 1))
    fi
    echo ""
done

# Summary
echo "======================================="
echo -e "${BLUE}📊 Python Test Results Summary${NC}"
echo "======================================="
echo -e "Total tests run: ${total_tests}"
echo -e "${GREEN}Passed: ${passed_tests}${NC}"
echo -e "${RED}Failed: ${failed_tests}${NC}"

if [ $failed_tests -eq 0 ]; then
    echo -e "${GREEN}🎉 All Python tests passed!${NC}"
    exit 0
else
    echo -e "${RED}💥 Some Python tests failed!${NC}"
    exit 1
fi