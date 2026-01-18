---
name: [Agent Name]
type: [verification|implementation|review|utility|specialized]
language: [rust|javascript|python|language-agnostic|multiple]
purpose: Brief one-sentence description of agent purpose
created: YYYY-MM-DD
author: "Main Agent" or "Team Name"
license: "MIT" or other appropriate license
metadata:
  version: "1.0"
  last_updated: YYYY-MM-DD
  complexity: "simple | moderate | complex"
  tags:
    - verification
    - rust
    - testing
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

### Frontmatter Fields Explained

**REQUIRED Fields:**

- **`name`**: Clear, descriptive agent name
  - Use title case (e.g., "Rust Verification Agent")
  - Be specific about what the agent does
- **`type`**: Agent category
  - `verification`: Validates code quality, runs tests, checks standards
  - `implementation`: Writes code, implements features
  - `review`: Reviews code, provides feedback
  - `utility`: Helper agent for specific tasks
  - `specialized`: Domain-specific agent
- **`language`**: Programming language or scope
  - Specific language: `rust`, `javascript`, `python`, etc.
  - Multiple languages: `multiple`
  - Any language: `language-agnostic`
- **`purpose`**: One-sentence summary (10-15 words max)
  - Must be immediately understandable
  - Describes exactly what agent does
  - Used by Main Agent for selection
- **`created`**: Date agent was created (YYYY-MM-DD)
- **`author`**: Who created the agent
  - Examples: "Main Agent", "Team Name", "Developer Name"
- **`license`**: License for agent documentation and code
  - Examples: "MIT", "Apache-2.0", "Proprietary"
- **`metadata`**: Structured metadata object
  - **`version`**: Semantic version (e.g., "1.0", "2.1.0")
  - **`last_updated`**: Date of last update (YYYY-MM-DD)
  - **`complexity`**: Agent complexity level
    - `simple`: Straightforward, single purpose
    - `moderate`: Multiple responsibilities, some complexity
    - `complex`: Advanced logic, many dependencies
  - **`tags`**: Array of categorization tags
    - Use lowercase with hyphens
    - Examples: `verification`, `testing`, `rust`, `code-quality`, `security`
    - Minimum 2 tags, recommended 3-5
- **`tools_required`**: List of tools this agent needs
  - Include all required tools
  - Specify versions if critical
- **`spawned_by`**: Who can spawn this agent
  - `main-agent`: Only Main Agent can spawn
  - `sub-agent-name`: Specific sub-agent can spawn
  - `both`: Main Agent or sub-agents can spawn
- **`related_rules`**: Array of relevant rule numbers
  - Reference rules this agent must follow
  - Examples: "Rule 03", "Rule 07"
- **`status`**: Current agent status
  - `active`: Fully functional, ready for use
  - `deprecated`: Old version, use newer agent instead
  - `experimental`: Testing phase, may change

**OPTIONAL Fields (use when applicable):**

- **`skills_required`**: List of skills from `.agents/skills/` directory
  - Only include if agent needs specific skills
  - Use skill directory names
- **`spawns`**: List of agents this agent can spawn
  - Only if this agent spawns sub-agents
  - Examples: `["Rust Implementation Agent", "Test Runner Agent"]`

### Main Agent Frontmatter Enforcement (CRITICAL)

**Main Agent MUST validate and enforce complete frontmatter** when creating agent documentation.

#### When Creating .agents/agents/*.md:

Main Agent **MUST** include ALL required frontmatter fields:
- ✅ `name`: Clear, descriptive agent name (title case)
- ✅ `type`: verification | implementation | review | utility | specialized
- ✅ `language`: Specific language, multiple, or language-agnostic
- ✅ `purpose`: One-sentence summary (10-15 words max)
- ✅ `created`: YYYY-MM-DD (date of creation)
- ✅ `author`: "Main Agent" or "Team Name"
- ✅ `license`: "MIT" or other appropriate license
- ✅ `metadata`: Complete object with:
  - `version`: "1.0" (semantic version)
  - `last_updated`: YYYY-MM-DD
  - `complexity`: simple | moderate | complex
  - `tags`: Array with minimum 2 tags
- ✅ `tools_required`: Array of required tools
- ✅ `spawned_by`: main-agent | sub-agent-name | both
- ✅ `related_rules`: Array of relevant rule numbers
- ✅ `status`: active | deprecated | experimental
- ✅ `skills_required`: (if applicable) Array of skill names
- ✅ `spawns`: (if applicable) Array of spawnable agents

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
