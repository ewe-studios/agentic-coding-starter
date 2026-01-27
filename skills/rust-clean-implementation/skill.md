---
name: "Rust Clean Implementation"
description: "Implement clean, clear Rust code with proper WHY/WHAT/HOW documentation patterns, panic handling that articulates behaviors and exceptions, preferring result-based returns over unwrap or expect."
approved: No
created: 2026-01-27
license: "MIT"
metadata:
  author: "Main Agent"
  version: "1.0"
  last_updated: "2026-01-27"
  tags:
    - rust
    - clean-code
    - documentation
    - error-handling
tools:
  - Rust
  - Cargo
files:
  - examples/writing-clear-docs.rs: "Example of WHY/WHAT/HOW doc comments and panic docs"
assets:
---
# Clean Implementation in Rust

## Overview

This skill guides the implementation of clean, clear Rust code with proper documentation patterns. Every function must articulate its **WHY** (purpose), **WHAT** (behavior/responsibilities), and **HOW** (implementation approach) through comprehensive doc comments.

The goal is to produce self-documenting code where behavior expectations are explicit before reading any logic details.

## When to Use This Skill

- Implementing new Rust modules or functions
- Refactoring existing code with unclear documentation
- Writing public APIs that need clear contracts
- Creating libraries intended for reuse by others
- Any situation requiring precise specification of error conditions and panic behaviors

## Prerequisites

- Basic understanding of Rust ownership, borrowing, and lifetimes
- Familiarity with the `rust.md` stack file conventions (mandatory)
- Understanding of Result<T, E> vs Option<T> patterns in idiomatic Rust

## Core Concepts

### WHY/WHAT/HOW Documentation Pattern

Every public function must contain three clear sections:

1. **WHY** - Why does this exist? What problem does it solve?
2. **WHAT** - What behavior is expected? Inputs, outputs, side effects
3. **HOW** - How does it work internally? Key algorithmic decisions or design choices

### Panic Documentation Standards

Panic scenarios must be documented with explicit preconditions:

```rust
/// # Panics
///
/// * When `index >= self.len()` when accessing by index
/// * When the underlying resource has been invalidated (see [Self::validate] for details)
```

This articulates:
- The exact precondition that will cause panic
- Any additional conditions related to external state

### Result-Based Error Handling

**MANDATORY**: Never use `unwrap()` or `expect()` in production code. All error paths must be handled explicitly.

For libraries: Use **thiserror** for deriving custom error types with proper messages.
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Invalid input length: expected {expected}, got {actual}")]
    InvalidLength { expected: usize, actual: usize },
}
```

For applications: Use **anyhow** for context-rich error propagation.
```rust
use anyhow::{Context, Result};

fn process() -> Result<()> {
    let data = fs::read_to_string("config.toml")
        .context("Failed to read configuration file")?;
    // ...
    Ok(())
}
```

## Step-by-Step Guide

### Step 1: Write the WHY/WHAT/HOW Documentation First

Before writing any implementation, draft doc comments:

```rust
/// # Purpose (WHY)
///
/// This function validates user input before database insertion. It ensures data integrity by
/// checking format constraints and business rules.
impl DataValidator {
    /// Validates a username according to system policy requirements.

    /// Args:
    ///
    /// * `username` - The username string to validate, must be non-empty ASCII text

    /// Returns:
    ///
    /// A validated Username struct if all checks pass; error otherwise
    pub fn validate_username(&self, username: &str) -> Result<Username> {
        // Implementation...
    }
}
```

### Step 2: Document Panic Scenarios Explicitly

Identify every panic point and document it:

```rust
/// # Panics
///
/// This function will panic if:
///
/// * `username` is empty - usernames must contain at least one character per [USERNAME_FORMAT]
fn validate_required_field(value: &str) -> Result<&str> {
    value.parse().map_err(|_| ValidationError::EmptyField)?
}
```

### Step 3: Use Result<T, E> for All Error Paths

Never silently ignore errors. Either handle or propagate:

```rust
// BAD - Silent error suppression
fn process(input: &str) -> Option<bool> {
    let parsed = input.parse::<i64>().unwrap(); // PANIC if parse fails!
    Some(parsed > 0)
}

