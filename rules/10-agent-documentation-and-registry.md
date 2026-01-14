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

**Example - Duplicate Found**:
```
Existing: rust-verification.md
  name: Rust Verification Agent
  type: verification
  purpose: Verify Rust code quality, tests, and standards

New request: rust-code-checker.md
  name: Rust Code Checker
  type: verification
  purpose: Verify Rust code quality and run tests

→ DUPLICATE! Same purpose, same type, same language
→ Action: Merge into rust-verification.md (keep existing)
→ Delete: rust-code-checker.md concept
```

**Example - Not Duplicate**:
```
Existing: rust-verification.md
  name: Rust Verification Agent
  type: verification
  purpose: Verify Rust code quality, tests, and standards

New request: rust-security-audit.md
  name: Rust Security Audit Agent
  type: specialized
  purpose: Perform security audit and vulnerability scanning

→ NOT DUPLICATE: Different specialization (general vs security)
→ Action: Create rust-security-audit.md
→ Clarify: frontmatter must clearly distinguish from rust-verification.md
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

### Context Management: Module Documentation (CRITICAL)

**IMPORTANT**: In addition to agent documentation, sub-agents often need to work with module documentation (`documentation/[module]/doc.md`). Large module documentation files (>8-10KB) should NOT be loaded by Main Agent.

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

**Spawn Pattern with Module Documentation:**
```
You are an Implementation Agent.

CRITICAL: Read your agent documentation FIRST:
- File: .agents/agents/implementation.md

Module Documentation (MUST READ):
- File: documentation/foundation_core/doc.md
- This provides overview of the module you'll be modifying
- Use Grep/Glob to find specific implementations after reading overview

After reading documentation:
1. Read module doc for high-level understanding
2. Use Grep/Glob/Read to find specific code locations
3. Make your changes
4. Update module documentation if structure changed
5. Report back with documentation status

Your task: [specific task]
```

**Why This Pattern:**
- **Context Efficiency**: Main Agent doesn't waste context on large docs
- **Targeted Loading**: Sub-agents only load docs they need
- **Tool Usage**: Sub-agents use Grep/ripgrep for specifics
- **Documentation Freshness**: Sub-agents update docs as they work
- **Clear Responsibility**: Agent working on module maintains its documentation

**Sub-Agent Responsibilities with Module Documentation:**
- ✅ Load assigned module documentation
- ✅ Use tools (Grep/Glob) to find specific implementations
- ✅ Update documentation when making code changes
- ✅ Report documentation changes to Main Agent
- ✅ Keep documentation synchronized with code
- ❌ NOT skip documentation because "it's too large"
- ❌ NOT ignore documentation updates

**Tool Strategy for Sub-Agents:**
```
Documentation provides → High-level architecture, module structure
Tools (Grep/Glob) provide → Specific line numbers, exact implementations
Combined approach → Fast orientation + precise work
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
1. Main Agent identifies need:
   "Implementation complete, need to verify Rust code"

2. Main Agent scans registry frontmatter ONLY:
   - Reads .agents/agents/*.md filenames and frontmatter
   - Finds rust-verification.md:
     * name: Rust Verification Agent
     * type: verification
     * language: rust
     * purpose: "Verify Rust code quality, run tests, check clippy and formatting"
     * tools_required: [cargo, clippy, rustfmt]
   - Identifies as correct agent (frontmatter is clear)
   - Does NOT read full documentation (not needed)

3. Main Agent verifies requirements:
   - cargo available? Yes
   - clippy available? Yes
   - rustfmt available? Yes
   - Can provide context? Yes

4. Main Agent spawns agent WITH documentation path:
   Task(
     subagent_type: "general-purpose",
     description: "Verify Rust code quality",
     prompt: "You are a Rust Verification Agent.

     CRITICAL: Read your agent documentation FIRST:
     - File: .agents/agents/rust-verification.md

     After reading your documentation:
     1. Read AGENTS.md
     2. Read relevant rules
     3. Execute your documented workflow

     Your task: Verify the following Rust files...
     Files changed: [list]
     Specification: specifications/03-user-authentication/

     [additional context]"
   )

5. Rust Verification Agent starts:
   - Checks for documentation path: ✅ Found (.agents/agents/rust-verification.md)
   - Reads .agents/agents/rust-verification.md FIRST
   - Understands workflow: cargo fmt, clippy, test, build, audit sequence
   - Reads AGENTS.md
   - Reads relevant rules
   - Executes all checks in documented order
   - Generates report
   - Reports to Main Agent

✅ Correct workflow: Documentation path provided, sub-agent reads it first
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
*Last Updated: 2026-01-14*
*Version: 1.1 - Added mandatory documentation path requirement and duplicate prevention*
