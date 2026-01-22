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

When using features, the main specification files change **significantly**:

**CRITICAL**: Main files should be **concise high-level overviews**, NOT detailed documents.

**requirements.md** (CONCISE - should be 2-4KB, NOT 15KB+):
- High-level overview and goals (2-3 paragraphs)
- Requirements conversation summary (brief)
- Feature table with descriptions, dependencies, and links
- Overall success criteria (high-level checklist)
- File structure overview
- Module documentation references
- **NO detailed code samples** - those go in feature.md files
- **NO implementation details** - those go in feature.md files

**tasks.md** (CONCISE - should be 1-2KB):
- Feature priority order with status
- Feature task counts (e.g., "Tasks: 12")
- Dependencies between features
- Total task summary table
- **NO individual task checkboxes** - those go in feature tasks.md files
- **NO implementation notes** - those go in feature tasks.md files

**Feature files contain ALL the details:**
- `feature.md` - Detailed requirements, code samples, API examples, implementation patterns
- `tasks.md` - Individual task checkboxes, implementation order, code snippets

**Rationale**: This structure keeps context size manageable and ensures:
1. Main Agent can quickly understand scope without loading 15KB+ files
2. Implementation Agents load only the feature(s) they're working on
3. Clear separation between high-level planning and detailed implementation

**Example**: See `.agents/templates/examples/feature-based-tasks-example.md` for a complete example of feature-based tasks.md structure.

#### Feature Frontmatter

See `.agents/templates/examples/feature-frontmatter-examples.md` for complete examples of feature.md and feature tasks.md frontmatter.

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

**Example**: See `.agents/templates/examples/builds-on-example.md` for complete frontmatter example with `builds_on` field.

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
  - **`stack_files`**: Array of stack files from `.agents/stacks/` (e.g., `[".agents/stacks/rust.md"]`)
  - **`skills`**: Array of skill names from `.agents/skills/` (e.g., `["skill-name"]`, or `[]` if none)
- **`has_features`**: Boolean - true if using features/ directory structure
- **`has_fundamentals`**: Boolean - true if fundamentals/ documentation directory needed

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

## Self-Contained Specification Requirement (MANDATORY)

### Purpose
Every `requirements.md` file MUST be **self-contained**, meaning an agent can receive ONLY the specification file and understand exactly which rules to load for their role. This eliminates the need for agents to guess or search for applicable rules.

### Why Self-Contained Specifications Matter

**Without Agent Rules Reference:**
- Agents don't know which rules apply to them
- Agents may miss critical rules (verification, safety, etc.)
- Users must manually specify rules when passing specs to agents
- Inconsistent rule loading across different agent invocations

**With Agent Rules Reference:**
- Agents read the spec and immediately know their required rules
- Users can pass `requirements.md` directly to any agent
- Consistent rule loading guaranteed
- Specification is the single source of truth

### Agent Rules Reference Section (MANDATORY)

Every `requirements.md` **MUST** include an "Agent Rules Reference" section containing:

#### Location Headers
The section MUST clearly specify file locations:
- **Rules Location**: `.agents/rules/`
- **Stacks Location**: `.agents/stacks/`
- **Skills Location**: `.agents/skills/`

#### 1. Mandatory Rules for All Agents
All agents working on the specification MUST load Rules 01-04 from `.agents/rules/`:

| Rule | File | Purpose |
|------|------|---------|
| 01 | `.agents/rules/01-rule-naming-and-structure.md` | File naming conventions |
| 02 | `.agents/rules/02-rules-directory-policy.md` | Directory policies |
| 03 | `.agents/rules/03-dangerous-operations-safety.md` | Dangerous operations safety |
| 04 | `.agents/rules/04-work-commit-and-push-rules.md` | Work commit and push rules |

#### 2. Role-Specific Rules
A table mapping agent types to their additional required rules (all from `.agents/rules/`):

| Agent Type | Additional Rules to Load |
|------------|--------------------------|
| **Review Agent** | `.agents/rules/06-specifications-and-requirements.md` |
| **Implementation Agent** | `.agents/rules/13-implementation-agent-guide.md`, `.agents/rules/11-skills-usage.md` if skills used |
| **Verification Agent** | `.agents/rules/08-verification-workflow-complete-guide.md`, stack file |
| **Documentation Agent** | `.agents/rules/06-specifications-and-requirements.md` |

#### 3. Stack Files
Specify which language stack file(s) agents should load from `.agents/stacks/`:
- Format: `**Language**: [language] → .agents/stacks/[language].md`
- Example: `**Language**: Rust → .agents/stacks/rust.md`

