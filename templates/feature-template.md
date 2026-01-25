---
feature: feature-name
description: Brief one-sentence description of what this feature implements
status: pending
priority: medium
depends_on: []
estimated_effort: medium
created: YYYY-MM-DD
last_updated: YYYY-MM-DD
author: Main Agent
tasks:
  completed: 0
  uncompleted: 0
  total: 0
  completion_percentage: 0
files_required:
  implementation_agent:
    rules:
      - .agents/rules/01-rule-naming-and-structure.md
      - .agents/rules/02-rules-directory-policy.md
      - .agents/rules/03-dangerous-operations-safety.md
      - .agents/rules/04-work-commit-and-push-rules.md
      - .agents/rules/13-implementation-agent-guide.md
      - .agents/rules/11-skills-usage.md
      - .agents/stacks/[language].md
    files:
      - ../requirements.md
      - ./feature.md
      - ./templates/ # If feature has templates
      - ../fundamentals/* # If parent spec has_fundamentals: true
  verification_agent:
    rules:
      - .agents/rules/01-rule-naming-and-structure.md
      - .agents/rules/02-rules-directory-policy.md
      - .agents/rules/03-dangerous-operations-safety.md
      - .agents/rules/04-work-commit-and-push-rules.md
      - .agents/rules/08-verification-workflow-complete-guide.md
      - .agents/stacks/[language].md
    files:
      - ../requirements.md
      - ./feature.md
---

# [Feature Name]

## Overview

Brief summary of what this feature implements and its purpose within the larger specification.

## Dependencies

This feature depends on:
- `[other-feature]` - Why this dependency exists

This feature is required by:
- `[dependent-feature]` - Why this feature is needed

## Requirements

### Functional Requirements

1. **Requirement 1**
   - Detail about the requirement
   - Expected behavior

2. **Requirement 2**
   - Detail about the requirement
   - Expected behavior

### Technical Requirements

- **Pattern to follow**: Description of required patterns
- **Types to create**: List of types/structs to implement
- **Integrations**: What this connects to

## Implementation Details

### Key Structures

```rust
// Example structure - or reference templates/
pub struct ExampleStruct {
    // fields
}
```

### Key Functions

| Function | Purpose | Location |
|----------|---------|----------|
| `function_name()` | What it does | `file.rs` |

## Templates

See `templates/` directory for:
- `example-struct.rs` - Base structure template
- `example-impl.rs` - Implementation template

---

## Tasks

> **Task Tracking**: Mark tasks as `[x]` after completing AND verifying each task. Update frontmatter counts (completed/uncompleted/total/completion_percentage) immediately. Commit after task completion + verification pass (Rule 04).
>
> **Important**: Each feature manages its own task tracking. Update this file's frontmatter as tasks complete.

### Implementation Tasks
- [ ] Task 1: Implement core structure
- [ ] Task 2: Add key functions
- [ ] Task 3: Integrate with dependencies

### Testing Tasks
- [ ] Write unit tests for [component]
- [ ] Write integration tests
- [ ] Run verification commands

### Documentation Tasks
- [ ] Document public APIs
- [ ] Add usage examples

---

## Success Criteria

- [ ] Criterion 1 - specific and verifiable
- [ ] Criterion 2 - specific and verifiable
- [ ] All unit tests pass
- [ ] Code passes `cargo fmt` and `cargo clippy`

## Verification Commands

```bash
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test --package [package] -- [test_filter]
cargo build --package [package]
```

## Notes for Agents

### Before Starting (MANDATORY)

**CRITICAL**: Main Agent MUST spawn Review Agent before starting this feature.

**Review Agent Responsibilities**:
1. ✅ Read parent specification's requirements.md
2. ✅ Read this feature.md file completely
3. ✅ **VERIFY in code** that dependent features are ACTUALLY complete:
   - Check that code exists (not just documentation claims)
   - Verify tests pass for dependencies
   - Validate types/functions this feature needs are present
4. ✅ Read any templates referenced in templates/ directory
5. ✅ Analyze current codebase state vs claimed completion status
6. ✅ Assess readiness: GO / STOP / CLARIFY

**Why This Matters**:
- Documentation may claim features complete when they're not
- Previous work may have gaps or issues
- Prevents building on broken foundations
- **USER EXPECTS verification before implementation starts**

### Implementation Guidelines
- Follow existing patterns in codebase
- Use types from dependent features
- Update Tasks section and frontmatter counts as work progresses
- Follow TDD: Write tests FIRST, verify they fail, then implement
- Self-review before reporting completion
- Document learnings in ../LEARNINGS.md

---
*Created: YYYY-MM-DD*
*Last Updated: YYYY-MM-DD*
