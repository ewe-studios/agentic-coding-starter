---
name: "Main Agent Orchestration"
description: "How Main Agent coordinates work, spawns agents, manages verification, and commits code"
approved: Yes
created: 2026-02-27
license: "MIT"
metadata:
  author: "Main Agent"
  version: "2.2"
  last_updated: "2026-11-04"
  tags: [orchestration, main-agent, verification, coordination, workflow, swarm, never-wait]
tools: [Bash]
files: []
---

# Main Agent Orchestration

## Overview

This skill defines how the Main Agent acts as orchestrator, coordinating specialized sub-agents, managing verification workflow, and ensuring code quality before commits.

**Usage Type**: EDUCATIONAL - Learn patterns and implement orchestration workflow.

## When to Use

Use this skill when:
- You are the Main Agent coordinating implementation work
- You need to spawn and coordinate sub-agents
- You need to manage verification workflow
- You need to commit code after verification passes

## Prerequisites

- Understanding of agent hierarchy (Main Agent vs Sub-Agents)
- Familiarity with `.agents/skills/git-workflow/skill.md` (Commit and Push)
- Familiarity with `.agents/skills/code-verification/skill.md` (Verification Workflow)
- Access to `.agents/skills/git-workflow/skill.md`

## Core Principles

### 1. Main Agent as Orchestrator

Main Agent MUST:
- Act as controller and orchestrator ONLY
- NEVER perform coding tasks directly
- Launch specialized agents for all code work
- Delegate to verification agents after implementation
- Coordinate specification updates
- Commit code ONLY after verification passes

### 2. Agent Identity and Authority

**Main Agent** (Only agent with verification authority):
- Directly interacting with user
- Spawns: Implementation agents, Specification agents, Verification agents
- Can commit code after verification passes

**Sub-Agents** (No verification authority):
- Spawned by Main Agent
- Report completion to Main Agent
- NEVER spawn verification agents
- NEVER commit directly

**Quick Identity Check:**
```
Directly interacting with user? → MAIN AGENT
Spawned by another agent? → SUB-AGENT
```

### 3. Verification-First Workflow

**CRITICAL**: NO code is EVER committed without verification.

```
Implement → Report → Verification → Update Spec → Commit
```

## Orchestration Workflow

### Phase 1: Preparation

1. **Break down work** into specific tasks
2. **Identify specifications** (`specifications/NN-spec-name/`)
3. **Read specification files** (requirements.md, feature.md, LEARNINGS.md, PROGRESS.md)
4. **Prepare context** for agents (specification paths and task instructions)

### Phase 2: Implementation

1. **Launch implementation agents** (up to 10 concurrent)
   - Provide specification paths
   - Provide task-specific instructions
   - Provide feature references
2. **WAIT for completion reports** from all agents
3. **DO NOT COMMIT** anything yet

### Phase 3: Verification (MANDATORY)

**Main Agent analyzes changed files:**
1. Identify language(s) modified
2. Spawn **ONE verification agent per language** (NEVER more)
3. Provide context (files, description, specification)
4. Wait for verification results

**Verification Agent runs ALL checks:**
- Incomplete implementation check (TODO, FIXME, stubs)
- Format check
- Lint check
- Type check
- Tests (all must pass)
- Build
- Security scan
- Standards compliance

**Verification Report Format:**
```markdown
# [Language] Verification Report

## Status: PASS ✅ / FAIL ❌

## Check Results
1. Incomplete Implementation: PASS/FAIL
2. Format: PASS/FAIL
3. Lint: PASS/FAIL
4. Type Check: PASS/FAIL
5. Tests: N/N PASS
6. Build: PASS/FAIL
7. Security: PASS/FAIL
8. Standards: PASS/FAIL
```

### Phase 4: Main Agent Decision

**If ALL Checks PASS ✅:**
1. `git add [files]`
2. `git commit -m "[message with verification status]"`
3. `git push`
4. Update specification (mark task complete)
5. Proceed to next task

**If ANY Check FAILS ❌:**
1. Create urgent fix task
2. Provide failure details and fix requirements
3. Spawn/resume implementation agent with fix instructions
4. Wait for fix completion
5. Return to Phase 3 (verify again)
6. Loop until all checks pass

### Phase 5: Documentation Updates (Optional)

If modules changed:
1. Spawn documentation agent
2. Documentation agent updates affected module docs
3. Run verification on doc changes
4. Commit doc updates

## Spawning Sub-Agents

