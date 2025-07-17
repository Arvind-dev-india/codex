#!/usr/bin/env python3
"""
Cross-Project Analysis Test Suite using MCP Server
This tests the enhanced BFS approach with cross-project skeleton generation.

USAGE:
1. Start MCP server in another terminal:
   ./target/release/code-analysis-server --sse --project-dir /home/arvkum/projects/codex/codex-rs/test_cross_project_mcp/MainProject --supplementary SkeletonProject:/home/arvkum/projects/codex/codex-rs/test_cross_project_mcp/SkeletonProject --verbose

2. Run this test:
   python3 test_cross_project_mcp_suite.py http://localhost:3000

EXPECTED RESULTS:
- ✅ UserService.cs analysis: 4+ symbols found
- ✅ Cross-project skeleton: 3+ files from SkeletonProject with proper parsed skeletons
- ✅ Symbol references: Cross-project references detected
- ✅ Symbol definitions: ValidationHelper found in SkeletonProject
"""

import json
import time
import requests
import sys

def test_mcp_endpoint(endpoint_url, test_name, request_data):
    """Test an MCP endpoint and return results"""
    print(f"\n--- {test_name} ---")
    
    try:
        response = requests.post(
            endpoint_url,
            headers={'Content-Type': 'application/json'},
            json=request_data,
            timeout=30
        )
        
        if response.status_code == 200:
            result = response.json()
            if 'result' in result and not result.get('isError', False):
                print(f"✅ {test_name} SUCCESS")
                return result
            else:
                print(f"❌ {test_name} FAILED - MCP Error")
                print(f"Response: {json.dumps(result, indent=2)}")
                return None
        else:
            print(f"❌ {test_name} FAILED - HTTP {response.status_code}")
            print(f"Response: {response.text}")
            return None
            
    except Exception as e:
        print(f"❌ {test_name} FAILED - Exception: {e}")
        return None

