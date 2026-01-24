---
name: Specification Update Agent
type: utility
language: language-agnostic
purpose: Update task status in requirements.md after verification, create/delete VERIFICATION.md, manage specification tracking
tools_required:
  - Read
  - Write
  - Edit
skills_required:
  - markdown-editing
  - specification-management
spawned_by: main-agent
spawns: []
related_rules:
  - Rule 06
  - Rule 12
status: active
---

# Specification Update Agent - Documentation

## Overview
The Specification Update Agent is a utility agent responsible for updating task status in requirements.md and managing VERIFICATION.md based on verification results. It is spawned by Main Agent AFTER verification completes.

## Purpose and Responsibility
This agent maintains specification accuracy by updating task statuses and creating verification failure reports. Main Agent NEVER updates specification files directly - it always delegates to this agent.

## Agent Type
**Utility** - Specification file management

## Capabilities

### When Verification PASSES
1. Read specifications/[NN-spec-name]/requirements.md
2. Mark completed tasks as [x] in the tasks section
3. Update frontmatter task counts (completed/uncompleted/completion_percentage)
4. Delete VERIFICATION.md if it exists (cleanup from previous failure)
5. Save requirements.md
6. Report completion to Main Agent

### When Verification FAILS
1. Create specifications/[NN-spec-name]/VERIFICATION.md with:
   - Detailed failure report
   - Error messages with line numbers
   - Recommended fixes
2. Add URGENT task to TOP of tasks section in requirements.md
3. Update frontmatter counts (uncompleted +1)
4. Save both files
5. Report completion to Main Agent

## Workflow

```
1. Spawned by Main Agent with context:
   - Verification status (PASS/FAIL)
   - Verification report
   - Specification path
   - Completed tasks (if PASS)
   ↓
2. Read requirements.md
   ↓
3. If PASS:
   - Mark tasks complete in tasks section
   - Update frontmatter counts
   - Delete VERIFICATION.md if exists
   ↓
4. If FAIL:
   - Create VERIFICATION.md
   - Add urgent task to tasks section in requirements.md
   - Update frontmatter counts
   ↓
5. Save files
   ↓
6. Report to Main Agent
```

## Version History

### Version 1.1 - 2026-01-24
- Updated to work with requirements.md (tasks now integrated)
- Changed references from verification.md to VERIFICATION.md (uppercase per Rule 06)
- Updated to modify tasks section within requirements.md instead of separate tasks.md

### Version 1.0 - 2026-01-14
- Initial documentation

---
*Last Updated: 2026-01-24*
