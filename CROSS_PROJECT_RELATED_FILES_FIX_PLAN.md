# Cross-Project Related Files Skeleton Fix Plan

## 🔍 **Problem Analysis from MCP Server Logs**

### **✅ What's Working:**
- **Main project initialization**: 1164 files, 10914 symbols, 9073 nodes, 36690 edges ✅
- **Supplementary project parsing**: 867 files, 12265 symbols from 'fabric' project ✅
- **Cross-project edge creation**: 606 cross-project edges created ✅
- **Combined stats**: 21338 total nodes, 37296 total edges ✅

### **❌ What's Missing:**
- **Related files skeleton** not showing supplementary files in results
- **Cross-project boundary detection** not working in BFS traversal
- **Supplementary files** not being included in skeleton generation

## 🎯 **Root Cause Analysis**

### **Issue 1: BFS Traversal Not Using Cross-Project Edges**
The current `get_related_files_skeleton` implementation uses the old BFS logic that doesn't know about the 606 cross-project edges that were created.

### **Issue 2: Missing Integration with Existing Graph**
Our enhanced BFS traversal (`find_related_files_optimized`) is not integrated with the actual MCP server's graph that contains the cross-project edges.

### **Issue 3: CrossProjectDetector Not Using Supplementary Registry**
The existing `CrossProjectDetector` in `repo_mapper.rs` is not using our optimized supplementary registry approach.

## 🔧 **Detailed Fix Plan**

### **Phase 1: Integrate Enhanced BFS with Existing Graph (High Priority)**

#### **1.1 Update get_related_files_skeleton_handler**
**File**: `core/src/code_analysis/tools.rs`
**Current**: Uses old `find_related_files_bfs_with_cross_project_detection`
**Fix**: Replace with our enhanced version that uses supplementary registry

```rust
// Current problematic code (around line 2838):
let (in_project_files, cross_project_files) = find_related_files_bfs_with_cross_project_detection(
    &input.active_files,
    input.max_depth,
    &detector,
)?;

// Replace with:
let (in_project_files, cross_project_files) = find_related_files_with_supplementary_registry(
    &input.active_files,
    input.max_depth,
    &supplementary_registry,
)?;
```

#### **1.2 Create Supplementary Registry from Existing Data**
**File**: `core/src/code_analysis/tools.rs`
**Add**: Function to create supplementary registry from existing cross-project data

```rust
fn get_or_create_supplementary_registry() -> Result<SupplementarySymbolRegistry, String> {
    // Extract supplementary symbols from existing cross-project edges
    // Use the 12265 supplementary symbols already parsed
}
```

### **Phase 2: Fix Cross-Project Edge Detection in BFS (High Priority)**

#### **2.1 Update BFS to Use Actual Graph Edges**
**File**: `core/src/code_analysis/enhanced_bfs_traversal.rs`
**Current**: Simplified version that doesn't use real graph
**Fix**: Integrate with actual repo mapper graph that has 606 cross-project edges

```rust
pub fn find_related_files_with_actual_graph(
    active_files: &[String],
    max_depth: usize,
    supplementary_registry: &SupplementarySymbolRegistry,
) -> Result<(Vec<String>, Vec<String>), String> {
    // Use the actual graph with 36690 + 606 edges
    // Detect cross-project boundaries using supplementary registry
    // Include supplementary files when cross-project edges are traversed
}
```

#### **2.2 Cross-Project Edge Recognition**
**Logic**: When BFS encounters an edge that leads to a supplementary file:
1. **Add supplementary file** to results
2. **Stop traversal** at that boundary (don't traverse into supplementary project)
3. **Mark as cross-project** for skeleton generation

### **Phase 3: Enhance Skeleton Generation (Medium Priority)**

#### **3.1 Mixed Project Skeleton Generation**
**File**: `core/src/code_analysis/tools.rs`
**Add**: Enhanced skeleton generation that handles both project types

```rust
fn generate_mixed_project_skeletons_integrated(
    main_files: &[String],
    supplementary_files: &[String],
    max_tokens: usize,
) -> Result<Vec<Value>, String> {
    // Generate skeletons for main project files (existing logic)
    // Generate skeletons for supplementary files (direct analysis)
    // Annotate with project type information
}
```

### **Phase 4: Integration Points (Medium Priority)**

#### **4.1 Update CrossProjectDetector**
**File**: `core/src/code_analysis/repo_mapper.rs`
**Enhancement**: Make it aware of supplementary registry

#### **4.2 Update Graph Manager Integration**
**File**: `core/src/code_analysis/graph_manager.rs`
**Add**: Support for supplementary registry in graph operations

## 📋 **Implementation Steps**

### **Step 1: Extract Supplementary Registry from Existing Data**
1. **Access existing supplementary symbols** (12265 symbols from 'fabric')
2. **Create SupplementarySymbolRegistry** from this data
3. **Test registry creation** with real MCP server data

### **Step 2: Fix BFS Traversal Integration**
1. **Update find_related_files_optimized** to use actual graph
2. **Integrate with repo_mapper.get_graph()** that has cross-project edges
3. **Test with real cross-project edges** (606 edges from logs)

### **Step 3: Update get_related_files_skeleton_handler**
1. **Replace old BFS logic** with enhanced version
2. **Integrate supplementary registry** creation
3. **Test end-to-end** with MCP server

### **Step 4: Enhanced Skeleton Generation**
1. **Update skeleton generation** to handle mixed projects
2. **Add project type annotations** to skeleton output
3. **Test with real supplementary files**

## 🧪 **Testing Strategy**

### **Test 1: Registry Creation from Real Data**
```bash
# Test with actual MCP server data
./target/release/code-analysis-server --project-dir /main/project --supplementary fabric:/supplementary/project
# Verify supplementary registry is created with 12265 symbols
```

### **Test 2: Cross-Project Edge Detection**
```bash
# Test BFS traversal finds supplementary files
# Should show files from both main and supplementary projects
```

### **Test 3: Related Files Skeleton**
```bash
# Test get_related_files_skeleton includes supplementary files
# Should generate skeletons for both project types
```

## 🎯 **Expected Results After Fix**

### **Before Fix (Current):**
- ❌ Related files skeleton: Only main project files
- ❌ Cross-project edges: Created but not used in BFS
- ❌ Supplementary files: Not included in skeleton results

### **After Fix (Expected):**
- ✅ Related files skeleton: Main + supplementary files
- ✅ Cross-project edges: Used in BFS traversal for boundary detection
- ✅ Supplementary files: Included with proper project type annotation
- ✅ Performance: Efficient (no duplicate processing)

## 🚀 **Success Metrics**

1. **✅ Supplementary files appear** in get_related_files_skeleton results
2. **✅ Cross-project boundaries detected** during BFS traversal
3. **✅ Mixed project skeletons generated** with proper annotations
4. **✅ Performance maintained** (no regression in speed)
5. **✅ All existing tests pass** (no breaking changes)

## 📝 **Implementation Priority**

### **High Priority (Immediate):**
- Step 1: Extract supplementary registry from existing data
- Step 2: Fix BFS traversal integration
- Step 3: Update get_related_files_skeleton_handler

### **Medium Priority (Next):**
- Step 4: Enhanced skeleton generation
- Integration testing with real MCP server

### **Low Priority (Future):**
- Performance optimizations
- Additional cross-project tools enhancement

This plan will fix the issue where supplementary files are not appearing in the related files skeleton, leveraging the 606 cross-project edges that are already being created successfully.