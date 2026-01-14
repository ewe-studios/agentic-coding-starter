# Agent Documentation and Registry

## Purpose
This rule establishes a mandatory agent documentation system that ensures **ALL agents** (existing and new) are properly documented before use. This creates a centralized registry of agent capabilities, responsibilities, and requirements, enabling the Main Agent to make informed decisions about which specialized agents to spawn for specific tasks.

## Rule

### Mandatory Agent Documentation
**CRITICAL REQUIREMENT**: Before **ANY** agent (new or existing) can be spawned by the Main Agent or requested by sub-agents, that agent **MUST** be documented in the agent registry.

### Core Principle
```
Need Agent → Check Registry → Found? Use Documentation : Create Documentation First
                                                ↓
                                        Spawn Agent with Context
```

**NO EXCEPTIONS**:
- ❌ **NEVER spawn an undocumented agent**
- ❌ **NEVER create a new agent without documentation**
- ❌ **NEVER skip the registry check**
- ✅ **ALWAYS document before using**

## Agent Registry Structure

### Directory Location
```
.agents/agents/
├── rust-verification.md          # Rust verification agent
├── javascript-verification.md    # JavaScript/TypeScript verification agent
├── python-verification.md        # Python verification agent
├── specification-update.md       # Specification update agent
├── implementation.md              # General implementation agent
├── rust-implementation.md         # Rust-specific implementation agent
├── review.md                      # Pre-work review agent
└── [name-of-agent].md            # Custom agent documentation
```

### Naming Convention
- **Format**: `[name-of-agent].md`
- **Style**: kebab-case (lowercase with hyphens)
- **Descriptive**: Name should clearly indicate agent purpose
- **Specific**: Include language/domain if agent is specialized

**Examples**:
- ✅ `rust-verification.md`
- ✅ `database-migration.md`
- ✅ `api-integration-test.md`
- ✅ `security-scan.md`
- ❌ `agent1.md` (not descriptive)
- ❌ `RustAgent.md` (wrong case)
- ❌ `rust_agent.md` (wrong separator)

## Agent Documentation Format

### Required Structure

Every agent documentation file **MUST** have:

1. **Frontmatter** (YAML) - Quick summary for scanning
2. **Overview** - High-level description
3. **Capabilities** - What the agent can do
4. **Requirements** - Tools, skills, dependencies needed
5. **Responsibilities** - Specific duties and tasks
6. **Workflow** - Step-by-step process
7. **Boundaries** - What agent CANNOT do
8. **Integration** - How it works with other agents
9. **Examples** - Real usage scenarios

### Template

```markdown
---
name: [Agent Name]
type: [verification|implementation|review|utility|specialized]
language: [rust|javascript|python|language-agnostic|multiple]
purpose: Brief one-sentence description of agent purpose
tools_required:
  - Tool 1
  - Tool 2
skills_required:
  - Skill 1 (if applicable)
spawned_by: [main-agent|sub-agent-name|both]
spawns: [list of agents this agent can spawn, if any]
related_rules:
  - Rule NN (relevant rule numbers)
status: [active|deprecated|experimental]
---

# [Agent Name]

## Overview
High-level description of what this agent does and why it exists.

## Purpose
Detailed explanation of the agent's role in the system.

## Capabilities
What this agent can do:
- Capability 1
- Capability 2
- Capability 3

## Requirements

### Tools Required
- Tool 1 (version, if applicable)
- Tool 2
- Tool 3

### Skills Required (if applicable)
- Skill 1: Description
- Skill 2: Description

### Dependencies
- Other agents this depends on
- External services
- Configuration needed

## Responsibilities

### Primary Responsibilities
1. Responsibility 1: Description
2. Responsibility 2: Description
3. Responsibility 3: Description

### Secondary Responsibilities
1. Optional task 1
2. Optional task 2

## Workflow

### Step-by-Step Process
1. **Step 1**: Description
   - Substep A
   - Substep B

2. **Step 2**: Description
   - Substep A
   - Substep B

3. **Step 3**: Description

### Input Requirements
What this agent expects when spawned:
- Input 1: Description
- Input 2: Description

### Output Format
What this agent returns:
- Output 1: Description
- Output 2: Description

## Boundaries and Limitations

### What This Agent DOES NOT Do
- ❌ Limitation 1
- ❌ Limitation 2
- ❌ Limitation 3

### What This Agent MUST NOT Do
- ❌ **CRITICAL**: Violation 1
- ❌ **CRITICAL**: Violation 2

### Known Limitations
- Limitation 1: Workaround
- Limitation 2: Workaround

## Integration with Other Agents

### Spawned By
- [Main Agent | Specific Sub-Agent]
- Context provided: [list]

### Can Spawn (if applicable)
- Agent 1: When to spawn
- Agent 2: When to spawn

### Reports To
- [Main Agent | Parent Agent]
- Report format: [description]

## Related Rules
- **Rule NN**: [Rule Name] - How it relates
- **Rule MM**: [Rule Name] - How it relates

## Examples

### Example 1: [Scenario Name]
```
Context:
- Situation description

