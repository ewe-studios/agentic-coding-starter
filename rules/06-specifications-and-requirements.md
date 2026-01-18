# Specifications and Requirements Management

## Purpose
This rule establishes a mandatory requirements-gathering and specification-tracking system that ensures all work begins with a documented conversation between the main agent and user, creating a clear record of requirements and tasks in the `specifications/` directory.

## Rule

### Requirements-First Development
Before **ANY** work begins on new features, enhancements, or significant changes, the main agent **MUST**:

1. **Engage in a conversation** with the user about requirements
2. **Document the requirements** in a specification directory
3. **Create a task list** for tracking work progress
4. **Have agents read specifications** before starting work
5. **Verify and update status** as work progresses

### No Exceptions
- **NO coding** without documented requirements
- **NO starting work** without a specification
- **NO skipping** the requirements conversation
- This applies to **ALL significant development work**

### Requirements Conversation Process (MANDATORY)

**CRITICAL REQUIREMENT**: The Main Agent **MUST NOT** passively accept the user's initial request. Instead, the Main Agent **MUST** actively engage in a detailed conversation to fully understand requirements.

#### Main Agent Responsibilities During Conversation

**The Main Agent MUST**:

1. ✅ **Listen to the initial request carefully**
   - Understand what the user is asking for
   - Identify the high-level goal or problem to solve

2. ✅ **Ask clarifying questions proactively**
   - **NEVER assume** details not explicitly stated
   - **ALWAYS ask** when requirements are ambiguous
   - **ALWAYS probe** for edge cases and constraints
   - **ALWAYS confirm** technical approaches before documenting

3. ✅ **Ask questions in these critical areas**:
   - **Scope**: What exactly should be included/excluded?
   - **Technical Approach**: Which technologies, patterns, or architectures?
   - **Constraints**: Performance requirements, limitations, dependencies?
   - **Success Criteria**: How will we know when this is complete?
   - **Edge Cases**: What unusual scenarios should be handled?
   - **Integration**: How does this fit with existing systems?
   - **Priority**: What's most important if trade-offs are needed?
   - **Timeline**: Are there deadlines or milestones?

4. ✅ **Continue asking until all details are clear**
   - Don't stop after one round of questions
   - If answers reveal new ambiguities, ask more questions
   - Ensure every requirement is specific and actionable

5. ✅ **Confirm understanding before documenting**
   - Summarize what you understood
   - Ask user to confirm your summary is correct
   - Make corrections based on user feedback

**The Main Agent MUST NOT**:

- ❌ Accept vague requests without clarification
- ❌ Make assumptions about unspecified requirements
- ❌ Skip questioning to "save time"
- ❌ Proceed with incomplete understanding
- ❌ Document requirements without user confirmation

#### Minimum Questions Required

**Main Agent MUST ask AT LEAST**:
- ✅ 3-5 clarifying questions for small features
- ✅ 5-10 clarifying questions for medium features
- ✅ 10+ clarifying questions for large/complex features

**If fewer questions are needed**, it may indicate:
- The user provided exceptionally detailed initial requirements (rare)
- OR the Main Agent is not probing deeply enough (more likely - ASK MORE)

#### Examples

**❌ BAD - Passive Acceptance:**
- User: "Add user authentication to the app"
- Main Agent: "Ok, I'll create a specification for user authentication."
- **VIOLATION**: No clarifying questions asked

**✅ GOOD - Active Engagement:**
- User: "Add user authentication to the app"
- Main Agent asks about: authentication method, user data storage, security requirements, self-registration, password reset, session timeout, rate limiting, etc.
- User provides detailed answers
- Main Agent confirms understanding
- Main Agent creates specification with comprehensive requirements
- **CORRECT**: Thorough questioning, confirmation, then documentation

### Main Agent Frontmatter Enforcement (CRITICAL)

**Main Agent MUST validate and enforce complete frontmatter** when creating or updating specifications.

#### When Creating specifications/*/requirements.md:

Main Agent **MUST** include ALL required frontmatter fields:
- ✅ `description`: One-sentence summary
- ✅ `status`: in-progress | completed | blocked
- ✅ `priority`: high | medium | low
- ✅ `created`: YYYY-MM-DD (date of creation)
- ✅ `author`: "Main Agent" or "User Name"
- ✅ `metadata`: Complete object with:
  - `version`: "1.0" (semantic version)
  - `last_updated`: YYYY-MM-DD
  - `estimated_effort`: small | medium | large | xl
  - `tags`: Array with minimum 1 tag
- ✅ `builds_on`: (if applicable) Array of parent specs
- ✅ `related_specs`: (if applicable) Array of related specs

#### When Creating specifications/*/tasks.md:

Main Agent **MUST** include ALL required frontmatter fields:
- ✅ `completed`: Count of [x] tasks
- ✅ `uncompleted`: Count of [ ] tasks
- ✅ `created`: YYYY-MM-DD
- ✅ `author`: "Main Agent" or "User Name"
- ✅ `metadata`: Complete object with:
  - `version`: "1.0"
  - `last_updated`: YYYY-MM-DD
  - `total_tasks`: completed + uncompleted
  - `completion_percentage`: (completed / total) * 100
- ✅ `tools`: Array of tools/technologies
- ✅ `skills`: (if applicable) Array of skill names

#### Validation and Updates:

Before creating any specification file, Main Agent MUST:
1. **Check frontmatter completeness** - All REQUIRED fields present
2. **Validate field values** - Status, priority, dates are valid
3. **Calculate derived fields** - completion_percentage from counts
4. **Report if validation fails** - Stop and request correction

