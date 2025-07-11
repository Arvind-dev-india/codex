#!/usr/bin/env python3
"""
Comprehensive skeleton generation test for all files in test suites
This script tests skeleton generation quality across C#, C++, and Python files
and provides detailed analysis of element preservation and areas for improvement.
"""

import json
import requests
import subprocess
import time
import os
from pathlib import Path

def start_server():
    """Start the code analysis server"""
    print("Starting code analysis server...")
    project_dir = "/home/arvkum/project/codex"
    cmd = [
        "./codex-rs/target/release/code-analysis-server",
        "--sse",
        "--project-dir", project_dir
    ]
    
    process = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    time.sleep(5)  # Give time for graph building
    return process

def get_all_test_files():
    """Get all test files organized by language"""
    test_files = {
        "C#": [],
        "C++": [],
        "Python": []
    }
    
    # Find all test files
    for root, dirs, files in os.walk("codex-rs/test_files"):
        for file in files:
            file_path = os.path.join(root, file)
            if file.endswith('.cs'):
                test_files["C#"].append(file_path)
            elif file.endswith(('.cpp', '.h')):
                test_files["C++"].append(file_path)
            elif file.endswith('.py'):
                test_files["Python"].append(file_path)
    
    # Sort files for consistent output
    for lang in test_files:
        test_files[lang].sort()
    
    return test_files

def analyze_file_first(file_path):
    """Ensure file is analyzed and added to graph manager"""
    payload = {
        "jsonrpc": "2.0",
        "id": f"analyze_{os.path.basename(file_path)}",
        "method": "tools/call",
        "params": {
            "name": "analyze_code",
            "arguments": {
                "file_path": file_path
            }
        }
    }
    
    try:
        response = requests.post(
            "http://localhost:3000/mcp",
            json=payload,
            headers={"Content-Type": "application/json"},
            timeout=30
        )
        return response.status_code == 200
    except:
        return False

def get_skeleton(file_path):
    """Get skeleton for a file"""
    payload = {
        "jsonrpc": "2.0",
        "id": f"skeleton_{os.path.basename(file_path)}",
        "method": "tools/call",
        "params": {
            "name": "get_multiple_files_skeleton",
            "arguments": {
                "file_paths": [file_path],
                "max_tokens": 4000
            }
        }
    }
    
    try:
        response = requests.post(
            "http://localhost:3000/mcp",
            json=payload,
            headers={"Content-Type": "application/json"},
            timeout=30
        )
        
        if response.status_code == 200:
            result = response.json()
            if "result" in result and "content" in result["result"]:
                content = result["result"]["content"][0]["text"]
                skeleton_data = json.loads(content)
                if "files" in skeleton_data and len(skeleton_data["files"]) > 0:
                    return skeleton_data["files"][0]["skeleton"]
        return None
    except:
        return None

def analyze_original_file(file_path):
    """Analyze the original file to understand its structure"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        lines = content.splitlines()
        analysis = {
            "total_lines": len(lines),
            "non_empty_lines": len([l for l in lines if l.strip()]),
            "comment_lines": len([l for l in lines if l.strip().startswith(('//','#','/*'))]),
        }
        
        # Language-specific analysis
        if file_path.endswith('.py'):
            analysis.update({
                "classes": content.count('class '),
                "functions": content.count('def '),
                "imports": content.count('import ') + content.count('from '),
                "docstrings": content.count('"""') // 2,
                "decorators": content.count('@'),
            })
        elif file_path.endswith('.cs'):
            analysis.update({
                "namespaces": content.count('namespace '),
                "classes": content.count('class ') + content.count('interface '),
                "methods": content.count('public ') + content.count('private ') + content.count('protected '),
                "using_statements": content.count('using '),
                "properties": content.count('{ get; set; }') + content.count('get;') + content.count('set;'),
            })
        elif file_path.endswith(('.cpp', '.h')):
            analysis.update({
                "includes": content.count('#include'),
                "classes": content.count('class '),
                "functions": content.count('::') + len([l for l in lines if '(' in l and not l.strip().startswith('//')]),
                "public_sections": content.count('public:'),
                "private_sections": content.count('private:'),
                "templates": content.count('template<'),
            })
        
        return analysis
    except Exception as e:
        return {"error": str(e)}

