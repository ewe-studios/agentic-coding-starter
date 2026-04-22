---
name: "Rust Clean Code"
description: "Comprehensive Rust development practices covering implementation, testing, configuration, and async patterns"
approved: Yes
created: 2026-02-23
license: "MIT"
metadata:
  author: "Main Agent"
  version: "1.0"
  last_updated: "2026-02-23"
  tags: [rust, clean-code, best-practices, testing, async, configuration]
tools: []
files:
  - implementation/skill.md: "Clean implementation patterns with proper documentation and error handling"
  - testing/skill.md: "Testing excellence with real code validation and property-based testing"
  - async/skill.md: "Async/await and Tokio patterns for non-blocking I/O"
  - directory-and-configuration/skill.md: "Project setup, toolchain installation, and configuration"
---

# Rust Clean Code

## ⚡ Fast Check Workflow

**CRITICAL:** When doing Rust checks and you're not sure about something, use quick `cargo check` with `tee` to collect warnings rapidly. **5-6 minutes should be enough** for the initial pass. If after 5-6 minutes you see nothing, increase to 10 minutes on the next run. If it finishes with no issues — great. If you see issues early, collect enough in 5 minutes then go fix them. This increases iteration speed and reduces wasted wait time.

---
## Overview

This skill consolidates comprehensive Rust development practices into a modular structure. Each sub-skill focuses on a specific aspect of Rust development, providing detailed guidance, examples, and best practices.

Whether you're setting up a new project, implementing features, writing tests, or working with async code, this skill provides the foundational knowledge and patterns needed for clean, idiomatic Rust development.

All sub-skills are approved and ready for use. Each can be read independently based on your current task.

## Which Sub-Skill Should I Read?

**Choose based on your current task:**

**Project Setup & Configuration:**
1. **Setting up new project** → [`directory-and-configuration/skill.md`](directory-and-configuration/skill.md)
2. **Installing Rust toolchain** → [`directory-and-configuration/skill.md`](directory-and-configuration/skill.md)

**Writing Code:**
3. **Implementing features** → [`implementation/skill.md`](implementation/skill.md)
4. **Documenting code** → [`implementation/skill.md`](implementation/skill.md)
5. **Error handling** → [`implementation/skill.md`](implementation/skill.md)

**Testing:**
6. **Writing/reviewing tests** → [`testing/skill.md`](testing/skill.md)
7. **Property-based testing** → [`testing/skill.md`](testing/skill.md)

**Async Programming:**
8. **Working with async/await** → [`async/skill.md`](async/skill.md)
9. **Using Tokio runtime** → [`async/skill.md`](async/skill.md)
10. **Non-blocking I/O** → [`async/skill.md`](async/skill.md)

## Sub-Skills Reference

### 1. Directory and Configuration
**Path**: [`directory-and-configuration/skill.md`](directory-and-configuration/skill.md)

Install Rust toolchain, configure projects, and set up proper directory structure. Covers:
- Complete Rust toolchain installation
- Step-by-step project setup
- Cargo.toml and .cargo/config.toml configuration

### 2. Clean Implementation
**Path**: [`implementation/skill.md`](implementation/skill.md)

Write clean, well-documented Rust code with proper error handling and no_std/std support. Covers:
- WHY/WHAT/HOW documentation patterns
- Error handling with derive_more
- Security best practices
- Iterator and trait implementation patterns
- no_std compatibility

### 3. Testing Excellence
**Path**: [`testing/skill.md`](testing/skill.md)

Write proper, clear tests that validate both valid and invalid inputs with explicit assertions. Covers:
- Unit tests, integration tests, benchmarks
- Validating both valid AND invalid inputs
- Feature-gated test modules
- Property-based testing with proptest
- Docker and testcontainers for integration testing

### 4. Async Code
**Path**: [`async/skill.md`](async/skill.md)

Write robust async/await code using tokio with proper non-blocking patterns. Covers:
- Using tokio as async runtime
- Non-blocking I/O patterns
- Task spawning and coordination
- Avoiding blocking the event loop
- Async testing with proper isolation

## How to Use This Skill

1. **Identify your task** - Use the table above to determine which sub-skill applies
2. **Read the specific sub-skill** - Each sub-skill is complete and standalone
3. **Follow cross-references** - Sub-skills reference each other when topics overlap
4. **Check examples** - Each sub-skill has its own `examples/` directory with detailed guides

**Important**: This parent skill provides navigation only. The actual implementation guidance, examples, and patterns are in the individual sub-skills.

## Related Skills

- [`dst-tokio-rust`](../dst-tokio-rust/skill.md) - Distributed systems and Tokio patterns (builds on async sub-skill)

---

**Usage Type**: EDUCATIONAL

This is a navigation skill. Load the specific sub-skills for actual implementation guidance.

## ⚡ Rust Check Reminder

When running Rust checks: use `cargo check 2>&1 | tee /tmp/cargo-check.log` with a **5-6 minute timeout** for quick scans. Only extend to 10 minutes if the first pass shows nothing. Fix issues as soon as they appear — don't wait for a full check to complete.
