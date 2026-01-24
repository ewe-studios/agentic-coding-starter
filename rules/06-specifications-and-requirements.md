# Specifications and Requirements Management

## Purpose
This rule establishes a mandatory requirements-gathering and specification-tracking system that ensures all work begins with a documented conversation between the main agent and user, creating a clear record of requirements and tasks in the `specifications/` directory.

## Core Requirements

### Requirements-First Development
Before **ANY** work begins on new features, enhancements, or significant changes, the main agent **MUST**:

1. **Engage in a conversation** with the user about requirements
2. **Document the requirements** in a specification directory
3. **Create a task list** for tracking work progress
4. **Have agents read specifications** before starting work
5. **Verify and update status** as work progresses

**No Exceptions**:
- **NO coding** without documented requirements
- **NO starting work** without a specification
- **NO skipping** the requirements conversation
- **NO implementation** until user explicitly approves and requests it
- This applies to **ALL significant development work**

### User Approval Required Before Implementation (MANDATORY)

**CRITICAL**: After creating a complete specification (requirements.md with integrated tasks), the Main Agent **MUST**:

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

1. ✅ **Listen carefully and identify the high-level goal**

2. ✅ **Ask clarifying questions proactively** - NEVER assume details, ALWAYS ask when ambiguous, ALWAYS probe edge cases, ALWAYS confirm technical approaches

3. ✅ **Ask questions in critical areas**: Scope, Technical Approach, Constraints, Success Criteria, Edge Cases, Integration, Priority, Timeline

4. ✅ **Continue asking until all details are clear** - iterate through multiple rounds if needed

5. ✅ **Confirm understanding before documenting** - summarize and get user confirmation

**The Main Agent MUST NOT**:
- ❌ Accept vague requests, make assumptions, skip questioning, proceed with incomplete understanding, or document without confirmation

#### Minimum Questions Required

**Main Agent MUST ask AT LEAST**: 3-5 for small features, 5-10 for medium features, 10+ for large/complex features. If fewer needed, user likely provided exceptional detail (rare) OR agent not probing enough (more likely - ASK MORE).

#### Examples

**❌ BAD**: User: "Add authentication" → Agent: "Ok, I'll create spec" (No questions)

**✅ GOOD**: User: "Add authentication" → Agent asks about: method, storage, security, registration, password reset, timeout, rate limiting → User answers → Agent confirms → Creates comprehensive spec

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
  - `stack_files`: Array of stack files from `.agents/stacks/`
  - `skills`: Array of skill names from `.agents/skills/` (or `[]` if none)
- ✅ `has_features`: Boolean - true if using features/ directory
- ✅ `has_fundamentals`: Boolean - true if fundamentals/ documentation needed
- ✅ `builds_on`: (if applicable) Array of parent specs
- ✅ `related_specs`: (if applicable) Array of related specs
- ✅ `files_required`: Complete object with entries for each agent type (see Self-Contained Specification Requirements section)

#### Task Tracking in requirements.md:

Main Agent **MUST** include task tracking frontmatter in requirements.md:
- ✅ `tasks`: Complete object with:
  - `completed`: Count of [x] tasks
  - `uncompleted`: Count of [ ] tasks
  - `total`: completed + uncompleted
  - `completion_percentage`: (completed / total) * 100
- ✅ `metadata.tools`: Array of tools/technologies used

**NOTE**: The `tasks` section is integrated into requirements.md, not a separate file.

#### Validation and Updates:

Before creating any specification file, Main Agent MUST:
1. **Check frontmatter completeness** - All REQUIRED fields present
2. **Validate field values** - Status, priority, dates are valid
3. **Calculate derived fields** - completion_percentage from counts
4. **Report if validation fails** - Stop and request correction

When sub-agents update specifications:
- ✅ Sub-agents MUST update `metadata.last_updated`
- ✅ Sub-agents MUST increment `metadata.version` if significant changes
- ✅ Sub-agents MUST update task counts in requirements.md frontmatter
- ✅ Sub-agents MUST add new tools to `tools` array
- ❌ Sub-agents MUST NOT modify other frontmatter without approval

## Directory Structure

