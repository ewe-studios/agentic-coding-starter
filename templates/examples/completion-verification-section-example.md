# Mandatory Completion and Verification Section

This example shows the complete "MANDATORY Completion and Verification Requirements" section that **MUST** be included in every `requirements.md` file.

## Purpose

This section enforces 100% completion with zero tolerance for "optional" tasks or partial completion. It ensures every specification is truly done before being marked complete.

## Complete Section Template

Copy this section verbatim into every `requirements.md` file:

```markdown
## MANDATORY Completion and Verification Requirements

**CRITICAL**: Before marking this specification as complete, ALL of the following MUST be verified:

### 1. Task Completion Verification (100% REQUIRED)

**NO EXCEPTIONS**: Every task in `tasks.md` MUST be completed.

- [ ] Open `tasks.md` and verify ALL tasks are marked `[x]`
- [ ] Verify `completed` count in frontmatter matches actual `[x]` count
- [ ] Verify `uncompleted` count is `0`
- [ ] Verify `completion_percentage` is `100`
- [ ] NO tasks left as `[ ]` (incomplete)
- [ ] NO optional tasks - everything is mandatory unless user explicitly says otherwise

**Validation Command**:
\`\`\`bash
# Must return 0
grep -c "^- \[ \]" tasks.md
\`\`\`

### 2. Code/Implementation Verification (100% REQUIRED)

For each task in `tasks.md`:
- [ ] Verify the code/file actually exists in the codebase
- [ ] Verify the implementation matches the task description
- [ ] Verify all tests for that component pass
- [ ] NO placeholder implementations
- [ ] NO commented-out code marked as "TODO"

### 3. Documentation Verification (100% REQUIRED - NO OPTIONAL)

**If has_fundamentals: true**:
- [ ] ALL fundamental documents listed in tasks.md exist
- [ ] Each fundamental doc is comprehensive (not stub/placeholder)
- [ ] Code examples in docs compile and work
- [ ] Cross-references between docs are valid

**Always Required**:
- [ ] `learnings.md` created with implementation insights
- [ ] `progress.md` created with timeline and status
- [ ] `verification.md` or `VERIFICATION_SIGNOFF.md` created

### 4. Quality Verification (100% REQUIRED - ZERO TOLERANCE)

**Build and Test**:
- [ ] Build succeeds with 0 errors
- [ ] Tests show 100% passing
- [ ] NO ignored or skipped tests (unless user-approved)

**Code Quality** (language-specific):
- [ ] Linter shows 0 warnings (cargo clippy, eslint, etc.)
- [ ] Code formatter applied and clean
- [ ] NO warnings suppressed without justification

**Documentation Quality**:
- [ ] All public APIs documented
- [ ] Documentation builds without errors
- [ ] NO broken links

### 5. Specification Tracking Verification (MANDATORY)

- [ ] `tasks.md` shows 100% completion
- [ ] `learnings.md` exists and is comprehensive
- [ ] `progress.md` exists with timeline
- [ ] `verification.md` exists with results
- [ ] `requirements.md` frontmatter status correct

### 6. Verification Issue Resolution (MANDATORY)

**NO OPTIONAL FIXES**: All verification issues MUST be resolved.

- [ ] Check `verification.md` for FAILED or WARNING items
- [ ] ALL failed checks fixed (no exceptions)
- [ ] ALL warnings addressed or user-accepted
- [ ] Re-run verification to confirm PASS
- [ ] Update `verification.md` with final PASS status

**If ANY failures exist**:
1. ❌ DO NOT mark specification complete
2. ❌ DO NOT mark tasks done
3. ✅ FIX all issues
4. ✅ Re-run verification
5. ✅ Only complete after 100% PASS
```

## When to Use This Section

**REQUIRED for**:
- ✅ Every new specification
- ✅ Every requirements.md file
- ✅ Both simple and feature-based specifications
- ✅ All specification updates/revisions

**Location in requirements.md**:
- Near the end of the document
- Before final notes/version history
- After all requirements and success criteria
- Before the bottom cross-reference link

## Why This Section Is Critical

**Prevents Common Issues**:
- ❌ Marking specifications "complete" with unfinished tasks
- ❌ Ignoring test failures as "minor issues"
- ❌ Skipping documentation because "code is self-documenting"
- ❌ Accepting linter warnings as "acceptable"
- ❌ Forgetting to create mandatory files

**Enforces Quality Standards**:
- ✅ 100% task completion (no partial work)
- ✅ All tests passing (no broken features)
- ✅ Zero linter warnings (clean code)
- ✅ Complete documentation (professional quality)
- ✅ Full verification sign-off (quality assurance)

**User Impact**:
- User trusts "completed" status is accurate
- User knows all work is truly done
- User receives fully functional implementation
- User gets comprehensive documentation
- User avoids surprises from "forgotten" tasks

## Zero Tolerance Enforcement

These violations have **ZERO TOLERANCE**:

❌ **Marking complete with unchecked tasks**
- Even "just one small task"
- Even "optional" tasks (there are no optional tasks)
- Even "we can do it later"

❌ **Ignoring verification failures**
- "The test is flaky" - fix the test
- "Clippy is too strict" - fix the warnings
- "Documentation can wait" - write it now

❌ **Missing mandatory files**
- "We don't need learnings.md" - yes you do
- "Progress.md seems redundant" - still required
- "Verification passed verbally" - write VERIFICATION_SIGNOFF.md

## Validation Script Example

Main Agent can use this to validate before marking complete:

```bash
#!/bin/bash
# Specification completion validator

SPEC_DIR="specifications/01-example-spec"

echo "Validating specification completion..."

# 1. Check uncompleted tasks
UNCOMPLETED=$(cd "$SPEC_DIR" && grep -c "^- \[ \]" tasks.md)
if [ "$UNCOMPLETED" -ne 0 ]; then
  echo "❌ FAIL: $UNCOMPLETED tasks still uncompleted"
  exit 1
fi

# 2. Check mandatory files exist
for file in tasks.md learnings.md progress.md verification.md requirements.md; do
  if [ ! -f "$SPEC_DIR/$file" ]; then
    echo "❌ FAIL: Missing $file"
    exit 1
  fi
done

# 3. Check build and tests
if ! cargo build 2>&1 | grep -q "Finished"; then
  echo "❌ FAIL: Build failed"
  exit 1
fi

if ! cargo test 2>&1 | grep -q "test result: ok"; then
  echo "❌ FAIL: Tests failed"
  exit 1
fi

# 4. Check linter
if ! cargo clippy -- -D warnings 2>&1 | grep -q "Finished"; then
  echo "❌ FAIL: Clippy warnings present"
  exit 1
fi

echo "✅ PASS: All validation checks passed"
```

---

*Created: 2026-01-22*
*Referenced in: Rule 06, section "Mandatory 100% Completion Verification"*