def analyze_skeleton_quality(skeleton, original_analysis, file_path, language):
    """Analyze skeleton quality against original file"""
    if not skeleton:
        return {"error": "No skeleton generated"}
    
    skeleton_lines = len(skeleton.splitlines())
    original_lines = original_analysis.get("total_lines", 0)
    
    quality = {
        "skeleton_lines": skeleton_lines,
        "original_lines": original_lines,
        "compression_ratio": (skeleton_lines / original_lines * 100) if original_lines > 0 else 0,
        "tree_sitter_used": "Generated from Tree-sitter parsed symbols" in skeleton,
        "fallback_used": "fallback skeleton" in skeleton.lower(),
    }
    
    # Check element preservation
    if language == "Python":
        quality["elements"] = {
            "classes": skeleton.count('class '),
            "functions": skeleton.count('def '),
            "imports": skeleton.count('import ') + skeleton.count('from '),
            "line_refs": skeleton.count('# Line ') + skeleton.count('// Line'),
        }
        quality["preservation"] = {
            "classes": (quality["elements"]["classes"] / max(original_analysis.get("classes", 1), 1)) * 100,
            "functions": (quality["elements"]["functions"] / max(original_analysis.get("functions", 1), 1)) * 100,
            "imports": (quality["elements"]["imports"] / max(original_analysis.get("imports", 1), 1)) * 100,
        }
    elif language == "C#":
        quality["elements"] = {
            "namespaces": skeleton.count('namespace '),
            "classes": skeleton.count('class ') + skeleton.count('interface '),
            "methods": skeleton.count('public ') + skeleton.count('private '),
            "using_statements": skeleton.count('using '),
            "line_refs": skeleton.count('// Line'),
        }
        quality["preservation"] = {
            "namespaces": (quality["elements"]["namespaces"] / max(original_analysis.get("namespaces", 1), 1)) * 100,
            "classes": (quality["elements"]["classes"] / max(original_analysis.get("classes", 1), 1)) * 100,
            "methods": (quality["elements"]["methods"] / max(original_analysis.get("methods", 1), 1)) * 100,
        }
    elif language == "C++":
        quality["elements"] = {
            "includes": skeleton.count('#include'),
            "classes": skeleton.count('class '),
            "functions": skeleton.count('::') + skeleton.count('('),
            "line_refs": skeleton.count('// Line'),
        }
        quality["preservation"] = {
            "includes": (quality["elements"]["includes"] / max(original_analysis.get("includes", 1), 1)) * 100,
            "classes": (quality["elements"]["classes"] / max(original_analysis.get("classes", 1), 1)) * 100,
            "functions": (quality["elements"]["functions"] / max(original_analysis.get("functions", 1), 1)) * 100,
        }
    
    return quality