### Overview
```
specifications/
├── Spec.md                          # Master index of all specifications
├── 01-simple-specification/         # Simple spec (no features needed)
│   ├── requirements.md              # (MANDATORY) Requirements and conversation summary
│   ├── (tasks integrated in requirements.md)
│   ├── templates/                   # (OPTIONAL) Code/structure templates
│   ├── PROGRESS.md                  # (MANDATORY) Ephemeral task-focused status
│   ├── FINAL_REPORT.md              # (MANDATORY) Comprehensive completion summary
│   ├── VERIFICATION_SIGNOFF.md      # (MANDATORY) Official verification report
│   └── LEARNINGS.md                 # (MANDATORY) Permanent lessons learned
│
├── 02-complex-specification/        # Complex spec with features
│   ├── requirements.md              # High-level requirements + feature references
│   ├── features/                    # (OPTIONAL) Feature breakdown directory
│   │   ├── foundation/
│   │   │   ├── feature.md           # Feature requirements + integrated tasks
│   │   │   └── templates/           # Feature-specific templates
│   │   └── ...
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
| `feature.md` | Yes | Feature-specific requirements, context, success criteria, and integrated task list |
| `templates/` | No | Feature-specific code templates, examples |

**CRITICAL**: Tasks are integrated directly into feature.md files, not in separate tasks.md files.

#### Main Specification Files (With Features)

When using features, the main specification files change **significantly**:

**CRITICAL**: Main files should be **concise high-level overviews**, NOT detailed documents.

**requirements.md** (CONCISE - should be 2-4KB, NOT 15KB+):
- High-level overview and goals (2-3 paragraphs)
- Requirements conversation summary (brief)
- Feature table with descriptions, dependencies, and links
- Tasks section with feature priority order and status
- Feature task counts in Tasks section (e.g., "Feature 1: 12 tasks")
- Dependencies between features
- Overall success criteria (high-level checklist)
- File structure overview
- Module documentation references
- **NO detailed code samples** - those go in feature.md files
- **NO implementation details** - those go in feature.md files
- **NO individual task checkboxes** - those go in feature.md task sections

**Feature files contain ALL the details:**
- `feature.md` - Detailed requirements, code samples, API examples, implementation patterns, AND integrated task list with checkboxes

**Rationale**: This structure keeps context size manageable and ensures:
1. Main Agent can quickly understand scope without loading 15KB+ files
2. Implementation Agents load only the feature(s) they're working on
3. Clear separation between high-level planning and detailed implementation

**Example**: See `.agents/templates/requirements-template.md` for the unified structure with integrated tasks.

#### Feature Frontmatter

Feature.md files should include task tracking frontmatter similar to requirements.md:
```yaml
tasks:
  completed: [count]
  uncompleted: [count]
  total: [count]
  completion_percentage: [percentage]
