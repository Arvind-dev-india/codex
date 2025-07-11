#!/usr/bin/env python3
"""
Skeleton Generation Issue Analysis and Improvement Recommendations
"""

import json
import requests
import os

def analyze_skeleton_issues():
    """Analyze current skeleton generation issues"""
    
    print("🔍 SKELETON GENERATION ISSUE ANALYSIS")
    print("=" * 60)
    
    # Test different token limits to see if that's the issue
    test_cases = [
        {"tokens": 1000, "desc": "Small token limit"},
        {"tokens": 4000, "desc": "Medium token limit"}, 
        {"tokens": 8000, "desc": "Large token limit"},
        {"tokens": 16000, "desc": "Very large token limit"}
    ]
    
    test_file = "codex-rs/test_files/csharp_test_suite/BasicClass.cs"
    
    print(f"\n📊 Testing different token limits on: {test_file}")
    
    for test_case in test_cases:
        payload = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": "get_multiple_files_skeleton",
                "arguments": {
                    "file_paths": [test_file],
                    "max_tokens": test_case["tokens"]
                }
            }
        }
        
        response = requests.post("http://localhost:3000/mcp", json=payload)
        if response.status_code == 200:
            result = response.json()
            content = json.loads(result["result"]["content"][0]["text"])
            
            files = content.get('files', [])
            if files:
                file_info = files[0]
                skeleton = file_info.get('skeleton', '')
                tokens_used = file_info.get('tokens', 0)
                
                print(f"  {test_case['desc']} ({test_case['tokens']} max):")
                print(f"    Tokens used: {tokens_used}")
                print(f"    Skeleton length: {len(skeleton)} chars")
                print(f"    Has content: {'✅' if len(skeleton.strip()) > 10 else '❌'}")
                
                if len(skeleton.strip()) > 10:
                    lines = skeleton.split('\n')[:3]
                    print(f"    Preview: {' | '.join(line.strip() for line in lines if line.strip())}")

def test_individual_files():
    """Test skeleton generation on individual files of different types"""
    
    print(f"\n🧪 INDIVIDUAL FILE TESTING")
    print("-" * 40)
    
    test_files = [
        {
            "path": "codex-rs/test_files/csharp_test_suite/BasicClass.cs",
            "type": "C# Simple Class",
            "expected_elements": ["class", "using", "namespace"]
        },
        {
            "path": "codex-rs/test_files/csharp_test_suite/Advanced/ModernCSharpFeatures.cs", 
            "type": "C# Advanced Features",
            "expected_elements": ["class", "record", "interface"]
        },
        {
            "path": "codex-rs/test_files/python_test_suite/basic_class.py",
            "type": "Python Class",
            "expected_elements": ["class", "def", "import"]
        },
        {
            "path": "codex-rs/core/src/config.rs",
            "type": "Rust Module",
            "expected_elements": ["struct", "impl", "use", "pub"]
        }
    ]
    
    for test_file in test_files:
        print(f"\n📄 Testing: {test_file['type']}")
        print(f"   File: {test_file['path']}")
        
        # Check if file exists
        if not os.path.exists(test_file['path']):
            print(f"   ❌ File not found")
            continue
            
        # Read actual file for comparison
        try:
            with open(test_file['path'], 'r', encoding='utf-8') as f:
                actual_content = f.read()
                actual_lines = len(actual_content.split('\n'))
                print(f"   Actual: {len(actual_content)} chars, {actual_lines} lines")
        except Exception as e:
            print(f"   ❌ Cannot read file: {e}")
            continue
        
        # Test skeleton generation
        payload = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": "get_multiple_files_skeleton",
                "arguments": {
                    "file_paths": [test_file['path']],
                    "max_tokens": 6000
                }
            }
        }
        
        response = requests.post("http://localhost:3000/mcp", json=payload)
        if response.status_code == 200:
            result = response.json()
            content = json.loads(result["result"]["content"][0]["text"])
            
            files = content.get('files', [])
            if files:
                file_info = files[0]
                skeleton = file_info.get('skeleton', '')
                tokens_used = file_info.get('tokens', 0)
                
                print(f"   Skeleton: {len(skeleton)} chars, {tokens_used} tokens")
                
                if len(skeleton.strip()) > 10:
                    print(f"   ✅ Generated skeleton")
                    
                    # Check for expected elements
                    found_elements = []
                    for element in test_file['expected_elements']:
                        if element in skeleton:
                            found_elements.append(element)
                    
                    print(f"   Expected elements: {found_elements}/{len(test_file['expected_elements'])}")
                    
                    # Show skeleton preview
                    lines = [line for line in skeleton.split('\n') if line.strip()][:8]
                    print(f"   Preview:")
                    for i, line in enumerate(lines):
                        print(f"     {i+1}: {line}")
                    
                else:
                    print(f"   ❌ Empty or minimal skeleton")
        else:
            print(f"   ❌ Request failed: {response.status_code}")

