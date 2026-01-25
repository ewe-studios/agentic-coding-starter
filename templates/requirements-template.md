---
description: Brief one-sentence description
status: in-progress
priority: medium
created: YYYY-MM-DD
author: Main Agent
metadata:
  version: "1.0"
  last_updated: YYYY-MM-DD
  estimated_effort: "medium"
  tags:
    - tag1
    - tag2
  stack_files:
    - .agents/stacks/[language].md
  skills: []
  tools:
    - [Tool names]
builds_on: []
related_specs: []
has_features: true # DEFAULT: true unless spec is very simple (1-3 trivial tasks)
has_fundamentals: true # DEFAULT: true unless user explicitly says no - create user documentation
# Choose ONE based on has_features:
features: # If has_features: true
  completed: 0
  uncompleted: [N]
  total: [N]
  completion_percentage: 0
tasks: # If has_features: false
  completed: 0
  uncompleted: [N]
  total: [N]
  completion_percentage: 0
files_required:
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

  # STRUCTURE DIFFERS BASED ON has_features:
  # - If has_features: false → include implementation_agent section (agents read requirements.md)
  # - If has_features: true → NO implementation_agent section (agents read feature.md files)
  #
  # Example for has_features: false only:
  # implementation_agent:
  #   rules:
  #     - .agents/rules/01-rule-naming-and-structure.md
  #     - .agents/rules/02-rules-directory-policy.md
  #     - .agents/rules/03-dangerous-operations-safety.md
  #     - .agents/rules/04-work-commit-and-push-rules.md
  #     - .agents/rules/13-implementation-agent-guide.md
  #     - .agents/rules/11-skills-usage.md
  #     - [stack_file from metadata.stack_files]
  #   files:
  #     - ./requirements.md
  #     - ./fundamentals/* (if has_fundamentals: true)
---

# [Specification Name] - Requirements

> **Specification Structure**:
> - **has_features: false** → This file contains COMPLETE requirements with detailed tasks
> - **has_features: true** → This file is HIGH-LEVEL OVERVIEW ONLY. Detailed requirements are in `features/*/feature.md`

---

## IF has_features: false (SIMPLE SPECS - Rare)

**Use this structure ONLY for trivial specs (1-3 simple tasks)**

### Overview

[Brief summary - 1-2 paragraphs]

### Requirements Conversation Summary

#### User's Initial Request
[What user asked for]

#### Clarifying Questions Asked
1. Question → Answer
2. Question → Answer

#### Final Requirements Agreement
[What was agreed]

### Detailed Requirements

#### Functional Requirements
1. Requirement 1
2. Requirement 2

#### Technical Specifications
- **Stack**: [Technologies]
- **Dependencies**: [Libraries]
- **Location**: [Code location]

### Tasks

> Update tasks after completion + verification. Commit after each task completion (Rule 04).

#### Implementation Tasks
- [ ] Task 1: Description
- [ ] Task 2: Description

#### Testing Tasks
- [ ] Unit tests for [component]
- [ ] Integration tests

#### Verification Tasks
- [ ] All tests pass
- [ ] Linter: 0 warnings
- [ ] Formatter: clean

### Success Criteria
- [ ] All tasks complete
- [ ] All tests passing
- [ ] Code quality checks pass

### Module Documentation References
- **Module**: `documentation/[module]/doc.md`

---

## IF has_features: true (FEATURE-BASED SPECS - DEFAULT)

**Use this structure for all non-trivial work**

### Overview

[Brief summary of specification purpose - 2-3 paragraphs maximum]

**Key Approach**: [High-level technical approach]

### Known Issues/Limitations (if any)

#### [Issue Name] (OUT OF SCOPE / IN SCOPE)
- **Issue**: [Description]
- **Root Cause**: [Cause]
- **Impact**: [What this affects]
- **Scope**: [OUT OF SCOPE / IN SCOPE]
- **Decision**: [How handled]

### Requirements Conversation Summary

#### User's Initial Request
[What user asked for]