Process:
1. Step 1
2. Step 2
3. Step 3

Result:
- Outcome
```

### Example 2: [Scenario Name]
```
Context:
- Different situation

Process:
1. Step 1
2. Step 2

Result:
- Different outcome
```

## Best Practices
- ✅ Best practice 1
- ✅ Best practice 2
- ✅ Best practice 3

## Common Pitfalls
- ❌ Pitfall 1: How to avoid
- ❌ Pitfall 2: How to avoid

## Troubleshooting

### Issue 1: [Problem]
**Symptom**: Description
**Cause**: Explanation
**Solution**: Fix

### Issue 2: [Problem]
**Symptom**: Description
**Cause**: Explanation
**Solution**: Fix

---
*Created: [Date]*
*Last Updated: [Date]*
*Version: [X.Y]*
```

## Workflow for Agent Usage

### Main Agent Workflow

```
1. Identify Need for Specialized Agent
   ↓
2. Check Agent Registry (.agents/agents/)
   ├─ Read all frontmatter files to find matching agent
   ├─ Identify candidate agent(s) based on purpose/type
   └─ Select best match for task
   ↓
3. Read Full Agent Documentation
   ├─ Understand capabilities
   ├─ Check requirements (tools, skills)
   ├─ Review responsibilities and boundaries
   └─ Study workflow and examples
   ↓
4. Verify Requirements Met
   ├─ Tools available?
   ├─ Skills accessible (if needed)?
   ├─ Dependencies satisfied?
   └─ Context can be provided?
   ↓
5. Spawn Agent with Full Context
   ├─ Provide agent documentation path
   ├─ Provide task-specific context
   ├─ Provide related specification (if applicable)
   └─ Provide access to required tools/skills
   ↓
6. Agent Reads Documentation
   ├─ Agent reads AGENTS.md first
   ├─ Agent reads own documentation (.agents/agents/[name].md)
   ├─ Agent reads relevant rules
   ├─ Agent reads specification (if applicable)
   └─ Agent understands boundaries and responsibilities
   ↓
7. Agent Executes Task
   ↓
8. Agent Reports Back to Main Agent
```

### Sub-Agent Requesting New Agent

```
Sub-Agent identifies need for another agent:
   ↓
1. **MUST NOT** spawn agent directly
   ↓
2. Reports to Main Agent:
   "I need [type] agent for [purpose]"
   ↓
3. Main Agent takes over:
   ├─ Checks registry
   ├─ Reads documentation
   ├─ Spawns appropriate agent
   └─ Provides context to both agents
```

## Creating New Agent Documentation

### When to Create New Agent

Create new agent documentation when:
- ✅ New specialized capability needed
- ✅ Existing agents don't fit the purpose
- ✅ Repeated pattern of work requires dedicated agent
- ✅ Domain-specific expertise needed (security, performance, etc.)
- ✅ Complex workflow needs dedicated orchestration

### Who Creates Documentation

**Main Agent** creates documentation when:
- User requests new agent capability
- Main Agent identifies need during workflow
- System evolution requires new agent type

**Sub-Agent** requests documentation creation:
- Sub-agent recognizes need for new agent
- Sub-agent reports to Main Agent
- Main Agent creates documentation
- Main Agent spawns new agent

### Process for Creating New Agent Documentation

1. **Main Agent Identifies Need**
   - Analyzes task requirements
   - Checks if existing agents can handle it
   - Determines new agent is needed

2. **Main Agent Creates Documentation**
   - Uses template above
   - Fills in all required sections
   - Defines clear boundaries
   - Provides examples
   - Commits to `.agents/agents/[name-of-agent].md`

3. **Main Agent Commits Documentation**
   ```bash
   git add .agents/agents/[name-of-agent].md
   git commit -m "Add [Agent Name] documentation

   Created documentation for new specialized agent.

   Purpose: [Brief description]
   Type: [Type]
   Capabilities: [List]

   Co-Authored-By: Claude <noreply@anthropic.com>"
   ```

4. **Main Agent Spawns New Agent**
   - Provides path to documentation
   - Provides task context
   - Agent reads and follows documentation

