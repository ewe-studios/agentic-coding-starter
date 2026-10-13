---
name: "Rust Clean Implementation"
description: "Write clean, well-documented Rust code with proper error handling and no_std/std support"
approved: Yes
created: 2026-01-27
license: "MIT"
metadata:
  author: "Main Agent"
  version: "4.1-tracing-enforced"
  last_updated: "2026-04-06"
tags:
  - rust
  - clean-code
  - documentation
  - error-handling
  - no_std
  - abstraction
  - tracing
  - logging
files:
  - examples/documentation-patterns.md: "WHY/WHAT/HOW doc patterns with mandatory panic documentation"
  - examples/error-handling-guide.md: "Error handling with derive_more"
  - examples/security-guide.md: "Security best practices"
  - examples/iterator-patterns.md: "Iterator and trait implementation patterns"
  - examples/dependency-hierarchy.md: "Project first, stdlib second, external last - with examples"
  - examples/no-std-support.md: "Supporting both no_std and std environments"
  - examples/performance-tips.md: "Performance optimization patterns and benchmarking"
  - examples/trait-patterns.md: "Trait implementation best practices"
  - examples/basic-template.md: "Basic implementation template"
  - examples/tracing-logging.md: "Mandatory tracing crate usage for all logging"
---

# Rust Clean Implementation

## When to Use This Skill

Read this skill when **implementing new Rust code** (not tests or async). For testing, see [rust-testing-excellence](../testing/skill.md). For async code, see [rust-with-async-code](../async/skill.md).

---

## 🎯 Core Principles (CRITICAL - Always Apply)

### 1. Dependency Hierarchy: Project → Stdlib → External 🚨

**MANDATORY:** Check what the project already has before adding dependencies.

```
1. Project modules/crates (FIRST - search codebase)
   ↓ Can't fulfill need?
2. Rust stdlib (SECOND - use std::* when possible)
   ↓ Can't fulfill need?
3. External crates (LAST RESORT - truly necessary)
```

**Quick check:**
```bash
# Search for existing types
rg "struct.*Http" --type rust
rg "enum.*Error" --type rust
```

📖 **Read when adding dependencies:** [`dependency-hierarchy.md`](examples/dependency-hierarchy.md) - Complete examples and decision process

### 2. Iron Rule: Always Use `tracing` Crate for Logging 🚨

**NON-NEGOTIABLE:** ALL logging MUST use the `tracing` crate macros. No exceptions.

**Required dependencies in EVERY crate:**

```toml
[dependencies]
tracing = { version = "0.1" }

[dev-dependencies]
tracing-test = { version = "0.2.5", features = ["no-env-filter"] }
serial_test = "3.3.1"
```

**Use tracing macros for ALL logging:**
```rust
use tracing::{debug, error, info, trace, warn};

// Info for important runtime events
info!("Server started on port {}", port);
info!("Model downloaded to: {}", path.display());

// Debug for development/diagnostic info
debug!("Processing request: {:?}", request);

// Trace for fine-grained tracing
trace!("Entering function with args: {:?}", args);

// Warn for recoverable issues
warn!("Retrying failed operation (attempt {}/{})", attempt, max_attempts);

// Error for actual errors
error!("Failed to connect: {}", error);
```

**Tests MUST use `#[traced_test]`:**
```rust
use tracing_test::traced_test;

#[test]
#[traced_test]
fn test_something() {
    info!("Test started");
    // Test code - logs will appear in test output
}
```

**Why tracing:**
- Structured logging with spans for better debugging
- Async-safe (unlike `println!` or `log`)
- Zero overhead when not collecting traces
- Integrates with observability tools
- Project standard - consistency matters

**Forbidden:**
- ❌ `println!()` / `eprintln!()` (except for CLI output)
- ❌ `dbg!()` in production code (fine for temporary debugging)
- ❌ `log` crate macros (`log::info!`, etc.)
- ❌ Any other logging crate

📖 **Read for complete guide:** [`tracing-logging.md`](examples/tracing-logging.md)

### 3.0 Things to note

1. Implement Debug and Display for structs, so they can be printed in logs always.
2. Use derive_more for custom errors and its supplied capabilities
3. Preferrably unless specified not to, use foundation_errstack for ewe_platform projects.
4. For ewe_platform always read the valtron skills.
4. Unless the user indicates, no clippy, cargo checks, build errors are pre-existing, fix it (some dead_code suppression is allowed, confirm with users) but everything else should be fixed and not suppresed.

