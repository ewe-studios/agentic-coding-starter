---
name: "Rust Directory and Configuration"
description: "Install Rust toolchain, configure projects, and set up proper directory structure"
approved: Yes
created: 2026-01-27
license: "MIT"
metadata:
  author: "Main Agent"
  version: "2.2-approved"
  last_updated: "2026-01-28"
tags:
  - rust
  - installation
  - configuration
  - toolchain
  - project-setup
files:
  - examples/rust-installation.md: "Complete Rust toolchain installation guide"
  - examples/rust-project-setup.md: "Step-by-step project setup and structure"
  - examples/cargo-config.md: "Cargo.toml and .cargo/config.toml examples"
---

# Rust Directory and Configuration

## When to Use This Skill

Read this when:
- Setting up a new Rust project
- Installing Rust on a new machine
- Configuring project structure
- Setting up development environment

---

## Installation

### 1. Install Rust Toolchain

```bash
# Install rustup (official Rust installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add to PATH (add to ~/.bashrc or ~/.zshrc)
export PATH="${HOME}/.cargo/bin:${PATH}"

# Verify installation
rustc --version
cargo --version
```

### 2. Configure Rust Version

```bash
# Use latest stable (recommended)
rustup default stable

# Or specific version
rustup default 1.75.0

# Verify
rustc --version
```

### 3. Install Additional Development Tools

```bash
# Essential components
rustup component add rustfmt clippy rust-analyzer

# Additional cargo tools for development
cargo install cargo-audit       # Security vulnerability scanner
cargo install cargo-deny        # Dependency license and advisory checker
cargo install cargo-expand      # Macro expansion tool (debugging)
cargo install cargo-nextest     # Fast test runner (recommended)
cargo install cargo-flamegraph  # Profiling tool

# Keep toolchain updated
rustup update
```

---

## Project Setup

### Initialize New Project

```bash
# Create project directory
mkdir my-rust-project
cd my-rust-project

# Initialize (creates Cargo.toml and src/)
cargo init

# Or create new project in one step
cargo new my-rust-project
cd my-rust-project
```

### Configure Cargo.toml

```toml
[package]
name = "my-rust-project"
version = "0.1.0"
edition = "2021"  # Use latest edition
rust-version = "1.75"  # MSRV (Minimum Supported Rust Version)

[dependencies]
# Add dependencies here

[dev-dependencies]
# Test-only dependencies

[features]
default = []
# Optional feature flags

# CRITICAL: Optimize for release builds
[profile.release]
opt-level = 3           # Maximum optimization
lto = "fat"            # Full link-time optimization (slower compile, faster runtime)
codegen-units = 1      # Better optimization, slower compile
strip = true           # Strip symbols from binary
panic = "abort"        # Smaller binary, no unwinding

# IMPORTANT: Fast compilation for debug builds
[profile.dev]
opt-level = 0          # No optimization for fast compile
debug = true           # Full debug info
split-debuginfo = "unpacked"  # Faster on macOS/Linux

# Optimize dependencies even in debug mode (faster debug experience)
[profile.dev.package."*"]
opt-level = 2

# For testing with optimizations
[profile.test]
opt-level = 1

# For benchmarking
[profile.bench]
opt-level = 3
lto = "fat"
codegen-units = 1
```

---

## Recommended Project Structure

```
my-rust-project/
├── Cargo.toml              # Package manifest
├── Cargo.lock              # Dependency lock file (commit this!)
├── src/
│   ├── lib.rs              # Library crate root
│   ├── main.rs             # Binary crate root (if applicable)
│   ├── models/             # Domain data structures
│   │   ├── mod.rs
│   │   └── user.rs
│   ├── services/           # Business logic
│   │   ├── mod.rs
│   │   └── auth_service.rs
│   └── utils/              # Shared utilities
│       ├── mod.rs
│       └── crypto.rs
├── tests/                  # Integration tests (project root!)
│   ├── crate_name/
│   │   └── api_tests.rs
│   └── common/
│       └── mod.rs
├── benches/                # Benchmarks (project root!)
│   └── crate_name/
│       └── benchmarks.rs
├── examples/               # Example programs
│   └── basic_usage.rs
└── README.md
```

