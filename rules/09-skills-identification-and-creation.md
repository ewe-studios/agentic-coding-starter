# Skills Creation and Review (Main Agent Only)

## Purpose

This rule is for **Main Agent only** when creating new skills or reviewing skill documents. Sub-agents should load **Rule 11 (Skills Usage)** instead.

**Context Optimization**: Sub-agents only need Rule 11 (~150 lines) instead of this full rule.

---

## Overview

Skills are documented know-how for complex technical tasks. They capture:

- Tool/library usage (Playwright, Docker, Kubernetes)
- Pattern implementations (auth flows, caching strategies)
- Operations (migrations, API integration)
- Problem solutions (performance, security)

## Core Principles

### 1. User Approval Required

No skill can be used until user approves. This ensures user control over methodologies.

### 2. Last Resort Only

Skills are created ONLY when:

- ✅ Fundamental understanding is missing
- ✅ No existing skill covers the need
- ✅ No alternative approach possible
- ❌ NOT for trivial tasks
- ❌ NOT for basic programming

---

## Skills Directory Structure

### Location and Layout

```
.agents/skills/
├── [skill-name]/
│   ├── skill.md           # REQUIRED - Main documentation
│   ├── learnings.md       # OPTIONAL - Practical insights from usage
│   ├── assets/            # OPTIONAL - Supporting assets
│   │   ├── diagrams/      # Architecture diagrams, flowcharts
│   │   ├── configs/       # Config file templates
│   │   └── data/          # Sample data, fixtures
│   ├── docs/              # OPTIONAL - Extended documentation
│   │   ├── deep-dive.md   # Detailed explanations
│   │   ├── troubleshooting.md
│   │   └── faq.md
│   ├── templates/         # OPTIONAL - Code templates for TEMPLATE skills
│   │   ├── client.ts
│   │   └── helpers.ts
│   ├── scripts/           # OPTIONAL - Executable scripts for EXECUTABLE skills
│   │   └── run.sh
│   └── examples/          # OPTIONAL - Reference implementations
│       ├── basic-usage.rs
│       └── advanced-pattern.rs
```

### Directory Contents

| Directory      | Purpose                                      | When to Include                 |
| -------------- | -------------------------------------------- | ------------------------------- |
| `skill.md`     | Main documentation (REQUIRED)                | Always                          |
| `learnings.md` | Practical insights from usage                | After skill is used             |
| `assets/`      | Diagrams, configs, sample data               | When visual/config aids helpful |
| `docs/`        | Extended documentation, FAQ, troubleshooting | For complex skills              |
| `templates/`   | Code templates (TEMPLATE skills)             | When copying files to project   |
| `scripts/`     | Executable tools (EXECUTABLE skills)         | When running as external tools  |
| `examples/`    | Reference implementations (EDUCATIONAL)      | For learning patterns           |

### What Goes Where

**User-facing documentation** → `docs/`

- Deep dives the user might read for context
- FAQ for common questions
- Troubleshooting guides

**Agent-facing assets** → `assets/`, `templates/`, `scripts/`, `examples/`

- Items agents use during implementation
- Referenced by `skill.md`

**Critical Rule**: `skill.md` is the **single source of truth**. It references other files but contains all essential information for agents to use the skill.

### Naming Convention

- Skills use kebab-case: `playwright-web-interaction`, `kubernetes-deployment`
- **NO numeric prefixes** - skills are referenced by name
- Name clearly describes purpose

---

## Three Usage Types

### 1. TEMPLATE (Copy and Customize)

- Files in `templates/` are copied to project
- Agent customizes copied files
- ❌ **NEVER import from `.agents/skills/` in project code**

### 2. EXECUTABLE (Run as Tools)

- Scripts in `scripts/` run as external commands
- Consume output in project
- Never modify scripts

### 3. EDUCATIONAL (Learn and Implement)

- Examples in `examples/` teach patterns
- Install external dependencies listed
- Write fresh implementation
- ❌ **NEVER import from `.agents/skills/` in project code**

**See**: `.agents/templates/skill-usage-examples.md` for detailed code examples.