#### 4. Skills Referenced
List any skills from `.agents/skills/` that agents should use:
- If skills are needed: List skill names with full paths (e.g., `.agents/skills/skill-name.md`)
- If no skills needed: Write "None"

### Main Agent Responsibilities

When creating `requirements.md`, Main Agent **MUST**:

1. ✅ **Include Agent Rules Reference section** (see template)
2. ✅ **Identify the primary language** and specify stack file
3. ✅ **List any skills** that will be used
4. ✅ **Ensure all rule file paths are correct** (verify files exist)
5. ✅ **Use the requirements template** which includes this section

### Validation

Before committing `requirements.md`, Main Agent **MUST** verify:
- ✅ Agent Rules Reference section exists
- ✅ All 4 mandatory rules are listed
- ✅ Role-specific rules table is complete
- ✅ Stack file is specified (or "N/A" for non-code specs)
- ✅ Skills are listed (or "None")

**Template Location**: `.agents/templates/requirements-template.md`

---

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
- ✅ **Include Agent Rules Reference section** in requirements.md (self-contained specs)
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
- ❌ **Missing Agent Rules Reference section in requirements.md**
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
- **Rule confusion**: Agents don't know which rules to load

**THE USER WILL BE UPSET** if work proceeds without:
- Proper requirements conversation with clarifying questions
- Status verification
- Mandatory review agent execution
- Accurate module documentation
- Agent Rules Reference section (for self-contained specs)
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

**Core Principle**: Never start significant work without documented requirements and clear task list. **Always engage in thorough requirements conversation with clarifying questions first (3-10+ questions).** Always launch review agent to verify specifications before implementation. Never trust checkboxes blindly. Always create all 6 mandatory documentation files. **Always create/verify module documentation before implementation.** **Always include Agent Rules Reference for self-contained specifications.**

**Key Requirements:**
- ✅ **Requirements conversation FIRST** (3-10+ clarifying questions mandatory)
- ✅ **Main Agent MUST ask proactively, never assume**
- ✅ **Confirm understanding before documenting**
- ✅ **Include Agent Rules Reference section** (self-contained specs)
- ✅ **Create/verify module documentation** (after requirements, before implementation)
- ✅ **Launch review agent BEFORE implementation** (zero tolerance)
- ✅ **Act on review report** (GO/STOP/CLARIFY)
- ✅ **Verify module docs match code** (STOP if mismatch)
- ✅ **Create all 6 mandatory files** (requirements, tasks, PROGRESS, FINAL_REPORT, LEARNINGS, VERIFICATION_SIGNOFF)
- ✅ **Verify status by searching codebase**
- ✅ **Update tasks.md and module docs as work progresses**
- ❌ **Never skip clarifying questions**
- ❌ **Never skip Agent Rules Reference section**
- ❌ **Never skip review agent**
- ❌ **Never ignore review STOP/CLARIFY**
- ❌ **Never skip module documentation**
- ❌ **Never assume specs or module docs are accurate**

**Mandatory Files for Every Specification:**
1. **requirements.md** - Requirements and conversation (created at start)
   - **MUST include Agent Rules Reference section** for self-contained specs
2. **tasks.md** - Task list with checkboxes (created at start)
3. **PROGRESS.md** - Mid-work progress report (~50% completion)
4. **FINAL_REPORT.md** - Comprehensive completion summary (at completion)
5. **LEARNINGS.md** - Lessons learned and insights (at completion)
6. **VERIFICATION_SIGNOFF.md** - Official verification report (after verification)

**Self-Contained Specification Requirement:**
Every `requirements.md` MUST include an "Agent Rules Reference" section containing:
- **Location headers**: `.agents/rules/`, `.agents/stacks/`, `.agents/skills/`
- Mandatory rules (01-04) for all agents with full paths
- Role-specific rules table (Review, Implementation, Verification, Documentation agents)
- Stack file(s) for the language(s) used (from `.agents/stacks/`)
- Skills referenced with full paths (or "None")

This allows users to pass `requirements.md` directly to any agent, and the agent will know exactly which rules to load and where to find them.

**Feature-Based Specifications (For Complex Work):**
- Use when spec is large (>15KB) or has multiple distinct components
- Create `features/[feature-name]/` with `feature.md` and `tasks.md` per feature
- **Main `requirements.md` is CONCISE** (2-4KB): high-level overview, feature table, NO code samples
- **Main `tasks.md` is CONCISE** (1-2KB): feature priority order with task counts, NOT individual tasks
- **Feature files contain ALL details**: code samples, implementation patterns, individual task checkboxes
- Verification files (PROGRESS, FINAL_REPORT, etc.) remain at main level ONLY
- Templates: `.agents/templates/feature-template.md` and `feature-tasks-template.md`