### 3. Documentation: WHY/WHAT/HOW Pattern

**Every public item needs documentation:**

```rust
/// WHY: Validates user input to prevent injection attacks
///
/// WHAT: Checks that input contains only alphanumeric characters
///
/// HOW: Uses regex pattern `^[a-zA-Z0-9]+$`
///
/// # Errors
/// Returns `Error::InvalidInput` if input contains special characters
///
/// # Panics
/// Never panics
pub fn validate_input(input: &str) -> Result<(), Error> {
    // Implementation
}
```

**Mandatory sections:**
- ✅ WHY - Purpose and motivation
- ✅ WHAT - What it does (one sentence)
- ✅ HOW - How it works (algorithm/approach)
- ✅ Errors - Document all error cases
- ✅ Panics - Document panic conditions (or state "Never panics")

📖 **Read for complete patterns:** [`documentation-patterns.md`](examples/documentation-patterns.md)

### 4. Error Handling with derive_more

**Use `derive_more` for clean error types:**

```rust
use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[display(fmt = "invalid input: {}", _0)]
    InvalidInput(String),

    #[display(fmt = "not found")]
    NotFound,

    #[from]
    Io(std::io::Error),
}
```

**Pattern:**
- Use `derive_more` for Display/Error/From
- Provide context in display messages
- Use `#[from]` for automatic conversions
- Always implement Debug

📖 **Read for complete guide:** [`error-handling-guide.md`](examples/error-handling-guide.md)

### 5. No_std Support (When Required)

**Pattern for libraries:**

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
use core::{fmt, result};

#[cfg(feature = "std")]
use std::{fmt, result};
```

```toml
[features]
default = ["std"]
std = []
```

📖 **Read when implementing:** [`no-std-support.md`](examples/no-std-support.md) - Complete patterns and testing

### 6. Stream I/O Bodies — Never Buffer Them 🚨

HTTP, RPC, Docker and any other I/O bodies are consumed as **streams** (`StreamIterator` / `FutureStream` / `Stream`), never read whole into memory.

- **Check the status before touching the body.** On a non-success status, read only a bounded amount of the body to build the error message.
- Large or unbounded payloads (image pulls/pushes/exports, progress streams, logs, file transfers, model downloads) are returned to the caller as a stream of typed items; the caller decides how to consume them.
- A full read is acceptable **only** in tests, or when the body is small **and** its contents are needed for the next step (e.g. a small JSON response that is parsed and acted on).

```rust
// BAD: OOM risk, and the status is checked after the whole body is read
let bytes = response.read_to_end()?;
if !response.status().is_success() { return Err(...); }

// GOOD: status first, bounded error body, stream the success body
if !response.status().is_success() {
    let msg = response.read_bounded(MAX_ERROR_BODY)?;
    return Err(Error::Api { status, msg });
}
Ok(response.into_stream().map_done(parse_progress_line))
```

### 7. Use Generated Types — Fix the Generator, Don't Work Around It 🚨

Crates with generated API clients (Docker, cloud providers, other platform crates) must use their **generated request/response types** — no hand-written duplicate structs or untyped `serde_json::Value` where a generated type exists.

If a generated type is missing or wrong, or a generated function can't do what's needed (e.g. it drops a streaming body after the headers), **fix the generator and regenerate**. Hand-written wrappers that paper over generator gaps are not acceptable.

### 8. Async All the Way; Valtron Composes With It 🚨

Write async logic as native Rust `async fn`s, all the way down. Valtron hooks into async rather than replacing it:

- valtron can drive a future (`execute(future)` → `StreamIterator`);
- valtron streams/tasks can be used inside async code by turning them into a future or future stream (`into_ready_future`, `into_collect_future`, `into_future_stream`, `into_next_stream`).

Respect valtron's existing rules for when a task may cross into async (`Send + 'static`, the `!Send` constraint, blocking only at boundaries). Native valtron `TaskIterator` state machines stay valtron-native and are exposed to async callers through those adapters. See [`rust-valtron-usage`](../../rust-valtron-usage/skill.md).

---

## 📚 Implementation Patterns (Read When Needed)

### Security Best Practices

**When to read:** Before handling user input, crypto, or sensitive data

**Quick checklist:**
- [ ] Validate ALL user inputs
- [ ] Use `zeroize` for sensitive data
- [ ] Avoid `unsafe` unless absolutely necessary
- [ ] Document security assumptions

