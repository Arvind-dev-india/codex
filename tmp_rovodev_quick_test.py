#!/usr/bin/env python3
import json
import requests
import time

def test_both_apis():
    mcp_url = "http://localhost:3000/mcp"
    test_file = "codex-rs/test_files/csharp_test_suite/BasicClass.cs"
    
    print("🔍 Testing Both APIs with Timing")
    print(f"Target file: {test_file}")
    
    # Test analyze_code
    print("\n=== Testing analyze_code ===")
    start_time = time.time()
    analyze_payload = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "analyze_code",
            "arguments": {
                "file_path": test_file
            }
        }
    }
    
    response = requests.post(mcp_url, json=analyze_payload)
    analyze_time = time.time() - start_time
    
    if response.status_code == 200:
        result = response.json()
        content = result.get("result", {}).get("content", [])
        if content and len(content) > 0:
            text_content = content[0].get("text", "")
            try:
                data = json.loads(text_content)
                symbols = data.get("symbols", [])
                print(f"✅ analyze_code: {len(symbols)} symbols in {analyze_time:.2f}s")
            except:
                print(f"❌ analyze_code: Failed to parse response in {analyze_time:.2f}s")
        else:
            print(f"❌ analyze_code: No symbols in {analyze_time:.2f}s")
    else:
        print(f"❌ analyze_code: HTTP {response.status_code} in {analyze_time:.2f}s")
    
    # Test skeleton generation
    print("\n=== Testing skeleton generation ===")
    start_time = time.time()
    skeleton_payload = {
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "get_multiple_files_skeleton",
            "arguments": {
                "file_paths": [test_file],
                "max_tokens": 4000
            }
        }
    }
    
    response = requests.post(mcp_url, json=skeleton_payload)
    skeleton_time = time.time() - start_time
    
    if response.status_code == 200:
        result = response.json()
        content = result.get("result", {}).get("content", [])
        if content and len(content) > 0:
            text_content = content[0].get("text", "")
            try:
                data = json.loads(text_content)
                files = data.get("files", [])
                if files:
                    skeleton = files[0].get("skeleton", "")
                    if "Generated skeleton with" in skeleton and "symbols detected" in skeleton:
                        print(f"✅ skeleton: Tree-sitter used in {skeleton_time:.2f}s")
                    elif "Fallback skeleton generation" in skeleton:
                        print(f"❌ skeleton: Still using fallback in {skeleton_time:.2f}s")
                    else:
                        print(f"⚠️  skeleton: Unknown type in {skeleton_time:.2f}s")
                        print(f"First 200 chars: {skeleton[:200]}...")
            except Exception as e:
                print(f"❌ skeleton: Failed to parse response in {skeleton_time:.2f}s: {e}")
        else:
            print(f"❌ skeleton: No content in {skeleton_time:.2f}s")
    else:
        print(f"❌ skeleton: HTTP {response.status_code} in {skeleton_time:.2f}s")

if __name__ == "__main__":
    test_both_apis()