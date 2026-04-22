---
purpose: Central entry point for AI agent configuration
description: Simplified agent system with skill-based architecture
version: 6.1.0
last_updated: 2026-03-02
---

# Agent Configuration

## ⚡ Fast Rust Check Workflow

**CRITICAL:** When doing Rust checks and you're not sure about something, use quick `cargo check` with `tee` to collect warnings rapidly. **5-6 minutes should be enough** for the initial pass. If after 5-6 minutes you see nothing, increase to 10 minutes on the next run. If it finishes with no issues — great. If you see issues early, collect enough in 5 minutes then go fix them. This increases iteration speed and reduces wasted wait time.

---

## Core Principle

**Every agent reads their documentation file which specifies skills to load.**

Agents load **only what they need** to optimize context window usage.

## Agent Registry

All agents are documented in `.agents/agents/` directory:

1. **[Main Agent]** read `.agents/agents/main-agent.md`
2. **[Implementation Agent]** read `.agents/agents/implementation.md`
3. **[Rust Verification Agent]** read `.agents/agents/rust-verification.md`
4. **[JavaScript Verification Agent]** read `.agents/agents/javascript-verification.md`
5. **[Python Verification Agent]** read `.agents/agents/python-verification.md`
6. **[Generic Verification Agent]** read `.agents/agents/verification.md`
7. **[Specification Update Agent]** read `.agents/agents/specification-update.md`
8. **[Review Agent]** read `.agents/agents/review.md`
9. **[Documentation Agent]** read `.agents/agents/documentation.md`
10. **[Rust Cleanup Agent]** read `.agents/agents/rust-cleanup.md`

## For Sub-Agents

Main Agent provides documentation path when spawning. If not provided, request it.

---

## Critical Reminders

1. **Retrieval-Led Reasoning**: Read codebase FIRST, follow discovered patterns, verify assumptions
2. **Agent Documentation**: Each agent reads their own documentation file which specifies skills to load
3. **Skills-Based**: Load only required skills, not everything
4. **Main Agent**: Orchestrator only - delegates ALL work
5. **Verification Required**: NO commits without verification passing
6. **Incomplete Implementation Check**: MANDATORY first check in verification
7. **Sub-agents**: Never commit directly, never spawn verification agents
8. **Context Optimization**: Load ONLY what you need

**Enforcement**: Before any implementation, agents MUST demonstrate retrieval by:
1. Searching for similar implementations
2. Reading relevant existing code
3. Checking project conventions
4. Following discovered patterns

## Context Management

**When approaching 800K tokens (80% usage)**:

1. Write summary to PROGRESS.md (what's done, what's next)
2. Update compacted.md with compressed state
3. Clear context, reload from saved files
4. Continue work

**PROGRESS.md must include**:
- Completed work list
- Files modified with specific changes
- Remaining tasks (numbered steps)
- Next immediate action

---

## Directory Structure

```
.agents/
├── AGENTS.md           # This file
├── agents/             # Agent documentation (read your own)
├── skills/             # Reusable skills (load as specified by agent docs)
└── templates/          # File templates

specifications/         # Read when working on features
documentation/          # Read for modules you're changing
```

---

_Version: 6.1.0 - Last Updated: 2026-03-02_

_Simplified architecture: rule.md → agents → skills_
_Added: Context Management guidelines for token optimization_

## ⚡ Rust Check Reminder

When running Rust checks: use `cargo check 2>&1 | tee /tmp/cargo-check.log` with a **5-6 minute timeout** for quick scans. Only extend to 10 minutes if the first pass shows nothing. Fix issues as soon as they appear — don't wait for a full check to complete.