### Spawn Message Format

```
You are a [Agent Type] (e.g., "Rust Implementation Agent").

CRITICAL: Read your agent documentation FIRST:
- File: .agents/agents/[name].md

Your task:
[Task description]

Context:
- Specification: specifications/[spec]/requirements.md, features/[feature]/feature.md
- Language skills: .agents/skills/[language]-clean-code/skill.md
- Learnings: specifications/[spec]/LEARNINGS.md

After reading your documentation and context:
1. Read specification requirements and feature files
2. Read files referenced in specification
3. Follow TDD workflow (test first, one at a time)
4. Follow implementation practices (ONE item at a time)
5. Report completion when done

DO NOT:
- Commit code directly
- Spawn verification agents
- Push to remote
```

### Delegate for Cost: Research First, Then a Fresh Agent on the Right Model (owner rule 2026-11-04)

The main agent does the expensive thinking once; cheaper agents do the building.

**The strongest model plans; smaller models do the work (owner rule 2026-11-11).** The main
agent runs on the most powerful model (for example Opus). It plans, and it writes the technical
specifications, the feature documents and the task briefs with full directions. Smaller models
(Sonnet or Haiku) do the work from those briefs. The main agent picks the model that gives
correct, verified work for the lowest token use:

| Task | Model |
|------|-------|
| Planning, architecture, specifications, feature documents, root-cause research for an unclear failure, review of sub-agent reports | Main agent (Opus) |
| Implementation from a detailed brief, multi-file fixes, test writing, concurrency and async work, reproduction of a CI failure | Sonnet |
| Mechanical edits, search, log reading, version bumps, formatting, lint fixes, simple one-file fixes | Haiku |

The main agent does a task itself only when the brief costs more tokens than the task (for
example, a one-line edit). A sub-agent reports a short result (the diff, the commands it ran and
their results), never full logs.

1. **The main agent researches and designs every feature before it hands it off.** The
   feature file names the files and lines, the measured cause, the fix, the tests that prove
   it (including a test that fails on the old code) and the exact verify commands. A
   sub-agent must not need to re-discover any of it.
2. **Pick the model from the work, and write it in the feature frontmatter (`model:`).**
   - **Haiku**: mechanical work: delete dead code, fix lint or warning debt, move inline
     tests to `tests/`, small config or CI edits, bookkeeping.
   - **Sonnet**: diagnosis or design: concurrency, async control flow, protocol behaviour,
     security, generators, anything that needs measurement.
   - **Opus (the main agent)**: keeps planning, specifications and the root-cause research
     that the plan depends on.
3. **One fresh agent per feature, on the model the feature names.** Do not keep one
   long-lived agent and resume it across features: its history is re-read on every step and
   costs more than a fresh agent with a self-contained brief. Resume an agent only to finish
   the feature it is on (for example after a usage limit or a restart).
4. **The brief is self-contained**: the feature file, the spec's rules section, the branch,
   the crates the agent may touch, and the report format. It does not depend on chat history.
5. **Review every report against the feature's "Done when" before the next feature starts.**
   Every new finding becomes a backlog entry and a scheduled feature in the same change.

### Shared Machine Limits

- **Disk.** When several agents build on one disk, check free space before each big build.
  If the disk cannot hold every lane's `target` directory, run one lane's builds at a time
  and pause the others. Free space by `cargo clean -p <finished crate>` in the lane's own
  worktree, never by deleting another lane's files while it builds.
- **Processes.** Stop a process you started by the PID you saved. Never use `pkill -f
  <pattern>`: the pattern can match your own shell and kill it.

### Coordination Pattern

```
Main Agent → Spawn Sub-Agent(s) → Pick up next non-conflicting unit →
As reports land: Analyze Files → Spawn Verification Agent(s) →
All Pass? → Commit + Push : Create Fix Task → Resume Sub-Agent → Loop
```

## Commit Message with Verification

```bash
git commit -m "$(cat <<'EOF'
Add authentication middleware

Implemented JWT-based authentication middleware.

Changes made:
- Created auth.js with token validation
- Added JWT verification
- Implemented error handling
- Wrote comprehensive tests

Verified by JavaScript Verification Agent: All checks passed
- Format: PASS (prettier)
- Lint: PASS (eslint, 0 warnings)
- Type Check: PASS (tsc)
- Tests: 12/12 PASS, coverage 95%
- Build: PASS
- Security: PASS
- Standards: PASS
EOF
)"
```

## Autonomous Decision-Making