When sub-agents update specifications:
- ✅ Sub-agents MUST update `metadata.last_updated`
- ✅ Sub-agents MUST increment `metadata.version` if significant changes
- ✅ Sub-agents MUST update counts in tasks.md
- ✅ Sub-agents MUST add new tools to `tools` array
- ❌ Sub-agents MUST NOT modify other frontmatter without approval

## Directory Structure

### Overview
```
specifications/
├── Spec.md                          # Master index of all specifications
├── 01-specification-name/
│   ├── requirements.md              # (MANDATORY) Requirements and conversation summary
│   ├── tasks.md                     # (MANDATORY) Task list with checkboxes
│   ├── PROGRESS.md                  # (MANDATORY) Mid-work progress report
│   ├── FINAL_REPORT.md              # (MANDATORY) Comprehensive completion summary
│   ├── VERIFICATION_SIGNOFF.md      # (MANDATORY) Official verification report
│   └── LEARNINGS.md                 # (MANDATORY) Lessons learned and insights
├── 02-another-specification/
│   └── [same structure]
└── ...

documentation/
├── module-1/
│   ├── doc.md                       # (MANDATORY) Detailed module documentation
│   ├── diagrams/                    # (OPTIONAL) Architecture diagrams
│   └── assets/                      # (OPTIONAL) Additional assets
└── ...
```

**CRITICAL**: The `documentation/` directory exists at project root level, parallel to `specifications/`.

### Naming Convention
- Each specification gets its own numbered directory
- Format: `NN-descriptive-name/` where NN is two-digit (01, 02, 03, etc.)
- Use dash separators for multi-word names

**Examples:**
- ✅ `01-build-http-client/`
- ✅ `02-implement-user-authentication/`
- ❌ `http-client/` (missing number prefix)
- ❌ `1-http-client/` (single digit)

### Specification Versioning and Evolution (CRITICAL)

**MANDATORY RULE**: Once a specification has been marked as **completed** (status: completed, with FINAL_REPORT.md and VERIFICATION_SIGNOFF.md), that specification is **immutable** and represents historical fact.

**Any new additions, changes, or enhancements to a completed specification MUST become a new specification.**

#### Why This Matters:
- **Historical Record**: Preserves complete history of requirements and implementations
- **Traceability**: Clear lineage showing how features evolved over time
- **Audit Trail**: Know exactly what was done, when, and why
- **No Confusion**: Prevents mixing old and new requirements

#### When Specification is Completed:

If a specification has:
- ✅ Status: `completed` in requirements.md
- ✅ FINAL_REPORT.md created
- ✅ VERIFICATION_SIGNOFF.md created
- ✅ All tasks marked as complete

Then that specification is **DONE** and **LOCKED**.

#### Adding to Completed Specification:

When user requests new work related to a completed specification:

1. **Main Agent MUST create a NEW specification** (next available number)
2. **New specification MUST reference the old specification** in requirements.md
3. **New specification explains how it builds upon the old one**
4. **Old specification remains untouched** (historical record)

**requirements.md frontmatter MUST include `builds_on` field:**

```markdown
---
description: [New enhancement description]
status: in-progress
builds_on:
  - specifications/NN-original-spec-name
related_specs:
  - specifications/PP-related-spec
---
```

#### Exception: In-Progress Specifications

Specifications that are **NOT completed** can be modified:
- Status is NOT "completed"
- No FINAL_REPORT.md exists
- Work is still ongoing

## Frontmatter Fields Reference

### requirements.md Frontmatter

**REQUIRED Fields:**

- **`description`**: One-sentence summary
- **`status`**: Current state (in-progress | completed | blocked)
- **`priority`**: Importance level (high | medium | low)
- **`created`**: Date specification created (YYYY-MM-DD)
- **`author`**: Who created it ("Main Agent", "John Doe", etc.)
- **`metadata`**: Structured metadata object
  - **`version`**: Semantic version (e.g., "1.0")
  - **`last_updated`**: Date of last update (YYYY-MM-DD)
  - **`estimated_effort`**: Size estimate (small | medium | large | xl)
  - **`tags`**: Array of tags (lowercase with hyphens, minimum 1)

**OPTIONAL Fields:**

- **`builds_on`**: Array of parent specifications (creates lineage chain)
- **`related_specs`**: Array of related specifications (context only)

### tasks.md Frontmatter

**REQUIRED Fields:**

- **`completed`**: Total count of [x] completed tasks
- **`uncompleted`**: Total count of [ ] pending tasks
- **`created`**: Date tasks file created (YYYY-MM-DD)
- **`author`**: Who created it
- **`metadata`**: Structured metadata object
  - **`version`**: Semantic version
  - **`last_updated`**: Date of last update
  - **`total_tasks`**: completed + uncompleted
  - **`completion_percentage`**: (completed / total) * 100
- **`tools`**: List of tools/technologies used

**OPTIONAL Fields:**

- **`skills`**: List of skill names from `.agents/skills/`

## File Templates

### requirements.md Template

