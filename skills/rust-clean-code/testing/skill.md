---
name: "Rust Testing Excellence"
description: "Write proper, clear tests that validate both valid and invalid inputs with explicit assertions"
approved: Yes
created: 2026-01-27
license: "MIT"
metadata:
  author: "Main Agent"
  version: "5.0-streamlined"
  last_updated: "2026-03-03"
tags:
  - rust
  - testing
  - validation
  - docker
  - testcontainers
files:
  - examples/test-organization.md: "MUST READ - Complete project structure for tests (tests/units/, tests/integration/)"
  - examples/three-test-validations.md: "Read when writing tests - The three validations every test must perform"
  - examples/integration-theater-vs-real-testing.md: "Read when deciding mock vs real - Examples of bad mocking vs good testing"
  - examples/common-pitfalls.md: "Read when tests fail unexpectedly - Common mistakes and solutions"
  - examples/http-testing-with-project-types.md: "Read when testing HTTP - Using project's own types to create TestHttpServer"
  - examples/tcp-testing-stdlib.md: "Read when testing TCP/network - Pure stdlib testing without dependencies"
  - examples/docker-for-testing.md: "Read when testing databases/services - Docker-compose and CI integration"
  - examples/testcontainers-examples.md: "Read when using testcontainers - PostgreSQL, Redis, MongoDB examples"
  - examples/test-dependency-decisions.md: "Read when adding dependencies - Decision trees for choosing stdlib vs external deps"
  - examples/feature-gated-tests.md: "Read when testing with features - Module-level feature gates"
  - examples/property-based-testing-basics.md: "Read when testing invariants - Quick proptest patterns"
  - examples/intro-to-property-based-testing.md: "Read for comprehensive proptest guide - Complete beginner to advanced"
  - examples/running-tests.md: "Read when running tests - Cargo commands, features, parallel execution"
---

# Rust Testing Excellence

## When to Use This Skill

Read this skill when **writing or reviewing tests**. For implementation patterns, see [rust-clean-implementation](../implementation/skill.md). For async code, see [rust-with-async-code](../async/skill.md).

---

## 🎯 Core Principles (CRITICAL - Always Apply)

### 1. Real Code Over Mocks 🚨

**The Fundamental Rule:** Tests must validate actual code behavior, not mock behavior.

**✅ VALID Mock Usage (External Only):**
- Third-party services (Stripe, payment gateways)
- System resources (hardware devices, OS calls)
- Error injection (disk full, network partition)

**❌ INVALID Mock Usage (Our Own Code):**
- HTTP clients → Use real test servers
- Databases → Use testcontainers/Docker
- File I/O → Use `tempfile` with real filesystem
- Internal services → Test the real thing

**The Three Questions Before Every Mock:**
1. "Is this really external (third-party/OS)?" - If it's yours, **NO MOCK**
2. "Am I testing real logic or mock setup?" - If just mock config, **INVALID**
3. "Are integration points tested separately?" - Need real tests too

📖 **See examples:** [`integration-theater-vs-real-testing.md`](examples/integration-theater-vs-real-testing.md)

### 2. The Three Test Validations ✅

**Every test MUST validate:**
1. ✅ Valid input produces expected output
2. ✅ Invalid input is properly rejected with clear errors
3. ✅ Edge cases are handled (empty, max, boundary)

```rust
// ✅ Example - All three validations
#[test]
fn test_valid_registration() {
    let user = register("alice", "alice@example.com").unwrap();
    assert_eq!(user.username, "alice");
}

#[test]
fn test_invalid_email_rejected() {
    assert!(register("bob", "not-an-email").is_err());
}

#[test]
fn test_empty_username_rejected() {
    assert!(register("", "test@example.com").is_err());
}
```

📖 **See complete guide:** [`three-test-validations.md`](examples/three-test-validations.md)

### 3. Test Organization (MUST READ)

**CRITICAL:** ALL tests go in `tests/` directory. **NO** `#[cfg(test)]` modules in source files.

```
crate_root/
├── src/              # NO tests here
├── tests/            # ALL Rust tests here
│   ├── units/        # Unit tests: {crate}_{module}_tests.rs
│   └── integration/  # Rust integration tests: {crate}_{workflow}.rs
├── integration/      # Cross-language / external-runtime harnesses for THIS crate
└── benches/          # Benchmarks
```

