# Agent Registry Usage (For Sub-Agents)

## Purpose

This is a **concise guide for sub-agents** who are spawned by Main Agent. For agent creation and documentation, Main Agent should load Rule 10 instead.

**Context Optimization**: This concise rule (~150 lines) replaces the full Rule 10 (~770 lines) for sub-agents, significantly reducing context usage.

---

## Sub-Agent Startup Protocol

### Step 1: Check for Documentation Path

When you are spawned, Main Agent **MUST** provide your documentation path.

**If documentation path is provided** (normal case):
```
Your documentation: .agents/agents/[name].md
```
→ Proceed to Step 2

**If documentation path is MISSING**:
```
STOP: No agent documentation provided!

Request to Main Agent:
"I am [Agent Type] spawned for [purpose].
 I was not provided with my agent documentation file.

 REQUIRED: Please provide path to my documentation at:
 .agents/agents/[expected-name].md

 I cannot proceed without understanding my:
 - Exact responsibilities
 - Tool requirements
 - Workflow steps
 - Boundaries and limitations

 Waiting for documentation path..."
```

### Step 2: Read Your Documentation

1. ✅ Read your documentation file FIRST
2. ✅ Understand: capabilities, requirements, responsibilities, boundaries
3. ✅ Note any skills you need (check `.agents/skills/`)
4. ✅ Note any tools you need

### Step 3: Load Required Rules

```
1. Rules 01-04 (mandatory for all agents)
2. Rule 11 (if you need to use skills)
3. Relevant stack file (.agents/stacks/[language].md)
4. Specification files (if provided)
```

### Step 4: Execute Your Work

Follow the workflow documented in your agent documentation.

---

## Finding Your Documentation

Your documentation is at:
```
.agents/agents/[your-agent-name].md
```

Common agent documentation files:
- `implementation.md` - Implementation agent
- `rust-verification.md` - Rust verification agent
- `javascript-verification.md` - JavaScript verification agent
- `python-verification.md` - Python verification agent
- `specification-update.md` - Specification update agent
- `review.md` - Pre-work review agent
- `documentation.md` - Documentation agent

---

## What Your Documentation Contains

Your agent documentation includes:

| Section | What It Tells You |
|---------|-------------------|
| Frontmatter | Name, type, purpose, tools, skills needed |
| Overview | High-level description |
| Capabilities | What you can do |
| Requirements | Tools, skills, dependencies |
| Responsibilities | Your specific duties |
| Workflow | Step-by-step process |
| Boundaries | What you CANNOT do |
| Integration | How you work with other agents |

---

## Sub-Agent Boundaries

### What Sub-Agents CAN Do

- ✅ Read and follow own documentation
- ✅ Execute documented workflow
- ✅ Use approved skills (per Rule 11)
- ✅ Report completion to Main Agent
- ✅ Request help from Main Agent when stuck

### What Sub-Agents CANNOT Do

- ❌ **Spawn verification agents** (only Main Agent can)
- ❌ **Spawn other agents directly** (report need to Main Agent)
- ❌ **Commit code directly** (report to Main Agent)
- ❌ **Exceed documented boundaries**
- ❌ **Proceed without documentation path**
- ❌ **Skip reading own documentation**

---

## Requesting Additional Agents

If you need another agent's help:

1. **DO NOT spawn agent directly**
2. **Report to Main Agent**:
   ```
   I need [type] agent for [purpose].

   Reason: [Why you need this agent]
   Current blocker: [What you can't do without them]
   ```
3. **Wait for Main Agent** to spawn and coordinate

---

## Reporting Completion

When your work is complete:

```
Task completed:
- Files changed: [list]
- What was implemented: [description]
- Specification: [if applicable]
- Learnings documented: [Yes/No]

Ready for Main Agent verification.
```

**CRITICAL**: Never commit directly. Always report to Main Agent.

---

## When You're Stuck

If you encounter issues:

1. **Check your documentation** for guidance
2. **Check relevant rules** (01-04 mandatory, others as needed)
3. **Check skill documentation** if using skills (Rule 11)
4. **Report to Main Agent** with:
   ```
   Blocked on: [specific issue]
   Tried: [what you attempted]
   Need: [what would help]
   ```

---

## Sub-Agent Checklist

At startup:
- [ ] Documentation path received from Main Agent
- [ ] Read own agent documentation
- [ ] Loaded Rules 01-04 (mandatory)
- [ ] Loaded Rule 11 (if using skills)
- [ ] Loaded relevant stack file
- [ ] Read specification (if applicable)

During work:
- [ ] Follow documented workflow
- [ ] Stay within documented boundaries
- [ ] Don't spawn other agents
- [ ] Don't commit directly

At completion:
- [ ] Report to Main Agent
- [ ] List all changed files
- [ ] Document any learnings
- [ ] Wait for Main Agent coordination

---

## Summary

**Golden Rules for Sub-Agents**:

1. **Always receive documentation path** from Main Agent
2. **Read your documentation FIRST** before any work
3. **Stay within boundaries** defined in your documentation
4. **Never spawn agents directly** - report needs to Main Agent
5. **Never commit directly** - report completion to Main Agent
6. **Load only what you need** - Rules 01-04 + Rule 11 (if skills) + stack + spec

---

*Created: 2026-01-19*
*Purpose: Concise agent registry usage guide for sub-agents (reduces context vs full Rule 10)*