**Module Documentation System:**
Every affected module **MUST** have accurate documentation at `documentation/[module]/doc.md`:
- Created/verified after requirements.md, before implementation
- Contains: what it implements, imports, calls, does (with line numbers)
- Referenced in requirements.md so agents know to read it
- Verified to match actual code (never assume accurate)
- Updated when module structure changes

**Remember**: The user will be upset if work proceeds without proper requirements conversation, status verification, mandatory review agent, accurate module documentation, Agent Rules Reference section, or all mandatory documentation files!

---

## Self-Containment and Mandatory Verification Requirements (CRITICAL)

**Added**: 2026-01-22
**Purpose**: Ensure every requirements.md is self-contained with all necessary cross-references and mandatory 100% completion verification.

### 1. Requirements.md Self-Containment (MANDATORY)

Every `requirements.md` file MUST contain ALL of the following:

#### A. Cross-Reference Links at Top and Bottom

**At the top** (immediately after frontmatter, before Overview):
```markdown
> **Specification Tracking**: See [tasks.md](./tasks.md) for task progress and [learnings.md](./learnings.md) for implementation insights.
```

**At the bottom** (after Final Verification Checklist):
```markdown
> **Verification**: See [verification.md](./verification.md) or [VERIFICATION_SIGNOFF.md](./VERIFICATION_SIGNOFF.md) for complete verification results.
```

#### B. Enhanced Frontmatter (MANDATORY Additions)

Add these fields to requirements.md frontmatter:
```yaml
metadata:
  stack_files:  # NEW - moved from body
    - .agents/stacks/rust.md
  skills: []  # NEW - moved from body
has_features: false  # NEW - true if using features/ directory
has_fundamentals: false  # NEW - true if fundamentals/ directory needed
```

**Move from body to frontmatter**:
- Stack file references → `metadata.stack_files`
- Skill references → `metadata.skills`

This makes specifications machine-readable and self-documenting.

### 2. Fundamentals Documentation as First Priority (MANDATORY)

#### When Fundamentals Are Required

Set `has_fundamentals: true` in frontmatter when:
- Implementing new user-facing libraries or APIs
- Creating reusable components users need to understand deeply
- Introducing complex patterns, algorithms, or abstractions
- Building foundational primitives or developer tools
- User needs to make architectural decisions using this code

#### Main Agent Responsibilities

When creating a specification with `has_fundamentals: true`:

**See `.agents/templates/examples/fundamentals-section-example.md` for complete example of how to structure the "User-Facing Documentation Requirements" section and corresponding tasks.**

Key points:
1. **Add "User-Facing Documentation Requirements" section** to requirements.md
2. **Add fundamentals tasks to tasks.md as FIRST PRIORITY**

#### Implementation Agent Responsibilities

**CRITICAL ORDER**: Documentation BEFORE implementation.

1. **Read fundamentals list** from requirements.md
2. **Create fundamentals/ directory** first
3. **Write ALL fundamental documents** listed
4. **Mark fundamentals tasks complete**
5. **ONLY THEN** start implementation coding

**Why This Order**:
- Thinking about user documentation clarifies the API design
- Prevents building APIs that are hard to explain
- Catches design flaws early
- Ensures user-centric approach

### 3. Mandatory 100% Completion Verification

Every `requirements.md` MUST include this complete section (use updated template):

