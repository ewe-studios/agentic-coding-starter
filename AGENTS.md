---
purpose: Central entry point for AI agent configuration
description: Minimal configuration file that directs agents to detailed rules and standards
version: 4.0.0
last_updated: 2026-01-18
---

# Agent Configuration

## Core Principle

This file is the **entry point** for all AI agents. For detailed rules, standards, and workflows, agents **MUST** load the files referenced below.

**CRITICAL**: If conflicts arise, `.agents/rules/*` takes precedence over this file.

---

## Mandatory Loading Sequence

All agents **MUST** follow this sequence at the start of every session:

1. ✅ **Read `AGENTS.md`** (this file)
2. ✅ **Load ALL rules from `.agents/rules/*`** (in numerical order)
3. ✅ **Load ONLY relevant stack files** from `.agents/stacks/[language].md`
4. ✅ **Read specification files** (if working on a specific feature)

**Context Window Efficiency**: Only load stack files for languages you will actually use.

---

## Directory Structure

```
.agents/
├── AGENTS.md           # This file (entry point)
├── rules/              # Project rules (READ ALL)
├── stacks/             # Language-specific standards (READ ONLY WHAT YOU USE)
├── skills/             # Documented know-how for specific tasks
├── agents/             # Agent documentation and registry
└── templates/          # File templates

specifications/         # Feature specifications (PROJECT ROOT)
documentation/          # Module documentation (PROJECT ROOT)
CLAUDE.md              # Backward compatibility redirect
```

---

## What Each Directory Contains

| Directory | Purpose | Details In |
|-----------|---------|------------|
| `.agents/rules/` | HOW agents work (workflow, verification, commits) | Read all files |
| `.agents/stacks/` | HOW to write code (language standards) | `[language].md` |
| `.agents/skills/` | Specialized task knowledge | `skill.md` per skill |
| `.agents/agents/` | Agent capabilities and registry | `[agent-name].md` |
| `.agents/templates/` | Reusable file templates | Reference when creating files |
| `specifications/` | WHAT to build (requirements, tasks) | Rule 06 |
| `documentation/` | Module documentation | Rule 06 |

---

## Key Rules Summary

| Rule | Topic |
|------|-------|
| 01 | File naming conventions |
| 02 | Directory policies |
| 03 | Dangerous operations safety |
| 04 | Work commit and push rules |
| 05 | Coding practice and agent orchestration |
| 06 | Specifications and requirements |
| 07 | Language conventions and standards |
| 08 | Verification workflow |
| 09 | Skills identification and creation |
| 10 | Agent documentation and registry |

**⚠️ MANDATORY**: Read ALL rule files before starting any work.

---

## Critical Reminders

1. **Main Agent Role**: Orchestrator ONLY - delegates ALL work to specialized agents
2. **Verification Required**: NO code commits without verification passing
3. **Zero Deviation**: All standards must be followed exactly
4. **Delegation Always**: Main Agent spawns agents for specification updates
5. **Learning Logs**: Update when mistakes are made or patterns discovered

→ **For complete details**: Read `.agents/rules/*`

---

## Quick Reference

| Need To... | Read |
|------------|------|
| Understand workflow | Rule 05 |
| Write specifications | Rule 06 |
| Run verification | Rule 08 |
| Write Rust/JS/Python | `.agents/stacks/[language].md` |
| Create a skill | Rule 09 |
| Document an agent | Rule 10 |

---

**MANDATORY**: Load and read all relevant files before starting work.

---

_Last updated: 2026-01-18_
_Version: 4.0.0 - Simplified to minimal entry point (removed duplicated details now in Rule 06)_
