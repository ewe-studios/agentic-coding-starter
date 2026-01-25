---
description: Brief one-sentence description
status: in-progress
priority: medium
created: YYYY-MM-DD
author: "Main Agent"
metadata:
  version: "1.0"
  last_updated: YYYY-MM-DD
  estimated_effort: "medium"
  tags:
    - feature
  stack_files:
    - .agents/stacks/[language].md
  skills: []
  tools:
    - [List of tools used]
tasks:
  completed: 0
  uncompleted: 8
  total: 8
  completion_percentage: 0
builds_on: []
related_specs: []
has_features: true # DEFAULT: true unless spec is very simple (1-3 trivial tasks)
has_fundamentals: true # should always be true, we should always be generated these unless user explicitly says no
files_required:
  # IMPORTANT: Structure differs based on has_features value
  # - If has_features: false → implementation_agent reads requirements.md directly
  # - If has_features: true → implementation_agent reads requirements.md + specific feature.md

  main_agent:
    rules:
      - .agents/rules/01-rule-naming-and-structure.md
      - .agents/rules/02-rules-directory-policy.md
      - .agents/rules/03-dangerous-operations-safety.md
      - .agents/rules/04-work-commit-and-push-rules.md
      - .agents/rules/05-coding-practice-agent-orchestration.md
      - .agents/rules/06-specifications-and-requirements.md
    files:
      - ./requirements.md
      - ./LEARNINGS.md (if exists)
      - ./PROGRESS.md (if exists)

  review_agent:
    rules:
      - .agents/rules/01-rule-naming-and-structure.md
      - .agents/rules/02-rules-directory-policy.md
      - .agents/rules/03-dangerous-operations-safety.md
      - .agents/rules/04-work-commit-and-push-rules.md
      - .agents/rules/06-specifications-and-requirements.md
    files:
      - ./requirements.md
      - [stack_file from metadata.stack_files]

  implementation_agent:
    rules:
      - .agents/rules/01-rule-naming-and-structure.md
      - .agents/rules/02-rules-directory-policy.md
      - .agents/rules/03-dangerous-operations-safety.md
      - .agents/rules/04-work-commit-and-push-rules.md
      - .agents/rules/13-implementation-agent-guide.md
      - .agents/rules/11-skills-usage.md (if skills used)
      - [stack_file from metadata.stack_files]
    files:
      # IF has_features: false → Read requirements.md (contains everything)
      - ./requirements.md
      # IF has_features: true → Read requirements.md (overview) + specific feature.md (details)
      - ./requirements.md
      - ./features/[feature-name]/feature.md (specific feature assigned)
      - [fundamentals/* if has_fundamentals: true]

  verification_agent:
    rules:
      - .agents/rules/01-rule-naming-and-structure.md
      - .agents/rules/02-rules-directory-policy.md
      - .agents/rules/03-dangerous-operations-safety.md
      - .agents/rules/04-work-commit-and-push-rules.md
      - .agents/rules/08-verification-workflow-complete-guide.md
      - [stack_file from metadata.stack_files]
    files:
      - ./requirements.md

  documentation_agent:
    rules:
      - .agents/rules/01-rule-naming-and-structure.md
      - .agents/rules/02-rules-directory-policy.md
      - .agents/rules/03-dangerous-operations-safety.md
      - .agents/rules/04-work-commit-and-push-rules.md
      - .agents/rules/06-specifications-and-requirements.md
    files:
      - ./requirements.md
      - ./fundamentals/* (if has_fundamentals: true)
      - [documentation/[module]/doc.md for modules being documented]
---

# [Specification Name] - Requirements

> **Specification Structure**:
> - **has_features: false** → This file contains COMPLETE requirements with detailed tasks
> - **has_features: true** → This file is HIGH-LEVEL OVERVIEW ONLY. Detailed requirements are in `features/*/feature.md`
>
> **Agent Instructions**: Review the `files_required` section in frontmatter above. Each agent type has explicit rules and files to load. Load YOUR agent type's requirements before starting work.

## Overview

[Brief summary of what this specification covers and why it's needed - 2-3 paragraphs maximum]

---

## FOR SIMPLE SPECS (has_features: false) - Skip to "Requirements Conversation Summary" if using features

**NOTE**: The sections below (Detailed Requirements, Tasks) are ONLY for simple specifications. If this spec has `has_features: true`, skip these sections and use the Features section instead.

## Requirements Conversation Summary

### User's Initial Request

[Summary of what user initially asked for]

### Clarifying Questions Asked

1. Question about [topic]
   - Answer: [user's response]
2. Question about [topic]
   - Answer: [user's response]
3. Do we need to generate fundamental documentation for users?
   - Answer: [user's response]

### Final Requirements Agreement

Based on the conversation, we agreed on:

- [Clear statement of final understanding]
- [All important details confirmed]

---

## IF has_features: false - SIMPLE SPEC SECTIONS

**These sections are for simple specs ONLY. If using features, skip to "Features Section" below.**

### Detailed Requirements

#### Functional Requirements

1. [Requirement 1]
2. [Requirement 2]

#### Non-Functional Requirements

1. [Performance requirements]
2. [Security requirements]

#### Technical Specifications

- **Technology Stack:** [Technologies to be used]
- **Dependencies:** [Required libraries/tools]
- **Integration Points:** [How this integrates]

---

## Tasks

> **Task Tracking**: Mark tasks as `[x]` after completing AND verifying each task. Update frontmatter counts (completed/uncompleted/completion_percentage) immediately. Commit after task completion + verification pass (Rule 04). See Agent Reminders section at end for critical task tracking rules.
>
> **IMPORTANT**: requirements.md uses `has_features=false` format ONLY when specification is very simple (1-3 trivial tasks). For any non-trivial work, use `has_features=true` and organize work into features with clear dependencies.

If a requirements.md has `has_features=false` (rare - only for trivial specs), track tasks here. Otherwise, use the [Features](#features) section below.

### Current Status

- **Progress**: [X/Y tasks complete (Z%)]
- **Phase**: [Current phase of work]
- **Blockers**: [Any blockers, or "None"]

### Implementation Tasks

#### Phase 1: [Phase Name]

- [ ] Task 1: [Description]
- [ ] Task 2: [Description]
- [ ] Task 3: [Description]

#### Phase 2: [Phase Name]

- [ ] Task 4: [Description]
- [ ] Task 5: [Description]

### Testing Tasks

- [ ] Write unit tests for [component]
- [ ] Write integration tests for [feature]
- [ ] Add benchmark tests (if applicable)

### Documentation Tasks

- [ ] Write API documentation
- [ ] Add usage examples
- [ ] Create fundamentals documentation (if has_fundamentals: true)

### Verification Tasks

- [ ] Run all tests and verify 100% pass
- [ ] Run linter and verify 0 warnings
- [ ] Run formatter and verify clean
- [ ] Create verification report

---

## IF has_features: true - FEATURE-BASED SPEC SECTIONS

**These sections are for feature-based specs (DEFAULT). Requirements.md is HIGH-LEVEL OVERVIEW ONLY.**

### Known Issues/Limitations (if any)

**Document any pre-existing blockers or constraints**:

#### [Issue Name] (OUT OF SCOPE / IN SCOPE)
- **Issue**: [Describe the problem]
- **Root Cause**: [What's causing it]
- **Impact**: [What this prevents or affects]
- **Workaround**: [Temporary solution if any]
- **Scope**: [OUT OF SCOPE / IN SCOPE]
- **Decision**: [How this affects the specification]

### High-Level Architecture

[Describe the overall approach at a high level - NO implementation details]

**Architecture Pattern**: [e.g., Iterator-based, Event-driven, Layered, etc.]

**Key Components** (see features for details):
- Component A: [Brief purpose]
- Component B: [Brief purpose]
- Component C: [Brief purpose]

**Data Flow** (high-level):
```
Input → Component A → Component B → Output
```

**Integration Points**:
- Integrates with: [Other systems/modules]
- Provides: [What this spec provides]
- Requires: [What this spec needs]

---

## Features

> **Feature Tracking**: Mark features as `[x]` after completing AND verifying each feature. Update frontmatter counts (completed/uncompleted/completion_percentage) immediately. Commit after feature completion + verification pass (Rule 04). See Agent Reminders section at end for critical tracking rules.
>
> **DEFAULT APPROACH**: Most specifications should use `has_features=true`. Features provide clear organization, dependency management, and context optimization.
>
> **Agents**: Read the feature index below, then load ONLY the specific feature.md you're working on. Do NOT load all features.

### Feature Index

**Purpose**: Directory of all features with dependencies. Agents load specific features as needed.

| # | Feature | Description | Dependencies | Status |
|---|---------|-------------|--------------|--------|
| 0 | [foundation](./features/00-foundation/feature.md) | [Brief description] | None | ⬜ Pending |
| 1 | [core-api](./features/01-core-api/feature.md) | [Brief description] | foundation | ⬜ Pending |
| 2 | [integrations](./features/02-integrations/feature.md) | [Brief description] | core-api | ⬜ Pending |

**Status Key**: ⬜ Pending | 🔄 In Progress | ✅ Complete

**Notes**:
- Features are numbered for ordered execution
- Dependencies MUST be complete before dependent feature starts
- See individual feature.md files for detailed requirements and tasks
- Update feature status in table above as work progresses

---

## Success Criteria

### For Simple Specs (has_features: false)

**Implementation Success**:
- [ ] All functional requirements implemented
- [ ] All non-functional requirements met
- [ ] All tests passing

**Documentation Success** (if has_fundamentals: true):
- [ ] All fundamental documents created
- [ ] User documentation comprehensive and accurate
- [ ] Code examples compile and are correct
- [ ] Trade-offs and design decisions explained

**Quality Success** (MANDATORY - NO EXCEPTIONS):
- [ ] All tests passing (100%)
- [ ] Zero linter warnings
- [ ] Zero compiler warnings
- [ ] Code properly formatted
- [ ] All public items documented

### For Feature-Based Specs (has_features: true)

**Spec-Wide Criteria ONLY** (feature-specific criteria go in feature.md):

**All Features Complete**:
- [ ] All features in feature index marked complete
- [ ] All inter-feature integration tests passing
- [ ] Cross-feature functionality verified

**Spec-Wide Documentation** (if has_fundamentals: true):
- [ ] Fundamentals documentation covers all features
- [ ] Usage examples show feature combinations
- [ ] Architecture documentation complete

**Spec-Wide Quality** (MANDATORY):
- [ ] All features pass verification
- [ ] No cross-feature conflicts
- [ ] Consistent code quality across all features
- [ ] Complete REPORT.md and VERIFICATION.md files

---

## User-Facing Documentation (if has_fundamentals: true)

**CRITICAL**: When specification introduces new user-facing features, libraries, or APIs, create a `fundamentals/` directory with comprehensive user documentation.

### Fundamentals Documentation Structure

Create these documents in `specifications/[NN-spec-name]/fundamentals/`:

1. **00-overview.md** - Introduction, quick start, decision trees
2. **[Additional docs as needed]** - Deep dives into specific concepts

**Documentation Principles**:
- **Explain WHY**: Design decisions and trade-offs, not just how
- **Show internals**: Key implementation details with commentary
- **Provide examples**: Compilable, real-world usage
- **Discuss trade-offs**: When to use, when NOT to use
- **Be self-contained**: Reader can understand without external resources

**For feature-based specs**: Fundamentals should cover the overall specification. Feature-specific docs go in feature.md files.

## Module Documentation References

This specification modifies the following modules:

### [Module Name]

- **Documentation**: `documentation/[module]/doc.md`
- **Purpose**: [Brief summary]
- **Changes Needed**: [What will be changed]

**CRITICAL**: Agents MUST read module documentation BEFORE making changes.

**Agent File Loading**: All agent-specific rules and files to load are specified in the `files_required` section of the frontmatter above. Review your agent type's requirements there.

---

## MANDATORY Completion and Verification Requirements

**CRITICAL**: Before marking this specification as complete, ALL of the following MUST be verified:

### 1. Task Completion Verification (100% REQUIRED)

**NO EXCEPTIONS**: Every task in the Tasks section MUST be completed.

- [ ] Scroll to Tasks section and verify ALL tasks are marked `[x]`
- [ ] Verify `tasks/features.completed` count in frontmatter matches actual `[x]` count
- [ ] Verify `tasks/features.uncompleted` count is `0`
- [ ] Verify `tasks/features.completion_percentage` is `100`
- [ ] NO tasks/features left as `[ ]` (incomplete)
- [ ] NO optional tasks - everything is mandatory unless user explicitly says otherwise
- [ ] Always complete a task or features expectation and implementation, spending time to fix any broken, timing out or problematic tests cases, clarifying with user if necessary and making sensible choices when its clear.
- [ ] Never start a new task/feature until the previous one is completed and actually done.

**Validation Command**:

```bash
# Must return 0
grep -c "^- \[ \]" requirements.md
```

### 2. Code/Implementation Verification (100% REQUIRED)

For each task/feature in the Tasks/Features section:

- [ ] Verify the code/file actually exists in the codebase
- [ ] Verify the implementation matches the task description
- [ ] Verify all tests for that component pass
- [ ] NO placeholder implementations
- [ ] NO commented-out code marked as "TODO"

### 3. Documentation Verification (100% REQUIRED - NO OPTIONAL)

**If has_fundamentals: true**:

- [ ] ALL fundamental documents listed in the Tasks section exist
- [ ] Each fundamental doc is comprehensive (not stub/placeholder)
- [ ] Code examples in docs compile and work
- [ ] Cross-references between docs are valid

**Always Required**:

- [ ] `LEARNINGS.md` created with implementation insights
- [ ] `PROGRESS.md` created with timeline and status
- [ ] `verification.md` or `VERIFICATION_SIGNOFF.md` created

### 4. Quality Verification (100% REQUIRED - ZERO TOLERANCE)

**Build and Test**:

- [ ] `cargo build` (or equivalent) succeeds with 0 errors
- [ ] `cargo test` (or equivalent) shows 100% tests passing
- [ ] NO ignored or skipped tests (unless explicitly user-approved)

**Code Quality** (language-specific, see stack file):

- [ ] `cargo clippy -- -D warnings` (Rust) shows 0 warnings
- [ ] `npm run lint` (TypeScript/JavaScript) shows 0 errors
- [ ] Code formatter applied and clean
- [ ] NO code quality warnings ignored or suppressed without justification

**Documentation Quality**:

- [ ] All public APIs documented
- [ ] All documentation builds without errors
- [ ] NO broken links in documentation

### 5. Specification Tracking Verification (MANDATORY)

- [ ] Tasks section shows 100% completion
- [ ] `LEARNINGS.md` exists and documents key insights
- [ ] `PROGRESS.md` exists and shows timeline/achievements
- [ ] `verification.md` or `VERIFICATION_SIGNOFF.md` exists with verification results
- [ ] `requirements.md` frontmatter has correct `status` field and `tasks` counts

### 6. Verification Issue Resolution (MANDATORY)

**NO OPTIONAL FIXES**: All verification issues MUST be resolved.

- [ ] Check `verification.md` for any FAILED or WARNING items
- [ ] ALL failed checks must be fixed (no exceptions)
- [ ] ALL warnings must be addressed or explicitly accepted by user
- [ ] Re-run verification after fixes to confirm PASS status
- [ ] Update `verification.md` with final PASS status

**If verification shows ANY failures**:

1. ❌ DO NOT mark specification as complete
2. ❌ DO NOT mark tasks as done
3. ✅ FIX all issues
4. ✅ Re-run verification
5. ✅ Only mark complete after 100% PASS

---

## Final Verification Checklist

Before marking this specification as **completed**:

- [ ] All tasks/features in the Tasks/Features section are checked `[x]` (100%)
- [ ] All code exists and works (verified in codebase)
- [ ] All tests pass (100%, no failures/skips)
- [ ] All documentation complete (no stubs/placeholders)
- [ ] All quality checks pass (0 warnings)
- [ ] All verification issues resolved (100% PASS)
- [ ] No test (unit or integration) is skipped, ignored, silenced or removed without users consent.
- [ ] All tests are fully fixed, updated and resolved, no empty tests or fake passing test that actually provide no value or meet expectations.
- [ ] LEARNINGS.md exists and is comprehensive
- [ ] PROGRESS.md exists with timeline
- [ ] verification.md exists with PASS status
- [ ] fundamentals/ directory exists (if has_fundamentals: true)
- [ ] All fundamental docs listed are created

**Status can only be set to "completed" when ALL items above are checked.**

---

> **Verification**: See [verification.md](./verification.md) or [VERIFICATION_SIGNOFF.md](./VERIFICATION_SIGNOFF.md) for complete verification results.

---

## 🤖 Agent Reminders

**CRITICAL RULES - READ EVERY TIME**:

1. **Requirement Updates (MANDATORY)**:
   - ✅ Update this requirements.md file IMMEDIATELY when you identify new requirements
   - ✅ Update IMMEDIATELY when requirements changes are confirmed with user
   - ✅ If user grants full rights, auto-update requirements without seeking approval
   - ❌ DO NOT wait until task completion to update requirements
   - ❌ DO NOT forget to sync requirements with actual implementation

2. **Task/Feature Tracking (MANDATORY)**:
   - ✅ Update Tasks/Features section in this file IMMEDIATELY after completing AND verifying each task/feature
   - ✅ Mark task/feature as `[x]` the MOMENT verification passes
   - ✅ Update frontmatter `tasks` or `features` counts (completed/uncompleted/completion_percentage) immediately
   - ✅ Commit after verification passes (Rule 04: Task/Feature-level commits)
   - ❌ DO NOT commit incomplete tasks/features
   - ❌ DO NOT batch multiple tasks/features into one commit
   - ❌ DO NOT create separate task tracking files

3. **All Requirements and Tasks/Features Are Mandatory**:
   - ✅ Unless user explicitly states something is optional, ALL items are MANDATORY
   - ✅ All items must be implemented and completed
   - ❌ DO NOT skip items thinking they are optional
   - ❌ DO NOT treat any item as "nice-to-have" without explicit user confirmation

**These rules exist to ensure**:

- Requirements accurately reflect current understanding
- Real-time visibility into task progress
- No work is lost or forgotten
- User has real-time visibility into project state
- Future agents have accurate context
- This requirements.md file is the single source of truth

---

_Created: [Date]_
_Last Updated: [Date]_