```markdown
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
\`\`\`bash
# Must return 0
grep -c "^- \[ \]" tasks.md
\`\`\`

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
- [ ] Build succeeds with 0 errors
- [ ] Tests show 100% passing
- [ ] NO ignored or skipped tests (unless user-approved)

**Code Quality** (language-specific):
- [ ] Linter shows 0 warnings (cargo clippy, eslint, etc.)
- [ ] Code formatter applied and clean
- [ ] NO warnings suppressed without justification

**Documentation Quality**:
- [ ] All public APIs documented
- [ ] Documentation builds without errors
- [ ] NO broken links

### 5. Specification Tracking Verification (MANDATORY)

- [ ] `tasks.md` shows 100% completion
- [ ] `learnings.md` exists and is comprehensive
- [ ] `progress.md` exists with timeline
- [ ] `verification.md` exists with results
- [ ] `requirements.md` frontmatter status correct

### 6. Verification Issue Resolution (MANDATORY)

**NO OPTIONAL FIXES**: All verification issues MUST be resolved.

- [ ] Check `verification.md` for FAILED or WARNING items
- [ ] ALL failed checks fixed (no exceptions)
- [ ] ALL warnings addressed or user-accepted
- [ ] Re-run verification to confirm PASS
- [ ] Update `verification.md` with final PASS status

**If ANY failures exist**:
1. ❌ DO NOT mark specification complete
2. ❌ DO NOT mark tasks done
3. ✅ FIX all issues
4. ✅ Re-run verification
5. ✅ Only complete after 100% PASS
\`\`\`

### 4. Validation Before Marking Complete

**Main Agent MUST** perform these checks before setting status to "completed":

1. **Task Validation**:
   ```bash
   cd specifications/[NN-spec-name]/
   # Must return 0 (no unchecked tasks)
   grep -c "^- \[ \]" tasks.md
   # Must return total task count
   grep -c "^- \[x\]" tasks.md
   ```

2. **File Existence Validation**:
   ```bash
   # Must all exist
   ls tasks.md learnings.md progress.md verification.md
   # If has_fundamentals: true
   ls fundamentals/*.md
   ```

3. **Quality Validation**:
   ```bash
   # Must show 0 warnings, 100% tests passing
   cargo clippy -- -D warnings  # (or language equivalent)
   cargo test
   ```

4. **Frontmatter Validation**:
   - `tasks.md` frontmatter: `uncompleted: 0`, `completion_percentage: 100`
   - `requirements.md` frontmatter: correct `has_fundamentals` value
   - All cross-reference links present

**ONLY after ALL validations pass** can status be set to "completed".

### 5. Zero Tolerance Enforcement

**Violations with ZERO TOLERANCE**:
- ❌ Marking spec complete with tasks.md showing `[ ]` tasks
- ❌ Marking spec complete with verification showing FAIL
- ❌ Ignoring clippy/lint warnings as "optional"
- ❌ Missing learnings.md, progress.md, or verification.md
- ❌ Missing fundamentals/ when has_fundamentals: true
- ❌ Creating fundamentals AFTER implementation (must be FIRST)
- ❌ Missing cross-reference links in requirements.md

**Consequences**:
- Status MUST be reverted to "in-progress"
- ALL incomplete items MUST be completed
- Verification MUST be re-run from scratch
- Specification CANNOT be marked complete until 100% PASS

### 6. Template Updates

The updated requirements template at `.agents/templates/requirements-template.md` now includes:
- Enhanced frontmatter with stack_files, skills, has_features, has_fundamentals
- Cross-reference links at top and bottom
- User-Facing Documentation Requirements section
- Complete MANDATORY Completion and Verification Requirements section
- Final Verification Checklist

**Main Agent MUST use this updated template** for all new specifications.

### 7. Mandatory Git Commit and Push Requirements (CRITICAL)

**CRITICAL**: To ensure no work is lost and maintain safety, follow these git practices.

**See `.agents/templates/examples/git-workflow-examples.md` for complete examples of atomic commits and final commit workflows.**

#### Key Requirements

**During Implementation (Atomic Commits)**:
- ✅ Commit and push frequently (after each logical unit of work)
- ✅ After tests pass for that unit
- ✅ Every 30-60 minutes of active work
- ✅ Before taking breaks or ending sessions

**After Completion and Verification (MANDATORY PUSH)**:
1. ✅ **Verify all checks pass** (tasks 100%, tests 100%, clippy 0 warnings)
2. ✅ **Create final commit** with all remaining changes
3. ✅ **Push to remote IMMEDIATELY** - DO NOT DELAY
4. ✅ **Verify push succeeded**

#### Safety Rules

**MUST push immediately**:
- ✅ After marking specification complete
- ✅ After verification passes
- ✅ After all tasks show 100%
- ✅ After fixing all verification issues

**WHY this is critical**:
- Prevents work loss from system failures
- Ensures remote backup of completed work
- Allows team visibility into progress
- Creates audit trail of when work completed

**ZERO TOLERANCE**:
- ❌ DO NOT delay pushing after completion
- ❌ DO NOT "batch" pushes across specifications
- ❌ DO NOT leave completed work unpushed
- ❌ DO NOT forget to push after final commit

**User will be upset if**:
- Work is completed but not pushed (risk of loss)
- Specification marked complete but changes not in remote
- Hours of work lost due to unpushed commits

---

*Created: 2026-01-11*
*Last Updated: 2026-01-22*
*Version: 6.0 - Added self-containment, mandatory 100% verification, and git push requirements*
