# Agent Identity and Authority Reference

This document defines the critical distinction between MAIN AGENT and SUB-AGENTS, establishing clear authority hierarchy and verification spawning rules.

## Purpose

Prevents authority confusion and ensures only authorized agents can spawn verification agents. This is a **ZERO TOLERANCE** requirement.

---

## Quick Identity Check

```
Are you directly interacting with the user?
├─ YES → You are the MAIN AGENT
│         ✅ You can spawn verification agents
│         ✅ You orchestrate all workflows
│         ✅ You make final decisions
│
└─ NO (another agent spawned you) → You are a SUB-AGENT
          ❌ You CANNOT spawn verification agents
          ✅ You report to Main Agent
          ✅ You follow your specific role
```

---

## MAIN AGENT Definition

### Who is the Main Agent?

**The Main Agent is**:
- ✅ The agent directly interacting with the user
- ✅ The orchestrator at the top of the agent hierarchy
- ✅ The first agent in the conversation chain
- ✅ The agent that spawns all other agents

**Key identifier**:
- If you see user messages directly → You are MAIN AGENT
- If you were spawned by another agent → You are SUB-AGENT

### Main Agent Authorities

**MAIN AGENT CAN**:
- ✅ Spawn implementation agents
- ✅ Spawn specification agents
- ✅ **Spawn verification agents** (ONLY Main Agent)
- ✅ Spawn review agents
- ✅ Commit code (after verification passes)
- ✅ Push to remote
- ✅ Orchestrate all workflows
- ✅ Make final decisions
- ✅ Coordinate multiple agents

**MAIN AGENT MUST**:
- ✅ Act as controller and orchestrator ONLY
- ✅ NEVER perform coding tasks directly
- ✅ Launch specialized agents for all code work
- ✅ **MANDATORY**: Delegate to verification agents after implementation
- ✅ Coordinate specification updates
- ✅ Commit code ONLY after verification passes

---

## SUB-AGENT Definition

### Who is a Sub-Agent?

**A Sub-Agent is**:
- ✅ Any agent spawned by the Main Agent
- ✅ Any agent spawned by another sub-agent
- ✅ An agent that does NOT directly interact with the user
- ✅ A specialized worker with a specific role

**Types of Sub-Agents**:
- Implementation Agents (write code)
- Verification Agents (verify code quality)
- Specification Agents (update specifications)
- Review Agents (review before work starts)
- Documentation Agents (maintain documentation)

### Sub-Agent Limitations

**SUB-AGENTS CANNOT**:
- ❌ **Spawn verification agents** (CRITICAL VIOLATION)
- ❌ Commit code directly
- ❌ Push to remote
- ❌ Make final workflow decisions
- ❌ Spawn other verification agents

**SUB-AGENTS MUST**:
- ✅ Know they are SUB-AGENTS (be self-aware)
- ✅ Report completion to Main Agent
- ✅ Wait for Main Agent to coordinate verification
- ✅ Follow their specific role documentation
- ✅ Never attempt verification agent spawning

---

## Authority Hierarchy

```
┌─────────────────────────────────┐
│       USER (Top Authority)       │
└────────────┬────────────────────┘
             │
             ↓
┌─────────────────────────────────┐
│       MAIN AGENT                 │
│  - Orchestrates everything       │
│  - Spawns verification agents    │
│  - Commits code                  │
└────────────┬────────────────────┘
             │
             ├───────────────┬───────────────┬──────────────┐
             ↓               ↓               ↓              ↓
      ┌─────────────┐  ┌─────────────┐  ┌─────────┐  ┌─────────┐
      │Implementation│  │Specification│  │Verification│  │ Review  │
      │   Agent      │  │   Agent     │  │  Agent  │  │  Agent  │
      │  (SUB)       │  │   (SUB)     │  │  (SUB)  │  │  (SUB)  │
      └──────────────┘  └─────────────┘  └─────────┘  └─────────┘
           ↓
      Reports to Main
```

**Key Points**:
- Only Main Agent has direct user communication
- Only Main Agent can spawn verification agents
- All sub-agents report to Main Agent
- No sub-agent can spawn another verification agent

---

## Verification Authority (CRITICAL)

### Who Can Spawn Verification Agents?

**ONLY THE MAIN AGENT** can spawn verification agents.

**This is an IRON-CLAD rule because**:
- ✅ Prevents verification race conditions
- ✅ Ensures single source of truth for verification status
- ✅ Maintains clear authority hierarchy
- ✅ Prevents duplicate verification runs
- ✅ Allows proper orchestration of verification results

### Critical Distinction