---

## Module Organization

### lib.rs - Public API

```rust
// src/lib.rs
#![warn(missing_docs)]
#![warn(clippy::all)]
#![forbid(unsafe_code)]  // Unless you need unsafe

//! Top-level crate documentation.
//!
//! Provides functionality for X, Y, and Z.

// Re-export commonly used items
pub use models::{User, Post};
pub use services::UserService;
pub use error::{Error, Result};

// Public modules
pub mod models;
pub mod services;

// Private modules (not exposed)
mod utils;
mod error;

// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{Error, Result};
    pub use crate::models::{User, Post};
    pub use crate::services::UserService;
}
```

### Module Privacy

```rust
// src/models/user.rs
use crate::error::{Error, Result};

/// User account representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    id: UserId,             // Private - use accessor
    pub name: String,       // Public field
    email: Email,           // Private - use accessor
    credentials: Credentials, // Private - never expose
}

impl User {
    /// Creates a new user.
    pub fn new(name: String, email: String) -> Result<Self> {
        Ok(Self {
            id: UserId::new(),
            name,
            email: Email::parse(email)?,
            credentials: Credentials::default(),
        })
    }

    /// Returns the user's email address.
    pub fn email(&self) -> &str {
        self.email.as_str()
    }

    /// Returns the user's ID.
    pub fn id(&self) -> UserId {
        self.id
    }
}

// Private helper types
#[derive(Debug, Clone)]
struct Credentials {
    password_hash: String,
}
```

---

## Configuration Files

### rust-toolchain.toml (Recommended)

Pins project to specific Rust version and components:

```toml
# rust-toolchain.toml
[toolchain]
channel = "stable"
profile = "default"
components = ["rustfmt", "clippy", "rust-analyzer"]
```

### .rustfmt.toml (Optional)

Professional-grade formatting configuration:

```toml
# .rustfmt.toml - Code formatting
edition = "2021"
max_width = 100
hard_tabs = false
tab_spaces = 4
newline_style = "Unix"
use_small_heuristics = "Default"

# Import organization
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
reorder_imports = true
reorder_modules = true

# Code style
match_block_trailing_comma = true
trailing_comma = "Vertical"
use_field_init_shorthand = true
use_try_shorthand = true

# Documentation
format_code_in_doc_comments = true
normalize_comments = true
```

### .clippy.toml (Recommended)

Enforce error handling and code quality at configuration level:

```toml
# .clippy.toml - Linter configuration
msrv = "1.75"  # Minimum Supported Rust Version
warn-on-all-wildcard-imports = true

# Disallow unsafe error handling patterns
disallowed-methods = [
    { path = "std::option::Option::unwrap", reason = "use ? operator or proper error handling" },
    { path = "std::option::Option::expect", reason = "use ? operator or proper error handling" },
    { path = "std::result::Result::unwrap", reason = "use ? operator or proper error handling" },
    { path = "std::result::Result::expect", reason = "use ? operator or proper error handling" },
    { path = "std::result::Result::unwrap_err", reason = "use proper error handling" },
    { path = "std::panic::panic", reason = "use Result for recoverable errors" },
    { path = "std::unimplemented", reason = "implement the functionality or use todo!()" },
]

cognitive-complexity-threshold = 30
```

### .cargo/config.toml (Optional)

Build performance optimization with faster linkers:

```toml
# .cargo/config.toml
[build]
# Use mold/lld for faster linking (requires clang/lld installed)

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# For macOS, use zld or lld
# [target.x86_64-apple-darwin]
# rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]
```

---

## Linker Configuration: mold vs lld vs ld

When configuring `.cargo/config.toml` on Linux, you have three main linker options. Each has different trade-offs:

### Comparison Table

