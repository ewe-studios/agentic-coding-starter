---
name: "Code Verification"
description: "Complete code verification workflow including checks, standards, and quality validation before commits"
approved: Yes
created: 2026-02-27
license: "MIT"
metadata:
  author: "Main Agent"
  version: "1.0"
  last_updated: "2026-02-27"
  tags: [verification, testing, quality, validation, standards]
tools: [Bash]
files: []
---

# Code Verification

## Overview

Complete verification workflow ensuring NO code is EVER committed without passing ALL quality checks. **ZERO TOLERANCE** for violations.

**Usage Type**: EDUCATIONAL - Learn verification patterns and implement in workflow.

## When to Use

Use this skill when:
- Main Agent needs to verify code before commit
- Verification agent spawned to run quality checks
- Need to understand what checks are required
- Implementing verification in your workflow

## Prerequisites

- Access to language language skills (`language-specific skill files`)
- Understanding of verification tools (cargo, npm, pytest, etc.)
- Clear understanding of what files changed

## Core Principle

```
Task Complete → Report → Verify → Pass? → Commit → Push
                             ↓
                           Fail → Fix → Loop
```

**CRITICAL**: Code commits ONLY after ALL checks pass.

## Focused Local Checks, Full CI on GitHub (owner rule 2026-11-04)

- **Locally, check only what the change can break**: the format, lint and tests of the
  crates you changed, plus the tests of the workspace crates that depend on them and use the
  changed API (`cargo tree -i <crate> --workspace --depth 1`, then keep the ones that call
  what changed). Keep it fast. Do not run the whole-workspace CI script locally.
- **The full run is GitHub CI on the pull request.** After a push, the main agent reads the
  PR's CI result and sends each failure to the agent that owns the code. A run that was
  cancelled by a newer push is not a failure; read the run for the newest commit.
- **CI that does not run on a branch push** (only on pull requests) is started with
  `workflow_dispatch` for the changed packages, or by opening the PR.

## Test Against the Real Service, Not a Stub (owner rule 2026-11-03)

When a local tool runs the real thing, tests use it, not a hand-written stub. For example
`wrangler dev --local` runs the real D1, R2 and KV bindings and serves Cloudflare's REST API
locally (`/cdn-cgi/local/explorer/api`); test the bindings inside workerd and the REST
clients against that API. A stub adds behaviour of its own and hides real differences (it
once made a REST API look like it accepted JSON numbers when the real API takes strings
only). Use a stub only when no local form of the service exists, and say so in the test.

## Agent Hierarchy

**Main Agent** (Only agent with verification authority):
- Directly interacting with user
- ONLY agent that spawns verification agents
- Commits code after verification passes

**Sub-Agents**:
- NEVER spawn verification agents
- Report completion to Main Agent
- Wait for Main Agent to coordinate verification

**Identity Check**: Spawned by another agent? → You are SUB-AGENT (no verification authority)

## Verification Workflow

### Phase 1: Main Agent Analyzes

After implementation agent reports completion:
1. Identify language(s) modified from changed files
2. Spawn **ONE verification agent per language** (NEVER more)
3. Provide context (files, description, specification)
4. Wait for verification results

### Phase 2: Verification Agent Runs ALL Checks

**Check Order (MANDATORY):**
1. **Incomplete Implementation Check** (FIRST - if fail, stop immediately)
2. Format check
3. Lint check
4. Type check (if applicable)
5. Tests (ALL must pass)
6. Build
7. Security scan
8. Standards compliance

### Phase 3: Main Agent Decision

**If ALL Pass ✅:**
1. `git add [files]`
2. `git commit -m "[message with verification status]"`
3. `git push`
4. Update specification
5. Proceed to next task

**If ANY Fail ❌:**
1. Create urgent fix task
2. Provide failure details to implementation agent
3. Wait for fix
4. Return to Phase 2 (verify again)
5. Loop until all pass

## Mandatory Checks by Language

### Rust