---

## Skill File Format

### Frontmatter (Required)

```yaml
---
name: "Skill Name"
description: "1-2 sentence summary"
approved: No
created: YYYY-MM-DD
license: "MIT"
metadata:
  author: "Main Agent"
  version: "1.0"
  last_updated: "YYYY-MM-DD"
  tags: [tag-1, tag-2]
tools: [Tool1, Tool2]
files:
  - templates/client.ts: "API client template"
  - scripts/run.sh: "Execution script"
assets:
  - docs/deep-dive.md: "Extended explanation for users"
  - assets/diagrams/flow.png: "Architecture diagram"
---
```

**Required Fields**: `name`, `description`, `approved`, `created`, `license`, `metadata`, `tools`
**Optional Fields**: `files` (agent-facing), `assets` (user/informational)

### Content Structure

**Template Location**: `.agents/templates/skill-template.md`

Key sections:

- Overview (2-3 paragraphs)
- When to Use (scope and limitations)
- Prerequisites (knowledge, dependencies)
- **Usage Type declaration** (TEMPLATE/EXECUTABLE/EDUCATIONAL)
- Attached Files (with clear instructions)
- Core Concepts
- Step-by-Step Guide
- Common Patterns
- Pitfalls to Avoid
- Examples
- References

### skill.md vs learnings.md

| File           | Purpose                          | When Read                      |
| -------------- | -------------------------------- | ------------------------------ |
| `skill.md`     | Canonical truth - BEFORE using   | During discovery + usage       |
| `learnings.md` | Practical insights - AFTER using | Only when actively using skill |

**Template**: `.agents/templates/learnings-template.md`

---

## Workflow: Skill Creation

### Phase 1: Identification (Sub-Agent)

Decision tree:

```
Do I understand how to accomplish this?
├─ YES → Proceed
└─ NO → Existing skill covers this?
          ├─ YES → Use if approved
          └─ NO → Quick research works?
                    ├─ YES → Learn and proceed
                    └─ NO → Create skill
```

### Phase 2: Creation (Sub-Agent)

1. **Research thoroughly**: Official docs, multiple sources
2. **Create skill directory**: `mkdir -p .agents/skills/[skill-name]`
3. **Create supporting files** (as needed):
   - `templates/` for TEMPLATE skills
   - `scripts/` for EXECUTABLE skills
   - `examples/` for EDUCATIONAL skills
   - `docs/` for extended user documentation
   - `assets/` for diagrams, configs
4. **Write skill.md**:
   - Complete frontmatter (`approved: No`)
   - Clear Usage Type
   - Reference all files in `files` and `assets` fields
   - Unambiguous instructions
5. **Report to Main Agent** (see reporting template in `.agents/templates/skill-usage-examples.md`)

### Phase 3: Review (Main Agent)

1. Read skill.md - verify frontmatter, content
2. Review all attached files for safety
3. Validate accuracy (use search)
4. Assess necessity
5. Report to user for approval

### Phase 4: User Approval

- **Approved**: Update `approved: Yes`, proceed
- **Rejected**: Use user's alternative
- **Needs Revision**: Update skill, return to Phase 3

---

## Skill Clarity Verification (Two Checkpoints)

### Checkpoint 1: During Requirements (Main Agent)

**When**: After requirements.md created with skills listed

Main Agent MUST:

1. Review each skill's `skill.md` completely
2. Verify Usage Type is clear
3. Confirm instructions are unambiguous
4. **Document in requirements.md**:
   ```markdown
   ## Skills Clarity Verification

   **Verified by Main Agent**: [Date]

   - [skill-name]: TEMPLATE - Clear
   - [skill-name]: EXECUTABLE - Clear
   ```
5. **If unclear**: Block requirements, report to user

### Checkpoint 2: Before Usage (Sub-Agent)

**When**: About to use skill for implementation

Sub-Agent MUST:

1. Read complete `skill.md` and `learnings.md`
2. Verify understanding of Usage Type
3. **If unclear**: STOP, report to Main Agent (see template in `.agents/templates/skill-usage-examples.md`)

