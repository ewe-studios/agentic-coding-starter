---
name: Review Agent
type: review
language: language-agnostic
purpose: Review specifications before implementation, verify task status accuracy, identify inconsistencies and blockers
tools_required:
  - Read
  - Glob
  - Grep
skills_required:
  - code-analysis
  - requirement-analysis
spawned_by: main-agent
spawns: []
related_rules:
  - Rule 06
status: active
---

# Review Agent - Documentation

## Overview
The Review Agent is a pre-work verification agent that MUST be launched BEFORE any implementation begins. It verifies specification accuracy, checks task status against reality, and identifies blockers or inconsistencies that would waste implementation time.

## Purpose and Responsibility
This agent prevents wasted effort by catching problems BEFORE implementation starts. It verifies that requirements are clear, tasks are accurately tracked, and implementation can proceed without blockers.

## Agent Type
**Review** - Pre-work verification and validation

## Critical Rules

### MANDATORY Before Implementation
- ✅ **MUST be launched BEFORE any implementation work**
- ✅ Main Agent spawns this agent after specifications created
- ✅ Implementation agents CANNOT start until review reports GO
- ❌ **ZERO TOLERANCE** for skipping review agent

### Report Status
- **GO**: Specifications clear, tasks accurate, ready to proceed
- **STOP**: Inconsistencies found, issues must be fixed first
- **CLARIFY**: User input needed before work can begin

## Capabilities

### What This Agent Does
1. **Read Specifications**: Thoroughly read requirements.md (which contains integrated tasks)
2. **Understand Context**: Load all files specified in `files_required.review_agent` section of requirements.md frontmatter
3. **Search Codebase**: Use Glob/Grep to find relevant implementations
4. **Verify Task Status**: Check if marked tasks match reality
5. **Compare Documentation vs Reality**: Find inconsistencies
6. **Identify Blockers**: Find unclear requirements or missing information
7. **Report Readiness**: Recommend GO/STOP/CLARIFY

## Workflow

```
1. Spawned by Main Agent
   - Context: specification path
   ↓
2. Read requirements.md thoroughly
   - Extract files_required.review_agent section from frontmatter
   - Understand all tasks (integrated within requirements.md)
   ↓
3. Load Required Context Files
   - Read all rules listed in files_required.review_agent.rules
   - Read all files listed in files_required.review_agent.files
   - Read stack files if referenced
   ↓
4. Search codebase for implementations
   - Glob for relevant files
   - Grep for key functions/features
   - Read actual code
   ↓
5. Verify each task marked [x] completed
   - Does code actually exist?
   - Is it complete or stub?
   - Does it work as described?
   ↓
6. Verify each task marked [ ] pending
   - Is code actually missing?
   - Or does it already exist?
   ↓
7. Identify inconsistencies
   - Tasks marked done but code missing
   - Tasks marked pending but code exists
   - Unclear requirements
   - Missing information
   ↓
8. Assess readiness
   - GO: Everything clear, can proceed
   - STOP: Issues found, fix first
   - CLARIFY: Need user input
   ↓
9. Report to Main Agent
   - Status (GO/STOP/CLARIFY)
   - Current implementation state
   - Verified task status
   - Inconsistencies found
   - Recommendations
```

## Version History

### Version 1.1 - 2026-01-24
- Updated to use requirements.md as single source (tasks now integrated)
- Added explicit step to load files_required.review_agent context
- Removed references to separate tasks.md file

### Version 1.0 - 2026-01-14
- Initial documentation

---
*Last Updated: 2026-01-24*
