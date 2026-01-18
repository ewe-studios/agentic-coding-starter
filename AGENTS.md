---
purpose: Central entry point for AI agent configuration
description: Minimal configuration directing agents to load rules selectively
version: 5.0.0
last_updated: 2026-01-19
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
| 01 | File naming conventions |
| 02 | Directory policies |
| 03 | Dangerous operations safety |
| 04 | Work commit and push rules |

### By Role

| Agent Type | Load These Rules |
|------------|------------------|
| **Main Agent** | 01-04, 05, 06 (+ 09, 10 when creating skills/agents) |
| **Implementation Agent** | 01-04, 13, 11 (if skills), stack file, spec |
| **Verification Agent** | 01-04, 08, stack file |
| **Specification Agent** | 01-04, 06 |
| **Any Sub-Agent** | 01-04, 12, own agent doc, relevant stack |

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

_Version: 5.0.0 - Selective rule loading for context optimization_