**Commands:**
```bash
# 1. Incomplete implementation check (MANDATORY FIRST)
grep -rn "TODO\|FIXME\|unimplemented!\|todo!\|panic!(\"not implemented\")" src/

# 2. Format
cargo fmt -- --check

# 3. Lint (zero warnings)
cargo clippy -- -D warnings

# 4. Tests
cargo test

# 5. Build
cargo build --all-features

# 6. Documentation
cargo doc --no-deps

# 7. Security
cargo audit

# 8. Standards compliance (no unwrap(), proper docs)
```

### JavaScript/TypeScript

**Commands:**
```bash
# 1. Incomplete implementation check (MANDATORY FIRST)
grep -rn "TODO\|FIXME\|// stub" src/

# 2. Format
npx prettier --check .

# 3. Type check (zero errors)
npx tsc --noEmit

# 4. Lint (zero warnings)
npx eslint . --max-warnings 0

# 5. Tests (with coverage)
npm test

# 6. Build
npm run build

# 7. Security
npm audit

# 8. Standards compliance (no any type)
```

### Python

**Commands:**
```bash
# 1. Incomplete implementation check (MANDATORY FIRST)
grep -rn "TODO\|FIXME\|NotImplementedError\|pass  # stub" src/

# 2. Format
black --check .

# 3. Lint (zero errors)
ruff check .

# 4. Type check (strict mode)
mypy .

# 5. Tests (with coverage)
pytest --cov

# 6. Import check
python -m py_compile src/**/*.py

# 7. Security
pip-audit

# 8. Standards compliance (no mutable defaults)
```

## Incomplete Implementation Check (MANDATORY FIRST)

**Before any other checks, search for:**
- `TODO`, `FIXME` markers
- `unimplemented!()`, `todo!()`, `panic!("not implemented")`
- Stub methods (functions returning default/Ok(()) only)
- Functions with just `pass` in Python
- Functions with `NotImplementedError` in Python

**If ANY found → FAIL IMMEDIATELY**

**Why**: Features claiming "complete" cannot have incomplete implementations.

## Test Quality Validation

**Valid Tests:**
- ✅ Unit tests with real components (localhost, temp files)
- ✅ Integration tests with real local services (test servers, test DBs)
- ✅ End-to-end tests with full workflows
- ✅ Limited mocks only for external services (payment gateways, third-party APIs)

**Invalid Tests (FAIL verification):**
- ❌ Mocking our own code (HTTP clients, databases we wrote)
- ❌ Integration tests without integration (all calls mocked)
- ❌ Mock-only testing (no real validation)
- ❌ Untested integration points

**Example - Bad:**
```rust
#[test]
fn test_http_client() {
    let mock_dns = MockDnsResolver::new();  // ❌ Mocking our own code
    let mock_tcp = MockTcpConnection::new(); // ❌ Mocking our own code
    let client = HttpClient::new(mock_dns, mock_tcp);
    assert!(client.get("http://example.com").is_ok());
}
```

**Example - Good:**
```rust
#[tokio::test]
async fn test_http_client_real() {
    let server = TestHttpServer::new("127.0.0.1:8080"); // ✅ Real test server
    server.respond_with(200, "OK");

    let client = HttpClient::new(); // ✅ Real client
    let response = client.get("http://127.0.0.1:8080").await.unwrap();

    assert_eq!(response.status(), 200);
}
```

## Verification Report Format

```markdown
# [Language] Verification Report

## Status: PASS ✅ / FAIL ❌

## Files Verified
- [list of files]

## Check Results
1. Incomplete Implementation: PASS ✅ / FAIL ❌
2. Format: PASS ✅ / FAIL ❌
3. Lint: PASS ✅ / FAIL ❌
4. Type Check: PASS ✅ / FAIL ❌
5. Tests: N/N PASS ✅ / FAIL ❌
6. Build: PASS ✅ / FAIL ❌
7. Security: PASS ✅ / FAIL ❌
8. Standards: PASS ✅ / FAIL ❌

## Test Results
- Total: [N]
- Passed: [N]
- Failed: [N]
- Coverage: [N]%

## Failures (if any)
[Detailed failure information for each failed check]
```

