# Building for Linux

This guide shows how to build the Code Analysis Server on a Linux system.

## Method 1: Using the Build Script (Recommended)

1. **Navigate to the project directory**:
   ```bash
   cd codex
   ```

2. **Run the build script**:
   ```bash
   ./codex-rs/code-analysis-server/build.sh
   ```

3. **Find the binary**:
   ```bash
   ls codex-rs/dist/code-analysis-server-linux-x64/
   ```
   
   You should see:
   - `code-analysis-server` (executable)
   - `README.md`

## Method 2: Using Cargo Directly

1. **Navigate to the Rust workspace**:
   ```bash
   cd codex/codex-rs
   ```

2. **Build the server**:
   ```bash
   cargo build --release -p code-analysis-server
   ```

3. **Find the binary**:
   ```bash
   ls target/release/code-analysis-server
   ```

## Test the Build

Verify the binary works:

```bash
# Using build script output
./codex-rs/dist/code-analysis-server-linux-x64/code-analysis-server --help

# Using cargo output
./codex-rs/target/release/code-analysis-server --help
```

You should see the help message with available options.

**Next:** [Building for Windows](03-build-windows.md)