📖 [`security-guide.md`](examples/security-guide.md)

### Library-Owned CLI Pattern

**When to read:** When adding CLI subcommands that depend on library logic

**Pattern:** CLI command definitions (`clap::Command`) and handlers live in the library crate behind a `cli` feature flag. The platform binary is a thin wrapper that calls `command()` and `run()`. This allows the CLI to be used by standalone binaries or other consumers.

```rust
// In library crate (e.g., foundation_codegentools/src/cli/schema.rs):
#[must_use]
pub fn command() -> clap::Command { /* define args */ }
pub fn run(args: &clap::ArgMatches) -> Result<(), BoxedError> { /* handle */ }

// In library Cargo.toml:
// [features]
// cli = ["dep:clap"]

// In platform binary (thin wrapper):
pub fn register(command: clap::Command) -> clap::Command {
    command.subcommand(foundation_codegentools::cli::schema::command())
}
pub fn run(args: &clap::ArgMatches) -> Result<(), BoxedError> {
    foundation_codegentools::cli::schema::run(args)?;
    Ok(())
}
```

**Existing examples:** `foundation_testbed::cli`, `foundation_codegentools::cli`

### Iterator and Trait Patterns

**When to read:** Implementing custom iterators or traits

**Quick example:**
```rust
impl Iterator for MyType {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> { }
}
```

📖 [`iterator-patterns.md`](examples/iterator-patterns.md) - Iterator implementations

📖 [`trait-patterns.md`](examples/trait-patterns.md) - Trait best practices

### Performance Optimization

**When to read:** After profiling shows hot spots

**Core rules:**
1. **Measure first** - Use criterion benchmarks
2. **Profile** - Use flamegraph
3. **Optimize hot paths** - Focus on frequent code
4. **Avoid premature optimization** - Clarity first

📖 [`performance-tips.md`](examples/performance-tips.md) - Complete patterns and benchmarking

---

## ✅ Implementation Checklist

Every new module/function must have:

- [ ] `tracing` crate in dependencies (mandatory for ALL crates)
- [ ] `tracing-test` and `serial_test` in dev-dependencies
- [ ] All logging via tracing macros (`info!`, `debug!`, `error!`, `warn!`, `trace!`)
- [ ] Tests annotated with `#[traced_test]`
- [ ] WHY/WHAT/HOW documentation
- [ ] Error types with `derive_more`
- [ ] Errors section in docs
- [ ] Panics section in docs (or "Never panics")
- [ ] Used project types before external deps
- [ ] Security considerations (for user input/sensitive data)
- [ ] Tests (see [testing skill](../testing/skill.md))

**Forbidden:**
- ❌ `println!()` / `eprintln!()` for logging (except CLI output)
- ❌ `dbg!()` in production code
- ❌ `log` crate or other logging crates
- ❌ Undocumented public items
- ❌ Missing error documentation
- ❌ Adding external deps without checking project first
- ❌ `unwrap()`/`expect()` in library code (use `?`)
- ❌ Ignoring no_std support (if project supports it)

---

## 📖 When to Read Example Files

**Always check first:**
1. ⭐ [`dependency-hierarchy.md`](examples/dependency-hierarchy.md) - Before adding ANY dependency
2. ⭐ [`tracing-logging.md`](examples/tracing-logging.md) - Mandatory for ALL crates

**Read when you need to:**

3. **Documentation:** [`documentation-patterns.md`](examples/documentation-patterns.md) - WHY/WHAT/HOW patterns
4. **Error handling:** [`error-handling-guide.md`](examples/error-handling-guide.md) - derive_more examples
5. **Security:** [`security-guide.md`](examples/security-guide.md) - Input validation, crypto, unsafe
6. **No_std:** [`no-std-support.md`](examples/no-std-support.md) - Supporting both std and no_std
7. **Iterators:** [`iterator-patterns.md`](examples/iterator-patterns.md) - Custom iterator implementation
8. **Traits:** [`trait-patterns.md`](examples/trait-patterns.md) - Trait implementation patterns
9. **Performance:** [`performance-tips.md`](examples/performance-tips.md) - After profiling
10. **Template:** [`basic-template.md`](examples/basic-template.md) - Starting a new module

---

## 🔗 Related Skills

- [rust-testing-excellence](../testing/skill.md) - Writing tests
- [rust-with-async-code](../async/skill.md) - Async patterns
- [rust-directory-and-configuration](../directory-and-configuration/skill.md) - Project structure
