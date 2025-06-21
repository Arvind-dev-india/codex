# Building Codex CLI with Rust Client

This guide explains how to build and run the Rust implementation of Codex CLI.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain, 1.70.0 or newer)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (comes with Rust)
- Git

## Building from Source

1. Clone the repository (if you haven't already):
   ```bash
   git clone https://github.com/openai/codex.git
   cd codex
   ```

2. Build the Rust client:
   ```bash
   cd codex-rs
   cargo build --release
   ```

   This will create a binary at `./target/release/codex`.

3. Run the built binary:
   ```bash
   ./target/release/codex "Your prompt here"
   ```

## Creating an Alias

For easier access, you can create an alias to your built binary:

```bash
alias codex-rs="$(pwd)/target/release/codex"
```

Add this to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.) to make it permanent.

## Development Build

For development with debug symbols:

```bash
cargo build
./target/debug/codex "Your prompt here"
```

## Running Tests

```bash
cargo test
```

## Troubleshooting

### Missing Dependencies

If you encounter missing dependencies on Linux, install the required packages:

#### Ubuntu/Debian:
```bash
sudo apt-get install build-essential pkg-config libssl-dev
```

#### Fedora:
```bash
sudo dnf install gcc pkg-config openssl-devel
```

### Permission Issues

If you encounter permission issues when running the binary:

```bash
chmod +x ./target/release/codex
```

### Build Errors

If you encounter build errors, try updating your Rust toolchain:

```bash
rustup update
```