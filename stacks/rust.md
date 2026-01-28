# Rust Coding Standards

## Overview
- **Language**: Rust 1.75+ (stable channel, use latest stable)
- **Edition**: 2021 (latest edition with all modern features)
- **Use Cases**: Systems programming, high-performance backend services, CLI tools, embedded systems, WebAssembly, performance-critical code
- **Official Docs**: https://doc.rust-lang.org/
- **The Rust Book**: https://doc.rust-lang.org/book/

## Skill References (MANDATORY)

### For Implementation Work ✅

**Read BEFORE implementing any new features:**

1. [`rust-clean-implementation`](../skills/rust-clean-implementation/skill.md) - WHY/WHAT/HOW documentation, derive_more error patterns, no_std/std hybrid library strategies from Learning Log
2. **Examples**: [`.agents/skills/rust-clean-implementation/examples/`](../skills/rust-clean-implementation/examples/)
3. MANDATORY reading of [`rust-coding-practice-agent-guide.md`](./rules/05-coding-practice-agent-orchestration.md) for implementation agent guidance

### For Testing Work ✅

**Read BEFORE writing or reviewing tests:**

1. [`rust-testing-excellence`](../skills/rust-testing-excellence/skill.md) - Test location conventions, feature-gated modules, validation patterns from Learning Log
2. **Examples**: [`.agents/skills/rust-testing-excellence/examples/`](../skills/rust-testing-excellence/examples/)
3. MANDATORY reading of [`rust-verification-workflow-complete-guide.md`](./rules/08-verification-workflow-complete-guide.md) for testing verification

### For Async/Tokio Work ✅

**Read BEFORE implementing async code:**

1. [`rust-with-async-code`](../skills/rust-with-async-code/skill.md) - Non-blocking I/O patterns, task spawning with tokio channels, spawn_blocking usage
2. **Examples**: [`.agents/skills/rust-with-async-code/examples/`](../skills/rust-with-async-code/examples/)
3. MANDATORY reading of [`rust-coding-practice-agent-guide.md`](./rules/05-coding-practice-agent-orchestration.md) for async agent guidance

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
*Last Updated: 2026-01-27 - Consolidated content into specialized skills (implementation/testing/async)*
