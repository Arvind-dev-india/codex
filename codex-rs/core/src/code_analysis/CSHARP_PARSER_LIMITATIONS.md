# C# Parser Limitations and Workarounds

This document outlines the limitations of the Tree-sitter C# parser used in the code analysis tools and the workarounds implemented to ensure robust functionality.

## Supported C# Features

The Tree-sitter C# parser successfully handles these C# language features:

- ✅ Basic classes and interfaces
- ✅ Methods, properties, fields, and constructors
- ✅ Static classes and methods
- ✅ Namespaces
- ✅ Enums and enum members
- ✅ Delegates and events
- ✅ Generic types and constraints
- ✅ Basic inheritance and interface implementation
- ✅ Records (C# 9) - detected as classes, which is appropriate

## Limitations and Unsupported Features

The following C# features have limited or no support in the current Tree-sitter C# parser:

1. **File-scoped namespaces** (C# 10)
   - Example: `namespace TestApp;` (without braces)
   - Status: Not supported in current parser
   - Workaround: Use traditional namespace declarations with braces

2. **Implicit object creation** (C# 9)
   - Example: `var person = new() { Name = "John" };`
   - Status: Limited support
   - Workaround: Use explicit type names in object creation

3. **Using declarations** (C# 8)
   - Example: `using var file = File.OpenRead("file.txt");`
   - Status: Not supported in current parser
   - Workaround: Use traditional using statements with braces

4. **Top-level statements** (C# 9)
   - Example: Code directly in a file without a class or Main method
   - Status: Limited parsing support
   - Workaround: Use traditional program structure with explicit Main method

5. **Some newer pattern matching features** (C# 9+)
   - Status: Partial support
   - Workaround: Use simpler pattern matching constructs when possible

## Implementation Notes

1. The query file `csharp.scm` has been updated to comment out unsupported node types to prevent parsing errors.

2. Tests have been modified to be aware of these limitations and provide appropriate diagnostics.

3. The code analysis tools will still work with modern C# code, but may not detect all symbols in files using newer language features.

## Future Improvements

1. Monitor Tree-sitter C# grammar updates for improved support of modern C# features

2. Consider implementing fallback parsing strategies for unsupported constructs

3. Add more comprehensive tests for edge cases in C# parsing