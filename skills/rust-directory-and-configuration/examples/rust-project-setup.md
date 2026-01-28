# Rust Project Setup Guide

## Prerequisites

- Rust toolchain installed (rustc, cargo)
- Project directory ready
- Internet connection

## Step-by-Step Setup

### 1. Create Project Directory

Create a directory for your Rust project:

```bash
# Create project directory
mkdir my-rust-project
cd my-rust-project
```

### 2. Initialize New Project

Initialize a new Rust project using cargo:

```bash
# Initialize new project
cargo init
```

This creates:
- `Cargo.toml` - project manifest
- `src/lib.rs` - library entry point (or `src/main.rs` for executables)

### 3. Configure Cargo.toml

Edit the `Cargo.toml` file to set project metadata and dependencies:

```toml
[package]
name = "my-rust-project"
version = "0.1.0"
edition = "2021"

# Dependencies
[dependencies]
# Add your dependencies here

# Optional: Add dev dependencies
[dev-dependencies]
# Add dev dependencies here
```

### 4. Set Up Project Structure

Organize your project with the recommended structure:

```
my-rust-project/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   └── main.rs (optional)
├── tests/
│   └── integration_tests.rs (optional)
├── examples/
│   └── example1.rs (optional)
└── .gitignore
```

### 5. Add Git Repository (Optional)

If you plan to version control your project:

```bash
# Initialize git repository
git init

# Add all files
git add .

# Commit initial version
git commit -m "Initial commit"
```

## Configuration Options

### 1. Profile Configuration

You can configure different profiles in Cargo.toml:

```toml
[profile.dev]
opt-level = 0
debug = true

[profile.release]
opt-level = 3
debug = false
```

### 2. Features

Define features for conditional compilation:

```toml
[features]
# Enable specific features
default = ["std"]

std = []

# Feature-specific dependencies
debug = ["debug-features"]

# Feature-specific code
#[cfg(feature = "debug")]
#debug_code = true
```

## Verification

After setup, verify all components:

```bash
# Check formatting
cargo fmt --check

# Run clippy lints
cargo clippy

# Compile and test
cargo build
cargo test
```

## Notes

- The `lib.rs` file is the entry point for libraries
- The `main.rs` file is the entry point for executables
- Dependencies are listed in the `[dependencies]` section
- Dev dependencies are listed in the `[dev-dependencies]` section
- Cargo automatically manages dependencies and builds

## Resources

- Cargo Documentation: https://doc.rust-lang.org/cargo/
- Rust Official Documentation: https://doc.rust-lang.org/
- The Rust Book: https://doc.rust-lang.org/book/"