---

## Skill Updates

### What Requires User Approval

- ✅ Any change to `skill.md`
- ✅ Any change to scripts/code
- ✅ Any change to `learnings.md`
- ✅ Adding/removing files

### Update Workflow

1. Agent makes changes
2. Main Agent reviews (validates correctness)
3. User reviews and approves
4. Changes finalized

---

## Specification Integration

Specifications reference skills in frontmatter:

```yaml
---
skills:
  - playwright-web-interaction
  - jwt-authentication
---
```

Direction: Specifications → Skills (one-way)

---

## Skill Scanning (Efficient Discovery)

```bash
# Scan frontmatter only
for skill in .agents/skills/*/skill.md; do
  head -n 20 "$skill"
done
```

- Match by name/description
- Check `approved: Yes`
- Read full content only when using

---

## Requirements Summary

### Sub-Agent MUST

- ✅ Think deeply before creating skills
- ✅ Check Usage Type before using
- ✅ Perform clarity check (Checkpoint 2)
- ✅ Follow Usage Type rules exactly
- ✅ Report skill creation to Main Agent
- ✅ Never use unapproved skills
- ✅ Update specs with skill references

### Sub-Agent MUST NOT

- ❌ Create skills for trivial tasks
- ❌ Use unapproved skills
- ❌ Import from `.agents/skills/` in project code
- ❌ Modify files in `.agents/skills/`
- ❌ Proceed with unclear skills
- ❌ Partially copy TEMPLATE skills (copy ALL)

### Main Agent MUST

- ✅ Review all skill documents
- ✅ Verify skill clarity (Checkpoint 1)
- ✅ Block requirements with unclear skills
- ✅ Report to user for approval
- ✅ Validate accuracy via search
- ✅ Ensure correctness of all assets: scripts, templates, documentations

### Main Agent MUST NOT

- ❌ Approve without user consent
- ❌ Skip clarity verification
- ❌ Allow unapproved skill usage

---

## Critical Violations

- Using unapproved skills
- Skipping clarity verification
- Importing from `.agents/skills/` in project code
- Modifying original skill files
- Partial TEMPLATE copying
- Creating skills for trivial tasks

---

## Integration with Other Rules

- **Rule 05**: Skills identified during work
- **Rule 06**: Specs include `skills` field
- **Rule 11**: Sub-agent usage guide

---

## Summary

**Core Workflow**:

```
Need → Research → Check existing → Create if necessary →
Main Agent review → User approval → Checkpoint 1 (requirements) →
Checkpoint 2 (usage) → Implementation → Update learnings
```

**Directory Structure**:

```
.agents/skills/[skill-name]/
├── skill.md        # Required - main doc
├── learnings.md    # Optional - practical insights
├── assets/         # Optional - diagrams, configs, data
├── docs/           # Optional - extended user docs
├── templates/      # Optional - for TEMPLATE skills
├── scripts/        # Optional - for EXECUTABLE skills
└── examples/       # Optional - for EDUCATIONAL skills
```

**Three Usage Types**:

1. **TEMPLATE**: Copy `templates/` → project, customize
2. **EXECUTABLE**: Run `scripts/` as external tools
3. **EDUCATIONAL**: Study `examples/`, install libs, implement fresh

**Critical Rules**:

- ❌ **NEVER import from `.agents/skills/` in project code**
- ✅ User approval required for creation AND updates
- ✅ Two mandatory clarity checkpoints

**Templates**:

- Skill structure: `.agents/templates/skill-template.md`
- Learnings format: `.agents/templates/learnings-template.md`
- Usage examples: `.agents/templates/skill-usage-examples.md`

---

_Created: 2026-01-13_
_Last Updated: 2026-01-20 (Added asset/documentation directories, moved examples to templates)_

---

## Related Rules

- **Rule 11 (Skills Usage)**: Concise guide for sub-agents
- **Rule 06 (Specifications)**: How specifications reference skills
- **Rule 05 (Agent Orchestration)**: Skills identification during work