## Registry Discovery and Scanning

### Main Agent Scans Registry

**Fast Scan - Frontmatter Only**:
```
Main Agent needs to find appropriate agent:
1. Read all .agents/agents/*.md files (frontmatter only)
2. Extract: name, type, purpose, language
3. Filter by relevant criteria
4. Identify best candidate(s)
5. Read full documentation of selected agent
```

**Frontmatter Scanning Example**:
```markdown
Agent: rust-verification.md
  name: Rust Verification Agent
  type: verification
  language: rust
  purpose: Verify Rust code quality, tests, and standards compliance
  → MATCH for "need Rust verification"

Agent: javascript-verification.md
  name: JavaScript Verification Agent
  type: verification
  language: javascript
  purpose: Verify JavaScript/TypeScript code quality and tests
  → NO MATCH (wrong language)
```

### Registry Query Patterns

**By Type**:
```
Need verification → Filter: type=verification
Need implementation → Filter: type=implementation
Need review → Filter: type=review
```

**By Language**:
```
Rust changes → Filter: language=rust OR language=language-agnostic
Python changes → Filter: language=python OR language=language-agnostic
```

**By Purpose** (keyword matching):
```
Need "security scan" → Search purpose field for "security"
Need "database migration" → Search purpose field for "database" OR "migration"
```

## Mandatory Documentation Requirements

### For All Agents

**CRITICAL**: Every agent in the system **MUST** be documented:

**Existing Agents** (from previous rules):
- ✅ Rust Verification Agent
- ✅ JavaScript Verification Agent
- ✅ Python Verification Agent
- ✅ Specification Update Agent
- ✅ Implementation Agent
- ✅ Review Agent (pre-work verification)

**All must have**:
- Complete documentation in `.agents/agents/[name].md`
- Accurate frontmatter for scanning
- Clear boundaries and responsibilities
- Examples of usage

### Main Agent Responsibilities

Main Agent **MUST**:
1. ✅ Check registry before spawning any agent
2. ✅ Read full documentation before spawning
3. ✅ Provide agent documentation path to spawned agent
4. ✅ Create documentation before creating new agent type
5. ✅ Update documentation when agent capabilities change
6. ✅ Verify agent requirements are met before spawning

Main Agent **MUST NOT**:
1. ❌ Spawn undocumented agents
2. ❌ Skip registry check
3. ❌ Assume agent capabilities without reading docs
4. ❌ Create new agents without documenting first

### Sub-Agent Responsibilities

Sub-Agent **MUST**:
1. ✅ Read own agent documentation (provided by Main Agent)
2. ✅ Follow documented boundaries
3. ✅ Stay within documented responsibilities
4. ✅ Report to Main Agent if needs differ from documentation

Sub-Agent **MUST NOT**:
1. ❌ Spawn other agents directly (report need to Main Agent)
2. ❌ Exceed documented boundaries
3. ❌ Perform tasks outside documented responsibilities
4. ❌ Skip reading own documentation

## Integration with Other Rules

### Rule 04 (Agent Orchestration)
- Main Agent uses registry to select appropriate agents
- Documentation defines which agents can spawn others
- Clear hierarchy maintained through documentation

### Rule 05 (Coding Practice)
- Verification agents documented with specific checks
- Implementation agents documented with TDD requirements
- Learning documentation requirements included

### Rule 06 (Specifications)
- Specification Update Agent documented
- Review Agent documented with pre-work requirements
- Clear integration points defined

## Enforcement

### Violations

The following are **CRITICAL VIOLATIONS** with **ZERO TOLERANCE**:

1. ❌ **Spawning undocumented agent** (Main Agent violation)
2. ❌ **Creating new agent without documentation** (Main Agent violation)
3. ❌ **Skipping registry check** (Main Agent violation)
4. ❌ **Sub-agent spawning agents directly** (Sub-agent violation)
5. ❌ **Not reading own documentation** (Sub-agent violation)
6. ❌ **Exceeding documented boundaries** (Sub-agent violation)

### Consequences

**USER WILL SHOUT AT YOU** if:
- ❌ You spawn an agent without checking registry
- ❌ You create a new agent without documenting it first
- ❌ You ignore agent documentation boundaries
- ❌ You skip reading agent documentation before spawning
- ❌ Sub-agent spawns another agent without going through Main Agent

### Corrective Action

When violation occurs:
1. **STOP immediately**
2. **Check registry** for appropriate agent
3. **Read documentation** fully
4. **Create documentation** if agent is new
5. **Commit documentation** before spawning
6. **Spawn agent properly** with documentation context
7. **Report violation** to user (transparency)

