# Prerequisites

Before building the Code Analysis Server, ensure you have the following installed:

## Required Software

1. **Rust Toolchain**
   - Install via [rustup](https://rustup.rs/)
   - Run: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
   - Restart your terminal or run: `source ~/.cargo/env`

2. **Git**
   - Linux (Ubuntu/Debian): `sudo apt install git`
   - Linux (Fedora): `sudo dnf install git`
   - Windows: Download from [git-scm.com](https://git-scm.com/)

3. **Clone the Repository**
   ```bash
   git clone https://github.com/your-repo/codex.git
   cd codex
   ```

## Verify Installation

Check that everything is installed correctly:

```bash
# Check Rust version
rustc --version

# Check Cargo version
cargo --version

# Check Git version
git --version
```

You should see version numbers for all three commands.

**Next:** [Building for Linux](02-build-linux.md)