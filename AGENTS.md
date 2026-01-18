---
purpose: Central entry point for AI agent configuration
description: Minimal configuration file that directs agents to detailed rules and standards
version: 3.1.0
last_updated: 2026-01-18
---

# Agent Configuration

## Core Principle

This file is the **entry point** for all AI agents. For detailed rules, standards, and workflows, agents **MUST** load the files referenced below.

---

## Mandatory Loading Sequence

All agents **MUST** follow this sequence at the start of every session:

1. ✅ **Read `AGENTS.md`** (this file)
2. ✅ **Load ALL rules from `.agents/rules/*`** (read all files in numerical order)
3. ✅ **Load ONLY relevant stack files** from `.agents/stacks/[language].md`:
   - **ONLY** load stack files for languages you will actually use
   - Check specification requirements.md for Language Stack section
   - Use file names and frontmatter to identify relevant files
   - **DO NOT** load all stack files - wastes context window
4. ✅ **Read specification files** (if working on a specific feature)

**CRITICAL**: If conflicts arise, `.agents/rules/*` takes precedence over this file.

---

## Directory Structure

```
.agents/
├── AGENTS.md           # This file (entry point)
├── rules/              # Detailed project rules (READ ALL OF THESE)
│   ├── 01-rule-naming-and-structure.md
│   ├── 02-rules-directory-policy.md
│   ├── 03-dangerous-operations-safety.md
│   ├── 04-work-commit-and-push-rules.md
│   ├── 05-coding-practice-agent-orchestration.md
│   ├── 06-specifications-and-requirements.md
│   ├── 07-language-conventions-and-standards.md
│   ├── 08-verification-workflow-complete-guide.md
│   ├── 09-skills-identification-and-creation.md
│   ├── 10-agent-documentation-and-registry.md
│   └── ...
│
├── stacks/             # Language-specific standards (READ ONLY WHAT YOU USE)
│   ├── javascript.md   # JavaScript/TypeScript standards
│   ├── rust.md         # Rust standards
│   ├── python.md       # Python standards
│   └── ...
│
├── skills/             # Documented know-how for specific tasks
│   └── [skill-name]/
│       ├── skill.md    # Canonical skill documentation
│       └── learnings.md # Practical insights from usage
│
└── agents/             # Agent documentation and registry
    └── [agent-name].md # Agent capabilities and responsibilities

specifications/         # Feature specifications (PROJECT ROOT LEVEL)
├── Spec.md             # Master index
├── NN-spec-name/
│   ├── requirements.md
│   ├── tasks.md
│   ├── PROGRESS.md     # Mid-work progress report
│   ├── FINAL_REPORT.md # Completion summary
│   ├── LEARNINGS.md    # Lessons learned
│   ├── VERIFICATION_SIGNOFF.md # Official verification
│   ├── verification.md # (transient, created on verification failure)
│   └── learnings.md    # (optional, specification-specific insights)
└── ...

documentation/          # Module documentation (PROJECT ROOT LEVEL)
└── [module]/
    ├── doc.md          # Detailed module documentation
    └── assets/         # Supplementary documentation

CLAUDE.md              # Backward compatibility redirect
```

---

## What Each Directory Contains

### `.agents/rules/` - HOW Agents Must Work

**Purpose**: Defines workflow, orchestration, verification, and commit processes.

**Key Rules**:

- **Rule 01**: File naming conventions
- **Rule 02**: Directory policies
- **Rule 03**: Dangerous operations safety (Git Safety Checkpoint required)
- **Rule 04**: Work commit and push rules (immediate commit + auto-push)
- **Rule 05**: Coding practice and agent orchestration (IRON-CLAD verification)
- **Rule 06**: Specifications and requirements management
- **Rule 07**: Language conventions and standards
- **Rule 08**: Verification workflow complete guide
- **Rule 09**: Skills identification and creation
- **Rule 10**: Agent documentation and registry

**⚠️ MANDATORY**: Read ALL rule files before starting any work.

→ **For full details**: Read all files in `.agents/rules/`

### `.agents/stacks/` - HOW to Write Code

**Purpose**: Language-specific coding standards, conventions, verification workflows.

**Contains**:

- Coding standards and naming conventions
- Best practices and common pitfalls
- Verification workflow (commands to run)
- Learning Logs (self-improving)
- Tool configurations

**⚠️ CRITICAL - Context Window Efficiency**:

- **ONLY** read stack files for languages you will actually use
- Check `requirements.md` Language Stack section to identify languages
- Use file names (rust.md, javascript.md, python.md) to identify content
- Check file frontmatter for quick overview before loading
- **DO NOT** read all stack files - this wastes valuable context window space

**⚠️ MANDATORY**: Read relevant stack file(s) before writing ANY code.

→ **For full details**: Read `.agents/stacks/[language].md` for your language(s) ONLY

### `specifications/` - WHAT to Build

