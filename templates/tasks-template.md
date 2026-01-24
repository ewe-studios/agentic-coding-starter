---
completed: 0
uncompleted: 8
created: YYYY-MM-DD
author: "Main Agent"
metadata:
  version: "1.0"
  last_updated: YYYY-MM-DD
  total_tasks: 8
  completion_percentage: 0
tools:
  - TypeScript
  - Jest
skills: []
---

# ⚠️ DEPRECATED - DO NOT USE

**This template is deprecated.** Tasks are now integrated directly into `requirements.md`.

**Use instead**:
- For simple specifications: Add Tasks section to `requirements.md` (see `requirements-template.md`)
- For feature-based specifications: Add Tasks section to `feature.md` (see `feature-template.md`)

**Reason for deprecation**: Consolidating tasks into requirements/feature files reduces duplication, simplifies agent workflows, and creates self-contained specification documents.

---

# [Specification Name] - Tasks (DEPRECATED STRUCTURE)

## Task List

### Implementation Tasks
- [ ] Task 1
- [ ] Task 2
- [ ] Task 3

### Testing Tasks
- [ ] Write unit tests
- [ ] Write integration tests

### Documentation Tasks
- [ ] Write API documentation
- [ ] Add usage examples

## Notes
- [Any important notes about tasks]

---

## 🤖 Agent Reminders

**CRITICAL RULES - READ EVERY TIME**:

1. **Immediate Task Updates (MANDATORY)**:
   - ✅ Update this tasks.md file IMMEDIATELY after completing each task
   - ✅ Mark task as [x] the MOMENT you finish it
   - ✅ Update frontmatter counts (completed/uncompleted/completion_percentage) immediately
   - ❌ DO NOT wait until you're done with multiple tasks to update
   - ❌ DO NOT create other task tracking files - tasks.md is THE task tracker
   - ❌ DO NOT batch updates - update after EACH task completion

2. **All Tasks Are Mandatory**:
   - ✅ Unless user explicitly states a task is optional, ALL tasks are MANDATORY
   - ✅ All tasks must be completed before marking specification as complete
   - ❌ DO NOT skip tasks thinking they are optional
   - ❌ DO NOT leave tasks unchecked thinking "that can be done later"

3. **Requirements Tracking**:
   - ✅ If you discover new requirements while working, update requirements.md immediately
   - ✅ Keep tasks.md and requirements.md in sync at all times

**These rules exist to ensure**:
- Real-time visibility into task progress
- No completed work is forgotten or lost
- User can see progress as it happens
- Future agents have accurate task status
- tasks.md is the single source of truth for task tracking

**Why immediate updates matter**:
- System crashes won't lose your progress
- User can check status at any time
- Other agents can pick up where you left off
- Completed work is documented immediately

---
*Last Updated: YYYY-MM-DD*
