# Agent Documentation and Registry (Main Agent Only)

## Purpose

This rule is for **Main Agent only** when creating or documenting agents. Sub-agents should load **Rule 12 (Agent Registry Usage)** instead for a concise guide on being spawned and following their documentation.

**Context Optimization**: This full rule (~770 lines) is only needed when creating/documenting agents. Sub-agents only need Rule 12 (~150 lines).

---

## Overview

This rule establishes a mandatory agent documentation system that ensures **ALL agents** (existing and new) are properly documented before use. This creates a centralized registry of agent capabilities, responsibilities, and requirements, enabling the Main Agent to make informed decisions about which specialized agents to spawn for specific tasks.

## Rule

### Mandatory Agent Documentation
**CRITICAL REQUIREMENT**: Before **ANY** agent (new or existing) can be spawned by the Main Agent or requested by sub-agents, that agent **MUST** be documented in the agent registry.

### Core Principle
```
Need Agent → Check Registry → Found? Use Documentation : Create Documentation First
                                                ↓
                                    Spawn Agent WITH Documentation File Path
                                                ↓
                                    Sub-Agent Reads Own Documentation
```

**NO EXCEPTIONS**:
- ❌ **NEVER spawn an undocumented agent**
- ❌ **NEVER create a new agent without documentation**
- ❌ **NEVER skip the registry check**
- ❌ **NEVER spawn sub-agent without providing documentation file path**
- ❌ **NEVER allow duplicate agent documentation files**
- ✅ **ALWAYS document before using**
- ✅ **ALWAYS provide documentation path when spawning**
- ✅ **ALWAYS check for duplicates before creating new documentation**

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

### Duplicate Prevention

**CRITICAL**: No duplicate agent documentation is allowed.

**Before creating new agent documentation:**
1. ✅ Scan all existing `.agents/agents/*.md` frontmatter
2. ✅ Check if similar agent already exists
3. ✅ If duplicate found: merge into single comprehensive file
4. ✅ If similar but different: ensure clear differentiation in frontmatter

**Duplicate Detection Process**:
```
Main Agent creating new agent documentation:
1. Read all .agents/agents/*.md frontmatter
2. Compare: name, type, purpose, language fields
3. If match found:
   ├─ SAME purpose + SAME type + SAME language → DUPLICATE (merge)
   ├─ SIMILAR purpose but DIFFERENT specialization → OK (clarify difference)
   └─ DIFFERENT purpose → OK (proceed with creation)
4. If duplicate: merge both into single comprehensive file
5. Delete redundant file after merge
6. Commit changes
```

## Agent Documentation Format

### Frontmatter Importance (CRITICAL)

**Main Agent Decision-Making**: The Main Agent makes spawning decisions based **ONLY** on:
1. **Filename**: Descriptive name indicating agent purpose
2. **Frontmatter**: Quick summary at top of file

**Main Agent DOES NOT read full documentation** when scanning registry. Therefore:
- ✅ Frontmatter MUST be crystal clear and self-explanatory
- ✅ Purpose field MUST be immediately understandable
- ✅ All critical information MUST be in frontmatter
- ❌ Main Agent will NOT read detailed sections when selecting agent
- ❌ Do NOT rely on detailed sections for agent selection

**Frontmatter Writing Guidelines**:
- **name**: Clear, descriptive name (e.g., "Rust Verification Agent")
- **type**: Exact type from allowed list
- **language**: Specific language or "language-agnostic"
- **purpose**: ONE clear sentence (10-15 words max) stating exactly what agent does
  - ✅ GOOD: "Verify Rust code quality, run tests, check clippy and formatting"
  - ❌ BAD: "Handles Rust stuff" (too vague)
  - ❌ BAD: "Comprehensive Rust code verification including but not limited to quality checks..." (too verbose)
- **tools_required**: Complete list (Main Agent checks if available)
- **skills_required**: Complete list (Main Agent checks if accessible)

**Why This Matters**:
- Main Agent scans 10-20 agent files quickly
- Reading full documentation for each would be inefficient
- Frontmatter enables fast filtering and selection
- Clear frontmatter = correct agent selection
- Vague frontmatter = wrong agent spawned = wasted work

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

### Template Reference

