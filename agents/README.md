# Agent Registry

This directory contains documentation for all specialized agents used in the system.

## Purpose

The Agent Registry serves as a centralized catalog of all agents, their capabilities, requirements, and boundaries. This enables the Main Agent to make informed decisions about which agents to spawn for specific tasks.

## Structure

Each agent is documented in its own file:
```
.agents/agents/
├── README.md                      # This file
├── rust-verification.md           # Rust verification agent
├── javascript-verification.md     # JavaScript/TypeScript verification agent
├── python-verification.md         # Python verification agent
├── specification-update.md        # Specification update agent
├── implementation.md              # General implementation agent
└── [name-of-agent].md            # Additional agents...
```

## Usage

### For Main Agent

**Before spawning any agent:**
1. Check this registry for appropriate agent
2. Read agent's frontmatter to understand purpose/capabilities
3. Read full agent documentation
4. Verify requirements are met
5. Spawn agent with documentation path and context

### For Sub-Agents

**Never spawn agents directly:**
1. If you need another agent, report to Main Agent
2. Main Agent will check registry and spawn appropriately
3. Read your own agent documentation (provided by Main Agent)
4. Stay within your documented boundaries

## Documentation Format

Every agent file must contain:
- **Frontmatter**: Quick summary (name, type, purpose, requirements)
- **Overview**: What the agent does
- **Capabilities**: What it can do
- **Requirements**: Tools, skills, dependencies
- **Responsibilities**: Specific duties
- **Workflow**: Step-by-step process
- **Boundaries**: What it cannot do
- **Integration**: How it works with other agents
- **Examples**: Real usage scenarios

## Creating New Agents

**Before creating a new agent:**
1. Check if existing agent can handle the task
2. If new agent needed, create documentation FIRST
3. Use template from Rule 10
4. Commit documentation before spawning agent

**Process:**
```bash
# 1. Create agent documentation
nano .agents/agents/[name-of-agent].md

# 2. Fill in all required sections

# 3. Commit documentation
git add .agents/agents/[name-of-agent].md
git commit -m "Add [Agent Name] documentation"

# 4. Now agent can be spawned with documentation path
```

## Mandatory Requirement

**From Rule 10: Agent Documentation and Registry**

❌ **NEVER spawn an undocumented agent**
❌ **NEVER create a new agent without documentation**
❌ **NEVER skip the registry check**
✅ **ALWAYS document before using**

**USER WILL SHOUT AT YOU** if you violate these rules!

## Quick Reference

### Finding an Agent

**Fast scan (frontmatter only):**
```bash
# Read frontmatter of all agent files
# Filter by type, language, or purpose keyword
# Select best match
```

**Full documentation:**
```bash
# Once agent identified, read complete documentation
# Verify requirements and boundaries
# Prepare context for spawning
```

### Agent Types

- **verification**: Code quality, testing, standards compliance
- **implementation**: Writing code, implementing features
- **review**: Pre-work verification, requirement checking
- **utility**: Helper tasks, data processing
- **specialized**: Domain-specific tasks (security, performance, etc.)

## See Also

- **Rule 10**: Agent Documentation and Registry - Main Agent creating agents
- **Rule 12**: Agent Registry Usage - Sub-agents being spawned (concise)
- **Rule 05**: Agent Orchestration - Spawning and coordination

---
*This directory is governed by Rule 10 (Main Agent) and Rule 12 (Sub-agents)*