```markdown
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
builds_on: []
related_specs: []
---

# [Specification Name] - Requirements

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

### Success Criteria
- [ ] Criterion 1
- [ ] Criterion 2

## Module Documentation References

This specification modifies the following modules:

### [Module Name]
- **Documentation**: `documentation/[module]/doc.md`
- **Purpose**: [Brief summary]
- **Changes Needed**: [What will be changed]

**CRITICAL**: Agents MUST read module documentation BEFORE making changes.

## Important Notes for Agents

### Before Starting Work
- **MUST READ** both requirements.md and tasks.md
- **MUST VERIFY** completion status by searching codebase
- **MUST UPDATE** tasks.md to reflect actual status
- **MUST ADD** new tasks BEFORE starting work

### Verification Requirements
Agents **MUST**:
1. Search codebase for relevant implementations
2. Verify code exists and works as specified
3. Update task status based on findings
4. Mark completed only when fully verified

---
*Created: [Date]*
*Last Updated: [Date]*
```

### tasks.md Template

```markdown
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

# [Specification Name] - Tasks

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
*Last Updated: YYYY-MM-DD*
```

## Mandatory Documentation Files

Every specification **MUST** have these 6 files:

### 1. requirements.md (Created at Start)
Documents requirements and conversation summary. See template above.

### 2. tasks.md (Created at Start)
Task list with checkboxes. See template above.

### 3. PROGRESS.md (Created at ~50% Completion)

**When to Create**: At 40-60% completion or major phase transitions.

**Template:**
```markdown
# [Specification Name] - Progress Report

## Overall Status: [X%] Complete

### Completed Work
- ✅ [Completed task/feature]
- ✅ [Another completed item]

### Current Status
[What you're currently working on]

### Remaining Work
- [ ] [Task to be done]
- [ ] [Another pending task]

### Blockers/Issues
[Any problems or blockers, or "None"]

### Statistics
- Files modified: [N]
- Lines changed: [N]
- Tests added: [N]

### Next Steps
1. [Immediate next action]
2. [Following action]

---
*Progress Report Created: [Date and Time]*
```

### 4. FINAL_REPORT.md (Created When Work Complete)

**When to Create**: When all tasks are 100% complete.

**Template:**
```markdown
# [Specification Name] - Final Report

## Mission Accomplished! 🎉

All work for this specification has been completed successfully.

## Work Completed ([X]/[Y] tasks)

**Status**: ✅ All tasks complete
**Completion**: 100%

### Task Breakdown
- Implementation: [X]/[Y] tasks
- Testing: [X]/[Y] tasks
- Documentation: [X]/[Y] tasks

## Detailed Accomplishments

### Implementation
- [Accomplishment 1]
- [Accomplishment 2]

### Testing
- [Test coverage details]

### Documentation
- [Documentation created]

## Commits Created
- [commit-hash]: [commit message]
- [commit-hash]: [commit message]

## Remaining Work
[List any deferred items, or "None - all work complete"]

## Statistics
- Total files modified: [N]
- Total lines changed: [+N -M]
- Tests added: [N]
- Test coverage: [X%]

## Verification Results
✅ [Verification check passed]
✅ [Another check passed]

## Impact
**Before**: [State before this work]
**After**: [State after this work]

**Benefits**:
- [Benefit 1]
- [Benefit 2]

## Recommendation
✅ **Ready for merge/deployment**

[Or if issues exist: ⚠️ Conditional approval with notes]

---
*Final Report Created: [Date and Time]*
```

### 5. LEARNINGS.md (Created at Completion)

**When to Create**: When work is substantially complete.

**Template:**
```markdown
# [Specification Name] - Learnings

## Overview
[Brief summary of what was learned during this specification]

## Key Insights
1. **[Insight Category]**
   - [Specific learning or discovery]
   - [Why this matters]

2. **[Another Insight]**
   - [Details]

## Challenges and Solutions

### Challenge: [Challenge Description]
**Problem**: [What the problem was]
**Solution**: [How it was solved]
**Outcome**: [Result]

## Best Practices Discovered
- [Best practice 1]
- [Best practice 2]

## Anti-Patterns to Avoid
- [What not to do and why]

## Recommendations for Future Work
1. [Recommendation for next time]
2. [Another recommendation]

## Knowledge Gained

### Technical Knowledge
- [New technical knowledge acquired]

### Process Knowledge
- [Process improvements identified]

## Documentation Improvements Needed
- [Documentation gaps found]

## Technical Debt Identified
- [Technical debt discovered]

## Success Factors
[What made this specification successful]

---
*Learnings Documented: [Date]*
```

### 6. VERIFICATION_SIGNOFF.md (Created After Verification)

**When to Create**: After verification agent completes final verification.

**Template:**
```markdown
# [Specification Name] - Verification Sign-Off

## Executive Summary
**Specification**: [NN]: [Specification Name]
**Verification Date**: [YYYY-MM-DD]
**Verification Agent**: [Agent Name]
**Status**: ✅ APPROVED | ⚠️ APPROVED WITH NOTES | ❌ REJECTED
**Confidence Level**: [High/Medium/Low]

## ✅ Verification Results

### Requirements Compliance
- [X]/[Y] requirements fully met
- [ ] [Any unmet requirements]

**Compliance Rating**: [X%]

### Code Quality
✅ [Quality check passed]
✅ [Another check passed]
⚠️ [Warning or note if any]

### Testing Coverage
- Unit tests: [X%]
- Integration tests: [X%]
- Overall coverage: [X%]

## 📊 Quality Assessment

### Performance
[Performance assessment results]

### Security
[Security assessment results]

### Maintainability
[Code maintainability assessment]

## 🎯 Specification Compliance

### Functional Requirements
- ✅ [Requirement]: Fully implemented
- ✅ [Requirement]: Fully implemented

### Non-Functional Requirements
- ✅ [Requirement]: Met
- ✅ [Requirement]: Met

## ⚠️ Issues Found

[List any issues, or "None - all checks passed"]

## 🏆 Final Verdict

**Status**: ✅ APPROVED FOR PRODUCTION

[Or if conditional: ⚠️ APPROVED WITH RECOMMENDATIONS]

**Recommendation**: Ready for merge and deployment

**Conditions** (if any):
- [Condition 1]

## 📝 Verification Checklist

- [x] All requirements implemented
- [x] All tests passing
- [x] Code quality standards met
- [x] Documentation complete
- [x] No critical issues found

## 🚀 Sign-Off

**Verified By**: [Agent Name]
**Date**: [YYYY-MM-DD]
**Signature**: [Agent Identifier]

---
*Official Verification Report*
*This specification has been formally verified and approved*
```