**Full template available at**: `.agents/templates/agent-documentation-template.md`

**Quick frontmatter structure**:
```yaml
---
name: [Agent Name]
type: [verification|implementation|review|utility|specialized]
language: [rust|javascript|python|language-agnostic|multiple]
purpose: Brief one-sentence description
created: YYYY-MM-DD
author: "Main Agent" or "Team Name"
license: "MIT" or other
metadata:
  version: "1.0"
  last_updated: YYYY-MM-DD
  complexity: "simple | moderate | complex"
  tags: [verification, rust, testing]
tools_required: [Tool 1, Tool 2]
skills_required: [Skill 1, Skill 2]
spawned_by: [main-agent|sub-agent-name|both]
spawns: [list of spawnable agents]
related_rules: [Rule NN]
status: [active|deprecated|experimental]
---
```

### Frontmatter Fields Reference

**Complete Reference**: See `.agents/templates/examples/agent-frontmatter-reference.md` for comprehensive field documentation, validation rules, examples, and update guidelines.

**Quick Summary of Required Fields**:
- `name`, `type`, `language`, `purpose` (10-15 words, crystal clear)
- `created`, `author`, `license`
- `metadata`: `version`, `last_updated`, `complexity`, `tags` (min 2)
- `tools_required`, `spawned_by`, `related_rules`, `status`

**Optional Fields**: `skills_required`, `spawns`

**Critical for Selection**: Main Agent uses `purpose` field to select agents - must be immediately understandable and specific.

### Main Agent Frontmatter Enforcement (CRITICAL)

**Main Agent MUST validate and enforce complete frontmatter** when creating agent documentation.

#### Validation Requirements:

Before creating any agent documentation, Main Agent MUST:
1. **Check frontmatter completeness**
   - All REQUIRED fields present
   - All metadata sub-fields present
   - Dates in correct format (YYYY-MM-DD)
   - Arrays properly formatted
2. **Validate field values**
   - Type is valid enum value
   - Status is valid enum value
   - Language is appropriate
   - Purpose is concise (10-15 words)
   - Tags are lowercase with hyphens
   - Version follows semantic versioning
3. **Validate purpose clarity**
   - Purpose must be immediately understandable
   - Main Agent uses purpose for agent selection
   - Must be specific, not vague
4. **Report if validation fails**
   - Stop creation process
   - Report missing or invalid fields
   - Request correction before proceeding

#### Updates to Existing Agent Documentation:

When updating agent documentation:
- ✅ Main Agent MUST update `metadata.last_updated`
- ✅ Main Agent MUST increment `metadata.version` if significant changes
- ✅ Main Agent MUST update `status` if agent is deprecated
- ✅ Main Agent MUST add new tags if functionality expands
- ✅ Main Agent MUST update `tools_required` if requirements change
- ❌ Main Agent MUST NOT leave metadata stale

#### Enforcement Consequences:

**If Main Agent creates agent documentation without complete frontmatter:**
- ❌ Violation of Rule 10
- ❌ Agent documentation is invalid
- ❌ Agent cannot be properly discovered or selected
- ❌ Must be corrected before agent can be used

**If frontmatter purpose is vague:**
- ❌ Main Agent cannot make proper selection decisions
- ❌ Wrong agent may be spawned for tasks
- ❌ Purpose must be rewritten to be specific

## Workflow for Agent Usage

### Critical Requirement: Documentation File Path

**MANDATORY**: When spawning any sub-agent, Main Agent **MUST** provide:
1. ✅ Path to agent's documentation file (`.agents/agents/[name].md`)
2. ✅ Task-specific context
3. ✅ Related specification path (if applicable)
4. ✅ Any other required resources

**Sub-Agent MUST** receive documentation path in spawn prompt:
```
You are a [Agent Name].

CRITICAL: Read your agent documentation FIRST:
- File: .agents/agents/[name-of-agent].md

After reading your documentation:
1. Read AGENTS.md
2. Read relevant rules
3. Read specification (if provided)
4. Execute your documented responsibilities

Your task: [specific task description]
[additional context...]
```

### Sub-Agent Startup Protocol

**MANDATORY for all sub-agents** upon being spawned:

1. **Check for Documentation Path**
   - Look for `.agents/agents/[name].md` in spawn context
   - If NOT provided: **STOP immediately**