```

See `.agents/templates/examples/feature-frontmatter-examples.md` for complete examples of feature.md frontmatter.

#### Verification Files Remain at Main Level

**CRITICAL**: These files are ONLY at the main specification level, NOT in features:
- `PROGRESS.md` - Overall specification progress (ephemeral)
- `FINAL_REPORT.md` - Overall completion summary
- `VERIFICATION_SIGNOFF.md` - Overall verification
- `LEARNINGS.md` - Overall lessons learned (permanent)

Features do NOT have their own verification/report files.

### Simple Specifications (No Features)

For simple specifications, the structure remains as before:
- `requirements.md` - Full requirements with all details
- Tasks integrated in requirements.md
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

## Mandatory Files

### 1. requirements.md
- **Template**: `.agents/templates/requirements-template.md`
- **Purpose**: Requirements and conversation summary
- **Contains**: Overview, conversation summary, detailed requirements, success criteria, module documentation references, agent notes

### 2. Task Tracking (Integrated in requirements.md)
- **Template**: `.agents/templates/tasks-template.md`
- **Purpose**: Task list with checkboxes
- **Contains**: Task list organized by category, frontmatter with counts and tools

### 3. PROGRESS.md - Ephemeral Task-Focused Status (MANDATORY)

**CRITICAL**: PROGRESS.md is **EPHEMERAL** and task-focused, NOT a permanent record.

**Template**: `.agents/templates/PROGRESS-template.md`

**Purpose**: Track current task progress and immediate next steps.

**Lifecycle**:
- ✅ Created when starting work on a task/phase
- ✅ **CLEARED and REWRITTEN** after each major task/phase completion
- ✅ **DELETED** when specification marked complete (before FINAL_REPORT.md creation)
- ✅ Focus: "What am I doing RIGHT NOW and what's immediately next?"

**Content Focus**:
- Current task being worked on
- Immediate blockers for THIS task
- Next 2-3 steps for THIS task
- Statistics for CURRENT work session
- **NOT** cumulative history (that's LEARNINGS.md)
- **NOT** permanent insights (that's LEARNINGS.md)

**When to Create**:
- Starting work at 40-60% completion
- Major phase transitions

**When to Clear and Rewrite**:
- Completed a major task/phase
- Switching to different task/feature
- Major milestone reached
- Coming back after break (write fresh status)

**When to Delete**:
- ALL tasks complete (100%)
- Ready to create FINAL_REPORT.md
- Specification being marked as complete

**Contrast with LEARNINGS.md**:
- PROGRESS.md = ephemeral, current status only
- LEARNINGS.md = permanent, cumulative insights and lessons
- Insights from PROGRESS.md → transferred to LEARNINGS.md before clearing

### 4. LEARNINGS.md - Permanent Insights (MANDATORY)

**Template**: `.agents/templates/LEARNINGS-template.md`

**Purpose**: Permanent record of lessons learned and insights

**When**: Created at completion

**Contains**:
- Key insights
- Challenges and solutions
- Best practices
- Anti-patterns
- Recommendations
- Knowledge gained
- Technical debt

**CRITICAL DISTINCTION**:
- LEARNINGS.md is **PERMANENT** - never cleared or deleted
- Cumulative record of all insights across entire specification
- Survives PROGRESS.md clearing/deletion
- Single source of truth for what was learned

### 5. FINAL_REPORT.md (MANDATORY)

**Template**: `.agents/templates/FINAL_REPORT-template.md`

**When**: Created when all tasks are 100% complete

**Contains**:
- Work completed
- Task breakdown
- Detailed accomplishments
- Commits
- Statistics
- Verification results
- Impact
- Recommendation

### 6. VERIFICATION_SIGNOFF.md (MANDATORY)

**Template**: `.agents/templates/VERIFICATION_SIGNOFF-template.md`

**When**: Created after verification agent completes final verification

**Contains**:
- Executive summary
- Verification results
- Quality assessment
- Compliance check
- Issues found
- Final verdict
- Checklist
- Sign-off

### Feature Files (For Complex Specifications)

**7. feature.md** - Feature-specific requirements and tasks
- **Template**: `.agents/templates/feature-template.md`
- **When**: Created for each feature in `features/[feature-name]/`
- **Contains**: Feature overview, dependencies, requirements, implementation details, success criteria, AND integrated task list with checkboxes

**NOTE**: Tasks are integrated into feature.md, not in separate files.

## Specification File Organization and Consolidation (MANDATORY)

### File Consolidation Policy (ZERO TOLERANCE)

**CRITICAL RULE**: Each specification directory MUST contain ONLY the mandatory files listed below. Agents MUST NOT create additional report files, temporary documentation, or duplicate files.

**Violation Examples**:
- ❌ Creating `PROCESS_LEARNINGS.md` when `LEARNINGS.md` exists
- ❌ Creating `WASM_TESTING_REPORT.md` instead of adding to `FINAL_REPORT.md`
- ❌ Creating `WORK_SESSION_SUMMARY.md` instead of updating `FINAL_REPORT.md`
- ❌ Creating multiple verification files (`VERIFICATION_SIGNOFF.md`, `VERIFICATION_RESULTS.md`, `verification.md`)
- ❌ Keeping `PROGRESS.md` after specification complete

### Allowed Files Per Specification (EXHAUSTIVE LIST)

**ONLY these files are allowed** in `specifications/[spec-name]/`:

#### 1. **requirements.md** (MANDATORY)
- **Purpose**: Complete requirements with integrated tasks
- **Content**: Overview, conversation summary, requirements, task list with checkboxes, success criteria
- **Template**: `.agents/templates/requirements-template.md`
- **Status**: Permanent (never deleted)

#### 2. **LEARNINGS.md** (MANDATORY)
- **Purpose**: Single source of truth for ALL learnings, insights, and lessons
- **Content**: ALL learnings go here - process learnings, technical learnings, implementation insights, challenges, solutions, best practices, anti-patterns
- **Consolidation Rules**:
  - ✅ Process learnings → LEARNINGS.md
  - ✅ Technical learnings → LEARNINGS.md
  - ✅ Implementation insights → LEARNINGS.md
  - ✅ User-requested learnings → LEARNINGS.md
  - ❌ NEVER create `PROCESS_LEARNINGS.md`, `TECHNICAL_LEARNINGS.md`, or any other learning file
- **Update Pattern**: Append new learnings to existing sections, organize by category if needed
- **Template**: `.agents/templates/LEARNINGS-template.md`
- **Status**: Permanent (never deleted)

#### 3. **FINAL_REPORT.md** (MANDATORY AT COMPLETION)
- **Purpose**: Single comprehensive report for ALL reporting needs
- **Content**: ALL reports go here:
  - Work session summaries
  - Testing reports (including WASM testing)
  - Implementation progress
  - Verification results summary
  - Completion metrics
  - Any other reporting needs
- **Consolidation Rules**:
  - ✅ Work session summary → FINAL_REPORT.md (add section)
  - ✅ WASM testing report → FINAL_REPORT.md (add section)
  - ✅ Platform-specific testing → FINAL_REPORT.md (add section)
  - ✅ Integration testing summary → FINAL_REPORT.md (add section)
  - ❌ NEVER create separate report files (`WASM_TESTING_REPORT.md`, `WORK_SESSION_SUMMARY.md`, etc.)
- **Progressive Updates**: Can be updated progressively despite name containing "FINAL" - it's the final location for reports, not final timing
- **Template**: `.agents/templates/FINAL_REPORT-template.md`
- **Status**: Permanent (never deleted), created when specification nears completion

#### 4. **VERIFICATION.md** (MANDATORY AT COMPLETION)
- **Purpose**: Single verification signoff file
- **Content**: All verification results from verification agent
- **Naming**: Use `VERIFICATION.md` (not VERIFICATION_SIGNOFF.md, VERIFICATION_RESULTS.md, or verification.md)
- **Template**: `.agents/templates/VERIFICATION-template.md` (to be renamed from VERIFICATION_SIGNOFF-template.md)
- **Status**: Permanent (never deleted), created after final verification

#### 5. **PROGRESS.md** (OPTIONAL - EPHEMERAL)
- **Purpose**: Current task status and immediate next steps
- **Content**: Current work only - what's happening RIGHT NOW
- **Lifecycle**:
  - Created at 40-60% completion
  - Cleared and rewritten after each major task
  - **DELETED** when specification 100% complete
- **Template**: `.agents/templates/PROGRESS-template.md`
- **Status**: Ephemeral (deleted at completion)

#### 6. **fundamentals/** (OPTIONAL DIRECTORY)
- **Purpose**: User-facing documentation
- **When**: Only if `has_fundamentals: true`
- **Contains**: Numbered markdown files (00-overview.md, 01-theory.md, etc.)
- **Status**: Permanent

#### 7. **features/** (OPTIONAL DIRECTORY)
- **Purpose**: Feature-specific requirements
- **When**: Only if `has_features: true`
- **Contains**: Feature subdirectories with feature.md files
- **Status**: Permanent

#### 8. **templates/** (OPTIONAL DIRECTORY)
- **Purpose**: Code templates for implementation
- **When**: Only if needed for code generation
- **Contains**: Template files
- **Status**: Permanent

### Strict File Prohibition (ZERO TOLERANCE)

**FORBIDDEN FILES** - These files MUST NOT be created:

#### ❌ Duplicate Learning Files
- ❌ `PROCESS_LEARNINGS.md` - Use LEARNINGS.md
- ❌ `TECHNICAL_LEARNINGS.md` - Use LEARNINGS.md
- ❌ `IMPLEMENTATION_LEARNINGS.md` - Use LEARNINGS.md
- ❌ Any file with "learning" or "insight" in the name except LEARNINGS.md

#### ❌ Separate Report Files
- ❌ `WASM_TESTING_REPORT.md` - Add section to FINAL_REPORT.md
- ❌ `WORK_SESSION_SUMMARY.md` - Add section to FINAL_REPORT.md
- ❌ `TESTING_REPORT.md` - Add section to FINAL_REPORT.md
- ❌ `PLATFORM_TESTING_REPORT.md` - Add section to FINAL_REPORT.md
- ❌ `INTEGRATION_REPORT.md` - Add section to FINAL_REPORT.md
- ❌ Any file with "report" or "summary" in the name except FINAL_REPORT.md

#### ❌ Multiple Verification Files
- ❌ `VERIFICATION_SIGNOFF.md` - Use VERIFICATION.md
- ❌ `VERIFICATION_RESULTS.md` - Use VERIFICATION.md
- ❌ `verification.md` - Use VERIFICATION.md (capital letters)
- ❌ Any file with "verification" except VERIFICATION.md

#### ❌ Progress Tracking After Completion
- ❌ `PROGRESS.md` after 100% completion - MUST be deleted
- ❌ `STATUS.md` - Use PROGRESS.md during work, delete at completion
- ❌ Any ephemeral tracking files

#### ❌ Temporary or Scratch Files
- ❌ `NOTES.md`, `TODO.md`, `SCRATCH.md`
- ❌ `ISSUES.md` - Use GitHub issues or LEARNINGS.md
- ❌ Agent working files (create outside spec directory if needed)

### Consolidation Workflow

**When agent wants to create a new report/learning file**:

1. ✅ **STOP** - Do not create a new file
2. ✅ **Identify** the correct target file:
   - Learnings/insights → LEARNINGS.md
   - Reports/summaries → FINAL_REPORT.md
   - Verification → VERIFICATION.md
3. ✅ **Add section** to existing file with appropriate heading
4. ✅ **Update** existing file instead of creating new file

**Example - WASM Testing Report**:
```markdown
# Wrong Approach ❌
Create: specifications/04-condvar-primitives/WASM_TESTING_REPORT.md