## Commit Message with Verification

**After ALL checks pass:**

```bash
git commit -m "$(cat <<'EOF'
Add authentication middleware

Implemented JWT-based authentication middleware.

Changes made:
- Created auth.js with token validation
- Added JWT verification
- Implemented error handling
- Wrote comprehensive tests

Verified by JavaScript Verification Agent: All checks passed
- Incomplete Implementation: PASS
- Format: PASS (prettier)
- Lint: PASS (eslint, 0 warnings)
- Type Check: PASS (tsc)
- Tests: 12/12 PASS, coverage 95%
- Build: PASS
- Security: PASS (npm audit, 0 vulnerabilities)
- Standards: PASS

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

## Verification Patterns

### Pattern: Single Language

```
1. Implementation agent reports completion
2. Main Agent identifies language (e.g., Rust)
3. Main Agent spawns Rust verification agent
4. Verification agent runs ALL Rust checks
5. If PASS: Main Agent commits + pushes
6. If FAIL: Main Agent creates fix task, resumes implementation agent
```

### Pattern: Multiple Languages

```
1. Implementation agent reports completion
2. Main Agent identifies languages (e.g., Rust + TypeScript)
3. Main Agent spawns Rust verification agent
4. Main Agent spawns TypeScript verification agent
5. Wait for BOTH verification results
6. If ALL PASS: Main Agent commits + pushes
7. If ANY FAIL: Main Agent creates fix tasks for failed languages
```

### Pattern: Fix Loop

```
1. Verification FAILS
2. Main Agent creates urgent fix task with failure details
3. Main Agent resumes/spawns implementation agent with fix requirements
4. Implementation agent fixes ALL failures
5. Implementation agent reports fix completion
6. Main Agent spawns verification agent again
7. Loop until ALL checks PASS
```

## Common Verification Failures

### Incomplete Implementation
```
Found: TODO markers, unimplemented!() macros, stub methods
Fix: Complete all implementations before marking as done
```

### Format Failures
```
Found: Code not formatted per project standards
Fix: Run formatter (cargo fmt, prettier, black)
```

### Lint Failures
```
Found: Warnings from linter
Fix: Address all warnings (zero tolerance)
```

### Test Failures
```
Found: Tests failing or insufficient coverage
Fix: Fix failing tests, add tests for uncovered code
```

### Build Failures
```
Found: Compilation errors, missing dependencies
Fix: Resolve compilation errors, add missing dependencies
```

## Enforcement

### Must Do
1. Check for incomplete implementations FIRST
2. Run ALL checks from language language skill
3. Validate test quality (real vs mock)
4. Main Agent spawns verification agents (one per language)
5. Include verification status in commits
6. Fix ALL failures before commit
7. Push after successful commit

### Must Not Do
1. Skip any verification checks
2. Commit with incomplete implementations
3. Use mock-only testing
4. Sub-agents spawn verification
5. Commit before verification passes
6. Ignore failed checks

### Critical Violations
1. Committing code without verification
2. Sub-agent spawning verification agents
3. Committing with failed verification
4. Skipping required checks
5. Using integration theater (mocking own code)
6. Committing incomplete implementations (TODO, FIXME, stubs)

## Summary

**Verification Workflow:**
```
Implement → Report → Main Agent → Spawn Verification →
Run ALL Checks → PASS? → Commit + Push : Fix → Loop
```

**Mandatory Checks (per language):**
1. Incomplete implementation (FIRST)
2. Format
3. Lint
4. Type check
5. Tests (ALL pass)
6. Build
7. Security
8. Standards

**Key Principles:**
1. NO code committed without verification
2. ONLY Main Agent spawns verification agents
3. ALL checks must pass
4. Check incomplete implementations FIRST
5. Real tests over mocks
6. Fix ALL failures before commit
7. Include verification status in commits

---

_Version: 1.0 - Last Updated: 2026-02-27_
