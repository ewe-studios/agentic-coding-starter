---
name: "Rust Directory and Configuration"
description: "Install Rust toolchain, configure projects, and set up proper directory structure"
approved: No
created: 2026-01-27
license: "MIT"
metadata:
  author: "Main Agent"
  version: "2.0-restructured"
  last_updated: "2026-01-28"
tags:
  - rust
  - installation
  - configuration
  - toolchain
  - project-setup
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

[dependencies]
# Add dependencies here

[dev-dependencies]
# Test-only dependencies

[features]
default = []
# Optional feature flags
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

### .rustfmt.toml (Optional)

```toml
# .rustfmt.toml - Code formatting
edition = "2021"
max_width = 100
hard_tabs = false
tab_spaces = 4
newline_style = "Unix"
use_small_heuristics = "Default"
```

### .clippy.toml (Optional)

```toml
# .clippy.toml - Linter configuration
# See: https://rust-lang.github.io/rust-clippy/master/
```

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

## Related Skills

- [Rust Clean Implementation](../rust-clean-implementation/skill.md) - For writing code
- [Rust Testing Excellence](../rust-testing-excellence/skill.md) - For testing

---

*Last Updated: 2026-01-28*
*Version: 2.0-restructured*