# Correct Approach ✅
Update: specifications/04-condvar-primitives/FINAL_REPORT.md
Add section: "## WASM Testing Verification"
```

**Example - Process Learnings**:
```markdown
# Wrong Approach ❌
Create: specifications/04-condvar-primitives/PROCESS_LEARNINGS.md

# Correct Approach ✅
Update: specifications/04-condvar-primitives/LEARNINGS.md
Add section: "## Process Learnings" within existing file
```

### Agent Requirements Reminder (MANDATORY)

**CRITICAL**: At the end of every `requirements.md` file, MUST include this reminder:

```markdown
---

## CRITICAL REMINDER: Specification File Organization (MANDATORY)

**ONLY these files are allowed in this specification directory:**

1. ✅ `requirements.md` - Requirements with integrated tasks (permanent)
2. ✅ `LEARNINGS.md` - ALL learnings consolidated here (permanent)
3. ✅ `FINAL_REPORT.md` - ALL reports consolidated here (permanent)
4. ✅ `VERIFICATION.md` - Verification signoff (permanent)
5. ✅ `PROGRESS.md` - Current status ONLY (ephemeral - delete at 100%)
6. ✅ `fundamentals/` - User docs (if has_fundamentals: true)
7. ✅ `features/` - Feature files (if has_features: true)
8. ✅ `templates/` - Code templates (optional)

