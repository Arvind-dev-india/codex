# Testing Skeleton Generation with Large Projects

When working with large C# projects, the code analysis server may take significant time to initialize the code graph. Here are strategies to handle this effectively.

## Quick Testing Strategy

### 1. Quick Test Tool (Recommended for Large Projects)

Use the quick test tool to verify functionality with a single file first:

```bash
cd codex-rs/scripts

# Quick test with auto-selected small file
python3 quick_skeleton_test.py /mnt/c/One/Mgmt-RecoverySvcs-WkloadExtn/

# Quick test with specific file
python3 quick_skeleton_test.py /mnt/c/One/Mgmt-RecoverySvcs-WkloadExtn/ --file "SomeFile.cs"
```

**Benefits:**
- ✅ Tests functionality quickly
- ✅ Uses longer timeouts (3 minutes)
- ✅ Auto-selects smallest file for testing
- ✅ Verifies skeleton generation works before full setup

### 2. Enhanced Interactive Tool

The main interactive tool now has better timeout handling:

```bash
# Enhanced with progressive timeouts: 20s → 1min → 2min
python3 interactive_skeleton_test.py --project-dir /mnt/c/One/Mgmt-RecoverySvcs-WkloadExtn/
```

**Improvements:**
- ✅ Progressive timeout increases (20s, 60s, 120s)
- ✅ Better progress feedback
- ✅ Automatic retries with longer timeouts
- ✅ File count verification before initialization

## Troubleshooting Large Projects

### Common Issues & Solutions

#### 1. **Initialization Timeout**
```
❌ MCP initialization error: Read timed out
```

**Solutions:**
- Use the quick test tool first: `python3 quick_skeleton_test.py <dir>`
- Try a subdirectory: `--project-dir /path/to/smaller/subdir`
- Test with specific files: `--file "SpecificFile.cs"`

#### 2. **Server Startup Timeout**
```
❌ Server failed to start within 30 seconds
```

**Solutions:**
- Large projects need more time for code graph initialization
- The server is likely still working - be patient
- Check server logs for progress
- Try with a smaller subset first

#### 3. **Memory Issues**
For very large projects (>10k files), the server might need more memory.

**Solutions:**
- Test subdirectories instead of entire project
- Use specific file testing
- Consider project structure optimization

### Recommended Workflow for Large Projects

#### Step 1: Quick Verification
```bash
# Test that skeleton generation works
python3 quick_skeleton_test.py /large/project/path/
```

#### Step 2: Subdirectory Testing
```bash
# Test specific modules/directories
python3 interactive_skeleton_test.py --project-dir /large/project/path/src/core/
python3 interactive_skeleton_test.py --project-dir /large/project/path/src/models/
```

#### Step 3: Full Project (if needed)
```bash
# Only after verifying smaller parts work
python3 interactive_skeleton_test.py --project-dir /large/project/path/
# Be patient - may take 2-5 minutes for initialization
```

## Performance Tips

### 1. **Directory Structure Optimization**
- Test specific modules rather than entire solutions
- Focus on source directories, skip test/build directories
- Use subdirectories for faster initialization

### 2. **File Selection Strategy**
- Start with smaller, simpler files
- Test core business logic files
- Verify complex files after basic functionality works

### 3. **Incremental Testing**
```bash
# Test progression from small to large
python3 quick_skeleton_test.py /project/src/models/
python3 quick_skeleton_test.py /project/src/services/
python3 quick_skeleton_test.py /project/src/controllers/
python3 interactive_skeleton_test.py --project-dir /project/src/
```

## Example: Testing Your Project

Based on your path `/mnt/c/One/Mgmt-RecoverySvcs-WkloadExtn/`, here's the recommended approach:

### Step 1: Quick Test
```bash
cd codex-rs/scripts
python3 quick_skeleton_test.py /mnt/c/One/Mgmt-RecoverySvcs-WkloadExtn/
```

### Step 2: Find Subdirectories
```bash
# Look for source directories
ls -la /mnt/c/One/Mgmt-RecoverySvcs-WkloadExtn/
# Common patterns: src/, Source/, Models/, Services/, etc.
```

### Step 3: Test Subdirectories
```bash
# Test specific components
python3 interactive_skeleton_test.py --project-dir /mnt/c/One/Mgmt-RecoverySvcs-WkloadExtn/src/
python3 interactive_skeleton_test.py --project-dir /mnt/c/One/Mgmt-RecoverySvcs-WkloadExtn/Models/
```

### Step 4: Interactive Testing
```bash
# Once you find a working subdirectory
python3 interactive_skeleton_test.py --project-dir /mnt/c/One/Mgmt-RecoverySvcs-WkloadExtn/src/

[skeleton-test] search
🔍 Search: recovery
# Find and test recovery-related files

🔍 Search: service
# Find and test service files
```

## Expected Timeouts

| Project Size | Expected Init Time | Recommended Tool |
|--------------|-------------------|------------------|
| < 100 files  | 5-15 seconds     | Interactive tool |
| 100-1000 files | 15-60 seconds  | Interactive tool |
| 1000-5000 files | 1-3 minutes    | Quick test first |
| > 5000 files | 3+ minutes       | Subdirectory testing |

## Success Indicators

### Quick Test Success
```
✅ SUCCESS! Generated skeleton (425 chars)

=== SKELETON PREVIEW ===
 1: using System;
 2: using System.Collections.Generic;
 3: 
 4: // Lines 4-46
 5: // symbol: TestNamespace {
 6:     // Lines 9-45
 7:     public class BasicClass {
 8:         // ...
 9:     }
```

### Interactive Tool Success
```
🎮 Interactive Skeleton Testing Mode
Commands:
  search             - Fuzzy search and select file
  
[skeleton-test] search
🔍 Search: manager
📋 Search results for 'manager':
   1. Services/RecoveryManager.cs (2341 bytes)
   2. Models/BackupManager.cs (1876 bytes)
```

This approach ensures you can test skeleton generation effectively even with very large C# projects!