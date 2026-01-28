# Rust Installation Guide

## Prerequisites

- Linux, macOS, or Windows (WSL recommended)
- Internet connection
- Terminal access

## Step-by-Step Installation

### 1. Download and Install rustup

The recommended approach is to use rustup, the official Rust toolchain installer:

```bash
# Download and run the installer script
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Add to PATH

Add rustup to your shell configuration file (add to ~/.bashrc or ~/.zshrc):

```bash
# Add to ~/.bashrc or ~/.zshrc
export PATH="${HOME}/.cargo/bin:${PATH}"
```

### 3. Reload Shell

Reload your shell configuration:

```bash
source ~/.bashrc  # or ~/.zshrc
```

### 4. Verify Installation

Check that rustup is installed and the Rust toolchain is available:

```bash
# Check rustc version
rustc --version

# Check cargo version
cargo --version
```

## Troubleshooting

### Issue: PATH not updated

If the rustc command is not found:

1. Verify the PATH export is in your shell configuration file
2. Reload your shell configuration: `source ~/.bashrc` or `source ~/.zshrc`
3. Verify the path exists: `ls ~/.cargo/bin`

### Issue: Installation fails

If the installation script fails:

1. Ensure you have internet access
2. Check your system permissions
3. Try running with sudo if needed (on some systems)
4. Verify your terminal is not in a restricted environment

## Notes

- rustup manages multiple Rust versions and toolchains
- The default version is stable, which is recommended for most use cases
- You can install other versions (beta, nightly) using rustup
- Rustup automatically handles updates to the toolchain

## Resources

- Rust Official Installation Guide: https://www.rust-lang.org/tools/install
- rustup Documentation: https://rustup.rs/"