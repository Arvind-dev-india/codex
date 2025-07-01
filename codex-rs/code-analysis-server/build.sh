#!/bin/bash
# Build script for the code analysis server

set -e

# Determine the platform
PLATFORM=$(uname -s)
case $PLATFORM in
    Linux*)     PLATFORM_NAME=linux;;
    Darwin*)    PLATFORM_NAME=macos;;
    MINGW*|MSYS*|CYGWIN*) PLATFORM_NAME=windows;;
    *)          PLATFORM_NAME="unknown";;
esac

# Determine the architecture
ARCH=$(uname -m)
case $ARCH in
    x86_64|amd64) ARCH_NAME=x64;;
    arm64|aarch64) ARCH_NAME=arm64;;
    *)          ARCH_NAME="unknown";;
esac

echo "Building code-analysis-server for $PLATFORM_NAME-$ARCH_NAME..."

# Build the binary
cd "$(dirname "$0")/.."
cargo build --release -p code-analysis-server

# Create output directory
mkdir -p dist
OUTPUT_DIR="dist/code-analysis-server-$PLATFORM_NAME-$ARCH_NAME"
mkdir -p "$OUTPUT_DIR"

# Copy the binary and README
if [ "$PLATFORM_NAME" = "windows" ]; then
    cp target/release/code-analysis-server.exe "$OUTPUT_DIR/"
else
    cp target/release/code-analysis-server "$OUTPUT_DIR/"
fi
cp code-analysis-server/README.md "$OUTPUT_DIR/"

echo "Build complete! Binary available at: $OUTPUT_DIR"