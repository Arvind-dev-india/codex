# Building for Windows

This guide shows different ways to build the Code Analysis Server for Windows.

## Option 1: Building on Windows

### Prerequisites for Windows
- Install Rust from [rustup.rs](https://rustup.rs/)
- Install Git from [git-scm.com](https://git-scm.com/)
- Use Command Prompt or PowerShell

### Build Steps

1. **Clone and navigate**:
   ```cmd
   git clone https://github.com/your-repo/codex.git
   cd codex\codex-rs
   ```

2. **Build the server**:
   ```cmd
   cargo build --release -p code-analysis-server
   ```

3. **Find the binary**:
   ```cmd
   dir target\release\code-analysis-server.exe
   ```

### Test the Build

```cmd
target\release\code-analysis-server.exe --help
```

## Option 2: Cross-compile from Linux

This allows you to build Windows binaries without a Windows machine.

### Install Cross-compilation Tools

1. **Add Windows target**:
   ```bash
   rustup target add x86_64-pc-windows-gnu
   ```

2. **Install MinGW-w64**:
   ```bash
   # Ubuntu/Debian
   sudo apt update && sudo apt install mingw-w64
   
   # Fedora
   sudo dnf install mingw64-gcc
   
   # Arch Linux
   sudo pacman -S mingw-w64-gcc
   ```

### Build for Windows

1. **Use the cross-compilation script**:
   ```bash
   ./codex-rs/code-analysis-server/build-windows.sh
   ```

2. **Find the binary**:
   ```bash
   ls codex-rs/dist/code-analysis-server-windows-x64/code-analysis-server.exe
   ```

The resulting `.exe` file can be copied to any Windows machine and run without additional dependencies.

**Next:** [Basic Usage](04-basic-usage.md)