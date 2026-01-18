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
- **NO implementation** until user explicitly approves and requests it
- This applies to **ALL significant development work**

### User Approval Required Before Implementation (MANDATORY)

**CRITICAL**: After creating a complete specification (requirements.md + tasks.md), the Main Agent **MUST**:

1. ✅ **Present the specification to the user**
2. ✅ **Wait for explicit user approval**
3. ✅ **Wait for explicit user request to begin implementation**
4. ❌ **NEVER start implementation automatically** after creating specs
5. ❌ **NEVER assume approval** because user said "ok" to something else

**User phrases that grant implementation approval:**
- "Start implementation"
- "Go ahead and implement"
- "Begin coding"
- "Proceed with implementation"
- "Let's build it"

**User phrases that do NOT grant implementation approval:**
- "Ok" (could mean acknowledgment, not approval)
- "Looks good" (may want to review first)
- "Thanks" (acknowledgment only)

**When in doubt, ASK**: "Would you like me to begin implementation now?"

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
├── 01-simple-specification/         # Simple spec (no features needed)
│   ├── requirements.md              # (MANDATORY) Requirements and conversation summary
│   ├── tasks.md                     # (MANDATORY) Task list with checkboxes
│   ├── templates/                   # (OPTIONAL) Code/structure templates
│   ├── PROGRESS.md                  # (MANDATORY) Mid-work progress report
│   ├── FINAL_REPORT.md              # (MANDATORY) Comprehensive completion summary
│   ├── VERIFICATION_SIGNOFF.md      # (MANDATORY) Official verification report
│   └── LEARNINGS.md                 # (MANDATORY) Lessons learned and insights
│
├── 02-complex-specification/        # Complex spec with features
│   ├── requirements.md              # High-level requirements + feature references
│   ├── tasks.md                     # Feature priority list (not individual tasks)
│   ├── features/                    # (OPTIONAL) Feature breakdown directory
│   │   ├── foundation/
│   │   │   ├── feature.md           # Feature-specific requirements
│   │   │   ├── tasks.md             # Feature-specific task checkboxes
│   │   │   └── templates/           # Feature-specific templates
│   │   ├── connection/
│   │   │   ├── feature.md
│   │   │   ├── tasks.md
│   │   │   └── templates/
│   │   └── public-api/
│   │       ├── feature.md
│   │       ├── tasks.md
│   │       └── templates/
│   ├── PROGRESS.md                  # (MANDATORY) Overall progress report
│   ├── FINAL_REPORT.md              # (MANDATORY) Overall completion summary
│   ├── VERIFICATION_SIGNOFF.md      # (MANDATORY) Overall verification report
│   └── LEARNINGS.md                 # (MANDATORY) Overall lessons learned
└── ...

documentation/
├── module-1/
│   ├── doc.md                       # (MANDATORY) Detailed module documentation
│   ├── diagrams/                    # (OPTIONAL) Architecture diagrams
│   └── assets/                      # (OPTIONAL) Additional assets
└── ...
```

**CRITICAL**: The `documentation/` directory exists at project root level, parallel to `specifications/`.

### Feature-Based Specifications (OPTIONAL)

For complex specifications, the Main Agent MAY create a `features/` subdirectory to break down work into manageable components.

#### When to Use Features

**Use features when:**
- Specification is large (>15KB requirements.md)
- Multiple distinct components or phases
- User explicitly requests breakdown
- Different components have different dependencies
- Context size needs to be reduced for agent efficiency

**Keep simple (no features) when:**
- Specification is small and focused
- Single coherent piece of work
- Few tasks (< 10)
- No logical component boundaries

#### Feature Structure

Each feature directory contains:

| File | Required | Purpose |
|------|----------|---------|
| `feature.md` | Yes | Feature-specific requirements, context, success criteria |
| `tasks.md` | Yes | Feature-specific task checkboxes with progress tracking |
| `templates/` | No | Feature-specific code templates, examples |

#### Main Specification Files (With Features)

When using features, the main specification files change:

**requirements.md** contains:
- High-level overview and goals
- Requirements conversation summary
- Feature list with descriptions and dependencies
- Links to each feature's `feature.md`
- Overall success criteria
- Module documentation references

**tasks.md** contains:
- Feature priority order (not individual tasks)
- Feature status (pending, in-progress, completed)
- Dependencies between features
- Overall progress percentage

Example main tasks.md with features:
```markdown
---
completed: 1
uncompleted: 4
created: 2026-01-18
features:
  - foundation
  - connection
  - request-response
  - task-iterator
  - public-api
---

# HTTP Client - Feature Progress

## Feature Priority Order

