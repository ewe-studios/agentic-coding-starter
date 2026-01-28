# Rust Coding Standards

## Overview
- **Language**: Rust 1.75+ (stable channel, use latest stable)
- **Edition**: 2021 (latest edition with all modern features)
- **Use Cases**: Systems programming, high-performance backend services, CLI tools, embedded systems, WebAssembly, performance-critical code
- **Official Docs**: https://doc.rust-lang.org/
- **The Rust Book**: https://doc.rust-lang.org/book/

## Skill References (MANDATORY)

**ALL Rust skills MUST be consulted based on task type. Load skills selectively to optimize context.**

### Project Setup & Configuration ⚙️

**Read BEFORE setting up new Rust projects or configuring toolchain:**

- [`rust-directory-and-configuration`](../skills/rust-directory-and-configuration/skill.md)
  - Rust toolchain installation and configuration
  - Project structure and module organization
  - Cargo.toml profile optimizations (release, dev, test, bench)
  - rust-toolchain.toml for version pinning
  - Professional rustfmt and clippy configurations
  - .cargo/config.toml for faster linking (lld/mold)
  - Additional development tools (cargo-audit, cargo-deny, cargo-nextest, etc.)

### Implementation Work 🔨

**Read BEFORE implementing any new features:**

- [`rust-clean-implementation`](../skills/rust-clean-implementation/skill.md)
  - WHY/WHAT/HOW documentation patterns with panic documentation
  - derive_more error handling patterns
  - no_std/std hybrid library strategies
  - Naming conventions (RFC 430) for types, generics, lifetimes
  - #[must_use] attribute patterns
  - Performance patterns (SmallVec, inline hints, allocation reduction)
  - Security best practices (input validation, secrets management)
  - Iterator patterns and trait implementations
  - Type system mastery (newtypes, builders, type states)
  - **Examples**: [`.agents/skills/rust-clean-implementation/examples/`](../skills/rust-clean-implementation/examples/)

### Testing Work 🧪

**Read BEFORE writing or reviewing tests:**

- [`rust-testing-excellence`](../skills/rust-testing-excellence/skill.md)
  - Test location conventions (unit, integration, benchmarks)
  - Three test validations: input, output, error paths
  - Feature-gated test modules (NOT individual attributes)
  - Property-based testing with proptest
  - Anti-pattern: muted variables without assertions
  - Async test isolation with current_thread flavor
  - Test helper functions and organization
  - **Examples**: [`.agents/skills/rust-testing-excellence/examples/`](../skills/rust-testing-excellence/examples/)

### Async/Tokio Work ⚡

**Read BEFORE implementing async code:**

- [`rust-with-async-code`](../skills/rust-with-async-code/skill.md)
  - Core principle: Never block the event loop
  - Non-blocking I/O with timeouts
  - spawn_blocking for CPU-intensive work
  - Async test isolation (MANDATORY: current_thread flavor)
  - Channel patterns (unbounded, bounded, broadcast)
  - Task management and select! patterns
  - Common pitfalls (blocking, holding locks across await)
  - Stream processing patterns
  - **Examples**: [`.agents/skills/rust-with-async-code/examples/`](../skills/rust-with-async-code/examples/)

## Tooling Standards (Reference Only)

### Required Tools
- **rustup**: Rust toolchain installer and version manager
- **cargo**: Build system, package manager, test runner
- **rustfmt**: Official code formatter
- **clippy**: Lint tool for catching common mistakes
- **rust-analyzer**: LSP implementation for IDE support

### Configuration Files (Reference Only)

See Rule 05 (`./rules/04-work-commit-and-push-rules.md`) and [`agent-guide`](./skills/rust-clean-implementation/skill.md) examples:
- `Cargo.toml` - Package manifest with profile configurations
- `.clippy.toml` - Clippy lint configuration

## Verification Workflow (Reference Only)

**MANDATORY**: Every Rust code change MUST be verified by a dedicated verification agent before commit.

See [`verification-workflow-complete-guide`](./rules/08-verification-workflow-complete-guide.md) for complete workflow including:
1. Format check (`cargo fmt --check`)
2. Clippy linting
3. Compilation (debug + release)
4. Test execution

## Learning Log References (from consolidated skills)

### 2026-01-27: Testing Failures Anti-Patterns ⚠️
Tests that create variables but never validate their actual content are critical anti-patterns:
- ❌ Bad: `let result = do_work();` with no assertions about what happens after or validation of the variable's content.
- ✅ Good: Validates both valid and invalid inputs produce correct results/errors.

### 2026-01-23: no_std/std Implementation Strategy 📚
For libraries supporting both environments:
1. **no_std mode**: Always implement from scratch using `core` and atomics (see [`rust-clean-implementation`](../skills/rust-clean-implementation/skill.md))
2. **std mode**: Re-export std types when sufficient; wrap to add methods if needed

### 2026-01-24: Feature-Gated Type Architecture Pattern 📚
For managing complex feature combinations:
1. Create compatibility layers in higher-level dependencies (e.g., `foundation_nostd`)
2. Use explicit submodule paths - never wildcard re-exports for type imports
3. Move complexity up to dependency, keep consuming code simple and clear

---

*Created: 2026-01-11*
*Last Updated: 2026-01-28 - Enhanced with complete skill references and professional configuration patterns*
