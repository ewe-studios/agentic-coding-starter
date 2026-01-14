---
name: Specification Update Agent
type: utility
language: language-agnostic
purpose: Update tasks.md after verification, create/delete verification.md, manage specification tracking
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
  - Rule 05
  - Rule 06
  - Rule 08
status: active
---

# Specification Update Agent - Documentation

## Overview
The Specification Update Agent is a utility agent responsible for updating specification files (tasks.md, verification.md) based on verification results. It is spawned by Main Agent AFTER verification completes.

## Purpose and Responsibility
This agent maintains specification accuracy by updating task statuses and creating verification failure reports. Main Agent NEVER updates specification files directly - it always delegates to this agent.

## Agent Type
**Utility** - Specification file management

## Capabilities

### When Verification PASSES
1. Read specifications/[NN-spec-name]/tasks.md
2. Mark completed tasks as [x]
3. Update frontmatter counts (completed/uncompleted)
4. Delete verification.md if it exists (cleanup from previous failure)
5. Save tasks.md
6. Report completion to Main Agent

### When Verification FAILS
1. Create specifications/[NN-spec-name]/verification.md with:
   - Detailed failure report
   - Error messages with line numbers
   - Recommended fixes
2. Add URGENT task to TOP of tasks.md
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
2. Read tasks.md
   ↓
3. If PASS:
   - Mark tasks complete
   - Update counts
   - Delete verification.md if exists
   ↓
4. If FAIL:
   - Create verification.md
   - Add urgent task to tasks.md
   - Update counts
   ↓
5. Save files
   ↓
6. Report to Main Agent
```

## Version History

### Version 1.0 - 2026-01-14
- Initial documentation

---
*Last Updated: 2026-01-14*
