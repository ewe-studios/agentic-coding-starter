# Proposed Rule Updates from Spec 04 Learnings

**Date**: 2026-01-23
**Source**: specifications/04-condvar-primitives/PROCESS_LEARNINGS.md
**Status**: AWAITING APPROVAL

## Summary

Based on Specification 04 execution, we identified 6 key process improvements that require updates to:
1. **Rule 06** (Specifications and Requirements) - 4 new sections
2. **Rule 13** (Implementation Agent Guide) - 1 brief addition
3. **Rule 05** (Agent Orchestration) - 1 brief addition
4. **Templates** - tasks-template.md and requirements-template.md updates

---

## 1. Rule 06 Updates (Specifications and Requirements)

### Location: After "Agent Responsibilities During Implementation" section (around line 1100)

#### New Section 1: Tasks.md Update Protocol (MANDATORY)

```markdown
## Tasks.md Update Protocol (MANDATORY)

**CRITICAL**: This section applies to ALL agents working on specifications, not just Main Agent.

### Update Frequency Requirement

Tasks.md MUST be updated immediately after completing EACH meaningful unit of work, not batched at the end.

**Update Trigger Points**:
- ✅ After implementing each feature or component
- ✅ After fixing each issue (clippy errors, test failures, bugs)
- ✅ After completing each architectural decision
- ✅ After each commit that represents measurable progress
- ✅ After each phase/step completion
- ✅ Minimum: Every 1-2 completed subtasks

**Update Process** (agents MUST follow):
1. Open tasks.md file
2. Locate relevant task checkbox
3. Change `- [ ]` to `- [x]` immediately
4. Add completion notes in parentheses: `(✅ 2026-01-23: details)`
5. Update frontmatter counts:
   - Increment `completed` field
   - Decrement `uncompleted` field
   - Recalculate `completion_percentage`: (completed / total_tasks * 100)
6. Commit tasks.md separately with descriptive message
7. Push immediately (as per Rule 04)
8. Continue with next task

**Example Task Update**:

```markdown
### Step 1: RwLockCondVar Implementation
- [x] Implement wait_read() with poisoning support (✅ 2026-01-23: 8 tests passing)
- [x] Implement wait_write() with poisoning support (✅ 2026-01-23: 0 warnings)
- [ ] Implement wait_timeout_read() (NEXT)
```

**Frontmatter Update Example**:

```yaml
---
completed: 67  # Was 65, increment by 2
uncompleted: 82  # Was 84, decrement by 2
total_tasks: 149
completion_percentage: 45.0  # Was 43.6, recalculated
---
```

**Anti-Pattern** ❌: Completing 50+ tasks and updating all at once
**Best Practice** ✅: Update after each meaningful unit of work (1-2 tasks)

**Rationale**: Keeps progress transparent, allows anyone to see current status at any time, prevents loss of progress tracking if work is interrupted.
```

#### New Section 2: Pre-Implementation Environment Check (MANDATORY)