## Module Documentation System (MANDATORY)

### Purpose
The `documentation/` directory provides living, detailed documentation of individual code modules. This ensures agents have clear understanding of what each module implements **BEFORE** making changes.

### Why Module Documentation Is Critical

**Without Module Documentation:**
- Agents waste time using Grep/Glob to understand code
- Agents miss critical context about module purpose
- Agents make changes without understanding full impact
- No central place to understand module architecture

**With Module Documentation:**
- Agents read `documentation/[module]/doc.md` for immediate understanding
- Clear documentation of what module implements, imports, calls
- Line number references to key implementations
- Faster onboarding and safer changes

### Context Window Management

**CRITICAL OPTIMIZATION**: Large documentation (>8-10KB) wastes context.

**Main Agent Responsibility:**
- DOES NOT load large documentation into own context
- Delegates to sub-agents who work with the module
- References path, instructs sub-agents to read and update

**Sub-Agent Responsibility:**
- Load documentation if reasonable size (<8-10KB)
- If too large: Use Grep/Glob/Read tools instead
- Update documentation when making structural changes
- Report documentation status to Main Agent

**When Documentation Too Large for Sub-Agent:**
- Skip loading, use tools exclusively
- Complete implementation work
- Report to Main Agent: "Documentation too large, requesting Documentation Agent"
- Main Agent spawns Documentation Agent to update docs

### When Module Documentation Is Created

**After requirements.md Completed:**

1. **Main Agent identifies affected modules** (existing or new)
2. **Spawns Documentation Agent(s)** to create/update module docs
3. **Documentation agents verify accuracy** against actual code
4. **If mismatch found: STOP, fix docs first**
5. **Main Agent references docs in requirements.md**
6. **Main Agent commits module documentation**

### Documentation Agent Must STOP If Mismatch Found

If documentation doesn't match actual code:

1. **STOP immediately**
2. **Report to Main Agent** with detailed mismatch findings
3. **Main Agent halts specification work**
4. **Documentation updated FIRST**
5. **Only then resume implementation**

### doc.md File Structure

Every `documentation/[module]/doc.md` **MUST** contain:

```markdown
---
module: [Exact module name]
language: [rust|javascript|typescript|python|go|etc]
status: [active|deprecated|experimental|planning]
last_updated: [YYYY-MM-DD]
maintainer: [Primary agent/team]
related_specs:
  - specifications/NN-spec-name
---

# [Module Name] - Documentation

## Overview
[2-3 sentence summary of what this module does]

## Purpose and Responsibility
[Detailed explanation of module's purpose and role]

## Module Location
- **Path**: `[exact file path]`
- **Entry Point**: `[main file]`
- **Language**: [language and version]

## What It Implements

### Core Functionality
1. **[Feature Name]** (Line [NNN-MMM])
   - What: [What this does]
   - Why: [Why it exists]
   - How: [How it works]
   - Key Functions: `function_name()`

### Public API
**Exported Functions:**
- `function_name(args) -> return` (Line NNN): [Purpose]

**Exported Types/Classes:**
- `TypeName` (Line NNN): [Purpose]

## What It Imports

### External Dependencies
- `dependency-name` (v1.2.3): [Why used]

### Internal Dependencies
- `internal/module`: [What imported and why]

## What It Calls

### External Function Calls
- **Database**: Calls `db.query()` (Lines NNN-MMM)
  - Purpose: [Why]

### Internal Function Calls
- **Helpers**: `helper.validate()` (Lines NNN-MMM)

## What It Does (Step-by-Step)

### Primary Workflows

#### Workflow 1: [Name]
1. **Input**: [What triggers]
2. **Processing**:
   - Step 1: [Action] (Line NNN)
   - Step 2: [Action] (Line MMM)
3. **Output**: [What produces]
4. **Error Handling**: [How errors handled]

### Edge Cases
- **Case 1**: [Description] (Lines NNN-MMM)
  - Condition: [When]
  - Handling: [How handled]

## Architecture

### Design Patterns
- **Pattern Name**: [How/why used]

### Module Structure
```
module-directory/
├── main_file.ext         # [Purpose]
├── submodule.ext         # [Purpose]
└── tests/                # [Tests]
```

## Key Implementation Details

### Performance Considerations
- [Performance notes] (Lines NNN-MMM)

### Security Considerations
- [Security notes] (Lines NNN-MMM)

## Tests

### Test Coverage
- **Unit Tests**: `[path]` - [XX%] coverage
- **Integration Tests**: `[path]` - [XX%] coverage

## Dependencies and Relationships

### Depends On
- **Module A**: [Why]

### Used By
- **Module C**: [How used]

## Configuration

### Environment Variables
- `ENV_VAR`: [Purpose and default]

## Known Issues and Limitations

### Current Limitations
1. **[Limitation]**: [Description]

### Known Bugs
- **[Bug]**: [Impact] (Issue #NNN)

### Technical Debt
- **[Debt]**: [Description]

## Future Improvements

### Planned Enhancements
- **[Enhancement]**: [Description]

## Related Documentation

### Specifications
- [Links to related specs]

## Version History

### [Version X.Y.Z] - YYYY-MM-DD
- [Major changes]

---
*Last Updated: [Date]*
```

