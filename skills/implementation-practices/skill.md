---
name: "Implementation Practices"
description: "Complete implementation workflow: ONE item at a time, retrieval-led reasoning, TDD, self-review"
approved: Yes
created: 2026-02-27
license: "MIT"
metadata:
  author: "Main Agent"
  version: "3.0"
  last_updated: "2026-02-28"
  tags: [implementation, workflow, tdd, self-review, retrieval, reasoning, one-at-a-time]
tools: []
files: []
---

# Implementation Practices

## Read By

1. **Implementation Agent** reads this skill
2. Referenced by `.agents/agents/implementation.md`

## Overview

Complete guide for implementation agents with **MANDATORY one-item-at-a-time workflow**.

**Usage Type**: EDUCATIONAL - Learn implementation best practices.

## CRITICAL: Work ONE Item at a Time

❌ **NEVER do this:**
- Generate multiple functions/methods at once
- Write multiple tests simultaneously
- Create entire files with all functionality
- Work on several tasks in parallel

✅ **ALWAYS do this:**
1. **ONE test** at a time - write, verify fail, implement, verify pass
2. **ONE function** at a time - implement completely, test, move to next
3. **ONE file** at a time - finish one file before starting another
4. **ONE task** at a time - complete current task before next

**Why ONE at a time:**
- ✅ Stay focused and avoid mistakes
- ✅ Catch issues immediately
- ✅ Incremental verified progress
- ✅ Easy to debug when something breaks
- ✅ Clear rollback points

## Agent Identity (CRITICAL)

**You are SUB-AGENT if spawned by another agent.**

As sub-agent:
- ✅ Report completion to Main Agent
- ✅ Wait for Main Agent to coordinate verification
- ✅ Work ONE item at a time
- ❌ NEVER spawn verification agents (only Main Agent can)
- ❌ NEVER commit code directly
- ❌ NEVER push to remote
- ❌ NEVER generate multiple items at once

## Retrieval-Led Reasoning (MANDATORY)

**MUST follow retrieval-led reasoning, NOT pretraining-led reasoning.**

### Do This (Retrieval-Led)

1. Read codebase FIRST before assumptions
2. Use Grep/Glob/Read tools to understand existing patterns
3. Follow project-specific conventions found in code
4. Trust project rules over general best practices
5. Search for similar implementations as reference
6. Read stack files and learnings for project context
7. Verify assumptions by reading actual code

### Don't Do This (Pretraining-Led - FORBIDDEN)

- Making assumptions based on "typical" patterns
- Implementing without checking existing code
- Applying generic best practices without context
- Assuming file structures or naming conventions
- Guessing at project patterns without verification

**Why**: Every codebase has unique patterns. Reading actual code reveals true structure. Assumptions lead to inconsistent implementations.

**Enforcement**: Before implementation, MUST demonstrate retrieval by searching, reading, checking conventions.

## Before Starting Work

1. Read AGENTS.md (agent registry)
2. Read skills-management skill (if using skills)
3. Load your agent documentation (`.agents/agents/[name].md`)
4. Load relevant language skills (`.agents/skills/[language]-clean-code/skill.md`)
5. Read specification requirements and features
6. Parse FILES section and read ONLY listed files
7. Begin work with clean, minimal context

### Specification clarity and detail (CRITICAL)

When you write a specification, review it or are about to start it, always ask yourself:

1. Does this spec have hidden details not clarified in the spec or feature
2. Does this spec or feature make lots of assumptions on how some entity or behaviour functions without specifying the boundaries of its behaviour and execution or functionality
3. Does this spec or feature leave alot out for assumption creating potential areas for mis-interpretation or wrong execution
4. Can we make things more clear and more detailed to ensure the full scope, behaviours, boundaries, limitations is clearly articulated and written down.

If any of this is true, you should indicate to the user, pause and ask more questions to get everything clarified, documented and updated.

Update does not mean going to delete stuff unless its clear from the conversation that is what the user expects after clarification and discussion so be thoughtful, logical and surgical.

## Autonomous Decision-Making

**Act Autonomously (NO approval needed):**
- Fixing broken tests
- Completing incomplete tests (if requirements clear)
- Fixing build/compilation issues
- Resolving lint/format/type errors
- Following clear specifications
- Implementing established patterns
- Maintaining code quality

**Seek Approval:**
- Unclear requirements
- Breaking existing rules
- Multiple valid approaches
- Need further clarification

**Principle**: If you know what "good" looks like per rules/specs, DO IT. Only ask when truly unclear.

## Work Priority Order (MANDATORY - ONE at a TIME)

**Process items in this order, ONE at a time:**

1. Fix ONE broken test (highest priority)
2. When that test passes, fix NEXT broken test
3. Complete ONE incomplete test
4. Resolve ONE build/compilation issue
5. Fix ONE lint/format/type error
6. Implement ONE new feature/function

**Zero Tolerance**: No bugs, failures, or incomplete work in commits.

**Example Correct Workflow:**
```
1. Write test_function_a
2. Verify it fails
3. Implement function_a
4. Verify test_function_a passes
5. NOW write test_function_b (not before!)
6. Verify it fails
7. Implement function_b
8. Verify test_function_b passes
9. Continue...
```

## TDD Workflow (MANDATORY - ONE Test at a Time)

**See**: `.agents/skills/test-driven-development/skill.md`