```markdown
## Pre-Implementation Environment Check (MANDATORY)

**CRITICAL**: Main Agent MUST perform these checks BEFORE starting implementation or spawning implementation agents.

### Workspace Integrity Checks

Before any code work begins, verify:

**Build System**:
- [ ] `cargo build` (or equivalent) succeeds in target crate
- [ ] `cargo test` runs without workspace errors
- [ ] All workspace members referenced in Cargo.toml exist
- [ ] Build scripts, Makefiles, or task runners work correctly

**Target Architectures** (if applicable):
- [ ] WASM targets (wasm32-unknown-unknown) are installable and testable
- [ ] Cross-compilation targets work if needed
- [ ] Platform-specific code can be verified

**Dependencies**:
- [ ] All required dependencies are accessible and up-to-date
- [ ] External tools (rustfmt, clippy, criterion, etc.) are installed
- [ ] No conflicting dependency versions

**Git Repository**:
- [ ] Repository is in good state (no corrupt objects)
- [ ] Working directory is clean or changes are intentional
- [ ] Remote push access works

### Handling Discovered Blockers

**If Blockers Found**:

1. **Stop Before Implementation**: Do not proceed with coding
2. **Document in requirements.md** under new "Known Limitations" section:

   ```markdown
   ## Known Limitations

   ### [Blocker Name] (OUT OF SCOPE)
   - **Issue**: [Describe the problem]
   - **Root Cause**: [What's causing it]
   - **Impact**: [What this prevents]
   - **Workaround**: [If any temporary solution exists]
   - **Scope**: OUT OF SCOPE / IN SCOPE / NEEDS INVESTIGATION
   - **Tracking**: [How will this be addressed - new spec, external issue, etc.]
   ```

3. **Mark as OUT OF SCOPE** if external to specification:
   - Workspace configuration issues
   - CI/CD pipeline problems
   - External service unavailability
   - Project-level architectural issues

4. **Create Separate Specification** if blocker fix is within project scope:
   - "Fix workspace configuration for WASM tests"
   - "Upgrade CI pipeline to support new toolchain"

5. **Inform User** immediately before continuing:
   - "Found blocker: [description]. Marked as OUT OF SCOPE. Proceeding with specification but [feature X] cannot be tested until workspace is fixed."

6. **Update Success Criteria** in requirements.md to reflect blockers:

   ```markdown
   ## Success Criteria

   ### Core Functionality (REQUIRED)
   - [ ] Feature X implemented and unit tested
   - [ ] API complete and documented

   ### Full Specification (BLOCKED - See Known Limitations)
   - [ ] WASM tests passing (BLOCKED by workspace configuration)
   - [ ] Benchmarks executed (DEFERRED - infrastructure ready)

   ### Completion Levels:
   - **Core**: Can mark spec complete when Core tasks done
   - **Full**: Requires blocker resolution (may be out of scope)
   ```

**Example Blocker Documentation**:

```markdown
## Known Limitations

### WASM Test Execution (OUT OF SCOPE)
- **Issue**: Cannot run `cargo test --target wasm32-unknown-unknown`
- **Root Cause**: Workspace Cargo.toml references non-existent `backends/tests/` member
- **Impact**: WASM integration tests in `tests/wasm_tests.rs` cannot execute
- **Workaround**: Tests are written correctly and will work when workspace is fixed
- **Scope**: OUT OF SCOPE - requires project-level workspace cleanup
- **Tracking**: Create specification "05-workspace-cleanup" to address
```

**Rationale**: Identifies blockers early, sets clear expectations with user, prevents confusion about specification completion status, documents limitations for future work.
```

#### New Section 3: Test Architecture Clarification (MANDATORY for Testing Specs)

```markdown
## Test Architecture Clarification (MANDATORY for Testing/Infrastructure Specifications)

When specifications involve testing infrastructure, test utilities, or new test types, Main Agent MUST ask these questions during requirements conversation:

### Required Architecture Questions

**Test Location**:
1. "Where should the actual test implementations be placed?"
   - Answer format: `[crate-name]/tests/` for integration tests, `[crate-name]/src/` for unit tests

2. "What belongs in the infrastructure crate vs the tested crate?"
   - Infrastructure crate: Reusable harnesses, scenarios, metrics, utilities (TOOLS)
   - Tested crate's tests/ directory: Actual test implementations (USAGE OF TOOLS)

3. "Should test scenarios be reusable across multiple crates?"
   - If YES: Create infrastructure crate with reusable components
   - If NO: Keep tests within single crate

4. "Where do different test types belong?"
   - Unit tests: Inline with code (`src/`)
   - Integration tests: Crate's `tests/` directory
   - Stress tests: May use infrastructure but live in `tests/`
   - Benchmarks: Crate's `benches/` directory

### Documentation in requirements.md

Document the agreed architecture prominently in requirements.md:

```markdown
## Test Architecture

### Directory Structure
- **Infrastructure**: `backends/foundation_testing/` - Reusable test harnesses, scenarios, metrics
  - Provides: `StressConfig`, `ProducerConsumerQueue`, `MetricsReporter`
  - Does NOT contain: Actual test implementations

- **Integration Tests**: `backends/foundation_nostd/tests/` - Actual test implementations
  - Uses infrastructure utilities from foundation_testing
  - Contains: `integration_tests.rs`, `wasm_tests.rs`, `barrier_debug.rs`

- **Unit Tests**: `backends/foundation_nostd/src/` - Inline with code
  - Uses `#[cfg(test)]` modules
  - Contains: Module-specific unit tests

- **Benchmarks**: `backends/foundation_testing/benches/` - Criterion benchmarks
  - Infrastructure crate provides benchmark utilities
  - Uses foundation_nostd primitives as test subjects

### Principle
**Infrastructure = Tools, Tests Directory = Usage**

Test infrastructure crates provide reusable tools. Actual tests live in the crate being tested.
```

**Rationale**: Prevents architectural confusion, avoids rework from incorrect test placement, establishes clear separation of concerns, makes test organization obvious to all agents.
```

#### New Section 4: Agent Selection Protocol (MANDATORY)

```markdown
## Agent Selection Protocol (MANDATORY)

Before spawning ANY agent for specification work, Main Agent MUST follow this protocol.

### Agent Registry Check

**Step 1: List Available Agents**

```bash
ls .agents/agents/*.md
```

**Expected Output**:
- `implementation.md` - Implementation/coding work
- `rust-verification.md` - Rust code verification
- `python-verification.md` - Python code verification
- `javascript-verification.md` - JavaScript/TypeScript verification
- `documentation.md` - Documentation writing
- `review.md` - Code and specification review
- `specification-update.md` - Specification updates

**Step 2: Read Agent Documentation**

Before spawning, read the agent's documentation frontmatter:

```yaml
---
name: Implementation Agent  # Use this EXACT name when spawning
type: implementation
language: language-agnostic
purpose: Write code following TDD, implement features, create tests
tools_required: [Read, Write, Edit, Glob, Grep, Bash]
---
```

**Step 3: Match Agent to Task**

| Task Type | Agent File | Agent Name to Use |
|-----------|------------|-------------------|
| Implementation/coding | `implementation.md` | "Implementation Agent" |
| Rust verification | `rust-verification.md` | "Rust Verification Agent" |
| Python verification | `python-verification.md` | "Python Verification Agent" |
| JS/TS verification | `javascript-verification.md` | "JavaScript Verification Agent" |
| Documentation writing | `documentation.md` | "Documentation Agent" |
| Code/spec review | `review.md` | "Review Agent" |
| Spec updates | `specification-update.md` | "Specification Update Agent" |

**Step 4: Provide Complete Context**

When spawning agent, include:

```python
Task(
    name="Implementation Agent",  # Exact name from frontmatter
    description="Implement RwLockCondVar functionality",
    prompt="""
MANDATORY: Load these before starting:
1. .agents/agents/implementation.md (YOUR documentation)
2. .agents/rules/01-04 (core rules)
3. .agents/rules/13-implementation-agent-guide.md
4. .agents/stacks/rust.md (language standards)

Your task: [detailed task description]
Specification: specifications/04-condvar-primitives/requirements.md
Expected output: [specific deliverables]
"""
)
```

### Fallback: General-Purpose Agent

**If Specialized Agent Not Found**:
- Use `general-purpose` agent with explicit instructions
- Document in LEARNINGS.md that specialized agent was needed
- Consider creating specialized agent if need recurs

**Example**:

```python
Task(
    name="general-purpose",  # Fallback
    description="Implement feature (no specialized agent available)",
    prompt="""Note: Using general-purpose agent as specialized agent doesn't exist.

Load: [all relevant rules and documentation]
Task: [explicit instructions]
Expected: [clear deliverables]"""
)
```

**Rationale**: Ensures correct agent with proper capabilities is used for each task, leverages specialized workflows and expertise, prevents generic agents from missing important patterns, follows established agent architecture.
```

### Where to Add in Rule 06

Insert these sections AFTER "Agent Responsibilities During Implementation (CRITICAL)" section (around line 1100), BEFORE "Self-Containment and Mandatory Verification Requirements" section.

**New Section Order**:
1. Agent Responsibilities During Implementation (existing)
2. **Tasks.md Update Protocol** (NEW)
3. **Pre-Implementation Environment Check** (NEW)
4. **Test Architecture Clarification** (NEW)
5. **Agent Selection Protocol** (NEW)
6. Self-Containment and Mandatory Verification Requirements (existing)

---

## 2. Rule 13 Updates (Implementation Agent Guide)

### Location: After "Before Starting Work" section (around line 35)

#### Add Brief Reminder About Tasks.md Updates

```markdown
### Tasks.md Update Responsibility

**IMPORTANT**: While Main Agent coordinates overall specification progress, implementation agents MUST update tasks.md for their assigned work.

**When to Update**:
- After implementing each feature component
- After fixing clippy/test failures
- After completing logical milestones
- Minimum: Every 1-2 completed subtasks

**How to Update**:
1. Mark checkbox: Change `- [ ]` to `- [x]`
2. Add note: `(✅ 2026-01-23: details)`
3. Update frontmatter counts (completed/uncompleted/percentage)
4. Commit separately: "Update tasks.md: completed [feature name]"
5. Report update to Main Agent in completion report

**Reference**: See Rule 06 "Tasks.md Update Protocol" for complete details.

**Rationale**: Tasks.md reflects real-time progress. Updates must happen as work progresses, not batched at end.
```

---

## 3. Rule 05 Updates (Agent Orchestration)

### Location: In "Verification Workflow" section (search for verification mentions)

#### Add Continuous Verification Checkpoints

```markdown
### Continuous Verification (ADDITION to Final Verification)

**IMPORTANT**: Verification should happen continuously during implementation, not just at the end.

**Verification Checkpoints**:

**Checkpoint 1**: After each feature implementation
- Quick clippy check on modified modules
- Run relevant unit tests
- Smoke test compilation

**Checkpoint 2**: After each commit
- Full clippy: `cargo clippy -- -D warnings`
- Relevant test suite
- Format check: `cargo fmt -- --check`

**Checkpoint 3**: After phase completion
- Full test suite: `cargo test --all-features`
- Full clippy with all targets
- Release build: `cargo build --release`

**Checkpoint 4**: Final verification (existing workflow)
- Spawn dedicated Verification Agent
- Complete verification per Rule 08
- Create VERIFICATION.md

**Benefits**:
- Issues caught early (easier to fix)
- Final verification confirms rather than discovers
- Higher confidence in incremental progress

**Integration with Tasks.md**:
Update tasks after each checkpoint:
```markdown
- [x] Implement feature X
- [x] Run clippy (0 warnings) ✅
- [x] Run tests (all passing) ✅
```

**Reference**: Full continuous verification protocol in Rule 06.
```

---

## 4. Template Updates

### A. tasks-template.md

**Add at top, after frontmatter and title, BEFORE "## Task List"**:

```markdown
## 🤖 Agent Information

**Agents Working on This Specification**:

When working on this specification, agents should:

1. **Read Agent Documentation First**:
   - Implementation work: `.agents/agents/implementation.md`
   - Rust verification: `.agents/agents/rust-verification.md`
   - Python verification: `.agents/agents/python-verification.md`
   - JavaScript verification: `.agents/agents/javascript-verification.md`
   - Documentation: `.agents/agents/documentation.md`
   - Review: `.agents/agents/review.md`

2. **Load Required Rules** (all agents):
   - Rules 01-04 (core mandatory rules)
   - Rule 06 (specifications and requirements) - for context
   - Language-specific stack file: `.agents/stacks/[language].md`

3. **Additional Rules by Agent Type**:
   - Implementation agents: Rule 13 (implementation guide)
   - Verification agents: Rule 08 (verification workflow)
   - Documentation agents: Rule 09 (if creating skills)

4. **Update This Tasks.md**:
   - ✅ Update immediately after completing each task (see Rule 06 "Tasks.md Update Protocol")
   - ✅ Mark checkboxes [x] the moment work is done
   - ✅ Update frontmatter counts (completed/uncompleted/percentage)
   - ✅ Commit separately with clear message
   - ❌ DO NOT batch updates - update after each 1-2 tasks

---
```

### B. requirements-template.md

**Add at top, after frontmatter and title, BEFORE "## Overview"**:

```markdown
## 🤖 Agent Information

**For Agents Working on This Specification**:

### Required Reading (BEFORE starting work)

1. **Your Agent Documentation**:
   - Check `.agents/agents/` directory for your specialized agent documentation
   - Example: Implementation Agent reads `.agents/agents/implementation.md`

2. **Core Rules** (all agents MUST load):
   - Rules 01-04 from `.agents/rules/` (mandatory)
   - Rule 06 - Specifications and Requirements (for context)

3. **Language Stack File** (if applicable):
   - Rust: `.agents/stacks/rust.md`
   - TypeScript: `.agents/stacks/typescript.md`
   - Python: `.agents/stacks/python.md`

4. **Role-Specific Rules**:
   - Implementation: Rule 13 (implementation agent guide)
   - Verification: Rule 08 (verification workflow)
   - Documentation: May need Rule 09 (skills creation)

### Specification Files

- **requirements.md** (this file): Complete requirements and success criteria
- **tasks.md**: Task checkboxes - UPDATE THIS CONTINUOUSLY (see Rule 06)
- **LEARNINGS.md**: Document implementation insights as you discover them
- **PROGRESS.md**: Created mid-specification for progress reporting
- **REPORT.md**: Created when all work complete

### Critical Reminders

- ✅ Update tasks.md after every 1-2 completed subtasks (Rule 06 "Tasks.md Update Protocol")
- ✅ Run verification at checkpoints, not just at end (Rule 06 "Continuous Verification")
- ✅ Check `.agents/agents/` directory before spawning sub-agents (Rule 06 "Agent Selection Protocol")
- ✅ Document blockers immediately in requirements.md "Known Limitations" section

---
```

---

## Implementation Plan

### Phase 1: Review and Approval
- [ ] User reviews proposed changes
- [ ] User approves or requests modifications
- [ ] Confirm which rules to update

### Phase 2: Update Rules
- [ ] Update Rule 06 with 4 new sections
- [ ] Update Rule 13 with brief tasks.md reminder
- [ ] Update Rule 05 with continuous verification addition

### Phase 3: Update Templates
- [ ] Update tasks-template.md with agent information
- [ ] Update requirements-template.md with agent information

### Phase 4: Verification
- [ ] Review all changes for consistency
- [ ] Ensure no contradictions with existing rules
- [ ] Commit all updates with proper messages
- [ ] Update PROCESS_LEARNINGS.md status to "IMPLEMENTED"

---

## Summary of Changes

| File | Change Type | Size | Impact |
|------|-------------|------|--------|
| Rule 06 | Add 4 sections | +~500 lines | High - all specs |
| Rule 13 | Add 1 section | +~30 lines | Medium - impl agents |
| Rule 05 | Add 1 section | +~40 lines | Medium - main agent |
| tasks-template.md | Add header | +~35 lines | High - all new specs |
| requirements-template.md | Add header | +~45 lines | High - all new specs |

**Total Impact**: ~650 new lines across 5 files
**Benefit**: Prevents issues discovered in Spec 04 from recurring
**Priority**: HIGH - affects all future specifications

---

**Status**: AWAITING USER APPROVAL
**Next**: User review → approval → implementation → verification