def test_file_comprehensive(file_path, language, verbose=True):
    """Comprehensive test of a single file"""
    if verbose:
        print(f"\n{'='*80}")
        print(f"🧪 TESTING: {file_path}")
        print(f"Language: {language}")
        print(f"{'='*80}")
    
    # Analyze original file
    original_analysis = analyze_original_file(file_path)
    if "error" in original_analysis:
        if verbose:
            print(f"❌ Failed to analyze original file: {original_analysis['error']}")
        return None
    
    if verbose:
        print(f"📄 Original file: {original_analysis['total_lines']} lines, {original_analysis['non_empty_lines']} non-empty")
    
    # Ensure file is analyzed
    if not analyze_file_first(file_path):
        if verbose:
            print("⚠️  Failed to pre-analyze file")
    
    # Get skeleton
    skeleton = get_skeleton(file_path)
    if not skeleton:
        if verbose:
            print("❌ Failed to generate skeleton")
        return {
            "file_path": file_path,
            "language": language,
            "success": False,
            "original_analysis": original_analysis
        }
    
    # Analyze skeleton quality
    quality = analyze_skeleton_quality(skeleton, original_analysis, file_path, language)
    
    # Print results
    if verbose:
        print(f"✅ Skeleton generated: {quality['skeleton_lines']} lines ({quality['compression_ratio']:.1f}% of original)")
        
        if quality['tree_sitter_used']:
            print("🎉 Using Tree-sitter parsed symbols")
        elif quality['fallback_used']:
            print("⚠️  Using fallback generation")
        else:
            print("🔍 Using enhanced parsing")
        
        print(f"\n📊 Element Detection:")
        for element, count in quality['elements'].items():
            preservation = quality['preservation'].get(element, 0)
            status = "✅" if preservation >= 80 else "⚠️" if preservation >= 50 else "❌"
            print(f"  {status} {element}: {count} ({preservation:.1f}% preserved)")
        
        # Show skeleton preview
        print(f"\n📄 SKELETON PREVIEW (first 400 chars):")
        print("-" * 60)
        print(skeleton[:400])
        if len(skeleton) > 400:
            print("...")
        print("-" * 60)
    
    return {
        "file_path": file_path,
        "language": language,
        "success": True,
        "original_analysis": original_analysis,
        "quality": quality,
        "skeleton": skeleton
    }

def run_comprehensive_test(verbose=True):
    """Run the comprehensive test and return results"""
    if verbose:
        print("🔬 COMPREHENSIVE SKELETON GENERATION TEST")
        print("Testing all files in C#, C++, and Python test suites")
        print("="*80)
    
    server_process = start_server()
    
    try:
        # Get all test files
        test_files = get_all_test_files()
        
        if verbose:
            print(f"\n📁 Found test files:")
            for lang, files in test_files.items():
                print(f"  {lang}: {len(files)} files")
        
        all_results = {}
        
        # Test each language
        for language, files in test_files.items():
            if verbose:
                print(f"\n{'#'*80}")
                print(f"🔍 TESTING {language} FILES ({len(files)} files)")
                print(f"{'#'*80}")
            
            language_results = []
            
            for file_path in files:
                result = test_file_comprehensive(file_path, language, verbose)
                if result:
                    language_results.append(result)
            
            all_results[language] = language_results
        
        return all_results
        
    finally:
        if verbose:
            print("\n🛑 Stopping server...")
        server_process.terminate()
        server_process.wait()
        if verbose:
            print("✅ Server stopped")

def print_summary(all_results):
    """Print comprehensive summary"""
    print(f"\n{'='*80}")
    print("📊 COMPREHENSIVE SUMMARY")
    print(f"{'='*80}")
    
    for language, results in all_results.items():
        successful = [r for r in results if r['success']]
        tree_sitter_used = [r for r in successful if r.get('quality', {}).get('tree_sitter_used', False)]
        
        print(f"\n🔍 {language} Results:")
        print(f"  Total files: {len(results)}")
        print(f"  Successful: {len(successful)} ({len(successful)/len(results)*100:.1f}%)")
        print(f"  Tree-sitter used: {len(tree_sitter_used)} ({len(tree_sitter_used)/len(results)*100:.1f}%)")
        
        if successful:
            avg_compression = sum(r['quality']['compression_ratio'] for r in successful) / len(successful)
            print(f"  Avg compression: {avg_compression:.1f}%")
            
            # Element preservation analysis
            if successful[0]['quality'].get('preservation'):
                preservation_keys = successful[0]['quality']['preservation'].keys()
                for key in preservation_keys:
                    avg_preservation = sum(r['quality']['preservation'][key] for r in successful) / len(successful)
                    status = "✅" if avg_preservation >= 80 else "⚠️" if avg_preservation >= 50 else "❌"
                    print(f"  {status} Avg {key} preservation: {avg_preservation:.1f}%")

def main():
    """Main function"""
    all_results = run_comprehensive_test(verbose=True)
    print_summary(all_results)
    print(f"\n✅ Comprehensive testing completed!")

if __name__ == "__main__":
    main()