// GOOD - Explicit handling with Result
fn process(input: &str) -> Result<bool, ParseError> {
    let parsed = input.parse::<i64>()
        .map_err(|_| ParseError::InvalidFormat)?;
    Ok(parsed > 0)
}
```

### Step 4: Explain Error Variants Clearly

```rust
#[derive(Debug)]
pub enum ValidationError {
    /// Input contains invalid characters (per [SPECIFICATION_REGEX])
    InvalidCharacters { input: String, pattern: &'static str },

    /// Length exceeds maximum allowed size
    TooLong { max_length: usize, actual_length: usize },

    /// Required field is missing or empty after trimming whitespace
    EmptyField,
}
```

### Step 5: Document Edge Cases and Assumptions

```rust
/// # Errors
///
/// * `index` out of bounds - returns [None] rather than panicking (see behavior note)
fn get_or_default(index: usize) -> Option<&T> {
    self.data.get(index).or(Some(&self.default_value))
}

/// Note: This function never panics for invalid indices, always returning None.
```

## Common Patterns

### Pattern 1: Result Propagation with Context

```rust
use anyhow::{Context, Result};

pub async fn fetch_config() -> Result<Config> {
    let content = tokio::fs::read_to_string("config.yaml")
        .await
        .context("Failed to read configuration file")?;

    serde_yaml::from_str(&content)
        .context("Configuration YAML is malformed and cannot be parsed")
}
```

### Pattern 2: Panic Documentation Template

```rust
/// # Panics
///
/// This function will panic if:
///
/// * `index` >= `self.len()` when accessing by index - caller must validate bounds first
/// * The underlying database transaction has been rolled back externally (see [Self::validate] for details)
fn get_at_index(&self, index: usize) -> T {
    // Implementation that assumes valid input per precondition docs above
}
```

### Pattern 3: Builder Pattern with Clear Error Messages

```rust
impl ConfigBuilder {
    /// Creates a new builder instance.
    ///
    /// Args:
    ///
    /// * `default_port` - Default port to use if not specified (must be >= 1 and <= 65535)
    pub fn new(default_port: u16) -> Self {
        // Implementation...
    }

    /// Sets the server bind address.
    ///
    /// Must be a valid IPv4 or IPv6 address string. Invalid addresses will produce
    /// an error via [Self::error] indicating which parsing step failed (address vs port).
    pub fn with_address(mut self, addr: SocketAddr) -> Result<Self> {
        // Implementation...
    }
}
```

## Pitfalls to Avoid

### ❌ Bad Documentation - Missing WHY/WHAT/HOW Structure
```rust
/// Returns the user by id.
fn get_user(id: u32) -> User { /* ... */ }
// What? How? Why?
```

**✓ Good**: Explicit sections for each aspect.

### ❌ Silent Error Ignoration with unwrap()
```rust
let result = some_operation().unwrap(); // PANIC if error!
if let Ok(value) = parse(input) {
    return value;
}
return None; // Lost the error info entirely
```

**✓ Good**: Explicit handling:
```rust
match parse(input)? {  // Propagate with ?
    Some(v) => v,
    None => Err(ParseError::InvalidFormat),
}
```

### ❌ Vague Panic Documentation

Panic docs must be precise, not vague.

❌ `// Panics if invalid input`
✓: `"Panics when the underlying file handle is closed externally (see [Self::validate] for details)"`

## Examples

```rust
use thiserror::Error;
use std::path::PathBuf;

/// # Purpose (WHY)
///
/// Validates and loads a configuration file from disk. Ensures all required fields are present,
/// values follow business rules, and the structure is consistent with [CONFIG_SCHEMA].
impl ConfigLoader {
    /// Loads application configuration.

    /// Args:
    ///
    /// * `config_path` - Path to YAML configuration file; must exist on filesystem

    /// Returns:
    ///
    /// Parsed Configuration struct if validation passes
    pub fn load(&self, config_path: &Path) -> Result<Configuration> {
        // Implementation...
    }
}

/// # Errors
///
/// * `config_path` does not exist - see [std::fs::read_to_string] for file system errors
/// * File is empty or contains only whitespace
impl ConfigLoader {}
```

## References

- [`rust.md`](../stacks/rust.md) - Comprehensive Rust conventions (MANDATORY reading)
- The `thiserror` crate documentation: https://docs.rs/thiserror/latest/thiserror/
- The `anyhow` crate documentation: https://docs.rs/anyhow/latest/anyhow/

---

*Created: 2026-01-27*
