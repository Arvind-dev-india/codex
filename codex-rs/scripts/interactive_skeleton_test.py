#!/usr/bin/env python3
"""
Interactive C# Skeleton Generation Test Tool

This tool allows you to:
1. Start the code analysis server for a specific directory
2. Interactively test skeleton generation for any file
3. View and verify skeleton output quality

Usage:
    python3 interactive_skeleton_test.py [--project-dir <path>] [--port <port>]
    
Examples:
    python3 interactive_skeleton_test.py --project-dir ../test_files/csharp_test_suite
    python3 interactive_skeleton_test.py --project-dir . --port 3001
"""

import json
import requests
import time
import sys
import subprocess
import os
import argparse
import signal
import glob
from pathlib import Path
try:
    from fuzzywuzzy import fuzz, process
    FUZZY_AVAILABLE = True
except ImportError:
    FUZZY_AVAILABLE = False

class SkeletonTester:
    def __init__(self, project_dir=".", port=3000):
        self.project_dir = os.path.abspath(project_dir)
        self.port = port
        self.server_process = None
        self.base_url = f"http://localhost:{port}"
        self.initialized = False
        
    def start_server(self):
        """Start the code analysis server."""
        print(f"🚀 Starting code analysis server...")
        print(f"   Project directory: {self.project_dir}")
        print(f"   Server port: {self.port}")
        
        # Find the server binary
        server_binary = None
        possible_paths = [
            "../target/release/code-analysis-server",
            "./target/release/code-analysis-server",
            "../../target/release/code-analysis-server",
            "./codex-rs/target/release/code-analysis-server"
        ]
        
        for path in possible_paths:
            if os.path.exists(path):
                server_binary = path
                break
        
        if not server_binary:
            print("❌ Could not find code-analysis-server binary")
            print("   Please build it first: cd codex-rs && cargo build --release -p code-analysis-server")
            return False
        
        try:
            self.server_process = subprocess.Popen(
                [server_binary, '--port', str(self.port), '--project-dir', self.project_dir],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True
            )
            
            # Wait for server to start
            print("⏳ Waiting for server to start...")
            for i in range(15):  # Increased from 10 to 15 attempts
                time.sleep(2)  # Increased from 1 to 2 seconds
                try:
                    response = requests.get(f"{self.base_url}/health", timeout=3)
                    if response.status_code == 200:
                        print(f"✅ Server started successfully!")
                        return True
                except:
                    if i < 5:
                        print(f"   Starting up... ({i+1}/15)")
                    elif i < 10:
                        print(f"   Initializing code graph... ({i+1}/15)")
                    else:
                        print(f"   Still working... ({i+1}/15)")
            
            print("❌ Server failed to start within 30 seconds")
            print("💡 Large projects may need more time. Try a smaller directory first.")
            return False
            
        except Exception as e:
            print(f"❌ Failed to start server: {e}")
            return False
    
    def initialize_mcp(self, timeout=60):
        """Initialize the MCP server with extended timeout for large projects."""
        if self.initialized:
            return True
            
        print("🔧 Initializing MCP server...")
        print("⏳ This may take longer for large projects while building code graph...")
        
        init_request = {
            "jsonrpc": "2.0",
            "id": "init",
            "method": "initialize",
            "params": {
                "protocol_version": "2025-03-26",
                "capabilities": {},
                "client_info": {"name": "interactive-skeleton-tester", "version": "1.0"}
            }
        }
        
        # Try with progressively longer timeouts
        timeouts = [20, 60, 120]  # 20s, 1min, 2min
        
        for attempt, timeout_val in enumerate(timeouts, 1):
            try:
                print(f"   Attempt {attempt}/{len(timeouts)} (timeout: {timeout_val}s)...")
                response = requests.post(f"{self.base_url}/mcp", json=init_request, timeout=timeout_val)
                if response.status_code == 200:
                    self.initialized = True
                    print("✅ MCP server initialized successfully!")
                    return True
                else:
                    print(f"❌ MCP initialization failed: {response.status_code}")
                    if attempt < len(timeouts):
                        print("   Retrying with longer timeout...")
                        continue
                    return False
            except requests.exceptions.Timeout:
                print(f"⏰ Timeout after {timeout_val}s")
                if attempt < len(timeouts):
                    print("   Retrying with longer timeout...")
                    continue
                else:
                    print("❌ MCP initialization failed after all attempts")
                    print("💡 Try with a smaller directory or check server logs")
                    return False
            except Exception as e:
                print(f"❌ MCP initialization error: {e}")
                return False
        
        return False
    
    def get_all_files(self, pattern="*.cs"):
        """Get all available C# files in the project directory."""
        # Find files matching pattern
        search_path = os.path.join(self.project_dir, "**", pattern)
        files = glob.glob(search_path, recursive=True)
        
        # Make paths relative to project directory
        relative_files = []
        for file_path in files:
            rel_path = os.path.relpath(file_path, self.project_dir)
            relative_files.append(rel_path)
        
        return sorted(relative_files)
    
    def fuzzy_search_files(self, query, files, limit=10):
        """Perform fuzzy search on files."""
        if not FUZZY_AVAILABLE:
            # Fallback to simple substring matching
            matches = []
            query_lower = query.lower()
            for file_path in files:
                if query_lower in file_path.lower():
                    matches.append(file_path)
            return matches[:limit]
        
        # Use fuzzywuzzy for better matching
        if not query.strip():
            return files[:limit]
        
        # Get fuzzy matches
        matches = process.extract(query, files, scorer=fuzz.partial_ratio, limit=limit)
        return [match[0] for match in matches if match[1] > 30]  # Minimum score threshold
    
    def interactive_file_search(self):
        """Interactive file search with fuzzy matching."""
        all_files = self.get_all_files()
        
        if not all_files:
            print("❌ No C# files found in project directory")
            return None
        
        print(f"\n🔍 Fuzzy File Search ({len(all_files)} files available)")
        if not FUZZY_AVAILABLE:
            print("💡 Install fuzzywuzzy for better search: pip install fuzzywuzzy python-levenshtein")
        
        print("Type to search, press Enter to select first match, or type number to select:")
        print("Commands: 'list' to show all files, 'quit' to cancel")
        
        while True:
            try:
                query = input("\n🔍 Search: ").strip()
                
                if not query:
                    continue
                
                if query.lower() in ['quit', 'q', 'exit']:
                    return None
                
                if query.lower() == 'list':
                    print(f"\n📁 All files ({len(all_files)}):")
                    for i, file_path in enumerate(all_files, 1):
                        file_size = os.path.getsize(os.path.join(self.project_dir, file_path))
                        print(f"   {i:2d}. {file_path} ({file_size} bytes)")
                    continue
                
                # Check if query is a number (direct selection)
                try:
                    file_index = int(query) - 1
                    if 0 <= file_index < len(all_files):
                        selected_file = all_files[file_index]
                        print(f"✅ Selected: {selected_file}")
                        return selected_file
                    else:
                        print(f"❌ Invalid number. Use 1-{len(all_files)}")
                        continue
                except ValueError:
                    pass
                
                # Perform fuzzy search
                matches = self.fuzzy_search_files(query, all_files, limit=10)
                
                if not matches:
                    print("❌ No matches found. Try a different search term.")
                    continue
                
                print(f"\n📋 Search results for '{query}':")
                for i, file_path in enumerate(matches, 1):
                    file_size = os.path.getsize(os.path.join(self.project_dir, file_path))
                    print(f"   {i}. {file_path} ({file_size} bytes)")
                
                # Auto-select if only one match
                if len(matches) == 1:
                    print(f"✅ Auto-selected: {matches[0]}")
                    return matches[0]
                
                # Ask for selection
                while True:
                    try:
                        selection = input(f"\nSelect file (1-{len(matches)}) or press Enter for #{1}: ").strip()
                        
                        if not selection:
                            selected_file = matches[0]
                            print(f"✅ Selected: {selected_file}")
                            return selected_file
                        
                        if selection.lower() in ['back', 'b']:
                            break  # Go back to search
                        
                        index = int(selection) - 1
                        if 0 <= index < len(matches):
                            selected_file = matches[index]
                            print(f"✅ Selected: {selected_file}")
                            return selected_file
                        else:
                            print(f"❌ Invalid selection. Use 1-{len(matches)}")
                    
                    except ValueError:
                        print(f"❌ Invalid input. Enter a number 1-{len(matches)}")
                    except KeyboardInterrupt:
                        return None
                
            except KeyboardInterrupt:
                print(f"\n❌ Search cancelled")
                return None
            except EOFError:
                return None
    
    def get_skeleton(self, file_path, max_tokens=6000):
        """Get skeleton for a specific file."""
        if not self.initialized:
            if not self.initialize_mcp():
                return None
        
        # Normalize the file path
        original_path = file_path
        
        # If absolute path, make it relative to project directory
        if os.path.isabs(file_path):
            file_path = os.path.relpath(file_path, self.project_dir)
        
        # Check if file exists with the given path
        full_path = os.path.join(self.project_dir, file_path)
        if not os.path.exists(full_path):
            # Try to find the file by searching for it
            print(f"🔍 File not found at: {file_path}")
            print(f"   Searching for file...")
            
            # Search for the file in the project directory
            import glob
            search_patterns = [
                os.path.join(self.project_dir, "**", os.path.basename(file_path)),
                os.path.join(self.project_dir, "**", file_path),
                os.path.join(self.project_dir, file_path.replace("\\", "/")),
                os.path.join(self.project_dir, file_path.replace("/", "\\")),
            ]
            
            found_file = None
            for pattern in search_patterns:
                matches = glob.glob(pattern, recursive=True)
                if matches:
                    found_file = matches[0]
                    break
            
            if found_file:
                file_path = os.path.relpath(found_file, self.project_dir)
                print(f"   ✅ Found file at: {file_path}")
                full_path = found_file
            else:
                print(f"❌ File not found anywhere: {original_path}")
                print(f"   Tried searching for: {os.path.basename(file_path)}")
                return None
        
        print(f"📄 Generating skeleton for: {file_path}")
        print(f"   Max tokens: {max_tokens}")
        
        skeleton_request = {
            "jsonrpc": "2.0",
            "id": "skeleton_request",
            "method": "tools/call",
            "params": {
                "name": "get_multiple_files_skeleton",
                "arguments": {
                    "file_paths": [file_path],
                    "max_tokens": max_tokens
                }
            }
        }
        
        try:
            response = requests.post(f"{self.base_url}/mcp", json=skeleton_request, timeout=30)
            if response.status_code != 200:
                print(f"❌ Request failed: {response.status_code}")
                return None
            
            data = response.json()
            
            if 'result' in data and 'content' in data['result']:
                content = data['result']['content'][0]['text']
                
                # Parse the JSON content to get the actual skeleton
                try:
                    parsed_content = json.loads(content)
                    if 'files' in parsed_content and parsed_content['files']:
                        file_info = parsed_content['files'][0]
                        skeleton_text = file_info.get('skeleton', '')
                        tokens_used = file_info.get('tokens', 0)
                        
                        return {
                            'skeleton': skeleton_text,
                            'tokens_used': tokens_used,
                            'file_path': file_path,
                            'total_files': parsed_content.get('total_files', 1)
                        }
                except json.JSONDecodeError:
                    print(f"❌ Failed to parse skeleton response")
                    return None
            
            print(f"❌ Unexpected response format")
            return None
            
        except Exception as e:
            print(f"❌ Error getting skeleton: {e}")
            return None
    
    def analyze_skeleton(self, skeleton_result):
        """Analyze and display skeleton quality."""
        if not skeleton_result:
            return
        
        skeleton_text = skeleton_result['skeleton']
        file_path = skeleton_result['file_path']
        tokens_used = skeleton_result['tokens_used']
        
        # Read original file for comparison
        full_path = os.path.join(self.project_dir, file_path)
        try:
            with open(full_path, 'r', encoding='utf-8') as f:
                original_content = f.read()
        except:
            original_content = ""
        
        print(f"\n=== SKELETON ANALYSIS ===")
        print(f"File: {file_path}")
        print(f"Original: {len(original_content)} chars, {len(original_content.split('\n'))} lines")
        print(f"Skeleton: {len(skeleton_text)} chars, {len(skeleton_text.split('\n'))} lines")
        print(f"Tokens used: {tokens_used}")
        print(f"Compression: {len(skeleton_text)/len(original_content)*100:.1f}%" if original_content else "N/A")
        
        # Quality checks
        checks = [
            ("Line numbers", "// Lines" in skeleton_text),
            ("Using statements", skeleton_text.strip().startswith("using") or "using " in skeleton_text),
            ("Namespace/symbols", "namespace " in skeleton_text or "// symbol:" in skeleton_text),
            ("Class definitions", "class " in skeleton_text),
            ("Method signatures", "public " in skeleton_text and "(" in skeleton_text),
            ("Property signatures", "{ get; set; }" in skeleton_text or "get;" in skeleton_text),
            ("Ellipsis usage", "..." in skeleton_text),
            ("Code structure", skeleton_text.count("{") > 0),
        ]
        
        print(f"\n📋 Quality Checks:")
        passed = 0
        for check_name, result in checks:
            status = "✅" if result else "❌"
            print(f"   {status} {check_name}")
            if result:
                passed += 1
        
        print(f"\n🎯 Quality Score: {passed}/{len(checks)}")
        
        if passed >= len(checks) - 1:
            print("🎉 EXCELLENT skeleton quality!")
        elif passed >= len(checks) - 2:
            print("✅ GOOD skeleton quality")
        else:
            print("⚠️ Skeleton quality could be improved")
    
    def display_skeleton(self, skeleton_result, show_lines=50):
        """Display the skeleton content."""
        if not skeleton_result:
            return
        
        skeleton_text = skeleton_result['skeleton']
        
        print(f"\n=== SKELETON CONTENT ===")
        lines = skeleton_text.split('\n')
        
        for i, line in enumerate(lines[:show_lines], 1):
            print(f"{i:3d}: {line}")
        
        if len(lines) > show_lines:
            print(f"... ({len(lines) - show_lines} more lines)")
        
        print("=" * 50)
    
    def interactive_mode(self):
        """Run in interactive mode."""
        print(f"\n🎮 Interactive Skeleton Testing Mode")
        print(f"Project directory: {self.project_dir}")
        print(f"Server URL: {self.base_url}")
        print(f"\nCommands:")
        print(f"  search             - Fuzzy search and select file")
        print(f"  skeleton <file>    - Generate skeleton for file")
        print(f"  analyze <file>     - Generate and analyze skeleton")
        print(f"  show <file> [lines] - Generate and display skeleton")
        print(f"  tokens <number>    - Set max tokens (default: 6000)")
        print(f"  list [pattern]     - List all files (fallback)")
        print(f"  help              - Show this help")
        print(f"  quit              - Exit")
        
        max_tokens = 6000
        
        while True:
            try:
                command = input(f"\n[skeleton-test] ").strip()
                
                if not command:
                    continue
                
                parts = command.split()
                cmd = parts[0].lower()
                
                if cmd in ['quit', 'exit', 'q']:
                    break
                
                elif cmd == 'help':
                    print(f"\nCommands:")
                    print(f"  search             - Fuzzy search and select file")
                    print(f"  skeleton <file>    - Generate skeleton for file")
                    print(f"  analyze <file>     - Generate and analyze skeleton")
                    print(f"  show <file> [lines] - Generate and display skeleton")
                    print(f"  tokens <number>    - Set max tokens")
                    print(f"  list [pattern]     - List all files")
                    print(f"  help              - Show this help")
                    print(f"  quit              - Exit")
                
                elif cmd == 'search':
                    # Interactive fuzzy search
                    selected_file = self.interactive_file_search()
                    if selected_file:
                        # Automatically analyze the selected file
                        result = self.get_skeleton(selected_file, max_tokens)
                        if result:
                            self.display_skeleton(result, 50)
                            self.analyze_skeleton(result)
                
                elif cmd == 'list':
                    pattern = parts[1] if len(parts) > 1 else "*.cs"
                    all_files = self.get_all_files(pattern)
                    if all_files:
                        print(f"\n📁 Available files ({len(all_files)}):")
                        for i, file_path in enumerate(all_files, 1):
                            file_size = os.path.getsize(os.path.join(self.project_dir, file_path))
                            print(f"   {i:2d}. {file_path} ({file_size} bytes)")
                        print(f"\n💡 Usage tips:")
                        print(f"   • Use 'search' for fuzzy finding")
                        print(f"   • Use 'analyze <number>' to select by number (e.g., 'analyze 873')")
                        print(f"   • Use 'analyze <partial_path>' for direct selection")
                    else:
                        print(f"❌ No files found matching: {pattern}")
                
                elif cmd == 'tokens':
                    if len(parts) > 1:
                        try:
                            max_tokens = int(parts[1])
                            print(f"✅ Max tokens set to: {max_tokens}")
                        except ValueError:
                            print(f"❌ Invalid token count: {parts[1]}")
                    else:
                        print(f"Current max tokens: {max_tokens}")
                
                elif cmd in ['skeleton', 'analyze', 'show']:
                    if len(parts) < 2:
                        # If no file specified, start fuzzy search
                        print(f"💡 No file specified. Starting fuzzy search...")
                        selected_file = self.interactive_file_search()
                        if not selected_file:
                            continue
                        file_path = selected_file
                    else:
                        # Join all parts after the command (in case path has spaces)
                        file_path = ' '.join(parts[1:])
                        
                        # If it looks like a number, try to select from file list
                        try:
                            file_index = int(file_path) - 1
                            all_files = self.get_all_files()
                            if 0 <= file_index < len(all_files):
                                file_path = all_files[file_index]
                                print(f"✅ Selected file #{file_index + 1}: {file_path}")
                        except ValueError:
                            pass  # Not a number, treat as file path
                    
                    result = self.get_skeleton(file_path, max_tokens)
                    
                    if result:
                        if cmd == 'skeleton':
                            print(f"✅ Skeleton generated ({result['tokens_used']} tokens)")
                        elif cmd == 'analyze':
                            self.analyze_skeleton(result)
                        elif cmd == 'show':
                            show_lines = int(parts[2]) if len(parts) > 2 else 50
                            self.display_skeleton(result, show_lines)
                            self.analyze_skeleton(result)
                
                else:
                    print(f"❌ Unknown command: {cmd}")
                    print(f"💡 Try 'search' to find and analyze files, or 'help' for all commands")
            
            except KeyboardInterrupt:
                print(f"\n\n👋 Goodbye!")
                break
            except EOFError:
                break
    
    def cleanup(self):
        """Clean up resources."""
        if self.server_process:
            print(f"\n🧹 Shutting down server...")
            self.server_process.terminate()
            try:
                self.server_process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                self.server_process.kill()
            print(f"✅ Server stopped")