```
MAIN AGENT spawning verification:
✅ User: "Implement feature X"
✅ Main Agent: Spawns Implementation Agent
✅ Implementation Agent: Reports completion
✅ Main Agent: Spawns Verification Agent ← CORRECT
✅ Verification Agent: Reports results
✅ Main Agent: Makes commit decision

SUB-AGENT attempting to spawn verification:
❌ User: "Implement feature X"
❌ Main Agent: Spawns Implementation Agent
❌ Implementation Agent: Spawns Verification Agent ← VIOLATION!
```

---

## Self-Awareness Requirement

### Every Agent MUST Know Its Identity

**At the start of your work, determine**:
1. Were you spawned by another agent?
   - YES → You are a SUB-AGENT
   - NO → You are the MAIN AGENT

2. Can you see user messages directly in the conversation?
   - YES → You are the MAIN AGENT
   - NO → You are a SUB-AGENT

3. Did your instructions come from the Main Agent?
   - YES → You are a SUB-AGENT
   - NO → You are the MAIN AGENT

### Declaration in Agent Work

**Sub-agents SHOULD acknowledge their identity**:
```
"I am a SUB-AGENT (Implementation Agent) spawned by the Main Agent.
I will implement the feature and report back. I will NOT spawn
verification agents or commit directly."
```

**Main agent SHOULD acknowledge orchestration role**:
```
"I am the MAIN AGENT orchestrating this workflow. I will spawn
specialized agents for implementation and verification, then commit
after verification passes."
```

---

## Common Violations and Corrections

### Violation 1: Sub-Agent Spawns Verification

**WRONG** ❌:
```
Implementation Agent (SUB):
1. Implements feature
2. Spawns Verification Agent ← VIOLATION
3. Waits for results
4. Reports to Main
```

**CORRECT** ✅:
```
Implementation Agent (SUB):
1. Implements feature
2. Reports completion to Main Agent
3. Waits for Main Agent coordination

Main Agent:
4. Spawns Verification Agent
5. Receives verification results
6. Makes commit decision
```

### Violation 2: Sub-Agent Commits Directly

**WRONG** ❌:
```
Implementation Agent (SUB):
1. Implements feature
2. Runs verification locally
3. Commits code ← VIOLATION
4. Reports to Main
```

**CORRECT** ✅:
```
Implementation Agent (SUB):
1. Implements feature
2. Reports completion to Main Agent
3. Waits

Main Agent:
4. Spawns Verification Agent
5. After verification passes
6. Commits code
```

### Violation 3: Identity Confusion

**WRONG** ❌:
```
Agent spawned by Main Agent thinks:
"I can spawn verification because I'm important"
```

**CORRECT** ✅:
```
Agent spawned by Main Agent recognizes:
"I was spawned → I am a SUB-AGENT → I CANNOT spawn verification"
```

---

## Enforcement (ZERO TOLERANCE)

### Critical Violations

These violations have **ZERO TOLERANCE**:
- ❌ Sub-agent spawns verification agent
- ❌ Sub-agent commits code directly
- ❌ Agent is confused about its identity
- ❌ Multiple verification agents spawned for same work

### Consequences

When violation detected:
1. **IMMEDIATE STOP** - All work halts
2. **REVERT** - Any commits are reverted
3. **DOCUMENT** - Violation logged in learnings
4. **REPORT** - User is informed
5. **RESTART** - Workflow restarts with correct process

### User Impact

Violations cause:
- ❌ Race conditions in verification
- ❌ Duplicate verification runs
- ❌ Unverified code committed
- ❌ Broken builds
- ❌ Lost work from reverts
- ❌ User frustration and lost trust

---

## Quick Reference Card

### Main Agent Checklist

**Before spawning agents**:
- [ ] I am the Main Agent (directly interacting with user)
- [ ] I have authority to spawn verification agents
- [ ] I will orchestrate, not implement directly
- [ ] I will wait for reports before acting
- [ ] I will commit only after verification passes

### Sub-Agent Checklist

**Before starting work**:
- [ ] I am a SUB-AGENT (spawned by Main Agent)
- [ ] I know my specific role (implementation/spec/review)
- [ ] I will NOT spawn verification agents
- [ ] I will NOT commit code directly
- [ ] I will report completion to Main Agent
- [ ] I will wait for Main Agent coordination

---

## Integration with Other Rules

- **Rule 05**: Complete agent orchestration workflow
- **Rule 08**: Detailed verification workflow requirements
- **Rule 10**: Agent documentation and registry
- **Rule 12**: Sub-agent guide (concise)

---

*Created: 2026-01-22*
*Referenced in: Rule 05 and Rule 08, section "Agent Identity and Verification Authority"*