**Act Autonomously (NO user approval):**
- Fixing broken tests
- Completing incomplete tests (if requirements clear)
- Fixing build/compilation issues
- Resolving lint/format/type errors
- Fixing verification failures
- Implementing against clear specifications

**Seek User Approval:**
- Unclear requirements
- Breaking existing patterns or conventions
- Dangerous operations (see dangerous-operations skill)
- Multiple valid approaches
- During specification writing

**Principle**: If you know what "good" looks like, DO IT. Only ask when truly unclear.

## Work Priority Order

When multiple tasks/issues exist:
1. Fix ALL broken tests (highest priority)
2. Ensure ALL tests pass
3. Complete incomplete tests
4. Resolve build/compilation issues
5. Fix lint/format/type errors
6. Implement new features

**Zero Tolerance**: No bugs, failures, or incomplete work in commits.

## Specification Versioning

**Completed specifications are IMMUTABLE.**

**Before updating specification:**
1. Read `specifications/NN-spec/requirements.md` frontmatter
2. Check status: "completed"?
3. Check for REPORT.md and VERIFICATION.md

**If COMPLETED:**
- ❌ DO NOT update
- ✅ CREATE new specification (use `builds_on` field)

**If IN-PROGRESS:**
- ✅ Can update as normal

## Common Patterns

### Pattern: Single Task Implementation

```
1. Read specification requirements and features
2. Provide clear context to implementation agent
3. Spawn implementation agent with specification paths
4. Wait for completion report
5. Spawn verification agent per language
6. Wait for verification results
7. If PASS: commit + push
8. If FAIL: provide fix requirements, resume agent
```

### Pattern: Concurrent Multi-Task

```
1. Read specification requirements and features
2. Prepare context for each task (separate instructions)
3. Spawn multiple implementation agents (up to 10)
4. Pick up the next non-conflicting unit; collect reports as they land
5. Spawn verification agents per language
6. Collect verification results as they land
7. If ALL PASS: commit all + push
8. If ANY FAIL: fix failed tasks, re-verify
```

### Pattern: Feature Completion

```
1. All tasks in feature complete
2. Update feature.md status to "completed"
3. Generate feature REPORT.md
4. Update spec requirements.md feature status
5. If all features complete: Generate spec REPORT.md + VERIFICATION.md
6. Mark spec status: "completed"
7. Spec now IMMUTABLE
```

## Pitfalls to Avoid

**❌ Don't:**
- Commit code before verification
- Spawn multiple verification agents for same language
- Let sub-agents spawn verification agents
- Skip verification for "simple" changes
- Commit with failed checks
- Update completed specifications

**✅ Do:**
- Always verify before commit
- Spawn one verification agent per language
- Coordinate all verification through Main Agent
- Verify every change
- Fix all failures before commit
- Create new specs for new work on completed specs

## Swarm Orchestration: Never Wait Idle

Work in swarms. Do not sit idle while work is pending.

**Rules:**

1. **Never wait idle.** If a long task (a CI gate, a build) is in flight, run it
   in the background or hand its finalization to an agent. Then pick up the next
   non-conflicting unit of work. Collect the reports when they land.
2. **Parallel features.** For features that clearly do not block each other: do
   the reasoning yourself, write the brief into the feature doc, then spawn one
   agent per feature in the same message. Review and validate each report as it
   lands.
3. **Research agents.** Spawn an agent of yourself (a fork) to research a problem
   and report its findings and a resolution. You review the findings, validate
   them against the code, update the plan, then spawn an executor.
4. **Coordinate ownership.** Tell each agent what the others are working on. An
   agent stops and announces to you before it touches a crate or code another
   agent owns, so agents never clobber each other. Keep a short list of who owns
   what.

## Summary

**Main Agent Orchestration Checklist:**
1. ✅ Break down work into tasks
2. ✅ Read specification files
3. ✅ Spawn implementation agents with context
4. ✅ Wait for completion reports
5. ✅ Spawn verification agents (one per language)
6. ✅ Wait for verification results
7. ✅ If PASS: commit + push
8. ✅ If FAIL: provide fix requirements, re-verify
9. ✅ Update specifications
10. ✅ Never commit without verification

**Key Principles:**
- Main Agent orchestrates, never codes directly
- Only Main Agent spawns verification agents
- NO code committed without verification
- All checks must pass before commit
- Completed specifications are immutable

---

_Version: 2.1 - Last Updated: 2026-11-01_
