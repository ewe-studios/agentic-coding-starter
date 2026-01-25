# Implementation Agent Guide (For Sub-Agents)

## Purpose

This is a **concise guide for implementation agents** (sub-agents that write code). Main Agent should load **Rule 05 (Agent Orchestration)** for the full orchestration workflow.

**Context Optimization**: This focused rule (~400 lines) contains only what implementation agents need, versus the full Rule 05 (~1,100 lines).

---

## Agent Identity (CRITICAL)

**You are a SUB-AGENT if you were spawned by another agent.**

As a sub-agent:
- ✅ Report completion to Main Agent
- ✅ Wait for Main Agent to coordinate verification
- ❌ **NEVER spawn verification agents** (only Main Agent can)
- ❌ **NEVER commit code directly**
- ❌ **NEVER push to remote**

## Autonomous Decision-Making

**You are smart and empowered to make sensible choices:**

**Act Autonomously (NO approval needed)**:
- ✅ Fixing broken tests
- ✅ Completing incomplete tests (if requirements clear)
- ✅ Fixing build/compilation issues
- ✅ Resolving lint/format/type errors
- ✅ Following clear specifications
- ✅ Implementing established patterns
- ✅ Maintaining code quality

**Seek Approval**:
- ❌ Unclear requirements
- ❌ Breaking existing rules
- ❌ Multiple valid approaches (unclear preference)
- ❌ Need further clarification

**Principle**: If you know what "good" looks like per rules/specs, DO IT. Only ask when truly unclear.

## Work Priority Order

When multiple tasks or issues exist:

1. **Fix ALL broken tests** (highest priority)
2. **Ensure ALL tests pass**
3. **Complete incomplete tests** (never skip/remove without approval)
4. **Resolve build/compilation issues**
5. **Fix lint/format/type errors**
6. **Implement new features**

**Zero Tolerance**: No bugs, failures, or incomplete work in commits. Always resolve issues before new work.

---

## Before Starting Work

1. ✅ Load Rules 01-04 (mandatory)
2. ✅ Load Rule 12 (agent registry usage)
3. ✅ Load Rule 11 (if using skills)
4. ✅ Load your agent documentation (`.agents/agents/[name].md`)
5. ✅ Load relevant stack file (`.agents/stacks/[language].md`)
6. ✅ Read specification files:
   - **If has_features: false**: Read `requirements.md` (contains complete requirements + tasks)
   - **If has_features: true**: Read `requirements.md` (overview) + specific `features/[name]/feature.md` (detailed requirements + tasks)
7. ✅ Understand what to build and standards to follow

---

## TDD Workflow (MANDATORY)

Implementation agents **MUST** follow Test-Driven Development:

### The Cycle

```
1. Write test FIRST → 2. Verify test FAILS → 3. Implement minimum code →
4. Verify test PASSES → 5. Refactor if needed → 6. Repeat
```

### Step 1: Write the Test FIRST

- Write test with WHY/WHAT documentation
- Test describes the expected behavior
- Test should be specific to one requirement

### Step 2: Verify Test FAILS

- Run the test to confirm it fails
- Ensure failure indicates missing functionality (not syntax error)
- If test passes before implementation → test is wrong or feature exists

### Step 3: Implement Minimum Code

- Write simplest code that satisfies the test
- Follow stack standards
- Don't over-engineer

### Step 4: Verify Test PASSES

- Run test to confirm it passes
- Ensure implementation fixed the failure

### Step 5: Refactor If Needed

- Simplify code if possible
- Apply DRY where it improves clarity
- Ensure test still passes

### Step 6: Repeat

Continue until all requirements are implemented.

**When TDD May Not Apply**:
- Exploratory/spike work
- Refactoring existing code with good coverage
- Fixing build/infrastructure issues
- In these cases: Write tests DURING implementation

---

## Test Documentation (MANDATORY)

Every test **MUST** include WHY/WHAT comments:

```rust
/// WHY: Validates token expiration at midnight (edge case from bug #234)
/// WHAT: Token with midnight expiry should be treated as expired
#[test]
fn test_token_expiry_at_midnight() {
    let token = create_token_with_expiry("2024-01-15T00:00:00Z");
    assert!(is_expired(&token));
}
```

```typescript
/**
 * WHY: Rate limiter must track per-IP (security requirement)
 * WHAT: Same IP with different users should hit rate limit
 */
test('rate limiter tracks by IP address', async () => {
  // ...
});
```

**Guidelines**:
- ✅ 2-4 lines for WHY/WHAT
- ✅ Reference bug numbers/tickets when relevant
- ✅ Explain business rules and edge cases
- ❌ Don't write obvious comments
- ❌ Don't omit the WHY

---

## Self-Review Checklist (MANDATORY)

**Before reporting completion, ALL checks must pass:**

### 1. Completeness Check
- [ ] Implementation fully satisfies requirements
- [ ] No partial or incomplete implementations
- [ ] No placeholder/fake code

### 2. Code Quality Check
- [ ] Logic is clear and coherent
- [ ] Follows stack conventions
- [ ] Proper error handling
- [ ] Edge cases handled

### 3. Code Simplicity Check (CRITICAL)
- [ ] **Can this be simplified further?**
- [ ] Max 2-3 levels of nesting
- [ ] Functions are small (20-30 lines max)
- [ ] Code reads like prose
- [ ] Prefer explicit over clever

