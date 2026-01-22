# Failed Verification Workflow Example

This example shows what happens when verification fails, demonstrating the fix cycle and iterative improvement process.

## Purpose

Demonstrates the proper handling of verification failures, including urgent task creation, fix implementation, and re-verification until success.

## Complete Workflow (With Failure and Recovery)

```
1. User: "Add data validation to API endpoints"

2. Main Agent:
   - Spawns Python Implementation Agent
   - Provides specification path
   - Waits for completion report

3. Implementation Agent (SUB-AGENT):
   - Reads specification and requirements
   - Implements validation logic
   - Writes tests (but misses edge cases)
   - Performs self-review (appears complete)
   - Reports completion to Main Agent
   - DOES NOT commit

4. Main Agent:
   - Receives completion report
   - Spawns Python Verification Agent
   - Waits for verification results

5. Verification Agent (SUB-AGENT):
   - Runs all verification checks:
     * pytest → FAIL ❌ (2 test failures)
     * ruff check → FAIL ❌ (3 lint errors)
     * black --check → PASS ✅
     * Build → PASS ✅
   - Creates detailed report of failures
   - Reports FAIL ❌ to Main Agent
   - DOES NOT commit (failures present)

6. Main Agent (CRITICAL DECISION POINT):
   - Receives FAIL verification
   - DOES NOT COMMIT (iron-clad rule)
   - Spawns Specification Update Agent
   - Instructs to create verification.md with full report

7. Specification Agent (SUB-AGENT):
   - Creates verification.md with:
     * Failed checks listed
     * Error messages included
     * Line numbers and specifics
     * Recommendations for fixes
   - Adds urgent task to tasks.md:
     * [ ] URGENT: Fix validation test failures (2 tests)
     * [ ] URGENT: Fix ruff lint errors (3 errors)
   - Updates frontmatter (uncompleted count increased)
   - Reports completion to Main Agent

8. Main Agent:
   - Reports failure details to user
   - Waits for user decision or auto-spawns fix agent

9. User (or Main Agent auto-decision):
   - "Please fix the issues"

10. Main Agent:
    - Spawns Implementation Agent with fix context
    - Provides verification.md path
    - Provides tasks.md urgent tasks

11. Implementation Agent (FIX CYCLE):
    - Reads verification.md thoroughly
    - Identifies root causes:
      * Missing edge case tests for empty strings
      * Unused imports causing lint errors
    - Fixes all issues:
      * Adds edge case tests
      * Removes unused imports
      * Runs tests locally (all pass)
    - Marks urgent tasks complete in tasks.md
    - Reports fix completion to Main Agent

12. Main Agent:
    - Spawns Verification Agent AGAIN
    - Same checks, fresh run

13. Verification Agent (RE-VERIFICATION):
    - Runs all checks:
      * pytest → PASS ✅ (all tests passing)
      * ruff check → PASS ✅ (zero errors)
      * black --check → PASS ✅
      * Build → PASS ✅
    - All checks PASS ✅
    - Reports success to Main Agent

14. Main Agent:
    - Receives PASS verification
    - Spawns Specification Agent
    - Instructs to update tasks.md and delete verification.md

15. Specification Agent:
    - Marks original tasks complete
    - Removes urgent tasks (resolved)
    - Deletes verification.md (no longer needed)
    - Updates frontmatter (100% complete)
    - Reports completion to Main Agent

16. Main Agent:
    - Creates commit with:
      * Implementation changes
      * Fix changes
      * Updated tasks.md
      * Verification PASS status
    - Pushes to remote immediately
    - Reports success to user

17. User:
    - Receives working, verified code
    - Learns about issues caught and fixed
```

## Key Failure Handling Factors

**Why Failure Was Caught**:
- ✅ Verification ran before commit (mandatory gate)
- ✅ All checks executed (not just build)
- ✅ Detailed failure report created
- ✅ No code committed with failures

**Proper Recovery Process**:
- ✅ Main Agent did NOT commit failed code
- ✅ Verification report documented all issues
- ✅ Urgent tasks created for tracking
- ✅ Fix agent received full context
- ✅ Re-verification confirmed fixes
- ✅ Only committed after PASS

**Agent Authority Maintained**:
- ✅ SUB-AGENTS never committed (even after fixes)
- ✅ MAIN AGENT orchestrated entire fix cycle
- ✅ Proper escalation to Main Agent
- ✅ No shortcuts taken

## Workflow Diagram

```
User Request
    ↓
Main Agent → Implementation Agent
    ↓
Report (Complete)
    ↓
Main Agent → Verification Agent
    ↓
FAIL ❌ (2 tests, 3 lint errors)
    ↓
Report to Main (NO COMMIT!)
    ↓
Main Agent → Specification Agent
    ↓
Create verification.md + urgent tasks
    ↓
Report to Main → Report to User
    ↓
Main Agent → Implementation Agent (with verification.md)
    ↓
Fix all issues + mark urgent tasks done
    ↓
Report to Main
    ↓
Main Agent → Verification Agent (RE-RUN)
    ↓
PASS ✅ (all checks passing)
    ↓
Report to Main
    ↓
Main Agent → Specification Agent
    ↓
Update tasks, delete verification.md
    ↓
Main Agent → COMMIT + PUSH
    ↓
Success! ✅
```

## What Would Have Happened Without Verification

**WITHOUT verification workflow** (❌ BAD):
```
Implementation → Commit → Push → User discovers broken tests → Frustration
```

**Result**:
- ❌ Broken code in repository
- ❌ Tests failing for team
- ❌ Lint errors in codebase
- ❌ User loses trust
- ❌ Wasted time debugging later

**WITH verification workflow** (✅ GOOD):
```
Implementation → Verify → Fail → Fix → Verify → Pass → Commit → Push
```

**Result**:
- ✅ Only working code in repository
- ✅ All tests passing
- ✅ Clean, linted code
- ✅ User receives quality
- ✅ Issues caught early

## Time and Quality Impact

**Initial Cost**:
- Extra verification cycle time (5-10 minutes)
- Fix cycle time (10-20 minutes)
- Total: 15-30 minutes additional

**Value Gained**:
- Prevented broken build for entire team
- Prevented hours of debugging later
- Maintained code quality standards
- Preserved user trust
- **Net positive**: Saved hours of future pain

## Best Practices Demonstrated

1. **Verification as Gate**: No commit without PASS
2. **Detailed Reporting**: verification.md provided full context
3. **Urgent Task Tracking**: Issues tracked in tasks.md
4. **Fix Context**: Implementation agent received all details
5. **Re-Verification**: Confirmed fixes before commit
6. **Cleanup**: Removed verification.md after success
7. **No Shortcuts**: Followed full process despite failure

## Common Pitfalls Avoided

**Temptations to avoid**:
- ❌ "Just one small lint error, commit anyway" - NO!
- ❌ "Tests are flaky, ignore them" - NO!
- ❌ "Fix it later in next PR" - NO!
- ❌ "Skip re-verification, the fix looks good" - NO!

**Correct approach**:
- ✅ Fix ALL issues before commit
- ✅ Re-verify after every fix
- ✅ Only commit after 100% PASS
- ✅ Maintain zero tolerance policy

## Related Workflows

- See `workflow-success-example.md` for ideal path without failures
- See Rule 05 for complete agent orchestration guidelines
- See Rule 08 for detailed verification requirements
- See verification agent docs for check specifics

---

*Created: 2026-01-22*
*Referenced in: Rule 05, section "Complete Workflow Example"*
