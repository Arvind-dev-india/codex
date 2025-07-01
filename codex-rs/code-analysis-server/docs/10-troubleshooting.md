# Troubleshooting

Common issues and solutions when using the Code Analysis Server.

## Build Issues

### Rust Compilation Errors

**Issue**: Errors during `cargo build`

**Solutions**:
1. Update Rust: `rustup update`
2. Clean build artifacts: `cargo clean`
3. Check for missing dependencies:
   - Linux: `sudo apt install build-essential pkg-config`
   - Windows: Install Visual Studio Build Tools

### Cross-compilation Errors

**Issue**: Errors when cross-compiling for Windows

**Solutions**:
1. Verify MinGW is installed: `x86_64-w64-mingw32-gcc --version`
2. Check `.cargo/config.toml` exists with correct linker settings
3. Try using the `cross` tool: `cargo install cross && cross build --target x86_64-pc-windows-gnu`

## Runtime Issues

### Command Not Found

**Issue**: `code-analysis-server: command not found`

**Solutions**:
1. Use full path: `/path/to/code-analysis-server`
2. Make executable: `chmod +x code-analysis-server`
3. Add to PATH: `export PATH=$PATH:/path/to/directory`

### Windows Path Issues

**Issue**: Server can't find files on Windows

**Solutions**:
1. Use forward slashes: `C:/Users/name/project`
2. Escape backslashes: `C:\\Users\\name\\project`
3. Use absolute paths

### Server Crashes

**Issue**: Server crashes during analysis

**Solutions**:
1. Run with `--verbose` flag to see detailed errors
2. Check file permissions
3. Try analyzing a smaller project first
4. Increase memory limits if analyzing large codebases

## Communication Issues

### No Response from Server

**Issue**: Client sends request but gets no response

**Solutions**:
1. Check server is running
2. Ensure request has correct JSON format
3. Add newline (`\n`) at end of each request
4. Verify unique request IDs

### Invalid JSON

**Issue**: "Failed to parse JSON" errors

**Solutions**:
1. Validate JSON with a linter
2. Check for special characters that need escaping
3. Use a JSON library instead of manual string construction

### Tool Not Found

**Issue**: "Unknown tool" error

**Solutions**:
1. Check tool name spelling (case-sensitive)
2. Run `tools/list` to see available tools
3. Ensure server is initialized before calling tools

## NPX MCP Tool Issues

### Connection Errors

**Issue**: NPX tool can't connect to server

**Solutions**:
1. Check server command path is correct
2. Ensure server is executable
3. Try running server manually first to check for errors

### Command Timeout

**Issue**: Commands time out with large projects

**Solutions**:
1. Increase timeout: `--timeout 60000`
2. Analyze specific directories instead of entire project
3. Use file-specific tools rather than project-wide analysis

## Getting More Help

If you're still having issues:

1. **Check logs**: Run with `--verbose` flag
2. **Examine stderr**: Redirect stderr to a file: `2> error.log`
3. **Check system resources**: Memory usage, disk space
4. **File an issue**: Include:
   - OS and version
   - Rust version (`rustc --version`)
   - Command used
   - Error message
   - Project size (number of files)

**Next:** [Integration Examples](11-integration-examples.md)