**ONE Test at a Time:**
1. Write ONE test FIRST (before implementation)
2. Verify that ONE test FAILS
3. Implement minimum code to pass THAT ONE test
4. Verify THAT test PASSES
5. Refactor if needed
6. Move to NEXT test
7. Repeat

**Test Documentation**: Every test MUST have WHY/WHAT comments.

## Self-Review Checklist (MANDATORY)

**Before reporting completion, ALL checks must pass:**

### 1. Completeness Check
- Implementation fully satisfies requirements
- No partial/incomplete implementations
- No placeholder/fake code

### 2. Code Quality Check
- Logic is clear and coherent
- Follows stack conventions
- Proper error handling
- Edge cases handled

### 3. Code Simplicity Check (CRITICAL)
- Can this be simplified further?
- Max 2-3 levels of nesting
- Functions are small (20-30 lines max)
- Code reads like prose
- Prefer explicit over clever

**DRY vs Clarity:**
- OK to duplicate 2-5 lines if abstraction adds complexity
- Prefer inline clarity over forced abstraction
- Don't create convoluted abstractions to avoid small duplication

### 4. Requirements Alignment Check
- Verify against Tasks section in requirements.md
- Implementation matches specification intent

### 5. Test Coverage Check
- Tests exist for new functionality
- Tests cover happy paths and edge cases
- Tests are meaningful (not fake)
- Every test has WHY/WHAT documentation

**If ANY check fails**: Fix issues before reporting completion.

## Code Simplicity Example

❌ **Bad - Overly nested:**
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

✅ **Good - Flattened with early returns:**
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

## Learning Documentation

**See**: `.agents/skills/learning-documentation/skill.md`

**When to Document:**
- Discovered something important for future work
- Encountered failure that taught something critical
- Made non-obvious design decision
- There's a gotcha future agents should know

**Where to Document:**
- **Specification-specific** → `specifications/[spec]/LEARNINGS.md`
- **Language-generic** → Relevant language skill file (if applicable)

**How to Document:**
- 1-2 lines max per entry
- Use `→` for cause-effect
- Show actual code (2-5 lines) over prose
- No verbose paragraphs

## Reporting Completion

After self-review passes, report to Main Agent:

```
Task completed:
- Files changed: [list all files]
- Modules affected: [list modules needing doc updates]
- What implemented: [description]
- Language(s): [Rust/TypeScript/Python/etc.]
- Specification: [if applicable]
- TDD followed: Yes
- Learnings documented: [Yes/No, location]

Ready for Main Agent verification.
```

**Then STOP and WAIT** for Main Agent.

## What You MUST NOT Do

- Commit code directly
- Push to remote
- Update Tasks section in requirements.md directly
- Spawn verification agents (ONLY Main Agent can)
- Skip reporting to Main Agent
- Proceed without Main Agent approval
- Write implementation before tests (TDD!)
- Skip self-review

## If Verification Fails

Main Agent may resume you to fix issues:
1. Read `verification.md` from specification directory
2. Understand all failed checks
3. Fix ALL failures (not just some)
4. Ensure tests pass locally
5. Follow all stack standards
6. Report completion to Main Agent again

## Complete Workflow

```
1. Spawned by Main Agent
   ↓
2. Read agent documentation (.agents/agents/implementation.md)
   ↓
3. Read specification requirements and features
   ↓
4. Read language skills (.agents/skills/[language]-clean-code/skill.md)
   ↓
5. Retrieval-led reasoning (search, read existing code)
   ↓
6. TDD workflow (test first, verify fail, implement, verify pass)
   ↓
7. Self-review (completeness, quality, simplicity, alignment, coverage)
   ↓
8. Document learnings (if any)
   ↓
9. Report completion to Main Agent
   ↓
10. STOP and WAIT for Main Agent
   ↓
11. Main Agent spawns verification
   ↓
12. If PASS: Main Agent commits
13. If FAIL: Main Agent resumes you with fix requirements
```

## Common Patterns

### Pattern: Starting New Task

```
1. Receive task specification from Main Agent
2. Read specification requirements and feature files
3. Read files listed in specification
4. Search for similar implementations (retrieval-led)
5. Read existing patterns and conventions
6. Write test first (TDD)
7. Implement, verify, refactor
8. Self-review
9. Report completion
10. Wait for Main Agent
```

### Pattern: Fixing Verification Failures

```
1. Main Agent resumes with failure details
2. Read verification.md from spec directory
3. Understand ALL failed checks
4. Fix each failure:
   - Format issues → run formatter
   - Lint issues → fix warnings
   - Test failures → fix tests
   - Build issues → resolve compilation
5. Verify ALL checks pass locally
6. Report completion again
7. Wait for Main Agent to re-verify
```

## Summary

**Implementation Checklist:**
1. ✅ Read agent documentation
2. ✅ Read specification requirements and features
3. ✅ Use retrieval-led reasoning (read code first)
4. ✅ Follow TDD (test first)
5. ✅ Prioritize work (fix tests first)
6. ✅ Self-review (5 checks)
7. ✅ Document learnings
8. ✅ Report completion
9. ✅ WAIT for Main Agent

**Key Principles:**
1. Retrieval-led reasoning (read code first, NOT assumptions)
2. TDD mandatory (test first)
3. Autonomous fixing (if clear, just do it)
4. Self-review before reporting
5. Never commit (always report to Main Agent)
6. Never spawn verification (only Main Agent can)

**Smart Agent**: Make sensible choices that maintain quality. Read code to understand patterns. Only ask when truly unclear.

---

_Version: 3.0 - Last Updated: 2026-02-28_