2. **If Documentation Path Missing**:
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

3. **If Documentation Path Provided**:
   - Read documentation file FIRST
   - Understand: capabilities, requirements, responsibilities, boundaries
   - Then read AGENTS.md
   - Then read relevant rules
   - Then read specification (if applicable)
   - Then execute documented workflow

**Why This Matters**:
- Sub-agent needs to know exact responsibilities
- Sub-agent must understand boundaries
- Sub-agent requires workflow steps
- Prevents sub-agent from guessing or assuming
- Ensures consistent behavior across spawns

### Context Management: Module Documentation

**Main Agent Context Optimization:**

When dealing with module documentation:

1. **Main Agent DOES NOT load large module documentation** (>8-10KB)
2. **Main Agent references the path** in requirements.md or spawn instructions
3. **Main Agent delegates reading** to sub-agents who work with that module
4. **Sub-agents MUST**:
   - Load module documentation for their assigned modules
   - Use Grep/Glob/Read tools to find specific implementations
   - Keep module documentation up-to-date as they make changes
   - Report documentation updates to Main Agent

**Two-Tier Documentation Strategy:**

**Tier 1: Reasonable Size (<8-10KB)**
```
Sub-Agent Process:
1. Load module documentation
2. Use Grep/Glob for specific details
3. Make code changes
4. Update documentation directly
5. Report completion
```

**Tier 2: Large Documentation (>8-10KB)**
```
Sub-Agent Process:
1. Skip loading documentation (too large)
2. Use Grep/Glob/Read exclusively for code understanding
3. Make code changes
4. Report to Main Agent: "Documentation too large, need Documentation Agent"
5. Main Agent spawns Documentation Agent
6. Documentation Agent reviews changes and updates docs
7. Documentation Agent reports completion
```

### Documentation Agent: Comprehensive Asset Creation

When Main Agent spawns Documentation Agent to create or update module documentation, the Documentation Agent **MUST** create comprehensive documentation assets beyond just the `doc.md` file.

**Mandatory Asset Creation by Module Type:**

#### For API Modules:
- **OpenAPI Specification** (`assets/openapi.yaml` or `.json`)
- **API Examples** (`assets/examples/`) - cURL, client code, Postman collections

#### For Data Model Modules:
- **JSON Schema** (`assets/schemas/[model-name].json`)
- **TypeScript Definitions** (`assets/types/`) (optional)
- **GraphQL Schema** (`assets/schema.graphql`) (if applicable)

#### For Library/SDK Modules:
- **Usage Examples** (`assets/examples/`) - Basic/advanced patterns
- **Configuration Examples** (`assets/configs/`) - Config templates

#### For All Modules:
- **Architecture Diagrams** (`assets/diagrams/`) - Component, flow, sequence, ER diagrams
- **Reference Documentation** (`assets/references/`) - External links, RFCs

**Complete Asset Directory Structure:**
```
documentation/[module]/
├── doc.md                        # Main documentation
└── assets/                       # Supplementary documentation
    ├── openapi.yaml              # API specification
    ├── schemas/                  # JSON schemas
    ├── types/                    # TypeScript definitions
    ├── examples/                 # Code examples
    ├── configs/                  # Configuration templates
    ├── diagrams/                 # Visual documentation
    └── references/               # External resources
```

### Main Agent Workflow

```
1. Identify Need for Specialized Agent
   ↓
2. Check Agent Registry (.agents/agents/)
   ├─ Scan all *.md frontmatter (filename + YAML header ONLY)
   ├─ Compare: type, language, purpose fields
   ├─ Filter by requirements (tools, skills needed)
   ├─ Check for duplicates (same type+purpose+language)
   └─ Select best match for task
   ↓
3. If Agent Found in Registry:
   ├─ Note the filename (e.g., rust-verification.md)
   ├─ Frontmatter already read during scan
   ├─ DO NOT read full documentation (not needed for Main Agent)
   ├─ Verify requirements can be met
   └─ Proceed to spawning
   ↓
4. If Agent NOT Found in Registry:
   ├─ Check for potential duplicates first
   ├─ Create new agent documentation
   ├─ Fill in complete frontmatter (clear and concise)
   ├─ Fill in detailed sections
   ├─ Commit new documentation
   └─ Now proceed to spawning
   ↓
5. Spawn Agent with MANDATORY Documentation Path
   ├─ Provide: .agents/agents/[name-of-agent].md path
   ├─ Provide: task-specific context
   ├─ Provide: related specification (if applicable)
   ├─ Provide: access to required tools/skills
   └─ NEVER spawn without documentation path
   ↓
6. Sub-Agent Startup
   ├─ Sub-agent checks for documentation path
   ├─ If missing: STOPS and requests from Main Agent
   ├─ If provided: reads documentation FIRST
   ├─ Then reads AGENTS.md, rules, specification
   └─ Executes documented workflow
   ↓
7. Agent Executes Task
   ↓
8. Agent Reports Back to Main Agent
```

