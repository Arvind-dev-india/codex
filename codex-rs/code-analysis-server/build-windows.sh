#!/bin/bash
# Cross-compile the code analysis server for Windows from Linux
set -e

echo "Cross-compiling code-analysis-server for Windows from Linux..."

# Check if the Windows target is installed
if ! rustup target list | grep -q "x86_64-pc-windows-gnu (installed)"; then
    echo "Installing Windows target..."
    rustup target add x86_64-pc-windows-gnu
fi

# Check if MinGW is installed
if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "Error: MinGW-w64 cross compiler not found."
    echo "Please install it with your package manager:"
    echo "  Debian/Ubuntu: sudo apt install mingw-w64"
    echo "  Fedora: sudo dnf install mingw64-gcc"
    echo "  Arch Linux: sudo pacman -S mingw-w64-gcc"
    exit 1
fi

# Build the binary for Windows
cd "$(dirname "$0")/.."
echo "Building Windows binary..."
cargo build --release --target x86_64-pc-windows-gnu -p code-analysis-server

# Create output directory
mkdir -p dist
OUTPUT_DIR="dist/code-analysis-server-windows-x64"
mkdir -p "$OUTPUT_DIR"

# Copy the binary and README
cp target/x86_64-pc-windows-gnu/release/code-analysis-server.exe "$OUTPUT_DIR/"
cp code-analysis-server/README.md "$OUTPUT_DIR/"

echo "Build complete! Windows binary available at: $OUTPUT_DIR/code-analysis-server.exe"