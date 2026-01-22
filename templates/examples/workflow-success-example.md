# Successful Workflow Example

This example shows the complete successful workflow from user request through implementation, verification, and commit.

## Purpose

Demonstrates the ideal path where implementation is correct, verification passes on first attempt, and code is committed successfully.

## Complete Workflow

```
1. User: "Implement user authentication in Rust"

2. Main Agent:
   - Spawns Rust Implementation Agent with spec context
   - Provides path to specification
   - Waits for completion report

3. Implementation Agent (SUB-AGENT):
   - Reads AGENTS.md and all required rules
   - Reads specification (requirements.md, tasks.md)
   - Implements using TDD workflow:
     * Write test first (RED)
     * Implement code to pass test (GREEN)
     * Refactor for quality (REFACTOR)
   - Performs self-review checklist
   - Documents learnings in learnings.md
   - Reports completion to Main Agent
   - DOES NOT commit (no authority)

4. Main Agent:
   - Receives completion report
   - Spawns ONE Rust Verification Agent
   - Provides specification path
   - Waits for verification results

5. Verification Agent (SUB-AGENT):
   - Reads specification context
   - Runs all verification checks:
     * cargo build (compilation)
     * cargo test (all tests pass)
     * cargo clippy (zero warnings)
     * cargo fmt --check (formatting)
   - All checks PASS ✅
   - Reports success to Main Agent
   - DOES NOT commit (no authority)

6. Main Agent:
   - Receives PASS verification
   - Spawns Specification Update Agent
   - Instructs to mark tasks complete

7. Specification Agent (SUB-AGENT):
   - Updates tasks.md:
     * Marks relevant tasks [x]
     * Updates frontmatter counts
     * Updates completion_percentage
   - Reports completion to Main Agent

8. Main Agent:
   - Reviews all reports
   - Creates commit with:
     * Implementation changes
     * Updated tasks.md
     * Verification status in message
   - Pushes to remote immediately
   - Reports success to user

9. User:
   - Receives confirmation
   - Work is complete and verified
```

## Key Success Factors

**Why This Worked**:
- ✅ Implementation agent followed TDD workflow
- ✅ All tests written and passing before verification
- ✅ Self-review caught issues early
- ✅ Code quality maintained (no clippy warnings)
- ✅ Proper agent hierarchy respected
- ✅ Verification ran before commit
- ✅ Specification updated accurately

**Agent Authority Respected**:
- ✅ SUB-AGENTS never spawned verification agents
- ✅ SUB-AGENTS never committed directly
- ✅ MAIN AGENT orchestrated all phases
- ✅ Each agent knew its role and responsibilities

**Quality Gates Passed**:
- ✅ Build: Success
- ✅ Tests: 100% passing
- ✅ Linter: Zero warnings
- ✅ Formatter: Clean
- ✅ Self-review: Complete

## Time Saved

**Because verification passed on first attempt**:
- No fix cycle needed
- No re-verification needed
- No debugging time wasted
- User receives working code immediately
- Trust in process maintained

## Workflow Diagram

```
User Request
    ↓
Main Agent (orchestrate)
    ↓
Implementation Agent (TDD + Self-Review + Document)
    ↓
Report to Main
    ↓
Main Agent (spawn verification)
    ↓
Verification Agent (all checks PASS ✅)
    ↓
Report to Main
    ↓
Main Agent (spawn spec update)
    ↓
Specification Agent (mark tasks complete)
    ↓
Report to Main
    ↓
Main Agent (commit + push)
    ↓
Success! ✅
```

## Best Practices Demonstrated

1. **TDD Workflow**: Tests written first, code follows
2. **Self-Review**: Caught issues before verification
3. **Documentation**: Learnings captured during implementation
4. **Agent Hierarchy**: Proper authority respected throughout
5. **Verification Gate**: No commit without passing checks
6. **Specification Tracking**: Tasks updated based on reality
7. **Immediate Push**: Changes available to team right away

## Related Workflows

- See `workflow-failure-example.md` for what happens when verification fails
- See `workflow-fix-cycle-example.md` for the iterative fix process
- See Rule 05 for complete agent orchestration guidelines
- See Rule 08 for detailed verification requirements

---

*Created: 2026-01-22*
*Referenced in: Rule 05, section "Complete Workflow Example"*