**FORBIDDEN - DO NOT CREATE:**
- ❌ `PROCESS_LEARNINGS.md` - Use LEARNINGS.md
- ❌ `WASM_TESTING_REPORT.md` - Add section to FINAL_REPORT.md
- ❌ `WORK_SESSION_SUMMARY.md` - Add section to FINAL_REPORT.md
- ❌ `VERIFICATION_SIGNOFF.md` - Use VERIFICATION.md
- ❌ Any other report/learning/tracking files

**Consolidation Rules:**
- ALL learnings → LEARNINGS.md (add sections as needed)
- ALL reports → FINAL_REPORT.md (add sections as needed)
- ALL verification → VERIFICATION.md (single file)

**See Rule 06 Section "Specification File Organization and Consolidation" for complete policy.**

---
```

**Main Agent MUST**:
- ✅ Include this reminder in requirements.md template
- ✅ Verify this reminder exists when reviewing specs
- ✅ Enforce file consolidation policy

**All Agents MUST**:
- ✅ Read this reminder before creating any files
- ✅ Follow consolidation rules strictly
- ✅ Update existing files instead of creating new files

### Cleanup of Existing Specifications

**For specifications with file violations**:

1. ✅ Identify duplicate files (PROCESS_LEARNINGS.md, WASM_TESTING_REPORT.md, etc.)
2. ✅ Consolidate content into proper files
3. ✅ Delete the duplicate files
4. ✅ Commit with message: "Consolidate specification files per Rule 06"

**Example cleanup for Spec 04**:
```bash
# Consolidate PROCESS_LEARNINGS.md → LEARNINGS.md
# Consolidate WASM_TESTING_REPORT.md → FINAL_REPORT.md (add WASM section)
# Consolidate WORK_SESSION_SUMMARY.md → FINAL_REPORT.md (add session section)
# Delete PROGRESS.md (spec is 100% complete)
# Rename VERIFICATION_SIGNOFF.md → VERIFICATION.md
```

## Self-Contained Specification Requirements

### files_required Frontmatter Section (MANDATORY)

Every `requirements.md` **MUST** include a `files_required` section in the frontmatter that explicitly lists rules and files for each agent type.

#### Purpose

This makes specifications truly self-contained - agents can read the frontmatter and know exactly what to load without searching or guessing.

#### Structure

The `files_required` section contains entries for each agent type:

```yaml
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
      # Tasks are now in requirements.md
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
      # Tasks are now in requirements.md
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
      # Tasks are now in requirements.md
      - ./requirements.md
      - [feature.md if has_features: true]
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
      # Tasks are now in requirements.md
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
```

#### Agent Responsibilities

**Agents MUST**:
1. ✅ Read the `files_required` section in requirements.md frontmatter
2. ✅ Find their agent type (main_agent, review_agent, implementation_agent, etc.)
3. ✅ Load ALL rules listed for their agent type
4. ✅ Load ALL files listed for their agent type
5. ✅ Resolve dynamic references (e.g., `[stack_file from metadata.stack_files]`)

**Main Agent MUST**:
1. ✅ Include complete `files_required` section in requirements.md frontmatter
2. ✅ Use the requirements template (which includes this section)
3. ✅ Verify all file paths are correct
4. ✅ Update if specification structure changes (e.g., adding features)

#### Dynamic References

Some file references are dynamic and must be resolved from other frontmatter fields:

- `[stack_file from metadata.stack_files]` → Load the stack file(s) listed in `metadata.stack_files`
- `[feature.md if has_features: true]` → Load feature files if `has_features: true` (tasks are integrated in feature.md files)
- `[fundamentals/* if has_fundamentals: true]` → Load fundamentals docs if `has_fundamentals: true`
- `(if exists)` → Load file only if it exists (e.g., PROGRESS.md during active work)
- `(if skills used)` → Load rule only if `metadata.skills` is not empty

#### Benefits

**For Agents**:
- ✅ No guessing what to load
- ✅ Self-contained specification
- ✅ Consistent file loading across all agents
- ✅ Clear requirements for each agent type

**For Main Agent**:
- ✅ Single source of truth in frontmatter
- ✅ Easy to update when structure changes
- ✅ Template handles most cases automatically

**For Users**:
- ✅ Can pass requirements.md to any agent
- ✅ Agent knows exactly what to load
- ✅ No confusion about missing context

### Validation

Before committing `requirements.md`, Main Agent **MUST** verify:
- ✅ `files_required` section exists in frontmatter
- ✅ All agent types have `rules` and `files` lists
- ✅ All file paths are correct and exist
- ✅ Dynamic references are properly formatted
- ✅ Stack files match `metadata.stack_files`
- ✅ Skills rule included if `metadata.skills` not empty

**Template Location**: `.agents/templates/requirements-template.md`

### Cross-Reference Links (MANDATORY)

Every `requirements.md` file MUST contain:

**Top link** (after frontmatter, before Overview):
- Reference to Tasks section within the same file for task progress
- Links to `LEARNINGS.md` for implementation insights
- Links to `PROGRESS.md` for current work status
- **Agent instruction**: "Review the `files_required` section in frontmatter above"

**Bottom link** (after Final Verification Checklist):
- Links to `verification.md` or `VERIFICATION_SIGNOFF.md` for verification results

**Example**: See `.agents/templates/requirements-template.md` for the complete unified structure with integrated tasks.

## Module Documentation System (MANDATORY)

### Purpose
The `documentation/` directory provides living, detailed documentation of individual code modules. This ensures agents have clear understanding of what each module implements **BEFORE** making changes.

**What to Read**: Load files per `files_required.[agent_type]` from requirements.md frontmatter.

### Context Window Management

**CRITICAL OPTIMIZATION**: Large documentation (>8-10KB) wastes context.

**Main Agent**: Does NOT load large documentation - delegates to sub-agents, references path only.

**Sub-Agent**: Load if <8-10KB; otherwise use Grep/Glob/Read tools. Update when making structural changes. Report status to Main Agent.

**When Too Large for Sub-Agent**: Skip loading, use tools, complete work, report "Documentation too large, requesting Documentation Agent" - Main Agent spawns Documentation Agent.

### When Module Documentation Is Created

**After requirements.md Completed:**

1. Main Agent identifies affected modules (existing or new)
2. Spawns Documentation Agent(s) to create/update module docs
3. Documentation agents verify accuracy against actual code
4. **If mismatch: STOP, fix docs first**
5. Main Agent references docs in requirements.md and commits

### Documentation Agent Must STOP If Mismatch Found

If documentation doesn't match code: **STOP immediately** → Report to Main Agent with mismatch details → Main Agent halts specification work → Documentation updated FIRST → Then resume implementation.

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

**What to Read**: Load files per `files_required.implementation_agent` from requirements.md frontmatter.

When implementation agent spawned:

1. Read specification files, module documentation (all from frontmatter)
2. Verify docs match reality (spot check key functions)
3. **If mismatch: STOP, report to Main Agent**
4. Implement changes with full context
5. Update module docs if structure changes
6. Report completion with documentation status

## Spec.md Master Index

### Purpose
Central index and dashboard for all specifications.

### Required Contents

**Template**: `.agents/templates/Spec-md-template.md`

The master Spec.md file provides:
- Overview of how specifications work
- List of all specifications with status
- Status dashboard with counts and percentages
- Organized by completion status (Completed, In Progress, Pending)

## Workflow and Process

### Pre-Work Review Agent (MANDATORY)

Before **ANY** agent starts work on tasks, a **review agent MUST be launched first**. This is a **HARD REQUIREMENT** with **NO EXCEPTIONS**.

#### Review Agent Purpose

**What to Read**: Load files per `files_required.review_agent` from requirements.md frontmatter.

1. **Read specification files thoroughly**
2. **Analyze current codebase** (search for implementations)
3. **Compare reality vs documentation**
4. **Verify task status accuracy** (check each [x] and [ ])
5. **Identify issues and blockers**
6. **Assess work readiness** (GO/STOP/CLARIFY)

#### STOP WORK IF

Review agent **MUST STOP ALL WORK** if: Inconsistencies found (tasks marked wrong), requirements unclear/incomplete, tasks need refinement, user input required, conflicting information, or technical blockers.

#### Report to Main Agent

Review agent **MUST** report: Current implementation state, verified task status, inconsistencies found, readiness assessment (GO/STOP/CLARIFY), and recommendations.

### Complete Workflow

```
1. User Requests Feature
   ↓
2. Main Agent: Thorough Requirements Conversation (see Requirements Conversation section)
   ↓
3. User Provides Answers + Main Agent Confirms Understanding
   ↓
4. Create Specification Directory (specifications/NN-feature-name/)
   ↓
5. Create requirements.md (with conversation summary)
   ↓
6. Add Tasks section to requirements.md (with task tracking in frontmatter)
   ↓
7. Create/Update Module Documentation (see Module Documentation System section)
   ↓
8. Update Spec.md Master Index
   ↓
9. Commit Specification + Module Documentation
   ↓
10. LAUNCH REVIEW AGENT (see Pre-Work Review Agent section)
   ↓
11. Launch Implementation Agents (see Implementation Agent Workflow section)
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

### Immediate Updates (MANDATORY)

**ZERO TOLERANCE RULE**: Agents MUST update specification files IMMEDIATELY as work progresses. NO batching, NO waiting, NO exceptions.

#### Requirements.md Updates (IMMEDIATE)

Agents **MUST** update requirements.md IMMEDIATELY when: new requirement discovered, user clarifies/changes requirement, technical constraint requires adjustment, integration reveals additional requirements, or user approves a requirement change.

**If user grants full rights**: auto-update requirements without approval. **DO NOT wait** until task completion or forget to sync with actual implementation.

**Why**: Requirements reflect current understanding, no discoveries lost, real-time user visibility, accurate context for future agents, prevents specification drift.

#### Task Updates (IMMEDIATE - EVERY TASK)

Agents **MUST**: Update the Tasks section in requirements.md IMMEDIATELY after completing EACH task, mark [x] the MOMENT finished, update frontmatter `tasks` counts immediately. **DO NOT wait**, batch updates, or create separate tracking files - requirements.md Tasks section is THE task tracker.

**When**: IMMEDIATELY after any task, before next task, before breaks/sessions end, before switching specifications.

**Why**: Real-time progress visibility, no completed work forgotten, user can check status anytime, system crashes won't lose tracking, other agents pick up exactly where left off, requirements.md Tasks section remains single source of truth.

**Frontmatter update requirements**:
```yaml
tasks:
  completed: [count of [x] tasks]
  uncompleted: [count of [ ] tasks]
  total: [completed + uncompleted]
  completion_percentage: [(completed / total) * 100]
metadata:
  last_updated: [YYYY-MM-DD - TODAY'S DATE]
```

#### All Requirements and Tasks Are Mandatory (DEFAULT)

**CRITICAL ASSUMPTION**: Unless user EXPLICITLY states otherwise, ALL requirements and tasks are MANDATORY.

**For requirements.md**:
- ✅ Assume ALL requirements must be implemented
- ✅ Assume ALL items must be completed
- ❌ DO NOT skip requirements thinking they are optional
- ❌ DO NOT treat any requirement as "nice-to-have" without explicit user confirmation

**For Tasks section**:
- ✅ Assume ALL tasks must be completed
- ✅ All tasks must be done before marking specification as complete
- ❌ DO NOT skip tasks thinking they are optional
- ❌ DO NOT leave tasks unchecked thinking "that can be done later"

**How user indicates optional items**:
- User explicitly says: "This requirement is optional"
- User explicitly says: "This task can be skipped if needed"
- Requirement/task is marked with "(OPTIONAL)" prefix
- User provides priority levels and explicitly says lower priority items are optional

**If in doubt**: ASK the user. Never assume something is optional.

## Quality and Verification

### Verification Requirements

#### Critical: Verify Actual Implementation

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

### Fundamentals Documentation Priority

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
2. **Add fundamentals tasks to the Tasks section in requirements.md as FIRST PRIORITY**

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

### 100% Completion Verification

Every `requirements.md` MUST include a complete "MANDATORY Completion and Verification Requirements" section.

**Complete Section Template**: See `.agents/templates/examples/completion-verification-section-example.md` for the full section to copy into requirements.md.

**This section enforces**:
1. **Task Completion Verification** - 100% tasks complete, NO exceptions
2. **Code/Implementation Verification** - All code exists and works
3. **Documentation Verification** - All docs exist and are comprehensive
4. **Quality Verification** - 0 build errors, 0 test failures, 0 linter warnings
5. **Specification Tracking Verification** - All tracking files exist
6. **Verification Issue Resolution** - ALL issues fixed, NO optional fixes

### Git Commit and Push Requirements

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

## Enforcement

### Zero Tolerance Violations (Comprehensive List)

**Process Violations**:
- ❌ Passively accepting requests without clarifying questions
- ❌ Starting implementation without review agent
- ❌ Ignoring review agent's STOP or CLARIFY directive
- ❌ Skipping or incomplete requirements conversation
- ❌ Starting implementation without user approval

**Documentation Violations**:
- ❌ Missing Agent Rules Reference section in requirements.md
- ❌ Missing or inaccurate module documentation
- ❌ Not verifying module docs match code
- ❌ Missing any of the 5 mandatory files
- ❌ Missing cross-reference links in requirements.md
- ❌ **Not updating requirements.md when new requirements discovered**
- ❌ **Batching task updates instead of updating immediately after each task**
- ❌ **Creating separate task tracking files instead of using the Tasks section in requirements.md**

**Completion Violations**:
- ❌ Marking complete without 100% verification
- ❌ Marking complete with unchecked tasks
- ❌ Proceeding when docs don't match code
- ❌ **Skipping tasks or requirements assuming they are optional without explicit user confirmation**
- ❌ Ignoring test failures or linter warnings as "optional"
- ❌ Missing fundamentals/ when has_fundamentals: true
- ❌ Creating fundamentals AFTER implementation (must be FIRST)

**Git Violations**:
- ❌ Delaying push after completion
- ❌ Leaving completed work unpushed
- ❌ Forgetting to push after final commit

**User Impact**: Violations cause user frustration, wasted effort, lost context, false progress, broken changes, and trust erosion.

### Consequences

When violation detected:

**Status must be reverted**:
- Status MUST be reverted to "in-progress"
- ALL incomplete items MUST be completed
- Verification MUST be re-run from scratch
- Specification CANNOT be marked complete until 100% PASS

**Work must be corrected**:
1. **Stop immediately** - No further work until corrected
2. **Launch review agent** if skipped
3. **Create/verify module docs** if missing or inaccurate
4. **Complete missing items** (specs, tasks, mandatory files)
5. **Update specification files** if out of sync (requirements.md, tasks.md with correct counts)
6. **Verify all checks pass** before resuming
7. **Report violation** to user with corrective actions taken

**Only proceed** when review reports GO and all documentation accurate.

### Corrective Actions

**Mandatory Compliance Checklist** - All agents **MUST**:
- ✅ Engage in thorough requirements conversation (3-10+ questions minimum)
- ✅ Create self-contained specifications (Agent Rules Reference, cross-references, enhanced frontmatter)
- ✅ Create/verify module documentation after requirements, before implementation
- ✅ Launch review agent BEFORE any implementation (read and act on report)
- ✅ Read specifications, tasks, review report, and module docs before working
- ✅ Verify documentation matches reality (STOP if mismatch)
- ✅ **Update requirements.md IMMEDIATELY when new requirements identified**
- ✅ **Update the Tasks section in requirements.md IMMEDIATELY after EACH task completion (no batching)**
- ✅ **Assume ALL tasks/requirements are mandatory unless user explicitly states otherwise**
- ✅ Update module docs as work progresses
- ✅ Create all 5 mandatory files (requirements with tasks, PROGRESS, FINAL_REPORT, LEARNINGS, VERIFICATION_SIGNOFF)
- ✅ Commit specification changes following Rules 03 and 04
- ✅ Push immediately after completion and verification

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
- Use Tasks section to track research activities

### Documentation-Only Changes
For pure documentation updates:
- May not need full specification
- Use judgment based on scope
- Major documentation overhauls should get specification

## Integration with Other Rules

- **Rule 03 (Dangerous Operations)**: Specification files follow commit-after-every-change rule
- **Rule 04 (Work Commit and Push)**: Specification changes pushed immediately, no approval needed
- **Rule 05 (Agent Orchestration)**: Main agent creates specs before launching specialized agents

## Summary

**Core Principle**: Never start significant work without documented requirements, thorough requirements conversation (3-10+ questions), review agent verification, and complete documentation.

**Workflow Order** (NO exceptions):
1. Requirements conversation → 2. Create specs → 3. Module docs → 4. Review agent → 5. Implementation → 6. Verification

**Critical Requirements**:
- ✅ Active requirements conversation with 3-10+ clarifying questions
- ✅ Self-contained requirements.md with Agent Rules Reference, cross-references, enhanced frontmatter
- ✅ Module documentation created/verified before implementation
- ✅ Review agent launched before any implementation work
- ✅ All 6 mandatory files created (requirements, tasks, PROGRESS, FINAL_REPORT, LEARNINGS, VERIFICATION_SIGNOFF)
- ✅ **PROGRESS.md is EPHEMERAL** - cleared after phases, deleted when complete
- ✅ **LEARNINGS.md is PERMANENT** - cumulative insights, never deleted
- ✅ **IMMEDIATE updates to requirements.md when new requirements identified**
- ✅ **IMMEDIATE updates to Tasks section in requirements.md after EACH task completion**
- ✅ **ALL requirements and tasks are MANDATORY unless user explicitly states otherwise**
- ✅ 100% completion verification before marking complete
- ✅ Push immediately after completion and verification

**Templates**: All templates in `.agents/templates/` - use these for consistency.

**Remember**: User will be upset if work proceeds without proper requirements conversation, review agent, accurate module docs, immediate updates, or complete verification!

---

*Created: 2026-01-11*
*Last Updated: 2026-01-24*
*Version: 6.4 - Consolidated content using frontmatter-based references, reduced verbosity while preserving all critical requirements and enforcement rules*