| Linker | Speed | Compatibility | Notes |
|--------|-------|---------------|-------|
| `mold` | Fastest (10-20% faster than lld) | ⚠️ Some edge cases | Aggressive deduplication, can fail with certain C libraries |
| `lld` | Very fast | ✅ Excellent | LLVM's linker, great balance of speed and compatibility |
| `ld` (GNU) | Slow | ✅ Excellent | Default, reliable but significantly slower |

### Recommended Configuration

**Use `lld` as the default** - best balance of speed and compatibility:

```toml
[target.'cfg(target_os = "linux")']
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

### Known Issues

**`mold` with `aws-lc-sys` (and similar crates):**

The `mold` linker can fail with undefined symbol errors when linking crates that bundle large C libraries with versioned symbols (e.g., `aws_lc_0_38_0_*`). This is due to mold's aggressive symbol deduplication and stricter ELF handling.

**Symptom:**
```
mold: error: undefined symbol: aws_lc_0_38_0_EVP_sha256
mold: error: undefined symbol: aws_lc_0_38_0_AES_cbc_encrypt
...
```

**Fixes (in order of preference):**

1. **Switch to lld** (recommended):
   ```toml
   rustflags = ["-C", "link-arg=-fuse-ld=lld"]
   ```

2. **Try mold with additional flags** (may not always work):
   ```toml
   rustflags = ["-C", "link-arg=-fuse-ld=mold", "-C", "link-arg=--no-undefined-version"]
   ```

3. **Clean and rebuild** the problematic crate:
   ```bash
   cargo clean -p aws-lc-sys
   cargo build
   ```

### Platform-Specific Recommendations

**Linux:**
```toml
[target.'cfg(target_os = "linux")']
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

**macOS:**
```toml
[target.'cfg(target_os = "macos")']
# macOS default linker is usually fine, or use zld for speed
# rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]
```

**Windows (MSVC):**
```toml
[target.'cfg(target_os = "windows")']
# MSVC linker is the default and works well
# No configuration needed
```

### Installation

**For lld:**
```bash
# Arch Linux
sudo pacman -S lld

# Debian/Ubuntu
sudo apt install lld

# macOS (Homebrew)
brew install llvm  # Includes lld
```

**For mold:**
```bash
# Arch Linux
sudo pacman -S mold

# Debian/Ubuntu
sudo apt install mold

# Or build from source
git clone https://github.com/rui314/mold.git
cd mold && make && sudo make install
```

### Verification

Check which linker is being used:
```bash
# Verbose build to see linker invocation
cargo build -vv 2>&1 | grep "fuse-ld"
```

---

### .gitignore

```gitignore
/target/
Cargo.lock  # Include for binaries, exclude for libraries
**/*.rs.bk
.DS_Store
```

---

## Verification

After setup, verify everything works:

```bash
# Check formatting
cargo fmt --check

# Run lints
cargo clippy

# Build project
cargo build

# Run tests
cargo test

# Build documentation
cargo doc --open
```

---

## Common Commands

```bash
# Development
cargo build              # Debug build
cargo build --release    # Optimized build
cargo run                # Build and run binary
cargo test               # Run all tests
cargo bench              # Run benchmarks
cargo doc                # Generate documentation

# Dependencies
cargo add <crate>        # Add dependency (requires cargo-edit)
cargo update             # Update dependencies
cargo tree               # Show dependency tree

# Maintenance
cargo fmt                # Format code
cargo clippy             # Run linter
cargo clean              # Remove build artifacts
cargo audit              # Check for security vulnerabilities
```

---

## Examples

See `examples/` directory for detailed guides:

- `rust-installation.md` - Complete Rust toolchain installation guide
- `rust-project-setup.md` - Step-by-step project setup and structure
- `cargo-config.md` - Cargo.toml and .cargo/config.toml examples

## Related Skills

- [Rust Clean Implementation](../implementation/skill.md) - For writing code
- [Rust Testing Excellence](../testing/skill.md) - For testing

---

*Last Updated: 2026-01-28*
*Version: 2.2-approved*