**Location**: Project root level (parallel to `.agents/` directory)

**Purpose**: Feature requirements, task tracking, progress reports, verification.

**Contains**:

- `requirements.md`: What to build and why
- `tasks.md`: Task list with checkboxes and progress tracking
- `PROGRESS.md`: Mid-work progress report (~50% completion)
- `FINAL_REPORT.md`: Comprehensive completion summary
- `LEARNINGS.md`: Lessons learned and insights
- `VERIFICATION_SIGNOFF.md`: Official verification report
- `verification.md`: Detailed verification failure reports (transient)
- `learnings.md`: Specification-specific practical insights (optional)

**⚠️ MANDATORY**: Read specification files when working on a feature.

→ **For full details**: Read files in `specifications/NN-spec-name/`

### `documentation/` - Module Documentation

**Location**: Project root level (parallel to `.agents/` and `specifications/`)

**Purpose**: Living documentation of individual code modules.

**Contains**:

- `doc.md`: Detailed module documentation (what implements, imports, calls, does)
- `assets/`: Supplementary documentation (diagrams, schemas, examples)

### `.agents/skills/` - Specialized Knowledge

**Purpose**: Documented know-how for accomplishing specific technical tasks.

**Contains**:

- `skill.md`: Canonical skill documentation
- `learnings.md`: Practical insights from actual usage
- Supporting code files (templates, executables, examples)

**Usage Types**: TEMPLATE (copy all files), EXECUTABLE (run as tool), EDUCATIONAL (learn & implement)

→ **For full details**: Read `.agents/skills/[skill-name]/skill.md`

### `.agents/agents/` - Agent Registry

**Purpose**: Documentation of all available agents and their capabilities.

**Contains**:

- Agent documentation files (`[agent-name].md`)
- Frontmatter for quick scanning and selection
- Detailed capabilities, responsibilities, and boundaries

**⚠️ MANDATORY**: All agents must be documented before spawning.

→ **For full details**: Read `.agents/agents/[agent-name].md`

---

## Quick Start Checklist

Before starting ANY work:

- [ ] Load AGENTS.md (this file)
- [ ] **Read ALL files in `.agents/rules/`** (detailed workflow rules)
- [ ] **Identify languages from specification** (check requirements.md Language Stack section)
- [ ] **Read ONLY relevant `.agents/stacks/[language].md`** (DO NOT load all - use context efficiently)
- [ ] Read specification `requirements.md` and `tasks.md` (if applicable)
- [ ] Understand verification workflow from Rule 04 and stack files
- [ ] Follow orchestration model: Main Agent delegates, specialized agents do work

---

## Critical Reminders

1. **Main Agent Role**: Orchestrator ONLY. Delegates ALL work to specialized agents. Never performs work directly.

2. **Verification Requirement**: NO code commits without verification. Implementation agents report to Main Agent → Verification agent runs checks → Specification agent updates tasks → Main Agent commits.

3. **Zero Deviation**: All standards in rules and stack files must be followed exactly. No exceptions.

4. **Delegation Always**: Main Agent never reads/writes specification files directly. Always spawns Specification Update Agent.

5. **Learning Logs**: Update stack file Learning Logs when mistakes are made or new patterns discovered.

→ **For complete details on these principles**: Read `.agents/rules/04-coding-practice-agent-orchestration.md` and `.agents/rules/07-language-conventions-and-standards.md`

---

## Where to Find Detailed Information

| Topic                       | Location                                                   |
| --------------------------- | ---------------------------------------------------------- |
| Dangerous operations safety | `.agents/rules/03-dangerous-operations-safety.md`          |
| Work commit and push rules  | `.agents/rules/04-work-commit-and-push-rules.md`           |
| Workflow and orchestration  | `.agents/rules/05-coding-practice-agent-orchestration.md`  |
| Verification process        | `.agents/rules/05-*` + `.agents/stacks/[language].md`      |
| Complete verification guide | `.agents/rules/08-verification-workflow-complete-guide.md` |
| Specification format        | `.agents/rules/06-specifications-and-requirements.md`      |
| Language standards          | `.agents/stacks/[language].md`                             |
| Skills system               | `.agents/rules/09-skills-identification-and-creation.md`   |
| Agent registry              | `.agents/rules/10-agent-documentation-and-registry.md`     |

---

**Remember**: This file is just the entry point. The real details are in:

- `.agents/rules/*` (HOW agents work)
- `.agents/stacks/*` (HOW to write code)
- `.agents/skills/*` (WHAT specialized knowledge to use)
- `.agents/agents/*` (WHO does the work - agent registry)
- `specifications/*` (WHAT to build - project root)
- `documentation/*` (WHERE code lives - module docs, project root)

**MANDATORY**: Load and read all relevant files before starting work.

---

_Last updated: 2026-01-18_
_Version: 3.1.0 - Added Rules 09-10, corrected specifications/documentation locations to project root, added skills and agents directories_