📖 **MUST READ:** [`test-organization.md`](examples/test-organization.md) - Complete structure and examples

#### Crate-owned integration harnesses 🚨

**RULE:** Integration tests and harnesses for a specific crate live **inside that crate**, never
in a top-level project `/integrations` directory. This keeps everything for a crate discoverable
in one place.

- **Rust** integration tests → `{crate}/tests/integration/`.
- **Cross-language / external-runtime** harnesses (e.g. a `node:test` suite driving a crate's
  WASM/JS runtime, a Deno/browser runner, fixtures, mock host/DOM, and any helper build scripts)
  → `{crate}/integration/` (a sibling of `src/` and `tests/`).
- If a harness spans two crates, it lives with the crate that **owns the surface under test**
  (e.g. `foundation_wasm`'s ABI runtime tests live in `foundation_wasm/integration/`; the DOM-layer
  tests live in `foundation_wasm_ui/integration/` and reach the sibling crate via relative paths).
- A nested helper crate used only by the harness (e.g. a wasm fixture module) is a **standalone
  crate** (its own empty `[workspace]`) under `{crate}/integration/`, and added to the workspace
  `exclude` so it doesn't pollute the main build / feature unification.
- Keep these out of the published package (e.g. `include = ["/src", ...]` in `Cargo.toml`).

**Why:** a crate's tests — at every level, in every language — should travel with the crate, not
be scattered in a shared top-level folder that hides which crate they belong to.

---

## 🔧 Mandatory Workflow Principles

### 🚨 ONE Test at a Time (CRITICAL)

**⚠️ MANDATORY:** Write ONE test, make it pass, THEN move to next test.

**TDD Cycle:**
```
🔴 Write ONE test → Verify FAILS → Implement → Verify PASSES → Refactor
                              ↓ ONLY THEN
🔴 Write NEXT test → ...
```

**Never:**
- ❌ Write multiple tests at once
- ❌ Generate test file with all tests
- ❌ Skip ahead before current test passes

📖 **Complete TDD workflow:** [Test-Driven Development](../../test-driven-development/skill.md)

### Always Update tests/mod.rs

When writing test files, add them to `tests/mod.rs` so Rust includes them:

```rust
// tests/mod.rs or tests/units/mod.rs
mod myapp_parser_tests;
mod myapp_validation_tests;
```

### Run Correct Package

```bash
# Identify the right package
cargo test --package crate_name

# Verify tests are actually running
cargo test -- --list
```

### No False Claims

- ❌ Empty test bodies
- ❌ `assert!(true)` to fake passing
- ❌ Variables calculated but never asserted

```rust
// ❌ BAD - Cheating
#[test]
fn test_logic() { }

#[test]
fn test_logic() { assert!(true) }

// ✅ GOOD - Real test
#[test]
fn test_logic() {
    let result = compute(input);
    assert_eq!(result, expected);
}
```

---

## 📚 Testing Patterns (Read When Needed)

### When Testing HTTP

**Decision:** Project types → Stdlib → External deps

1. **Does project have HTTP types?** → Create `foundation_testing` crate
2. **Can stdlib do it?** → Use `std::net::TcpListener`
3. **Need external dep?** → Use minimal dep like `tiny_http`

📖 **Read when implementing:** [`http-testing-with-project-types.md`](examples/http-testing-with-project-types.md), [`tcp-testing-stdlib.md`](examples/tcp-testing-stdlib.md)

### When Testing Databases

**Decision tree:**

1. **Can run in Docker?** → Use docker-compose or testcontainers ✅ BEST
2. **Test instance available?** → Ask dev team for credentials
3. **Can use SQLite?** → Use `:memory:` for SQL
4. **Must mock?** → Last resort only

📖 **Read when implementing:** [`docker-for-testing.md`](examples/docker-for-testing.md), [`testcontainers-examples.md`](examples/testcontainers-examples.md)

Quick example:
```toml
[dev-dependencies]
testcontainers = "0.15"
```

```rust
let docker = clients::Cli::default();
let postgres = docker.run(images::postgres::Postgres::default());
// Test with real PostgreSQL
```

### When Testing with Features

Use **module-level gates**, not individual `#[cfg]` attributes:

```rust
#[cfg(test)]
mod tests {
    #[cfg(not(feature = "std"))]
    mod no_std_tests { /* ... */ }

    #[cfg(feature = "std")]
    mod std_tests { /* ... */ }
}
```