### Implementation Agent Workflow With Module Docs

When implementation agent spawned:

1. **Read specification files** (requirements.md, tasks.md)
2. **Read module documentation** (all referenced in requirements.md)
3. **Verify docs match reality** (spot check key functions)
4. **If mismatch: STOP, report to Main Agent**
5. **Implement changes** with full context
6. **Update module docs** if structure changes
7. **Report completion** with documentation status

## Spec.md File (Master Index)

### Purpose
Central index and dashboard for all specifications.

### Required Contents

```markdown
# Project Specifications

## Overview
This directory contains all project specifications. Each specification represents a significant feature or change.

## How Specifications Work
1. **Requirements-First**: Main agent discusses requirements with user
2. **Documentation**: Requirements and tasks documented in numbered directories
3. **Agent Reading**: Agents MUST read requirements.md and tasks.md before work
4. **Status Verification**: Agents MUST verify by searching codebase
5. **Task Updates**: Agents MUST update tasks.md as work progresses

## All Specifications

### [01: Build HTTP Client](./01-build-http-client/)
**Status:** ✅ Completed
**Description:** RESTful HTTP client

### [02: User Authentication](./02-user-authentication/)
**Status:** 🔄 In Progress
**Description:** JWT-based authentication

### [03: Database Migrations](./03-database-migrations/)
**Status:** ⏳ Pending
**Description:** Migration system

## Status Dashboard

### Summary
- **Total:** 3
- **Completed:** 1 (33%)
- **In Progress:** 1 (33%)
- **Pending:** 1 (33%)

### Completed ✅
- 01: Build HTTP Client

### In Progress 🔄
- 02: User Authentication

### Pending ⏳
- 03: Database Migrations

---
*Last updated: YYYY-MM-DD*
```

## Pre-Work Review Agent Requirement

### MANDATORY REVIEW AGENT REQUIREMENT

Before **ANY** agent starts work on tasks, a **review agent MUST be launched first**. This is a **HARD REQUIREMENT** with **NO EXCEPTIONS**.

#### Review Agent Purpose

1. **Read specification files thoroughly** (requirements.md, tasks.md)
2. **Analyze current codebase** (search for implementations)
3. **Compare reality vs documentation** (verify accuracy)
4. **Verify task status accuracy** (check each [x] and [ ])
5. **Identify issues and blockers** (unclear requirements, inconsistencies)
6. **Assess work readiness** (GO/STOP/CLARIFY)

#### STOP WORK IF

Review agent **MUST STOP ALL WORK** if:
- ❌ Inconsistencies found (tasks marked wrong)
- ❌ Requirements unclear or incomplete
- ❌ Tasks need refinement
- ❌ User input required
- ❌ Conflicting information
- ❌ Technical blockers

#### Report to Main Agent

Review agent **MUST** report:
1. **Current implementation state** (what exists)
2. **Verified task status** (accurate completion status)
3. **Inconsistencies found** (specific tasks incorrectly marked)
4. **Readiness assessment** (GO/STOP/CLARIFY)
5. **Recommendations** (corrections needed)

#### Enforcement - Zero Tolerance

- ❌ **FORBIDDEN**: Starting work without review agent first
- ❌ **FORBIDDEN**: Skipping review agent "to save time"
- ❌ **FORBIDDEN**: Assuming specifications are accurate
- ❌ **FORBIDDEN**: Proceeding when review reports STOP/CLARIFY

**VIOLATION CONSEQUENCES:**
1. Immediately stopped
2. Work discarded
3. Review agent run properly
4. Violation reported to user

## Workflow

### Complete Requirements-to-Implementation Workflow