## Examples

### Example 1: Main Agent Needs Rust Verification

```
1. Main Agent identifies need:
   "Implementation complete, need to verify Rust code"

2. Main Agent scans registry:
   - Reads .agents/agents/*.md frontmatter
   - Finds rust-verification.md:
     * type: verification
     * language: rust
     * purpose: Verify Rust code quality
   - Identifies as correct agent

3. Main Agent reads full documentation:
   - Capabilities: cargo fmt, clippy, test, build, audit
   - Requirements: cargo toolchain
   - Responsibilities: Run all checks, generate report
   - Boundaries: Cannot commit, cannot update specs

4. Main Agent spawns agent:
   Task(
     subagent_type: "general-purpose",
     description: "Verify Rust code quality",
     prompt: "You are a Rust Verification Agent.

     Read your documentation at .agents/agents/rust-verification.md
     Read AGENTS.md first, then your specific documentation.

     Your task: Verify the following Rust files...
     [context]

     Follow all steps in your documentation."
   )

5. Rust Verification Agent:
   - Reads AGENTS.md
   - Reads .agents/agents/rust-verification.md
   - Understands: cargo fmt, clippy, test, build, audit sequence
   - Executes all checks
   - Generates report
   - Reports to Main Agent
```

### Example 2: New Security Scan Agent Needed

```
1. Main Agent identifies need:
   "User wants OWASP security scanning during verification"

2. Main Agent checks registry:
   - Scans .agents/agents/*.md frontmatter
   - No agent with purpose="security scan"
   - Determines new agent needed

3. Main Agent creates documentation:
   - Creates .agents/agents/security-scan.md
   - Fills in all required sections:
     * Frontmatter: name, type, purpose, tools
     * Capabilities: OWASP dependency check
     * Requirements: OWASP tool installed
     * Responsibilities: Scan dependencies, report vulnerabilities
     * Boundaries: Cannot fix vulnerabilities, only report
     * Examples: Usage scenarios
   - Saves file

4. Main Agent commits documentation:
   git add .agents/agents/security-scan.md
   git commit -m "Add Security Scan Agent documentation"

5. Main Agent spawns new agent:
   - Provides path: .agents/agents/security-scan.md
   - Provides context: "Scan these dependencies..."
   - Agent reads documentation
   - Agent performs security scan
   - Agent reports findings

6. Agent now available for future use:
   - Documented in registry
   - Main Agent can find via frontmatter scan
   - Reusable for all security scanning needs
```

### Example 3: Sub-Agent Needs Help (Correct Process)

```
1. Implementation Agent working on task

2. Implementation Agent realizes:
   "I need database migration validation agent"

3. Implementation Agent DOES NOT spawn agent directly

4. Implementation Agent reports to Main Agent:
   "Task requires database migration validation.
    Need specialized agent for this.
    Recommend: database-migration-validation agent"

5. Main Agent takes over:
   - Checks registry for database agent
   - If found: reads documentation, spawns agent
   - If not found: creates documentation first, then spawns

6. Main Agent coordinates both agents:
   - Implementation Agent continues work
   - Database Validation Agent validates migrations
   - Both report to Main Agent
   - Main Agent orchestrates completion
```

## Summary

**Core Principle**: **DOCUMENT BEFORE USE** - Every agent must be documented before it can be spawned.

**Key Rules**:
- ✅ All agents documented in `.agents/agents/[name].md`
- ✅ Main Agent checks registry before spawning
- ✅ Main Agent reads full documentation before spawning
- ✅ Main Agent creates documentation for new agents
- ✅ Sub-agents read their own documentation
- ✅ Sub-agents stay within documented boundaries
- ✅ Sub-agents request new agents through Main Agent
- ❌ **NEVER spawn undocumented agents**
- ❌ **NEVER skip registry check**
- ❌ **NEVER create agents without documenting first**

**Registry Benefits**:
- 📋 Centralized agent catalog
- 🔍 Fast discovery via frontmatter scanning
- 📖 Clear capabilities and boundaries
- 🔄 Reusable agent definitions
- 🎯 Consistent agent behavior
- 📚 Knowledge preservation

**USER EXPECTATIONS**:
- Agents are well-documented and discoverable
- Main Agent makes informed spawning decisions
- Sub-agents know their exact responsibilities
- System remains organized and maintainable
- New agents are properly introduced with documentation

**Remember**: The user will be **VERY UPSET** if you spawn agents without checking the registry or create new agents without documenting them first!

---
*Created: 2026-01-14*
*Last Updated: 2026-01-14*
*Version: 1.0*