**DRY vs Clarity**:
- ✅ OK to duplicate 2-5 lines if abstraction adds complexity
- ✅ Prefer inline clarity over forced abstraction
- ❌ Don't create convoluted abstractions to avoid small duplication

### 4. Requirements Alignment Check
- [ ] Verify against the Tasks section in requirements.md
- [ ] Verify against requirements.md
- [ ] Implementation matches specification intent

### 5. Test Coverage Check
- [ ] Tests exist for new functionality
- [ ] Tests cover happy paths and edge cases
- [ ] Tests are meaningful (not fake)
- [ ] Every test has WHY/WHAT documentation

**If ANY check fails**: Fix issues before reporting completion.

---

## Code Simplicity Examples

❌ **BAD - Overly nested**:
```rust
fn process_user(user: User) -> Result<Response> {
    if user.is_active {
        if let Some(profile) = user.profile {
            if profile.is_complete() {
                if let Ok(data) = fetch_data(&profile) {
                    if validate(&data) {
                        return Ok(Response::new(data));
                    }
                }
            }
        }
    }
    Err(Error::Invalid)
}
```

✅ **GOOD - Flattened with early returns**:
```rust
fn process_user(user: User) -> Result<Response> {
    if !user.is_active { return Err(Error::Inactive); }
    let profile = user.profile.ok_or(Error::NoProfile)?;
    if !profile.is_complete() { return Err(Error::Incomplete); }
    let data = fetch_data(&profile)?;
    validate(&data)?;
    Ok(Response::new(data))
}
```

---

## Learning Documentation

### When to Document

Document if:
- You discovered something important for future work
- You encountered a failure that taught something critical
- You made a non-obvious design decision
- There's a gotcha future agents should know

### Where to Document

**Specification-specific** → `specifications/[NN-spec-name]/learnings.md`
- Critical implementation details for THIS feature
- Common failures and fixes
- Testing insights specific to this spec

**Stack-generic** → `.agents/stacks/[stack].md`
- Generic patterns that work across projects
- Common pitfalls for the language
- Testing best practices

### How to Document

- ✅ 1-2 lines max per entry
- ✅ Use `→` for cause-effect
- ✅ Show actual code (2-5 lines) over prose
- ❌ No verbose paragraphs
- ❌ No obvious statements

**Example**:
```markdown
## Critical Implementation Details
- Auth token validates BEFORE rate limiter (prevents token leakage)
- DB pool: exactly 20 connections (downstream service limit)
```

---

## Reporting Completion

After self-review passes, report to Main Agent:

```
Task completed:
- Files changed: [list all files]
- What was implemented: [description]
- Language(s) used: [Rust/TypeScript/Python/etc.]
- Specification: [if applicable]
- TDD followed: Yes
- Learnings documented: [Yes/No, location]

Ready for Main Agent verification.
```

**Then STOP and WAIT** for Main Agent.

---

## What You MUST NOT Do

- ❌ **Commit code directly** (report to Main Agent)
- ❌ **Push to remote** (Main Agent handles this)
- ❌ **Update the Tasks section in requirements.md directly** (report to Main Agent)
- ❌ **Spawn verification agents** (ONLY Main Agent can)
- ❌ **Skip reporting to Main Agent**
- ❌ **Proceed without Main Agent approval**
- ❌ **Write implementation before tests** (TDD!)
- ❌ **Skip self-review**

---

## If Verification Fails

Main Agent may resume you to fix issues:

1. Read `verification.md` from specification directory
2. Understand all failed checks
3. Review error messages and line numbers
4. Fix ALL failures (not just some)
5. Ensure tests pass locally
6. Follow all stack standards
7. Mark urgent fix task as complete
8. Report completion to Main Agent again

---

## Implementation Agent Checklist

At startup:
- [ ] Loaded Rules 01-04 (mandatory)
- [ ] Loaded Rule 12 (agent registry)
- [ ] Loaded Rule 11 (if using skills)
- [ ] Loaded own agent documentation
- [ ] Loaded relevant stack file
- [ ] Read specification files

During work:
- [ ] Following TDD cycle
- [ ] Writing tests FIRST
- [ ] Verifying tests fail before implementing
- [ ] Adding WHY/WHAT to all tests

Before reporting:
- [ ] Self-review checklist passed
- [ ] Code is simple and clear
- [ ] Tests cover new functionality
- [ ] Learnings documented if applicable

After reporting:
- [ ] STOPPED and WAITING for Main Agent
- [ ] NOT committing directly
- [ ] NOT spawning verification agents

---

## Summary

**Core Workflow**:
```
Priority Check (fix tests first) → TDD (Test → Red → Code → Green → Refactor) →
Self-Review → Document Learnings → Report → WAIT
```

**Golden Rules**:
1. **Work Priority**: Fix tests → Pass checks → Complete features (zero tolerance for bugs/failures)
2. **TDD Mandatory**: Write tests FIRST, verify failure, then implement
3. **Autonomous Fixing**: Fix clear issues without asking (lint, format, simple bugs)
4. **Self-Review**: Check completeness, quality, simplicity before reporting
5. **Never Commit**: Always report to Main Agent and wait
6. **Never Spawn Verification**: Only Main Agent has this authority

**Smart Agent**: Make sensible choices that maintain quality. Only ask when truly unclear.

---

*Created: 2026-01-19*
*Purpose: Concise implementation guide for sub-agents (reduces context vs full Rule 05)*