```
1. User Requests Feature
   ↓
2. Main Agent Conversation with User
   ├─ Ask clarifying questions
   ├─ Understand full requirements
   ├─ Confirm technical approach
   └─ Get user agreement
   ↓
3. Create Specification Directory
   ├─ Determine next number
   ├─ Create specifications/NN-feature-name/
   └─ Create requirements.md and tasks.md
   ↓
4. Document Requirements
   ├─ Fill frontmatter completely
   ├─ Write conversation summary
   ├─ List detailed requirements
   └─ Include agent notes
   ↓
4.5 Create/Update Module Documentation (MANDATORY)
   ├─ Identify affected modules
   ├─ Spawn Documentation Agent(s)
   ├─ Agents create/update documentation/[module]/doc.md
   ├─ Agents verify accuracy against code
   ├─ If mismatch: STOP, fix docs first
   ├─ Main Agent updates requirements.md with references
   └─ Main Agent commits module documentation
   ↓
5. Create Task List
   ├─ Fill frontmatter completely
   ├─ Break down work into tasks
   ├─ List all tools needed
   └─ All tasks start as [ ]
   ↓
6. Update Spec.md Master Index
   ├─ Add new specification
   ├─ Update status dashboard
   └─ Link to specification
   ↓
7. Commit Specification Files
   ├─ git add specifications/
   ├─ git commit (following Rule 03)
   ├─ git push (following Rule 05)
   └─ Verify success
   ↓
8. LAUNCH REVIEW AGENT (MANDATORY)
   ├─ Review agent reads requirements.md and tasks.md
   ├─ Review agent searches codebase
   ├─ Review agent verifies task accuracy
   ├─ Review agent identifies inconsistencies
   ├─ Review agent reports: GO / STOP / CLARIFY
   └─ Main agent evaluates report
      ↓
      IF GO: Continue to Step 9
      IF STOP/CLARIFY: Address issues, re-review
   ↓
9. Launch Specialized Agents (Rule 04)
   ├─ Agents MUST read requirements.md
   ├─ Agents MUST read tasks.md
   ├─ Agents MUST read review report
   ├─ Agents MUST read module documentation
   ├─ Agents MUST verify docs match reality
   ├─ If mismatch: STOP, report to Main Agent
   ├─ Agents work on verified tasks
   └─ Agents update module docs if changes occur
   ↓
10. Agent Updates During Work
    ├─ Add new tasks BEFORE starting work
    ├─ Update task checkboxes as complete
    ├─ Update frontmatter counts
    ├─ Update tools list
    └─ Commit changes after updates
   ↓
11. Create PROGRESS.md (MANDATORY)
    ├─ At ~40-60% completion checkpoint
    ├─ Document completed, current, remaining work
    ├─ Include statistics and blockers
    └─ Commit PROGRESS.md
   ↓
12. Complete All Implementation Work
    ├─ Finish all tasks
    ├─ Update tasks.md to reflect completion
    ├─ Search codebase to verify
    └─ Ensure all works and tests pass
   ↓
13. Create FINAL_REPORT.md (MANDATORY)
    ├─ When 100% complete
    ├─ Comprehensive summary
    ├─ Statistics and achievements
    ├─ List all commits
    ├─ Document impact
    └─ Commit FINAL_REPORT.md
   ↓
14. Create LEARNINGS.md (MANDATORY)
    ├─ Document lessons learned
    ├─ Capture challenges and solutions
    ├─ Include best practices
    ├─ Recommendations for future
    ├─ Identify technical debt
    └─ Commit LEARNINGS.md
   ↓
15. Run Final Verification (MANDATORY)
    ├─ Launch verification agent
    ├─ Check all requirements met
    ├─ Validate code quality
    ├─ Provide sign-off report
    └─ Receive GO/STOP assessment
   ↓
16. Create VERIFICATION_SIGNOFF.md (MANDATORY)
    ├─ After verification completes
    ├─ Official verification report
    ├─ Compliance rating
    ├─ All checks documented
    ├─ APPROVED/REJECTED status
    └─ Commit VERIFICATION_SIGNOFF.md
   ↓
17. Final Status Updates
    ├─ Update requirements.md: status → completed
    ├─ Update tasks.md: ensure accurate
    ├─ Update Spec.md with completion
    ├─ Commit all final updates
    └─ Final push
```

## Verification Requirements

### Critical: Verify Actual Implementation

Agents **MUST NOT** trust status or checkboxes blindly. Instead:

1. **Search the Codebase**
   - Use Glob to find relevant files
   - Use Grep to search for implementations
   - Read key files to verify functionality

2. **Verify Implementation Quality**
   - Code exists and is not stub
   - Tests present and passing
   - Documentation complete
   - Integration points work

3. **Update Status Accordingly**
   - Mark [x] only if truly implemented
   - Change to [ ] if missing/incomplete
   - Update frontmatter counts to match reality
   - Update requirements.md status if complete

4. **Report Discrepancies**
   - If status says "completed" but missing code, report
   - If tasks marked done but code doesn't exist, correct
   - User should know if specs are inaccurate

## Examples

### Good Practice ✅

**Example 1: Starting with Review Agent**
- User requests caching layer
- Main Agent asks clarifying questions (strategy, storage, invalidation)
- User provides answers
- Agent creates specification with full conversation
- Agent commits specification
- **Agent launches REVIEW AGENT FIRST** ✅
- Review agent reads, searches codebase, verifies, reports "GO"
- Agent launches implementation agents
- Agents read specs, verify status, work efficiently

**Example 2: Review Agent Finding Inconsistencies**
- Main agent assigned to database migrations spec
- **Main agent launches REVIEW AGENT FIRST** ✅
- Review agent finds: 2 tasks marked [x] but code missing, 1 task marked [ ] but code exists
- Review agent reports "STOP - Found inconsistencies"
- Main agent corrects tasks.md based on findings
- Main agent commits corrections
- NOW launches implementation agents with accurate status

**Example 3: Review Agent Requesting Clarification**
- Main agent on payment integration spec
- **Review agent runs first** ✅
- Review finds unclear requirements (no provider specified, no retry strategy)
- Review reports "CLARIFY - User input required"
- Main agent asks user for clarifications
- User provides details
- Main agent updates specification
- Review agent runs again, reports "GO"
- Implementation proceeds with clear requirements

### Bad Practice ❌

**Example 1: Starting Without Review Agent**
- User: "Implement authentication"
- Main Agent creates spec, commits
- **Immediately launches implementation WITHOUT review** ❌
- Agents assume task statuses are accurate
- Discover "completed" tasks aren't done
- Waste 3 hours on wrong approach
- **CRITICAL VIOLATION**: Skipped mandatory review agent

