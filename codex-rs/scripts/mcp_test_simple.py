#!/usr/bin/env python3
"""
Simple MCP test with very long timeouts for large projects.
"""

import json
import requests
import time

def test_with_patience():
    """Test MCP with very long timeouts."""
    print("=== Patient MCP Test for Large Projects ===\n")
    
    base_url = "http://localhost:3000"
    
    # Test health first
    try:
        response = requests.get(f"{base_url}/health", timeout=5)
        print(f"✅ Server health: {response.status_code}")
    except Exception as e:
        print(f"❌ Server not responding: {e}")
        return False
    
    # Initialize with very long timeout
    print("🔧 Initializing MCP (this may take several minutes for large projects)...")
    init_request = {
        "jsonrpc": "2.0",
        "id": "init",
        "method": "initialize",
        "params": {
            "protocol_version": "2025-03-26",
            "capabilities": {},
            "client_info": {"name": "patient-test", "version": "1.0"}
        }
    }
    
    try:
        print("   Waiting up to 5 minutes for initialization...")
        start_time = time.time()
        response = requests.post(f"{base_url}/mcp", json=init_request, timeout=300)  # 5 minutes
        elapsed = time.time() - start_time
        
        if response.status_code == 200:
            print(f"✅ MCP initialized successfully! (took {elapsed:.1f}s)")
            
            # Now test skeleton generation
            print("🔧 Testing skeleton generation...")
            skeleton_request = {
                "jsonrpc": "2.0",
                "id": "skeleton",
                "method": "tools/call",
                "params": {
                    "name": "get_multiple_files_skeleton",
                    "arguments": {
                        "file_paths": ["src/Tools/TestAADCallsForLinux/Program.cs"],
                        "max_tokens": 4000
                    }
                }
            }
            
            start_time = time.time()
            response = requests.post(f"{base_url}/mcp", json=skeleton_request, timeout=180)  # 3 minutes
            elapsed = time.time() - start_time
            
            if response.status_code == 200:
                print(f"✅ Skeleton generation successful! (took {elapsed:.1f}s)")
                data = response.json()
                
                # Extract skeleton
                if 'result' in data and 'content' in data['result']:
                    content = data['result']['content'][0]['text']
                    parsed = json.loads(content)
                    if 'files' in parsed and parsed['files']:
                        skeleton = parsed['files'][0]['skeleton']
                        print(f"✅ Generated skeleton: {len(skeleton)} chars")
                        print(f"\nFirst 10 lines of skeleton:")
                        for i, line in enumerate(skeleton.split('\n')[:10], 1):
                            print(f"  {i:2d}: {line}")
                        
                        print(f"\n🎉 SUCCESS! MCP server is working correctly")
                        print(f"💡 For MCP Inspector:")
                        print(f"   - Use HTTP transport (not SSE)")
                        print(f"   - URL: http://localhost:3000/mcp")
                        print(f"   - Increase timeout to at least 5 minutes")
                        print(f"   - Or use the built-in web interface: http://localhost:3000/test")
                        return True
                
            else:
                print(f"❌ Skeleton failed: {response.status_code}")
                return False
        else:
            print(f"❌ Init failed: {response.status_code}")
            return False
            
    except requests.exceptions.Timeout:
        print(f"❌ Still timing out - your project might be too large")
        print(f"💡 Try with a smaller subdirectory:")
        print(f"   ./target/release/code-analysis-server --sse --project-dir /mnt/c/One/Mgmt-RecoverySvcs-WkloadExtn/src/")
        return False
    except Exception as e:
        print(f"❌ Error: {e}")
        return False

if __name__ == "__main__":
    test_with_patience()