📖 **Read when implementing:** [`feature-gated-tests.md`](examples/feature-gated-tests.md)

### When Testing Properties (Not Specific Values)

Use property-based testing for:
- Roundtrip properties (serialize/deserialize)
- Invariants (sorted output stays sorted)
- Mathematical properties (commutative, associative)

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_roundtrip(data in any::<MyData>()) {
        let json = to_json(&data).unwrap();
        let parsed = from_json(&json).unwrap();
        assert_eq!(data, parsed);
    }
}
```

📖 **Read when implementing:** [`property-based-testing-basics.md`](examples/property-based-testing-basics.md) (quick), [`intro-to-property-based-testing.md`](examples/intro-to-property-based-testing.md) (comprehensive)

---

## ⚠️ Common Mistakes (Read When Issues Occur)

### Pitfall 1: Testing Implementation Details
❌ Testing internal state → ✅ Test observable behavior

### Pitfall 2: No Error Path Testing
❌ Only success cases → ✅ Test both success and failure

### Pitfall 3: Missing Initialization
❌ Tests pass for wrong reasons → ✅ Proper setup before each test

### Pitfall 4: Muted Variables
❌ Calculate but don't assert → ✅ Explicit assertions on all results

📖 **Read when debugging:** [`common-pitfalls.md`](examples/common-pitfalls.md)

---

## 🚀 Running Tests

```bash
# Basic
cargo test                    # All tests
cargo test test_name          # Specific test
cargo test -p crate_name      # Specific package

# Features
cargo test --all-features           # All features
cargo test --no-default-features    # no_std
cargo test --features "feature"     # Specific feature

# Ignored tests (network/slow)
cargo test -- --ignored             # Only ignored
cargo test -- --include-ignored     # All tests

# Debug
cargo test -- --nocapture           # Show output
cargo test -- --test-threads=1      # Sequential
```

📖 **Complete reference:** [`running-tests.md`](examples/running-tests.md)

---

## ✅ Test Requirements Checklist

Every test must have:

- [ ] Clear name describing what is tested
- [ ] Explicit assertions (no muted variables)
- [ ] Both success AND failure paths tested
- [ ] Edge cases covered (empty, max, boundary)
- [ ] Real dependencies (no mocks for our code)
- [ ] Proper setup/initialization
- [ ] Clear error messages in assertions

**Forbidden:**

- ❌ Tests in source files (`src/`)
- ❌ Empty test bodies or `assert!(true)`
- ❌ Mocking our own HTTP/database/services
- ❌ Tests without error path validation

---

## 📖 When to Read Example Files

**Always read first:**
1. ⭐ [`test-organization.md`](examples/test-organization.md) - Test file structure (MUST READ)

**Read when you need to:**

2. **Writing tests:** [`three-test-validations.md`](examples/three-test-validations.md) - Valid/invalid/edge pattern
3. **Mocking decisions:** [`integration-theater-vs-real-testing.md`](examples/integration-theater-vs-real-testing.md) - Real vs mock examples
4. **HTTP testing:** [`http-testing-with-project-types.md`](examples/http-testing-with-project-types.md) - Use project's types
5. **TCP testing:** [`tcp-testing-stdlib.md`](examples/tcp-testing-stdlib.md) - Pure stdlib approach
6. **Database testing:** [`testcontainers-examples.md`](examples/testcontainers-examples.md) - PostgreSQL/Redis/MongoDB
7. **Docker setup:** [`docker-for-testing.md`](examples/docker-for-testing.md) - docker-compose + CI
8. **Dependencies:** [`test-dependency-decisions.md`](examples/test-dependency-decisions.md) - Decision trees
9. **Feature flags:** [`feature-gated-tests.md`](examples/feature-gated-tests.md) - Module-level gates
10. **Property testing:** [`property-based-testing-basics.md`](examples/property-based-testing-basics.md) - Quick patterns
11. **Debugging:** [`common-pitfalls.md`](examples/common-pitfalls.md) - Common mistakes
12. **Running tests:** [`running-tests.md`](examples/running-tests.md) - Cargo commands

---

## 🔗 Related Skills

- [rust-clean-implementation](../implementation/skill.md) - Implementation patterns and error handling
- [rust-with-async-code](../async/skill.md) - Async code patterns and runtime management