**Example 2: Starting Without Requirements**
- User: "Add authentication"
- Main Agent launches agents immediately ❌
- No requirements conversation
- No specification created
- No review agent (no spec to review!)
- Agents don't know what to implement
- User expectations not met

**Example 3: Ignoring Review Agent STOP**
- Review agent reports "STOP - 5 tasks marked wrong"
- Main agent IGNORES and launches implementation anyway ❌
- Implementation agents work based on wrong status
- Build features that break
- Hours wasted debugging
- User upset: "Why didn't you verify first?"
- **CRITICAL VIOLATION**: Ignored STOP directive

**Example 4: Not Verifying Status**
- Agent reads spec: "[x] Implement Redis adapter"
- Agent assumes it's done ❌
- Actually no Redis code exists
- Leaves incomplete work as "completed"
- **VIOLATION**: Didn't search codebase to verify

## Rationale

### Why Requirements-First Development
1. **Clear Direction**: Agents know what to implement
2. **User Alignment**: Work meets expectations
3. **Scope Control**: Prevents unnecessary work
4. **Better Planning**: Estimate effort, identify dependencies
5. **Documentation**: Permanent record of decisions
6. **Onboarding**: Understand project evolution

### Why Verification is Critical
1. **Accuracy**: Status reflects reality
2. **Trust**: User relies on correct information
3. **Quality**: Ensures work is actually done
4. **Debugging**: Prevents confusion
5. **Handoffs**: Next agent gets accurate state

### Why Review Agent is Mandatory
Saves hours of wasted effort by:
- Verifying task accuracy before implementation
- Identifying unclear requirements needing clarification
- Catching inconsistencies between docs and code
- Preventing work based on false assumptions
- Ensuring specifications are actionable

### Why Module Documentation is Mandatory
Saves hours by:
- Providing immediate module understanding
- Preventing breaking changes from misunderstanding
- Catching documentation drift before implementation
- Giving clear context instead of forcing grep/glob searches
- Ensuring all agents have same accurate understanding

### Why Documentation Files are Mandatory
- **PROGRESS.md**: Mid-work visibility, tracks momentum
- **FINAL_REPORT.md**: Official completion record
- **LEARNINGS.md**: Knowledge transfer for future work
- **VERIFICATION_SIGNOFF.md**: Formal quality assurance

## Enforcement

### Mandatory Compliance

All agents **MUST**:
- Never begin work without documented requirements
- Create specification directory before implementation
- **Engage in thorough requirements conversation** (3-10+ questions)
- **Never passively accept user requests without clarification**
- Document requirements conversation thoroughly
- Create comprehensive task list
- **Create/verify module documentation after requirements**
- **Launch review agent BEFORE any implementation**
- **Read review agent's report before proceeding**
- **Stop work if review reports STOP or CLARIFY**
- Read requirements.md, tasks.md, and module docs before working
- Verify status by searching codebase
- Update tasks.md as work progresses
- **Create all 6 mandatory documentation files**
- Commit specification changes following Rule 03 and Rule 05

### Violations

**CRITICAL Violations:**
- ❌ Passively accepting user request without questions
- ❌ Making assumptions about unspecified requirements
- ❌ Documenting requirements without user confirmation
- ❌ Asking fewer than minimum required questions
- ❌ Starting implementation without running review agent first
- ❌ Ignoring review agent's STOP or CLARIFY directive
- ❌ Proceeding when review identifies blockers
- ❌ Starting implementation without module documentation
- ❌ Not verifying module documentation accuracy
- ❌ Proceeding when module docs don't match code
- ❌ Completing spec without PROGRESS.md
- ❌ Completing spec without FINAL_REPORT.md
- ❌ Completing spec without LEARNINGS.md
- ❌ Completing spec without VERIFICATION_SIGNOFF.md
- ❌ Marking complete without running verification

**Standard Violations:**
- Starting work without documented requirements
- Not creating specification directory
- Skipping requirements conversation
- Incomplete conversation summary
- Trusting task status without verifying
- Not updating tasks.md during work
- Not updating frontmatter counts
- Starting work on tasks not yet added
- Incomplete requirements documentation
- Not updating Spec.md master index

### User Impact

Violations cause:
- **User frustration**: Work doesn't meet expectations
- **Wasted effort**: Wrong or unnecessary implementation
- **Lost context**: Future agents don't understand requirements
- **False progress**: Status shows completion when incomplete
- **Confusion**: User can't understand what's been done
- **Rework**: Need to redo due to misunderstanding
- **Time waste**: Hours wasted on false assumptions
- **Trust erosion**: User loses confidence in agents
- **Breaking changes**: Bugs from misunderstanding modules
- **Documentation drift**: Docs become useless

**THE USER WILL BE UPSET** if work proceeds without:
- Proper requirements documentation and conversation
- Status verification
- Mandatory review agent execution
- Accurate module documentation
- All mandatory documentation files

### Corrective Action

When violation occurs:
1. **Stop immediately** if work started without requirements/review
2. **Launch review agent** if skipped (CRITICAL)
3. **Read and act on review report** (MANDATORY)
4. **Do not proceed** if review reports STOP/CLARIFY
5. **Create/verify module documentation** if missing (CRITICAL)
6. **Spawn documentation agent** to verify/update if needed
7. **Do not proceed** if module docs don't match reality
8. **Create specification** if missing
9. **Document requirements** via conversation with user
10. **Create task list** before proceeding
11. **Verify status** by searching codebase
12. **Update files** to reflect accurate status
13. **Commit changes** following proper workflow
14. **Re-run review agent** if specs updated
15. **Re-verify module docs** if code changed
16. **Only proceed** when review reports GO and module docs accurate