1. [x] **foundation** - Error types and DNS resolution
2. [ ] **connection** - URL parsing, TCP, TLS (depends on: foundation)
3. [ ] **request-response** - Request builder, response types (depends on: connection)
4. [ ] **task-iterator** - TaskIterator, executors (depends on: request-response)
5. [ ] **public-api** - User-facing API, integration (depends on: task-iterator)

## Notes
- Complete features in order due to dependencies
- Each feature has its own tasks.md with detailed checkboxes
```

#### Feature Frontmatter

**feature.md frontmatter:**
```yaml
---
feature: feature-name
description: Brief one-sentence description
status: pending | in-progress | completed
depends_on:
  - other-feature-name
estimated_effort: small | medium | large
---
```

**feature tasks.md frontmatter:**
```yaml
---
feature: feature-name
completed: 0
uncompleted: 5
last_updated: 2026-01-18
---
```

#### Verification Files Remain at Main Level

**CRITICAL**: These files are ONLY at the main specification level, NOT in features:
- `PROGRESS.md` - Overall specification progress
- `FINAL_REPORT.md` - Overall completion summary
- `VERIFICATION_SIGNOFF.md` - Overall verification
- `LEARNINGS.md` - Overall lessons learned

Features do NOT have their own verification/report files.

### Simple Specifications (No Features)

For simple specifications, the structure remains as before:
- `requirements.md` - Full requirements with all details
- `tasks.md` - All task checkboxes with progress
- `templates/` - (Optional) Code templates
- Standard verification files

### Naming Convention
- Each specification gets its own numbered directory
- Format: `NN-descriptive-name/` where NN is two-digit (01, 02, 03, etc.)
- Use dash separators for multi-word names
- Feature directories use lowercase with dashes: `feature-name/`

**Examples:**
- ✅ `01-build-http-client/`
- ✅ `02-implement-user-authentication/`
- ✅ `features/dns-resolution/`
- ❌ `http-client/` (missing number prefix)
- ❌ `1-http-client/` (single digit)
- ❌ `features/DnsResolution/` (wrong case)

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

All specification file templates are located in `.agents/templates/`. Reference these when creating files:

### Mandatory Files for Every Specification

1. **requirements.md** - Requirements and conversation summary
   - Template: `.agents/templates/requirements-template.md`
   - Contains: Overview, conversation summary, detailed requirements, success criteria, module documentation references, agent notes

2. **tasks.md** - Task list with checkboxes
   - Template: `.agents/templates/tasks-template.md`
   - Contains: Task list organized by category, frontmatter with counts and tools

3. **PROGRESS.md** - Mid-work progress report
   - Template: `.agents/templates/PROGRESS-template.md`
   - When: Created at 40-60% completion or major phase transitions
   - Contains: Completion status, completed work, current status, remaining work, blockers, statistics, next steps

4. **FINAL_REPORT.md** - Comprehensive completion summary
   - Template: `.agents/templates/FINAL_REPORT-template.md`
   - When: Created when all tasks are 100% complete
   - Contains: Work completed, task breakdown, detailed accomplishments, commits, statistics, verification results, impact, recommendation

5. **LEARNINGS.md** - Lessons learned and insights
   - Template: `.agents/templates/LEARNINGS-template.md`
   - When: Created at completion
   - Contains: Key insights, challenges and solutions, best practices, anti-patterns, recommendations, knowledge gained, technical debt

6. **VERIFICATION_SIGNOFF.md** - Official verification report
   - Template: `.agents/templates/VERIFICATION_SIGNOFF-template.md`
   - When: Created after verification agent completes final verification
   - Contains: Executive summary, verification results, quality assessment, compliance check, issues found, final verdict, checklist, sign-off

### Feature Files (For Complex Specifications)

7. **feature.md** - Feature-specific requirements
   - Template: `.agents/templates/feature-template.md`
   - When: Created for each feature in `features/[feature-name]/`
   - Contains: Feature overview, dependencies, requirements, implementation details, success criteria

8. **feature tasks.md** - Feature-specific tasks
   - Template: `.agents/templates/feature-tasks-template.md`
   - When: Created alongside feature.md
   - Contains: Feature-specific task checkboxes, implementation order

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

Every `documentation/[module]/doc.md` **MUST** contain these sections with complete frontmatter:

**Required Frontmatter:**
- `module`: Exact module name
- `language`: rust|javascript|typescript|python|go|etc
- `status`: active|deprecated|experimental|planning
- `last_updated`: YYYY-MM-DD
- `maintainer`: Primary agent/team
- `related_specs`: Array of specification references

**Required Sections:**
1. **Overview** - 2-3 sentence summary
2. **Purpose and Responsibility** - Detailed explanation of module's purpose
3. **Module Location** - Path, entry point, language
4. **What It Implements** - Core functionality with line numbers
5. **Public API** - Exported functions and types with line references
6. **What It Imports** - External and internal dependencies
7. **What It Calls** - External and internal function calls with line numbers
8. **What It Does (Step-by-Step)** - Primary workflows and edge cases
9. **Architecture** - Design patterns and module structure
10. **Key Implementation Details** - Performance and security considerations
11. **Tests** - Test coverage information
12. **Dependencies and Relationships** - Module dependencies and usage
13. **Configuration** - Environment variables
14. **Known Issues and Limitations** - Current limitations, bugs, technical debt
15. **Future Improvements** - Planned enhancements
16. **Related Documentation** - Links to related specs
17. **Version History** - Change log

**See example structure in project documentation templates.**

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

[... list all specifications ...]

## Status Dashboard

### Summary
- **Total:** N
- **Completed:** X (XX%)
- **In Progress:** Y (YY%)
- **Pending:** Z (ZZ%)

### Completed ✅
- [List completed specs]

### In Progress 🔄
- [List in-progress specs]

### Pending ⏳
- [List pending specs]

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
2. Main Agent: Thorough Requirements Conversation (3-10+ Questions)
   ↓
3. User Provides Answers + Main Agent Confirms Understanding
   ↓
4. Create Specification Directory (specifications/NN-feature-name/)
   ↓
5. Create requirements.md (with conversation summary)
   ↓
6. Create tasks.md (with complete frontmatter)
   ↓
7. Create/Update Module Documentation (MANDATORY)
   - Identify affected modules
   - Spawn Documentation Agent(s)
   - Verify docs match code (STOP if mismatch)
   - Reference in requirements.md
   ↓
8. Update Spec.md Master Index
   ↓
9. Commit Specification + Module Documentation
   ↓
10. LAUNCH REVIEW AGENT (MANDATORY)
    - Review reads specs, searches codebase
    - Reports: GO / STOP / CLARIFY
    - IF GO: Continue | IF STOP/CLARIFY: Fix issues first
   ↓
11. Launch Implementation Agents
    - Read specs, tasks, review report, module docs
    - Verify docs match reality (STOP if mismatch)
    - Implement verified tasks
    - Update module docs if changes occur
   ↓
12. Create PROGRESS.md (~40-60% completion)
   ↓
13. Complete Implementation (all tasks done)
   ↓
14. Create FINAL_REPORT.md
   ↓
15. Create LEARNINGS.md
   ↓
16. Run Final Verification Agent
   ↓
17. Create VERIFICATION_SIGNOFF.md
   ↓
18. Update requirements.md status → completed
   ↓
19. Update Spec.md, commit, push
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

**Starting with Review Agent**
- User requests caching layer
- Main Agent asks 8 clarifying questions (strategy, storage, invalidation, TTL, etc.)
- User provides detailed answers
- Agent creates specification with full conversation documented
- **Agent launches REVIEW AGENT FIRST** ✅
- Review agent verifies, reports "GO"
- Agent launches implementation agents with accurate context

### Bad Practice ❌

**Starting Without Review Agent**
- User: "Implement authentication"
- Main Agent creates spec, commits
- **Immediately launches implementation WITHOUT review** ❌
- Agents assume task statuses are accurate
- Discover "completed" tasks aren't done
- Waste hours on wrong approach
- **CRITICAL VIOLATION**: Skipped mandatory review agent

**Passive Acceptance Without Questions**
- User: "Add user authentication"
- Main Agent: "Ok, I'll create a specification"
- **No clarifying questions asked** ❌
- Missing: authentication method, storage, security requirements
- Implements wrong solution
- User expectations not met

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

### Mandatory Compliance Checklist

All agents **MUST**:
- ✅ Engage in thorough requirements conversation (3-10+ questions)
- ✅ Never passively accept user requests without clarification
- ✅ Never make assumptions about unspecified requirements
- ✅ Confirm understanding before documenting
- ✅ Create specification directory before implementation
- ✅ Document complete conversation in requirements.md
- ✅ Create comprehensive task list in tasks.md
- ✅ Create/verify module documentation after requirements
- ✅ Launch review agent BEFORE any implementation
- ✅ Read review agent's report before proceeding
- ✅ Stop work if review reports STOP or CLARIFY
- ✅ Read requirements.md, tasks.md, and module docs before working
- ✅ Verify module docs match actual code
- ✅ Verify status by searching codebase
- ✅ Update tasks.md as work progresses
- ✅ Create all 6 mandatory documentation files
- ✅ Commit specification changes following Rule 03 and Rule 05

### Critical Violations

**ZERO TOLERANCE for these violations:**
- ❌ Passively accepting user request without questions
- ❌ Asking fewer than minimum required questions
- ❌ Making assumptions about unspecified requirements
- ❌ Starting implementation without running review agent first
- ❌ Ignoring review agent's STOP or CLARIFY directive
- ❌ Starting implementation without module documentation
- ❌ Not verifying module documentation accuracy
- ❌ Proceeding when module docs don't match code
- ❌ Completing spec without PROGRESS.md
- ❌ Completing spec without FINAL_REPORT.md
- ❌ Completing spec without LEARNINGS.md
- ❌ Completing spec without VERIFICATION_SIGNOFF.md
- ❌ Marking complete without running verification

### User Impact

Violations cause:
- **User frustration**: Work doesn't meet expectations
- **Wasted effort**: Wrong or unnecessary implementation
- **Lost context**: Future agents don't understand requirements
- **False progress**: Status shows completion when incomplete
- **Time waste**: Hours wasted on false assumptions
- **Trust erosion**: User loses confidence in agents
- **Breaking changes**: Bugs from misunderstanding modules

**THE USER WILL BE UPSET** if work proceeds without:
- Proper requirements conversation with clarifying questions
- Status verification
- Mandatory review agent execution
- Accurate module documentation
- All mandatory documentation files

### Corrective Action

When violation occurs:
1. **Stop immediately**
2. **Launch review agent** if skipped (CRITICAL)
3. **Read and act on review report** (MANDATORY)
4. **Do not proceed** if review reports STOP/CLARIFY
5. **Create/verify module documentation** if missing (CRITICAL)
6. **Do not proceed** if module docs don't match reality
7. **Create specification** if missing
8. **Document requirements** via conversation with user
9. **Create task list** before proceeding
10. **Verify status** by searching codebase
11. **Update files** to reflect accurate status
12. **Commit changes** following proper workflow
13. **Re-run review agent** if specs updated
14. **Only proceed** when review reports GO and module docs accurate

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

**Core Principle**: Never start significant work without documented requirements and clear task list. **Always engage in thorough requirements conversation with clarifying questions first (3-10+ questions).** Always launch review agent to verify specifications before implementation. Never trust checkboxes blindly. Always create all 6 mandatory documentation files. **Always create/verify module documentation before implementation.**

**Key Requirements:**
- ✅ **Requirements conversation FIRST** (3-10+ clarifying questions mandatory)
- ✅ **Main Agent MUST ask proactively, never assume**
- ✅ **Confirm understanding before documenting**
- ✅ **Create/verify module documentation** (after requirements, before implementation)
- ✅ **Launch review agent BEFORE implementation** (zero tolerance)
- ✅ **Act on review report** (GO/STOP/CLARIFY)
- ✅ **Verify module docs match code** (STOP if mismatch)
- ✅ **Create all 6 mandatory files** (requirements, tasks, PROGRESS, FINAL_REPORT, LEARNINGS, VERIFICATION_SIGNOFF)
- ✅ **Verify status by searching codebase**
- ✅ **Update tasks.md and module docs as work progresses**
- ❌ **Never skip clarifying questions**
- ❌ **Never skip review agent**
- ❌ **Never ignore review STOP/CLARIFY**
- ❌ **Never skip module documentation**
- ❌ **Never assume specs or module docs are accurate**

**Mandatory Files for Every Specification:**
1. **requirements.md** - Requirements and conversation (created at start)
2. **tasks.md** - Task list with checkboxes (created at start)
3. **PROGRESS.md** - Mid-work progress report (~50% completion)
4. **FINAL_REPORT.md** - Comprehensive completion summary (at completion)
5. **LEARNINGS.md** - Lessons learned and insights (at completion)
6. **VERIFICATION_SIGNOFF.md** - Official verification report (after verification)

**Feature-Based Specifications (For Complex Work):**
- Use when spec is large (>15KB) or has multiple distinct components
- Create `features/[feature-name]/` with `feature.md` and `tasks.md` per feature
- Main `requirements.md` contains high-level overview + feature references
- Main `tasks.md` tracks feature priority order (not individual tasks)
- Verification files (PROGRESS, FINAL_REPORT, etc.) remain at main level ONLY
- Templates: `.agents/templates/feature-template.md` and `feature-tasks-template.md`

**Module Documentation System:**
Every affected module **MUST** have accurate documentation at `documentation/[module]/doc.md`:
- Created/verified after requirements.md, before implementation
- Contains: what it implements, imports, calls, does (with line numbers)
- Referenced in requirements.md so agents know to read it
- Verified to match actual code (never assume accurate)
- Updated when module structure changes

**Remember**: The user will be upset if work proceeds without proper requirements conversation, status verification, mandatory review agent, accurate module documentation, or all mandatory documentation files!

---
*Created: 2026-01-11*
*Last Updated: 2026-01-18*
*Version: 4.1 (Added feature-based specification support)*