**Key Points for Main Agent**:
- ✅ Only scans frontmatter (fast, efficient)
- ✅ Filename + frontmatter = enough for decision
- ✅ Does NOT read full documentation when selecting
- ✅ ALWAYS provides documentation path when spawning
- ✅ Checks for duplicates before creating new documentation
- ❌ NEVER spawns without documentation path
- ❌ NEVER creates duplicate documentation

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

### Process for Creating New Agent Documentation

1. **Main Agent Identifies Need**
   - Analyzes task requirements
   - Checks if existing agents can handle it
   - Determines new agent is needed

2. **Main Agent Creates Documentation**
   - Uses template from `.agents/templates/agent-documentation-template.md`
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

### Rule 05 (Agent Orchestration)
- Main Agent uses registry to select appropriate agents
- Documentation defines which agents can spawn others
- Clear hierarchy maintained through documentation

### Rule 06 (Specifications)
- Specification Update Agent documented
- Review Agent documented with pre-work requirements
- Clear integration points defined

### Rule 12 (Agent Registry Usage)
- Sub-agents load Rule 12 for concise usage guide
- Sub-agents read own documentation when spawned
- Reduces context usage for sub-agents

## Enforcement

### Violations

The following are **CRITICAL VIOLATIONS** with **ZERO TOLERANCE**:

1. ❌ **Spawning undocumented agent** (Main Agent violation)
2. ❌ **Creating new agent without documentation** (Main Agent violation)
3. ❌ **Skipping registry check** (Main Agent violation)
4. ❌ **Spawning sub-agent WITHOUT documentation path** (Main Agent violation - CRITICAL)
5. ❌ **Creating duplicate agent documentation** (Main Agent violation)
6. ❌ **Vague or unclear frontmatter** (Main Agent violation when creating docs)
7. ❌ **Sub-agent spawning agents directly** (Sub-agent violation)
8. ❌ **Not reading own documentation** (Sub-agent violation)
9. ❌ **Exceeding documented boundaries** (Sub-agent violation)
10. ❌ **Sub-agent proceeding without documentation path** (Sub-agent violation - MUST STOP)

### Consequences

**USER WILL SHOUT AT YOU** if:
- ❌ You spawn an agent without checking registry
- ❌ You create a new agent without documenting it first
- ❌ You ignore agent documentation boundaries
- ❌ You skip reading agent documentation before spawning
- ❌ **You spawn sub-agent without providing documentation file path**
- ❌ **Sub-agent proceeds without documentation path**
- ❌ You create duplicate agent documentation
- ❌ You write vague frontmatter that doesn't help selection
- ❌ Sub-agent spawns another agent without going through Main Agent

### Corrective Action

When violation occurs:
1. **STOP immediately**
2. **If sub-agent spawned without documentation path**:
   - Sub-agent STOPs and requests documentation
   - Main Agent provides correct path
   - Sub-agent reads documentation
   - Sub-agent proceeds
3. **If duplicate documentation detected**:
   - Merge both into single comprehensive file
   - Delete redundant file
   - Commit changes
4. **Check registry** for appropriate agent
5. **Read frontmatter** of all agents
6. **Create documentation** if agent is new (check for duplicates first)
7. **Commit documentation** before spawning
8. **Spawn agent properly** with documentation path in prompt
9. **Report violation** to user (transparency)

## Examples

### Example 1: Main Agent Needs Rust Verification