## Special Cases

### Small Bug Fixes
Very small bug fixes (single line) may not require full specification:
- Use judgment: if documentation takes longer than fix, proceed
- Still commit with detailed message per Rule 03
- Consider adding to existing specification if related

### Urgent Hotfixes
For critical production issues:
- Fix issue immediately
- Document requirements retroactively
- Create specification documenting what was done and why

### Research Tasks
For research/exploration without implementation:
- Create specification with research questions
- Document findings in requirements.md
- Use tasks.md to track research activities

### Documentation-Only Changes
For pure documentation updates:
- May not need full specification
- Use judgment based on scope
- Major documentation overhauls should get specification

## Integration with Other Rules

### Works With Rule 03 (Work Commit Rules)
- Specification files follow commit-after-every-change rule
- Each specification update gets own commit
- Commit messages explain what was added/changed

### Works With Rule 04 (Agent Orchestration)
- Main agent creates specifications before launching specialized agents
- Specialized agents MUST read specifications before working
- Agents report back with task completion status
- Main agent updates Spec.md based on reports

### Works With Rule 05 (Git Auto-Approval and Push)
- Specification changes are automatically pushed
- No approval needed for specification updates
- Each specification commit is pushed immediately

## Summary

**Core Principle**: Never start significant work without documented requirements and clear task list. **Always engage in thorough requirements conversation with clarifying questions first.** Always launch review agent to verify specifications before implementation. Never trust checkboxes blindly. Always create all mandatory documentation files. **Always create/verify module documentation before implementation.**

**Key Points**:
- ✅ **Requirements conversation with clarifying questions FIRST** (3-10+ questions)
- ✅ **Main Agent MUST ask clarifying questions proactively**
- ✅ **Main Agent MUST NOT passively accept requests**
- ✅ **Main Agent MUST confirm understanding before documenting**
- ✅ Document complete conversation in requirements.md
- ✅ Create comprehensive task list before work
- ✅ **Create/verify module documentation after requirements** (MANDATORY)
- ✅ **Spawn documentation agents to create/update module docs**
- ✅ **Never assume module documentation is accurate**
- ✅ **Verify module docs match actual code**
- ✅ **Reference module docs in requirements.md**
- ✅ **Launch review agent BEFORE implementation** (MANDATORY)
- ✅ **Act on review agent's report** (GO/STOP/CLARIFY)
- ✅ Agents read specifications and module docs before working
- ✅ **Agents verify module docs match reality** (spot check)
- ✅ **Agents STOP if module docs don't match code**
- ✅ Verify status by searching codebase
- ✅ Update tasks.md as work progresses
- ✅ **Update module docs if structure changes**
- ✅ **Create PROGRESS.md at ~50% completion** (MANDATORY)
- ✅ **Create FINAL_REPORT.md when complete** (MANDATORY)
- ✅ **Create LEARNINGS.md to capture insights** (MANDATORY)
- ✅ **Run final verification before completion** (MANDATORY)
- ✅ **Create VERIFICATION_SIGNOFF.md with report** (MANDATORY)
- ✅ Keep Spec.md master index current
- ❌ **Never skip clarifying questions** (CRITICAL)
- ❌ **Never make assumptions** (CRITICAL)
- ❌ **Never document without confirmation** (CRITICAL)
- ❌ **Never skip module documentation** (CRITICAL)
- ❌ **Never assume module docs accurate** (CRITICAL)
- ❌ **Never proceed when module docs don't match** (CRITICAL)
- ❌ **Never skip review agent** (CRITICAL)
- ❌ **Never ignore review STOP/CLARIFY** (CRITICAL)
- ❌ **Never skip mandatory documentation files**
- ❌ Never start work without requirements
- ❌ Never trust status without verification
- ❌ Never add tasks retroactively
- ❌ Never skip updating frontmatter

**Mandatory Files for Every Specification:**
1. **requirements.md** - Requirements and conversation (created at start)
2. **tasks.md** - Task list with checkboxes (created at start)
3. **PROGRESS.md** - Mid-work progress report (~50% completion)
4. **FINAL_REPORT.md** - Comprehensive completion summary (at completion)
5. **LEARNINGS.md** - Lessons learned and insights (at completion)
6. **VERIFICATION_SIGNOFF.md** - Official verification report (after verification)

**Module Documentation System:**
Every affected module **MUST** have accurate documentation:
1. **documentation/[module]/doc.md** - Detailed module documentation with:
   - What it implements (all features/functions)
   - What it imports (dependencies)
   - What it calls (function calls)
   - What it does (step-by-step workflows)
   - Line number references
2. **Created/verified after requirements.md, before implementation**
3. **Updated when module structure changes**
4. **Referenced in requirements.md** so agents know to read it
5. **Verified to match actual code** (never assume accurate)

**Review Agent Is Mandatory:**
Saves hours by:
- Verifying task status accuracy before implementation
- Identifying unclear requirements needing clarification
- Catching inconsistencies between docs and code
- Preventing work based on false assumptions
- Ensuring specifications are actionable

**Remember**: The user will be upset if work proceeds without proper requirements conversation, status verification, mandatory review agent, accurate module documentation, or all mandatory documentation files!

---
*Created: 2026-01-11*
*Last Updated: 2026-01-18*
*Version: 3.0 (Optimized - Reduced from 3,153 to 2,277 lines)*
