#!/bin/bash

# Script to run all C# related tests in codex-rs
# This script runs tests specifically for C# code analysis and parsing

set -e  # Exit on any error

echo "🔍 Running C# Tests for Codex-RS"
echo "================================="

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

echo -e "${YELLOW}📋 C# Specific Tests${NC}"
echo "-------------------"

# Core C# tests
csharp_tests=(
    "csharp_simple_test"
    "csharp_analysis_comprehensive" 
    "csharp_comprehensive_multifile_test"
    "csharp_intra_file_calls"
    "csharp_advanced_features_test"
    "debug_csharp_ast"
    "debug_csharp_parser_capabilities"
    "code_analysis_csharp_edges"
    "code_analysis_csharp_integration"
)

for test in "${csharp_tests[@]}"; do
    total_tests=$((total_tests + 1))
    if run_test "$test"; then
        passed_tests=$((passed_tests + 1))
    else
        failed_tests=$((failed_tests + 1))
    fi
    echo ""
done

echo -e "${YELLOW}📋 Multi-Language Tests (including C#)${NC}"
echo "--------------------------------------"

# Multi-language tests that include C#
multi_lang_tests=(
    "code_analysis_cpp_csharp_java"
    "code_analysis_line_numbers"
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

echo -e "${YELLOW}📋 C# Specific Test Functions${NC}"
echo "-----------------------------"

# Specific C# test functions within general test files
total_tests=$((total_tests + 1))
if run_test_function "debug_line_number_issue" "debug_line_number_issue"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
fi
echo ""

# Test C# functionality in general code analysis tests
total_tests=$((total_tests + 1))
if run_test_function "test_code_analysis_tools" "test_csharp_class_with_methods_line_numbers"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
fi
echo ""

# Summary
echo "======================================="
echo -e "${BLUE}📊 C# Test Results Summary${NC}"
echo "======================================="
echo -e "Total tests run: ${total_tests}"
echo -e "${GREEN}Passed: ${passed_tests}${NC}"
echo -e "${RED}Failed: ${failed_tests}${NC}"

if [ $failed_tests -eq 0 ]; then
    echo -e "${GREEN}🎉 All C# tests passed!${NC}"
    exit 0
else
    echo -e "${RED}💥 Some C# tests failed!${NC}"
    exit 1
fi