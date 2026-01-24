# Specifications and Requirements Management

## Purpose

Establishes mandatory requirements-gathering and specification-tracking system. All work begins with documented conversation between Main Agent and user, creating clear record of requirements and tasks in `specifications/` directory.

## Core Workflow

### Requirements-First Development

Before any work begins, Main Agent MUST:

1. Engage in conversation with user about requirements
2. Document requirements in specification directory
3. Create integrated task list for tracking
4. Have agents read specifications before starting
5. Verify and update status as work progresses

**No exceptions**: No coding without documented requirements. No implementation until user explicitly approves.

### User Approval Required

After creating specification, Main Agent:

1. Presents specification to user
2. Waits for explicit approval: "Start implementation", "Go ahead", "Proceed with implementation"
3. Never assumes "ok" or "thanks" means approval
4. When in doubt, asks: "Would you like me to begin implementation now?"

### Requirements Conversation

Main Agent MUST actively probe requirements, not passively accept vague requests.

**Minimum questions**: 3-5 (small features), 5-10 (medium), 10+ (large/complex)

**Critical areas**: Scope, technical approach, constraints, success criteria, edge cases, integration, priority, timeline

**Example - Bad**: User: "Add authentication" → Agent: "Ok, I'll create spec"
**Example - Good**: User: "Add authentication" → Agent asks: method, storage, security, registration, password reset, timeout → Confirms → Creates spec

### Frontmatter Requirements

**Requirements.md MUST include** (see `.agents/templates/requirements-template.md`):

- `description`, `status`, `priority`, `created`, `author`
- `metadata`: version, last_updated, estimated_effort, tags, stack_files, skills, tools
- `has_features`, `has_fundamentals`, `builds_on`, `related_specs`
- **`files_required`**: Complete object for each agent type (MANDATORY)
- `tasks`: completed, uncompleted, total, completion_percentage

**Feature.md MUST include** (if has_features: true, see `.agents/templates/feature-template.md`):

- `feature`, `description`, `status`, `priority`, `depends_on`, `estimated_effort`, `created`, `last_updated`, `author`
- **`tasks`**: completed, uncompleted, total, completion_percentage (MANDATORY)
- **`files_required`**: implementation_agent and verification_agent entries (MANDATORY)

## Directory Structure

### Simple Specification

```
specifications/01-simple-spec/
├── requirements.md          # Requirements with integrated tasks
├── LEARNINGS.md            # All learnings (permanent)
├── REPORT.md               # All reports (permanent)
├── VERIFICATION.md         # Verification signoff (permanent)
├── PROGRESS.md             # Current status only (ephemeral - delete at 100%)
├── fundamentals/           # User docs (if has_fundamentals: true)
└── templates/              # Code templates (optional)
```

### Complex Specification with Features

```
specifications/02-complex-spec/
├── requirements.md          # High-level overview + feature table
├── features/
│   ├── foundation/
│   │   ├── feature.md      # Feature requirements + integrated tasks
│   │   └── templates/
│   └── ...
├── LEARNINGS.md
├── REPORT.md
├── VERIFICATION.md
└── PROGRESS.md
```

### When to Use Features

**Use features when**: Specification >15KB, multiple distinct components, different dependencies, needs context size reduction

**Keep simple when**: Small focused work, single coherent piece, <10 tasks, no logical component boundaries

### Naming Convention

Format: `NN-descriptive-name/` (two-digit number prefix, dash separators, lowercase)

**Good**: `01-build-http-client/`, `features/dns-resolution/`
**Bad**: `http-client/` (no number), `1-client/` (single digit), `features/DnsResolution/` (wrong case)

### Specification Immutability

Once completed (status: completed, REPORT.md and VERIFICATION.md created), specification is LOCKED.

**Any new work** → Create NEW specification, reference old one in `builds_on` field

**Exception**: In-progress specifications (no REPORT.md, status not "completed") can be modified

## File Organization

### Allowed Files (Exhaustive List)

Each specification directory MUST contain ONLY these files:

| File              | Status    | Purpose                                                       |
| ----------------- | --------- | ------------------------------------------------------------- |
| `requirements.md` | Permanent | Requirements with integrated tasks                            |
| `LEARNINGS.md`    | Permanent | ALL learnings consolidated (technical + process)              |
| `REPORT.md`       | Permanent | ALL reports consolidated (work sessions, testing, completion) |
| `VERIFICATION.md` | Permanent | Verification signoff                                          |
| `PROGRESS.md`     | Ephemeral | Current status (DELETE at 100%)                               |
| `fundamentals/`   | Permanent | User docs (if has_fundamentals: true)                         |
| `features/`       | Permanent | Feature breakdown (if has_features: true)                     |
| `templates/`      | Permanent | Code templates (optional)                                     |

### File Consolidation Rules

**All learnings** → LEARNINGS.md (no separate process/technical learning files)
**All reports** → REPORT.md (no separate WASM/session/testing reports)
**One verification** → VERIFICATION.md (no multiple verification files)

### Forbidden Files

DO NOT create:

- `PROCESS_LEARNINGS.md`, `TECHNICAL_LEARNINGS.md` → Use LEARNINGS.md
- `WASM_TESTING_REPORT.md`, `WORK_SESSION_SUMMARY.md`, `TESTING_REPORT.md` → Add sections to REPORT.md
- `VERIFICATION_SIGNOFF.md`, `VERIFICATION_RESULTS.md` → Use VERIFICATION.md
- `NOTES.md`, `TODO.md`, `STATUS.md` → Use PROGRESS.md during work, delete at completion