def main():
    if len(sys.argv) < 2:
        print("❌ ERROR: Server URL required")
        print()
        print("USAGE:")
        print("1. Start MCP server in another terminal:")
        print("   ./target/release/code-analysis-server --sse --project-dir /home/arvkum/projects/codex/codex-rs/test_cross_project_mcp/MainProject --supplementary SkeletonProject:/home/arvkum/projects/codex/codex-rs/test_cross_project_mcp/SkeletonProject --verbose")
        print()
        print("2. Run this test:")
        print("   python3 test_cross_project_mcp_suite.py http://localhost:3000")
        return
    
    server_url = sys.argv[1].rstrip('/')
    mcp_endpoint = f"{server_url}/mcp"
    
    print("=== Cross-Project Analysis Test Suite ===")
    print(f"Testing MCP server at: {mcp_endpoint}")
    
    # Test 0: Check server health and available tools
    print("\n--- Test 0: Server Health Check ---")
    tools_request = {
        "jsonrpc": "2.0",
        "id": "test_tools",
        "method": "tools/list",
        "params": {}
    }
    
    tools_result = test_mcp_endpoint(mcp_endpoint, "List Available Tools", tools_request)
    if tools_result:
        tools = tools_result['result'].get('tools', [])
        tool_names = [tool['name'] for tool in tools]
        print(f"   Available tools: {', '.join(tool_names)}")
        
        required_tools = ['analyze_code', 'get_related_files_skeleton', 'find_symbol_references', 'find_symbol_definitions', 'get_symbol_subgraph']
        missing_tools = [tool for tool in required_tools if tool not in tool_names]
        if missing_tools:
            print(f"   ❌ Missing required tools: {missing_tools}")
        else:
            print("   ✅ All required tools available")
    else:
        print("   ❌ Failed to get tools list - server may not be running correctly")
        return
    
    # Test 1: Analyze main project file
    analyze_request = {
        "jsonrpc": "2.0",
        "id": "test_analyze",
        "method": "tools/call",
        "params": {
            "name": "analyze_code",
            "arguments": {
                "file_path": "UserService.cs"
            }
        }
    }
    
    analyze_result = test_mcp_endpoint(mcp_endpoint, "Analyze UserService.cs", analyze_request)
    if analyze_result:
        content = json.loads(analyze_result['result']['content'][0]['text'])
        symbols = content.get('symbols', [])
        print(f"   Found {len(symbols)} symbols in UserService.cs")
        for symbol in symbols:
            print(f"   - {symbol['name']} ({symbol['symbol_type']}) at lines {symbol['start_line']}-{symbol['end_line']}")
    
    # Test 2: Get related files skeleton (this should find cross-project files)
    skeleton_request = {
        "jsonrpc": "2.0",
        "id": "test_skeleton",
        "method": "tools/call",
        "params": {
            "name": "get_related_files_skeleton",
            "arguments": {
                "active_files": ["UserService.cs"],
                "max_tokens": 4000,
                "max_depth": 2
            }
        }
    }
    
    skeleton_result = test_mcp_endpoint(mcp_endpoint, "Get Related Files Skeleton", skeleton_request)
    if skeleton_result:
        content = json.loads(skeleton_result['result']['content'][0]['text'])
        total_files = content.get('total_files', 0)
        cross_project_files = content.get('cross_project_files', 0)
        cross_project_detected = content.get('cross_project_boundaries_detected', False)
        
        print(f"   Total related files: {total_files}")
        print(f"   Cross-project files: {cross_project_files}")
        print(f"   Cross-project boundaries detected: {cross_project_detected}")
        
        if cross_project_detected and cross_project_files > 0:
            print("   ✅ CROSS-PROJECT ANALYSIS WORKING!")
            files = content.get('files', [])
            for file_info in files:
                file_path = file_info.get('file_path', 'unknown')
                skeleton = file_info.get('skeleton', '')
                project_type = "CROSS-PROJECT" if "SkeletonProject" in file_path else "MAIN PROJECT"
                
                # Check skeleton quality
                if "SkeletonProject" in file_path:
                    if "Fallback skeleton generation" in skeleton:
                        skeleton_quality = "❌ FALLBACK"
                    else:
                        skeleton_quality = "✅ PARSED"
                    print(f"   - {file_path} ({project_type}) - {skeleton_quality} skeleton ({len(skeleton)} chars)")
                else:
                    print(f"   - {file_path} ({project_type})")
        else:
            print("   ❌ Cross-project analysis not working - no cross-project files found")
            print(f"   Summary: {content.get('summary', 'No summary')}")
    
    # Test 3: Find symbol references (should find cross-project references)
    references_request = {
        "jsonrpc": "2.0",
        "id": "test_references",
        "method": "tools/call",
        "params": {
            "name": "find_symbol_references",
            "arguments": {
                "symbol_name": "User"
            }
        }
    }
    
    references_result = test_mcp_endpoint(mcp_endpoint, "Find Symbol References for 'User'", references_request)
    if references_result:
        content = json.loads(references_result['result']['content'][0]['text'])
        references = content.get('references', [])
        print(f"   Found {len(references)} references to 'User'")
        
        main_refs = sum(1 for ref in references if "UserService.cs" in ref.get('file_path', ''))
        cross_refs = sum(1 for ref in references if "SkeletonProject" in ref.get('file_path', ''))
        
        print(f"   Main project references: {main_refs}")
        print(f"   Cross-project references: {cross_refs}")
    
    # Test 4: Find symbol definitions (should find both main and cross-project definitions)
    definitions_request = {
        "jsonrpc": "2.0",
        "id": "test_definitions",
        "method": "tools/call",
        "params": {
            "name": "find_symbol_definitions",
            "arguments": {
                "symbol_name": "ValidationHelper"
            }
        }
    }
    
    definitions_result = test_mcp_endpoint(mcp_endpoint, "Find Symbol Definitions for 'ValidationHelper'", definitions_request)
    if definitions_result:
        content = json.loads(definitions_result['result']['content'][0]['text'])
        definitions = content.get('definitions', [])
        print(f"   Found {len(definitions)} definitions for 'ValidationHelper'")
        
        for definition in definitions:
            file_path = definition.get('file_path', 'unknown')
            project_type = "CROSS-PROJECT" if "SkeletonProject" in file_path else "MAIN PROJECT"
            print(f"   - {file_path} ({project_type}) at lines {definition.get('start_line', 0)}-{definition.get('end_line', 0)}")
    
    # Test 5: Get symbol subgraph (should show cross-project connections)
    subgraph_request = {
        "jsonrpc": "2.0",
        "id": "test_subgraph",
        "method": "tools/call",
        "params": {
            "name": "get_symbol_subgraph",
            "arguments": {
                "symbol_name": "UserService",
                "max_depth": 2
            }
        }
    }
    
    subgraph_result = test_mcp_endpoint(mcp_endpoint, "Get Symbol Subgraph for 'UserService'", subgraph_request)
    if subgraph_result:
        content = json.loads(subgraph_result['result']['content'][0]['text'])
        nodes = content.get('nodes', [])
        edges = content.get('edges', [])
        print(f"   Found {len(nodes)} nodes and {len(edges)} edges in subgraph")
        
        cross_project_nodes = sum(1 for node in nodes if "SkeletonProject" in node.get('file_path', ''))
        main_project_nodes = len(nodes) - cross_project_nodes
        print(f"   Main project nodes: {main_project_nodes}")
        print(f"   Cross-project nodes: {cross_project_nodes}")
        
        if cross_project_nodes > 0:
            print("   ✅ Cross-project nodes found in subgraph")
        else:
            print("   ❌ No cross-project nodes in subgraph")
    
    # Test 6: Get multiple files skeleton (test batch processing)
    multiple_files_request = {
        "jsonrpc": "2.0",
        "id": "test_multiple_files",
        "method": "tools/call",
        "params": {
            "name": "get_multiple_files_skeleton",
            "arguments": {
                "file_paths": ["UserService.cs"],
                "max_tokens": 4000
            }
        }
    }
    
    multiple_files_result = test_mcp_endpoint(mcp_endpoint, "Get Multiple Files Skeleton", multiple_files_request)
    if multiple_files_result:
        content = json.loads(multiple_files_result['result']['content'][0]['text'])
        files = content.get('files', [])
        print(f"   Processed {len(files)} files in batch")
        
        for file_info in files:
            file_path = file_info.get('file_path', 'unknown')
            skeleton = file_info.get('skeleton', '')
            print(f"   - {file_path}: {len(skeleton)} chars")
    
    print("\n=== Test Suite Complete ===")
    print()
    print("🎯 EXPECTED RESULTS for working cross-project analysis:")
    print("1. ✅ UserService.cs should have 4+ symbols")
    print("2. ✅ Related files skeleton should find 3+ cross-project files from SkeletonProject with PARSED skeletons")
    print("3. ✅ Symbol references should find cross-project references")
    print("4. ✅ Symbol definitions should find ValidationHelper in SkeletonProject")
    print("5. ✅ Symbol subgraph should include cross-project nodes")
    print("6. ✅ Multiple files skeleton should work for batch processing")
    print()
    print("📋 TO RUN THIS TEST:")
    print("1. Start MCP server: ./target/release/code-analysis-server --sse --project-dir /home/arvkum/projects/codex/codex-rs/test_cross_project_mcp/MainProject --supplementary SkeletonProject:/home/arvkum/projects/codex/codex-rs/test_cross_project_mcp/SkeletonProject --verbose")
    print("2. Run test: python3 test_cross_project_mcp_suite.py http://localhost:3000")
    print()
    print("🔍 DEBUGGING TIPS:")
    print("- Check server logs for 'Supplementary registry with X symbols stored in graph manager'")
    print("- Look for 'DEBUG: Supplementary registry contains X symbols' in enhanced BFS logs")
    print("- Verify 'Cross-project boundaries detected: true' in skeleton results")

if __name__ == "__main__":
    main()