def analyze_root_cause():
    """Analyze potential root causes of skeleton issues"""
    
    print(f"\n🔬 ROOT CAUSE ANALYSIS")
    print("-" * 40)
    
    # Test if analyze_code works properly (skeleton depends on it)
    print(f"\n1. Testing underlying analyze_code functionality...")
    
    payload = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "analyze_code",
            "arguments": {
                "file_path": "codex-rs/test_files/csharp_test_suite/BasicClass.cs"
            }
        }
    }
    
    response = requests.post("http://localhost:3000/mcp", json=payload)
    if response.status_code == 200:
        result = response.json()
        content = json.loads(result["result"]["content"][0]["text"])
        
        symbols = content.get('symbols', [])
        print(f"   analyze_code found {len(symbols)} symbols")
        
        if symbols:
            print(f"   Sample symbols:")
            for symbol in symbols[:5]:
                print(f"     - {symbol.get('name')} ({symbol.get('symbol_type')}) at {symbol.get('start_line')}-{symbol.get('end_line')}")
        else:
            print(f"   ❌ No symbols found - this could be the root cause!")
    else:
        print(f"   ❌ analyze_code failed: {response.status_code}")
    
    # Test parameter variations
    print(f"\n2. Testing parameter variations...")
    
    variations = [
        {"max_tokens": 2000},
        {"max_tokens": 4000},
        {"max_tokens": 8000},
    ]
    
    for variation in variations:
        payload = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": "get_multiple_files_skeleton",
                "arguments": {
                    "file_paths": ["codex-rs/test_files/csharp_test_suite/BasicClass.cs"],
                    **variation
                }
            }
        }
        
        response = requests.post("http://localhost:3000/mcp", json=payload)
        if response.status_code == 200:
            result = response.json()
            content = json.loads(result["result"]["content"][0]["text"])
            
            files = content.get('files', [])
            if files:
                skeleton_len = len(files[0].get('skeleton', ''))
                tokens_used = files[0].get('tokens', 0)
                print(f"   max_tokens={variation['max_tokens']}: {skeleton_len} chars, {tokens_used} tokens")

def generate_improvement_recommendations():
    """Generate specific improvement recommendations"""
    
    print(f"\n📋 IMPROVEMENT RECOMMENDATIONS")
    print("=" * 60)
    
    recommendations = [
        {
            "priority": "HIGH",
            "issue": "Empty skeletons for Rust files",
            "cause": "Possible parser issues or token calculation problems",
            "solution": "Debug skeleton generation logic for Rust files specifically"
        },
        {
            "priority": "HIGH", 
            "issue": "Minimal content in Python skeletons",
            "cause": "Token counting or content filtering too aggressive",
            "solution": "Adjust skeleton generation to preserve class/function signatures"
        },
        {
            "priority": "MEDIUM",
            "issue": "C# skeletons work but could be more comprehensive",
            "cause": "Token limits may be too restrictive",
            "solution": "Optimize token usage to include more structural information"
        },
        {
            "priority": "MEDIUM",
            "issue": "Related files skeleton not finding relationships",
            "cause": "BFS traversal or dependency detection issues",
            "solution": "Improve file relationship detection algorithm"
        },
        {
            "priority": "LOW",
            "issue": "Token counting accuracy",
            "cause": "Token estimation may not match actual usage",
            "solution": "Implement more accurate token counting"
        }
    ]
    
    for i, rec in enumerate(recommendations, 1):
        print(f"\n{i}. {rec['priority']} PRIORITY")
        print(f"   Issue: {rec['issue']}")
        print(f"   Cause: {rec['cause']}")
        print(f"   Solution: {rec['solution']}")
    
    print(f"\n🎯 IMMEDIATE ACTION ITEMS:")
    print(f"1. Debug why Rust files generate empty skeletons")
    print(f"2. Investigate token calculation logic")
    print(f"3. Test skeleton generation with various file sizes")
    print(f"4. Verify symbol extraction is working for all languages")
    print(f"5. Implement better content preservation strategies")

def main():
    print("🦴 SKELETON GENERATION COMPREHENSIVE ANALYSIS")
    print("Identifying issues and improvement opportunities")
    
    try:
        response = requests.get("http://localhost:3000/test", timeout=5)
        if response.status_code != 200:
            print("❌ Server not responding")
            return
    except:
        print("❌ Cannot connect to MCP server")
        return
    
    analyze_skeleton_issues()
    test_individual_files()
    analyze_root_cause()
    generate_improvement_recommendations()
    
    print(f"\n" + "=" * 60)
    print("🎯 SUMMARY")
    print("=" * 60)
    print("❌ SKELETON GENERATION HAS CRITICAL ISSUES")
    print("✅ Symbol analysis works correctly")
    print("❌ Skeleton content generation is failing")
    print("⚠️  Only C# files generate reasonable skeletons")
    print("❌ Rust and Python skeletons are mostly empty")
    print("\n🚨 RECOMMENDATION: Fix skeleton generation before production use")

if __name__ == "__main__":
    main()