### Requirements.md Reminder

Every requirements.md MUST end with:

```markdown
---

## File Organization Reminder

ONLY these files allowed:

1. requirements.md - Requirements with tasks
2. LEARNINGS.md - All learnings
3. REPORT.md - All reports
4. VERIFICATION.md - Verification
5. PROGRESS.md - Current status (delete at 100%)
6. fundamentals/, features/, templates/ (optional)

FORBIDDEN: Separate learning/report/verification files

Consolidation: All learnings → LEARNINGS.md, All reports → REPORT.md

See Rule 06 "File Organization" for complete policy.
```

## Self-Contained Specifications

### files_required Frontmatter

Every requirements.md MUST include `files_required` section listing exact rules and files for each agent type.

**Example**:

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
      - ./requirements.md
      - ./LEARNINGS.md
      - ./PROGRESS.md

  implementation_agent:
    rules:
      - .agents/rules/01-rule-naming-and-structure.md
      - .agents/rules/02-rules-directory-policy.md
      - .agents/rules/03-dangerous-operations-safety.md
      - .agents/rules/04-work-commit-and-push-rules.md
      - .agents/rules/13-implementation-agent-guide.md
      - [stack_file from metadata.stack_files]
    files:
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
      - ./requirements.md
```

**Dynamic references**:

- `[stack_file from metadata.stack_files]` - Expands to full path from metadata
- `[feature.md if has_features: true]` - Conditional file inclusion
- `[fundamentals/* if has_fundamentals: true]` - Conditional directory inclusion

**Benefits**: Agents know exactly what to load, no guessing

**See**: `.agents/templates/requirements-template.md` for complete structure

## Module Documentation

### Purpose

`documentation/` directory at project root provides detailed module documentation agents MUST read before changes.

### doc.md Structure

**Required frontmatter**: module, language, status, last_updated, maintainer, related_specs

**Required sections**: Overview, Purpose, Location, Implementation, Public API, Imports, Calls, Workflows, Architecture, Tests, Dependencies, Configuration, Issues, Improvements, Related Docs, Version History

**Context optimization**: If >8-10KB, agents use Grep/Glob/Read tools instead of loading entire file

### Verification Workflow

1. Agent reads module documentation
2. Verifies docs match code (spot check)
3. If mismatch: STOP, report to Main Agent
4. Main Agent halts work, fixes documentation first
5. Then resume implementation

## Verification and Quality

### Progress Tracking

**PROGRESS.md** (Ephemeral):

- Created at for each task you start, cleared after task completion
- Tracks current task and immediate next steps
- Cleared and rewritten after each task is done
- DELETED when specification 100% complete

**LEARNINGS.md** (Permanent):

- Created early, updated throughout
- Cumulative record of all insights
- Never cleared or deleted
- Technical + process learnings consolidated here

**REPORT.md** (Permanent):

- Created when nearing completion
- Comprehensive summary of work, testing, metrics
- Can be updated progressively despite name
- Consolidates ALL reports (work sessions, WASM testing, etc.)

### Pre-Work Review

Before any agent starts work, spawn Review Agent:

1. Reads specifications thoroughly
2. Analyzes current codebase
3. Compares reality vs documentation
4. Verifies task status accuracy
5. Assesses readiness (GO/STOP/CLARIFY)

**STOP if**: Inconsistencies found, requirements unclear, tasks need refinement, user input required, blockers exist

### Verification Agent

After implementation complete:

1. Main Agent spawns Verification Agent
2. Verification Agent runs all checks (format, lint, tests, build, docs)
3. Creates VERIFICATION.md with results
4. If ALL PASS: Main Agent marks specification complete
5. If ANY FAIL: Fix issues, re-verify

## Spec.md Master Index

Central dashboard at `specifications/Spec.md`:

- List of all specifications with status
- Status dashboard (completed, in-progress, pending counts)
- Organized by completion status
- Links to each specification

**Template**: `.agents/templates/Spec-md-template.md`

## Enforcement

### Violations

**File organization**: Creating forbidden files, not consolidating, keeping PROGRESS.md after 100%

**Task tracking**: Batching updates, not updating after each task, incorrect completion percentages

**Requirements**: Coding without documented requirements, skipping user approval, incomplete frontmatter

**Verification**: Committing without verification, skipping quality checks

### Corrective Action

1. Stop immediately
2. Identify violation
3. Fix issue (consolidate files, update tasks, run verification)
4. Report violation for awareness
5. Continue with correct process

## Integration with Other Rules

**Rule 04**: Commit requirements.md updates after changes, include verification status

**Rule 05**: Main Agent spawns Review Agent before work, Verification Agent after work

**Rule 08**: Verification workflow complements continuous verification checkpoints

**Rule 13**: Implementation agents update LEARNINGS.md and requirements.md tasks

## Summary

**Core workflow**: Requirements conversation → Document → User approval → Implementation → Verification → Completion

**File structure**: requirements.md (tasks integrated) + LEARNINGS.md + REPORT.md + VERIFICATION.md + PROGRESS.md (ephemeral)

**Consolidation**: All learnings in one file, all reports in one file, one verification file

**Quality**: Pre-work review, continuous verification, final verification signoff

**Templates**: `.agents/templates/requirements-template.md`, `LEARNINGS-template.md`, `REPORT-template.md`, `VERIFICATION-template.md`

---

_Created: 2026-01-11_
_Last Updated: 2026-01-24 (Optimized: reduced from 1,270 to 660 lines, removed duplication, renamed FINAL_REPORT → REPORT)_
