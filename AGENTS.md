---
purpose: Central entry point for AI agent configuration
description: Minimal configuration directing agents to load rules selectively
version: 5.1.0
last_updated: 2026-01-24
---

# Agent Configuration

## Core Principle

Agents load rules **selectively** based on role and task to optimize context window usage.

**CRITICAL**: `.agents/rules/*` takes precedence over this file if conflicts arise.

---

## Rule Loading

### Mandatory (ALL Agents)

| Rule | Topic |
|------|-------|
| [01](./rules/01-rule-naming-and-structure.md) | File naming conventions |
| [02](./rules/02-rules-directory-policy.md) | Directory policies |
| [03](./rules/03-dangerous-operations-safety.md) | Dangerous operations safety |
| [04](./rules/04-work-commit-and-push-rules.md) | Work commit and push rules |

### By Role

| Agent Type | Load These Rules |
|------------|------------------|
| **Main Agent** | 01-04, [05](./rules/05-coding-practice-agent-orchestration.md), [06](./rules/06-specifications-and-requirements.md) (+ [09](./rules/09-skills-identification-and-creation.md), [10](./rules/10-agent-documentation-and-registry.md) when creating skills/agents) |
| **Implementation Agent** | 01-04, [13](./rules/13-implementation-agent-guide.md), [11](./rules/11-skills-usage.md) (if skills), stack file, spec |
| **Verification Agent** | 01-04, [08](./rules/08-verification-workflow-complete-guide.md), stack file |
| **Specification Agent** | 01-04, [06](./rules/06-specifications-and-requirements.md) |
| **Any Sub-Agent** | 01-04, [12](./rules/12-agent-registry-usage.md), own agent doc, relevant stack |

---

## Rules Reference

| Rule | For | Purpose |
|------|-----|---------|
| 01-04 | All | Core mandatory rules |
| 05 | Main Agent | Agent orchestration and verification coordination |
| 06 | Main Agent, Spec agents | Specifications and requirements |
| 07 | Code writers | Language conventions |
| 08 | Verification agents | Verification workflow |
| 09 | Main Agent | Skills creation and review |
| 10 | Main Agent | Agent documentation and creation |
| 11 | Sub-agents | Skills usage (concise) |
| 12 | Sub-agents | Agent registry usage (concise) |
| 13 | Implementation agents | Coding practice guide (concise) |

---

## Directory Structure

```
.agents/
├── AGENTS.md           # This file
├── rules/              # Load selectively
├── stacks/             # Load for your language only
├── skills/             # Scan frontmatter, read when using
├── agents/             # Scan frontmatter, read own doc
└── templates/          # Reference when creating files

specifications/         # Read when working on features
documentation/          # Read for modules you're changing
```

---

## Spawning Sub-Agents

Include in spawn prompt:
```
MANDATORY: Load Rules 01-04, Rule 12, your doc at .agents/agents/[name].md
OPTIONAL: Rule 11 (skills), Rule 13 (implementation), stack file, spec
```

---

## Critical Reminders

1. **Main Agent**: Orchestrator only - delegates ALL work
2. **Verification Required**: NO commits without verification passing
3. **Context Optimization**: Load ONLY what you need
4. **Sub-agents**: Never commit directly, never spawn verification agents

---

## Version History

### Version 5.1.0 - 2026-01-24
- Updated agent references to use requirements.md (tasks integrated into requirements.md)
- Emphasized files_required frontmatter as source of truth for agent context

### Version 5.0.0 - 2026-01-19
- Selective rule loading for context optimization

---

_Version: 5.1.0 - Selective rule loading with requirements.md as single source of truth_
