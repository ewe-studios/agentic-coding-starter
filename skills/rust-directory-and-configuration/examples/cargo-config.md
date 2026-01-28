# Cargo Configuration Examples

## Introduction

This document provides examples of common Cargo.toml configurations for different project types and requirements.

## Basic Project Configuration

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

## Library Project Configuration

```toml
[package]
name = "my-library"
version = "0.1.0"
edition = "2021"

# Dependencies
[dependencies]
serde = { version = "1.0", features = ["derive"] }

# Optional: Add dev dependencies
[dev-dependencies]
serde_test = "1.0"
```

## Executable Project Configuration

```toml
[package]
name = "my-executable"
version = "0.1.0"
edition = "2021"

# Dependencies
[dependencies]
clap = { version = "4.0", features = ["derive"] }

# Optional: Add dev dependencies
[dev-dependencies]
test_case = "0.5"
```

## Feature-Gated Configuration

```toml
[package]
name = "feature-gated-project"
version = "0.1.0"
edition = "2021"

# Features
[features]
default = ["std"]

std = []

# Feature-specific dependencies
debug = ["debug-features"]

# Feature-specific code
#[cfg(feature = "debug")]
#debug_code = true

# Feature-specific dependencies
debug-features = [
    "serde = { version = "1.0", features = ["derive"] }",
    "log = { version = "0.4", features = ["std"] }"
]
```

## Profile Configuration

```toml
[profile.dev]
opt-level = 0
debug = true

[profile.release]
opt-level = 3
debug = false

# Custom profile
[profile.custom]
opt-level = 2
debug = false
```

## Conditional Dependencies

```toml
[dependencies]
# Conditional dependency - only available when feature "std" is enabled
std_dep = { version = "1.0", optional = true, features = ["std"] }

# Conditional dependency - only available when feature "debug" is enabled
debug_dep = { version = "1.0", optional = true, features = ["debug"] }
```

## Testing Configuration

```toml
# Test configuration
[dev-dependencies]
# Add test dependencies here

# Test harness configuration
[lib]
# Add library-specific test configuration here
```

## Notes

- The `edition` field specifies the Rust edition (2021 is recommended)
- Dependencies are listed in the `[dependencies]` section
- Dev dependencies are listed in the `[dev-dependencies]` section
- Features allow conditional compilation of code
- Profiles control build settings for different environments
- Optional dependencies can be conditionally included

## Resources

- Cargo Documentation: https://doc.rust-lang.org/cargo/
- Rust Official Documentation: https://doc.rust-lang.org/
- The Rust Book: https://doc.rust-lang.org/book/"