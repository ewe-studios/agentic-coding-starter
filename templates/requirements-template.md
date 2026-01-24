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
builds_on: []
related_specs: []
has_features: false
has_fundamentals: false
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
      - ./tasks.md
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
      - ./tasks.md
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
      - ./tasks.md
      - ./requirements.md
      - [feature.md and feature/tasks.md if has_features: true]
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
      - ./tasks.md
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

> **Specification Tracking**: See [tasks.md](./tasks.md) for task progress and [learnings.md](./learnings.md) for implementation insights.
>
> **Agent Instructions**: Review the `files_required` section in frontmatter above. Each agent type has explicit rules and files to load. Load YOUR agent type's requirements before starting work.

## Overview
Brief summary of what this specification covers and why it's needed.

## Requirements Conversation Summary

### User's Initial Request
[Summary of what user initially asked for]

### Clarifying Questions Asked
1. Question about [topic]
   - Answer: [user's response]
2. Question about [topic]
   - Answer: [user's response]
[... all questions and answers ...]

### Final Requirements Agreement
Based on the conversation, we agreed on:
- [Clear statement of final understanding]
- [All important details confirmed]

## Detailed Requirements

### Functional Requirements
1. [Requirement 1]
2. [Requirement 2]

### Non-Functional Requirements
1. [Performance requirements]
2. [Security requirements]

### Technical Specifications
- **Technology Stack:** [Technologies to be used]
- **Dependencies:** [Required libraries/tools]
- **Integration Points:** [How this integrates]

## User-Facing Documentation Requirements (MANDATORY)

**CRITICAL**: If this specification introduces new user-facing features, libraries, or APIs, create a `fundamentals/` directory with comprehensive user documentation.

### Fundamentals Documentation (REQUIRED when has_fundamentals: true)

Create the following documents in `specifications/[NN-spec-name]/fundamentals/`:

1. **00-overview.md** - Introduction, quick start, decision trees
2. **[Additional fundamental docs as needed]** - Deep dives into concepts

**Documentation Principles**:
- **Explain WHY** - Design decisions and trade-offs, not just how
- **Show internals** - Key implementation details with commentary
- **Provide examples** - Compilable, real-world usage examples
- **Discuss trade-offs** - When to use, when NOT to use
- **Be self-contained** - Reader can understand without external resources

**Add fundamentals documentation tasks to tasks.md as HIGH PRIORITY items.**

## Success Criteria

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
- [ ] Zero clippy warnings
- [ ] Zero compiler warnings
- [ ] Code properly formatted
- [ ] All public items documented

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

**NO EXCEPTIONS**: Every task in `tasks.md` MUST be completed.

- [ ] Open `tasks.md` and verify ALL tasks are marked `[x]`
- [ ] Verify `completed` count in frontmatter matches actual `[x]` count
- [ ] Verify `uncompleted` count is `0`
- [ ] Verify `completion_percentage` is `100`
- [ ] NO tasks left as `[ ]` (incomplete)
- [ ] NO optional tasks - everything is mandatory unless user explicitly says otherwise

**Validation Command**:
```bash
# Must return 0
grep -c "^- \[ \]" tasks.md
```

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

- [ ] `tasks.md` shows 100% completion
- [ ] `learnings.md` exists and documents key insights
- [ ] `progress.md` exists and shows timeline/achievements
- [ ] `verification.md` or `VERIFICATION_SIGNOFF.md` exists with verification results
- [ ] `requirements.md` frontmatter has correct `status` field

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

- [ ] All tasks in tasks.md are checked `[x]` (100%)
- [ ] All code exists and works (verified in codebase)
- [ ] All tests pass (100%, no failures/skips)
- [ ] All documentation complete (no stubs/placeholders)
- [ ] All quality checks pass (0 warnings)
- [ ] All verification issues resolved (100% PASS)
- [ ] learnings.md exists and is comprehensive
- [ ] progress.md exists with timeline
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

2. **All Requirements Are Mandatory**:
   - ✅ Unless user explicitly states a requirement is optional, ALL requirements are MANDATORY
   - ✅ All items must be implemented and completed
   - ❌ DO NOT skip requirements thinking they are optional
   - ❌ DO NOT treat any requirement as "nice-to-have" without explicit user confirmation

3. **Task Tracking**:
   - ✅ Update tasks.md immediately after each task completion (see tasks.md for details)
   - ✅ Keep requirements.md and tasks.md in sync at all times

**These rules exist to ensure**:
- Requirements accurately reflect current understanding
- No work is lost or forgotten
- User has real-time visibility into project state
- Future agents have accurate context

---

*Created: [Date]*
*Last Updated: [Date]*