```
1. Main Agent identifies need: "Implementation complete, need to verify Rust code"

2. Main Agent scans registry frontmatter ONLY:
   - Reads .agents/agents/*.md filenames and frontmatter
   - Finds rust-verification.md with clear purpose
   - Identifies as correct agent
   - Does NOT read full documentation

3. Main Agent verifies requirements and spawns agent WITH documentation path

4. Rust Verification Agent starts:
   - Checks for documentation path: ✅ Found
   - Reads documentation FIRST
   - Executes all checks in documented order
   - Reports to Main Agent

✅ Correct workflow: Documentation path provided, sub-agent reads it first
```

### Example 2: Sub-Agent Needs Help (Correct Process)

```
1. Implementation Agent working on task

2. Implementation Agent realizes: "I need database migration validation agent"

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
```

## Summary

**Core Principle**: **DOCUMENT BEFORE USE** - Every agent must be documented before it can be spawned, and documentation path must ALWAYS be provided to sub-agents.

**Key Rules**:
- ✅ All agents documented in `.agents/agents/[name].md`
- ✅ Main Agent scans frontmatter ONLY (filename + YAML header)
- ✅ Main Agent does NOT read full documentation when selecting
- ✅ Frontmatter MUST be crystal clear and self-explanatory
- ✅ Main Agent checks for duplicates before creating new documentation
- ✅ **Main Agent ALWAYS provides documentation path when spawning**
- ✅ **Sub-agents MUST receive documentation path in spawn prompt**
- ✅ **Sub-agents read documentation FIRST before executing**
- ✅ **Sub-agents STOP if documentation path missing**
- ✅ Sub-agents stay within documented boundaries
- ✅ Sub-agents request new agents through Main Agent
- ❌ **NEVER spawn without providing documentation path**
- ❌ **NEVER allow sub-agent to proceed without documentation**
- ❌ **NEVER spawn undocumented agents**
- ❌ **NEVER skip registry check**
- ❌ **NEVER create duplicate agent documentation**
- ❌ **NEVER write vague frontmatter**

**Main Agent Responsibilities**:
- Scan frontmatter (fast decision-making)
- Select agent based on filename + frontmatter only
- Check for duplicates before creating
- Create clear, concise frontmatter
- **Provide documentation path in spawn prompt**
- Never spawn without documentation path

**Sub-Agent Responsibilities**:
- Check for documentation path immediately
- STOP if documentation path missing
- Request documentation from Main Agent if missing
- Read documentation FIRST (before anything else)
- Follow documented workflow exactly
- Stay within documented boundaries
- Never spawn other agents directly

**Duplicate Prevention**:
- Check all frontmatter before creating new agent
- Merge duplicates into single comprehensive file
- Delete redundant documentation
- Ensure clear differentiation in frontmatter if similar

**Registry Benefits**:
- 📋 Centralized agent catalog
- 🔍 Fast discovery via frontmatter scanning
- 📖 Clear capabilities and boundaries defined in detailed sections
- 🔄 Reusable agent definitions
- 🎯 Consistent agent behavior
- 📚 Knowledge preservation
- ⚡ Efficient selection (frontmatter only)
- 🚫 No duplicates allowed

**USER EXPECTATIONS**:
- Agents are well-documented and discoverable
- Main Agent makes informed spawning decisions efficiently
- Sub-agents ALWAYS receive their documentation
- Sub-agents know their exact responsibilities
- No duplicate agent documentation exists
- Frontmatter is clear enough for quick decisions
- System remains organized and maintainable
- New agents are properly introduced with documentation

**Remember**: The user will be **VERY UPSET** if:
- You spawn agents without checking the registry
- You create new agents without documenting them first
- **You spawn sub-agents without providing documentation path**
- **Sub-agents proceed without reading their documentation**
- You create duplicate agent documentation
- You write vague frontmatter that doesn't help selection

---
*Created: 2026-01-14*
*Last Updated: 2026-01-19 (Split into Rule 10 for documentation and Rule 12 for usage)*

---

## Related Rules

- **Rule 12 (Agent Registry Usage)**: Concise guide for sub-agents being spawned
- **Rule 05 (Agent Orchestration)**: How Main Agent spawns and coordinates agents
- **Rule 09 (Skills Creation)**: How to create skills that agents use