#### Clarifying Questions Asked
1. Question → Answer
2. Question → Answer
3. Question → Answer

#### Final Requirements Agreement
[Clear statement of agreed requirements]

### Feature Index

**Purpose**: Directory of features with dependencies. Load specific feature.md as needed.

| # | Feature | Description | Dependencies | Status |
|---|---------|-------------|--------------|--------|
| 0 | [name](./features/00-name/feature.md) | [Brief description] | None | ⬜ Pending |
| 1 | [name](./features/01-name/feature.md) | [Brief description] | 0 | ⬜ Pending |
| 2 | [name](./features/02-name/feature.md) | [Brief description] | 1 | ⬜ Pending |

**Status Key**: ⬜ Pending | 🔄 In Progress | ✅ Complete

**Notes**:
- Features implemented in dependency order
- Each feature.md contains detailed requirements, tasks, verification
- Update status in this table as features complete

### Success Criteria (Spec-Wide)

**All Features Complete**:
- [ ] All features in index marked complete (✅)
- [ ] Inter-feature integration tests passing
- [ ] Cross-feature functionality verified

**Spec-Wide Quality**:
- [ ] All features pass linter (zero warnings)
- [ ] All features pass tests
- [ ] Consistent code quality across features

**Documentation**:
- [ ] LEARNINGS.md created
- [ ] REPORT.md created at completion
- [ ] VERIFICATION.md created with signoff
- [ ] fundamentals/ directory created (if has_fundamentals: true)
- [ ] fundamentals/00-overview.md covers usage, patterns, examples

### Module Documentation References

Implementation agents MUST read before changes:
- **Module**: `documentation/[module]/doc.md`

---

## Pre-Work Review (MANDATORY)

**CRITICAL**: Main Agent MUST spawn Review Agent before ANY feature work begins.

### When Review Required

Review Agent MUST be spawned:
- ✅ Before starting ANY feature (even if documentation says previous features complete)
- ✅ When resuming work after pause/break
- ✅ When switching between features
- ✅ At start of each work session

### Review Agent Responsibilities

Review Agent MUST:
1. ✅ Read specification thoroughly (requirements.md, features/*/feature.md)
2. ✅ Analyze current codebase state (actual code, not just documentation)
3. ✅ Compare reality vs documentation:
   - Verify completed features are ACTUALLY complete (code exists, tests pass)
   - Check if claimed tasks are really done
   - Validate dependency chains
4. ✅ Verify accuracy of:
   - PROGRESS.md status claims
   - Feature.md task checkboxes
   - requirements.md feature completion counts
5. ✅ Assess readiness for next work:
   - Dependencies truly complete?
   - Code quality acceptable?
   - Tests actually passing?
   - Any blockers or issues?

### Review Agent Assessment

Review Agent returns one of:
- **GO**: Ready to proceed with [specific feature] - all dependencies verified complete
- **STOP**: Issues found - list specific problems that must be fixed first
- **CLARIFY**: Need user input - specify what needs clarification

### Main Agent Response to Review

Based on Review Agent assessment:
- **GO**: Proceed with implementation of specified feature
- **STOP**: Fix issues before proceeding, may need to re-verify previous work
- **CLARIFY**: Ask user for needed clarifications

**Why This Matters**:
- Documentation can be inaccurate (tasks marked done but not actually complete)
- Previous agents may have made mistakes
- Code may not match claimed completion status
- Dependencies might not actually work
- Prevents wasted implementation effort on wrong assumptions
- **USER EXPECTS thorough review before starting work**

---

> **INSTRUCTION FOR SPECIFICATION AGENT**:
>
> Copy the content from ONE of these files below based on `has_features` value:
> - **If has_features: true** → Copy content from `.agents/templates/examples/agent-instructions-with-features.md`
> - **If has_features: false** → Copy content from `.agents/templates/examples/agent-instructions-without-features.md`
>
> Paste the content here, then delete this instruction block.

---

_Created: YYYY-MM-DD_
_Last Updated: YYYY-MM-DD_
