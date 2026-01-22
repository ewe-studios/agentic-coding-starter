# Git Workflow Examples

This document provides examples of proper git commit and push practices for specification work.

## Overview

To ensure no work is lost and maintain safety, follow these git practices during specification implementation.

## 1. Atomic Commits During Implementation

### When to Commit

**MUST commit and push frequently** during work:
- ✅ After completing each major task or primitive
- ✅ After each logical unit of work (file, module, feature)
- ✅ After tests pass for that unit
- ✅ Every 30-60 minutes of active work
- ✅ Before taking breaks or ending sessions

### Atomic Commit Guidelines

- Each commit should be self-contained and buildable
- Commit message explains what was added/changed
- Include "WIP:" prefix if work is incomplete
- Push immediately after each commit

### Example: Atomic Commit

```bash
# Add a specific component
git add src/primitives/spin_mutex.rs

# Commit with clear description
git commit -m "feat(primitives): add SpinMutex with poisoning support

Implements a spinlock mutex with lock poisoning detection.
Includes tests for concurrent access patterns."

# Push immediately
git push origin feature/concurrency-primitives
```

## 2. Final Commit After Completion

### When to Create Final Commit

**AFTER 100% completion and verification PASS**:

1. ✅ Verify all checks pass (tasks 100%, tests 100%, clippy 0 warnings)
2. ✅ Create final commit with all remaining changes
3. ✅ Push to remote IMMEDIATELY - DO NOT DELAY
4. ✅ Verify push succeeded

### Final Commit Requirements

```bash
# Step 1: Verify everything is complete
grep -c "^- \[ \]" tasks.md  # Must return 0
cargo test  # Must show 100% passing
cargo clippy -- -D warnings  # Must show 0 warnings

# Step 2: Stage all specification files
git add specifications/01-build-http-client/
git add src/http/
git add tests/http/

# Step 3: Commit with comprehensive message
git commit -m "feat: complete HTTP client implementation

Implemented full-featured RESTful HTTP client with:
- Connection pooling and reuse
- TLS support with certificate validation
- Request/response builders with fluent API
- Comprehensive error handling
- Full test coverage

Verification:
- Tasks: 100% (47/47)
- Tests: 100% passing (124 tests)
- Quality: 0 warnings
- Documentation: Complete with fundamentals

Status: VERIFIED - PRODUCTION READY"

# Step 4: Push IMMEDIATELY
git push origin feature/http-client

# Step 5: Verify push succeeded
git log origin/feature/http-client --oneline -1
```

## 3. Safety Rules

### MUST Push Immediately

- ✅ After marking specification complete
- ✅ After verification passes
- ✅ After all tasks show 100%
- ✅ After fixing all verification issues

### Why This is Critical

- **Prevents work loss**: System failures won't lose completed work
- **Remote backup**: Changes are safely stored remotely
- **Team visibility**: Others can see progress
- **Audit trail**: Clear record of when work completed

### Zero Tolerance

- ❌ DO NOT delay pushing after completion
- ❌ DO NOT "batch" pushes across specifications
- ❌ DO NOT leave completed work unpushed
- ❌ DO NOT forget to push after final commit

## 4. Work-in-Progress Commits

### WIP Prefix Convention

Use "WIP:" prefix for incomplete work:

```bash
git commit -m "WIP: implement connection pooling

Partial implementation of connection pool manager.
Still need to add:
- Connection reaping
- Health checks
- Metrics
"
git push origin feature/http-client
```

### When to Use WIP

- Work is partially complete
- Tests not yet passing
- Taking a break but want to save progress
- End of work session with incomplete task

## 5. Multi-File Commits

### Grouping Related Changes

```bash
# Group related changes in single commit
git add src/http/client.rs
git add src/http/connection.rs
git add src/http/pool.rs
git add tests/http/client_tests.rs

git commit -m "feat(http): add connection pooling

Implements connection pool with:
- Automatic connection reuse
- Configurable pool size
- Idle connection timeout
- Connection health checks

Tests verify concurrent access and connection lifecycle."

git push origin feature/http-client
```

## 6. Specification File Commits

### Updating Specification Files

```bash
# Update task completion
git add specifications/01-http-client/tasks.md

git commit -m "docs(spec): mark connection pooling tasks complete

Updated tasks:
- [x] Implement ConnectionPool struct
- [x] Add connection reaping
- [x] Add health checks
- [x] Write pool tests

Tasks: 35/47 complete (74%)"

git push origin feature/http-client
```

## Common Mistakes to Avoid

### ❌ DON'T: Forget to Push

```bash
git commit -m "feat: complete feature"
# Forgot to push - work could be lost!
```

### ✅ DO: Always Push Immediately

```bash
git commit -m "feat: complete feature"
git push origin feature-branch  # Always push!
```

### ❌ DON'T: Batch Multiple Unrelated Changes

```bash
# Bad: mixing multiple unrelated changes
git add src/http/ src/database/ src/cache/
git commit -m "various changes"
```

### ✅ DO: Separate Commits for Separate Concerns

```bash
# Good: atomic commits
git add src/http/
git commit -m "feat(http): add client"
git push origin feature-branch

git add src/database/
git commit -m "feat(db): add connection pooling"
git push origin feature-branch
```

## Summary

**Key Principles**:
1. Commit frequently (every logical unit of work)
2. Push immediately after every commit
3. Use clear, descriptive commit messages
4. Create comprehensive final commit when 100% complete
5. Never leave completed work unpushed