def main():
    parser = argparse.ArgumentParser(description="Interactive C# Skeleton Generation Test Tool")
    parser.add_argument("--project-dir", "-d", default=".", 
                       help="Project directory to analyze (default: current directory)")
    parser.add_argument("--port", "-p", type=int, default=3000,
                       help="Port for the HTTP server (default: 3000)")
    parser.add_argument("--list-only", "-l", action="store_true",
                       help="Just list available files and exit")
    
    args = parser.parse_args()
    
    # Validate project directory
    if not os.path.exists(args.project_dir):
        print(f"❌ Project directory does not exist: {args.project_dir}")
        sys.exit(1)
    
    tester = SkeletonTester(args.project_dir, args.port)
    
    # Set up signal handler for cleanup
    def signal_handler(sig, frame):
        print(f"\n\n🛑 Interrupted by user")
        tester.cleanup()
        sys.exit(0)
    
    signal.signal(signal.SIGINT, signal_handler)
    
    try:
        if args.list_only:
            # Just list files and exit
            files = tester.get_all_files()
            print(f"📁 Found {len(files)} C# files:")
            for i, file_path in enumerate(files, 1):
                file_size = os.path.getsize(os.path.join(tester.project_dir, file_path))
                print(f"   {i:2d}. {file_path} ({file_size} bytes)")
            return
        
        # Start server
        if not tester.start_server():
            sys.exit(1)
        
        # Initialize MCP
        if not tester.initialize_mcp():
            sys.exit(1)
        
        # Check for files before full initialization
        print("📁 Scanning for C# files...")
        files = tester.get_all_files()
        
        if not files:
            print(f"❌ No C# files found in {args.project_dir}")
            print(f"💡 Make sure the directory contains .cs files")
            sys.exit(1)
        
        print(f"✅ Found {len(files)} C# files")
        
        # Enter interactive mode
        tester.interactive_mode()
        
    finally:
        tester.cleanup()

if __name__ == "__main__":
    main()