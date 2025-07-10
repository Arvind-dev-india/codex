#!/usr/bin/env python3
"""
Quick skeleton test for large projects - starts with a single file test.
"""

import json
import requests
import time
import sys
import subprocess
import os
import argparse
import signal

def quick_test(project_dir, test_file=None):
    """Quick test with a single file to verify functionality."""
    print(f"🚀 Quick Skeleton Test")
    print(f"Project directory: {project_dir}")
    
    # Find a test file if not specified
    if not test_file:
        import glob
        cs_files = glob.glob(os.path.join(project_dir, "**/*.cs"), recursive=True)
        if not cs_files:
            print("❌ No C# files found")
            return False
        
        # Pick a small file for testing
        test_file = min(cs_files, key=lambda f: os.path.getsize(f))
        test_file = os.path.relpath(test_file, project_dir)
        print(f"📄 Auto-selected test file: {test_file}")
    
    # Start server
    server_binary = None
    for path in ["../target/release/code-analysis-server", "./target/release/code-analysis-server"]:
        if os.path.exists(path):
            server_binary = path
            break
    
    if not server_binary:
        print("❌ Server binary not found")
        return False
    
    print("🚀 Starting server...")
    server_process = subprocess.Popen(
        [server_binary, '--port', '3001', '--project-dir', project_dir],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    try:
        # Wait for server
        print("⏳ Waiting for server (30s timeout)...")
        for i in range(15):
            time.sleep(2)
            try:
                response = requests.get("http://localhost:3001/health", timeout=2)
                if response.status_code == 200:
                    print("✅ Server ready!")
                    break
            except:
                print(f"   Attempt {i+1}/15...")
        else:
            print("❌ Server timeout")
            return False
        
        # Initialize with long timeout
        print("🔧 Initializing (this may take a while for large projects)...")
        init_request = {
            "jsonrpc": "2.0",
            "id": "init",
            "method": "initialize",
            "params": {
                "protocol_version": "2025-03-26",
                "capabilities": {},
                "client_info": {"name": "quick-test", "version": "1.0"}
            }
        }
        
        response = requests.post("http://localhost:3001/mcp", json=init_request, timeout=180)  # 3 minutes
        if response.status_code != 200:
            print(f"❌ Init failed: {response.status_code}")
            return False
        
        print("✅ Initialized!")
        
        # Test skeleton
        print(f"📄 Testing skeleton for: {test_file}")
        skeleton_request = {
            "jsonrpc": "2.0",
            "id": "test",
            "method": "tools/call",
            "params": {
                "name": "get_multiple_files_skeleton",
                "arguments": {
                    "file_paths": [test_file],
                    "max_tokens": 4000
                }
            }
        }
        
        response = requests.post("http://localhost:3001/mcp", json=skeleton_request, timeout=60)
        if response.status_code == 200:
            data = response.json()
            if 'result' in data and 'content' in data['result']:
                content = data['result']['content'][0]['text']
                parsed = json.loads(content)
                if 'files' in parsed and parsed['files']:
                    skeleton = parsed['files'][0]['skeleton']
                    print(f"✅ SUCCESS! Generated skeleton ({len(skeleton)} chars)")
                    print(f"\n=== SKELETON PREVIEW ===")
                    lines = skeleton.split('\n')[:20]
                    for i, line in enumerate(lines, 1):
                        print(f"{i:2d}: {line}")
                    if len(skeleton.split('\n')) > 20:
                        print("... (truncated)")
                    return True
        
        print("❌ Skeleton generation failed")
        return False
        
    except Exception as e:
        print(f"❌ Error: {e}")
        return False
    finally:
        server_process.terminate()
        try:
            server_process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            server_process.kill()

def main():
    parser = argparse.ArgumentParser(description="Quick skeleton test for large projects")
    parser.add_argument("project_dir", help="Project directory to test")
    parser.add_argument("--file", help="Specific file to test (optional)")
    
    args = parser.parse_args()
    
    if not os.path.exists(args.project_dir):
        print(f"❌ Directory not found: {args.project_dir}")
        sys.exit(1)
    
    success = quick_test(args.project_dir, args.file)
    
    if success:
        print(f"\n🎉 Quick test successful!")
        print(f"💡 You can now use the full interactive tool:")
        print(f"   python3 interactive_skeleton_test.py --project-dir '{args.project_dir}'")
    else:
        print(f"\n❌ Quick test failed")
        print(f"💡 Try with a smaller directory or specific file")
    
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()