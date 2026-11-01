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

## Crate Hygiene: Check + Clippy Per Crate

When you finish a crate, clear its warnings before you call the work done. You
do the hard thinking; agents do the execution. Two phases:

### Phase 1 — Plan

1. Spawn one fresh planner agent for the crate.
2. The planner runs both commands from the workspace root:
   - `cargo check -p <crate> --profile uat`
   - `cargo clippy -p <crate> --profile uat --no-deps` (without `--no-deps`,
     clippy re-lints every workspace path-dependency and floods the log)
3. The planner catalogues every warning and suggestion (file, line, message,
   lint name) and reasons about the right fix for each one. It sorts them into:
   - fix now (safe, mechanical, behaviour-preserving),
   - fix now but add a public API (a `#[must_use]`, a `Default` impl, an
     `is_empty`, a doc section),
   - decide (casts, API signature changes, large-function splits, layout
     changes).
   It writes the plan with the exact change for each item, and does not edit
   code.
4. You read the plan and check it makes sense. Correct it where it does not.

### Phase 2 — Execute

5. Spawn one or more executor agents to apply the plan. Put the acceptance
   criteria (zero warnings, tests green) and the correctness requirements (do
   not change behaviour, do not remove a feature-gated dependency, do not panic
   where the code returns an error) in each brief.
6. The executors fix the "fix now" items, then run the crate's tests
   (`cargo test -p <crate> --profile uat`) and report the real output. Green
   `cargo check` and `cargo clippy` are not enough: a clippy "simplification"
   can change behaviour that only a test catches, so an executor that does not
   run the tests has not finished. They report what they changed and what they
   left and why, and must not commit. Nothing is "pre-existing" or "not ours":
   a warning in code we touched and a warning in code we did not touch are the
   same defect, and both are ours to fix.
7. You review and validate the reports against the plan, decide the "decide"
   items, run the final verification (including the tests), then commit. Do not
   stand by waiting — move on to the next feature or crate and pick the reports
   up when they land.
8. After you commit, spawn one agent to run the crate's full CI inside the
   Linux Docker image: `scripts/ci/linux.sh <crate>` (image
   `ewestudios/ewe-platform-ci:latest`, platform `linux/amd64`). A macOS pass
   is not proof of Linux compatibility.

The goal is zero warnings: `cargo check`, `cargo clippy` and the tests all
green, with nothing printed. Do not suppress a warning with `#[allow]` or
`#![allow]` unless it is the only option — for example, a deliberately large
function you will not split — and say why in the code when you do.

Use `--profile uat` (the LLVM profile), not the default `dev` profile
(cranelift). Cranelift cannot unwind a panic or assemble candle gemm-f16 FP16
SIMD code; a check or clippy run under `dev` can fail for a crate that pulls
candle.

## One Agent Per Feature and Per Crate

When you move to a new feature, or to a new crate, spawn a fresh agent to work
on it. Put all the context the agent needs in its brief; it does not inherit
yours.

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
9. **I/O & Async (Rust)**: Stream bodies, never buffer them (status first); use generated API types and fix the generator rather than working around it; async `fn`s all the way down, with valtron composing via futures/future streams under its existing Send/!Send/boundary rules — see `skills/rust-clean-code/implementation` §6–8
10. **Executors must run the tests**: a green `cargo check`/`cargo clippy` is not a pass. An executor runs `cargo test -p <crate> --profile uat` and reports the output before it calls the work done.
11. **Linux compatibility check**: after you commit a crate, spawn one agent to run `scripts/ci/linux.sh <crate>` in the Linux CI Docker image, to compile and run its tests on Linux.
12. **Version bump on new public API**: when a crate gains a new public API (a new module, function, type, or trait method), bump its version and move every workspace consumer's `version` requirement to match in the same change. Do not defer it to release time: a `path`+`version` dependency resolves to the published crate on `cargo package`, so a deferred bump lets a packaged consumer pick the stale registry version without the new API.

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
