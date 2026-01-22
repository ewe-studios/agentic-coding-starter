# Agent Documentation Frontmatter Reference

Complete reference for agent documentation frontmatter fields and validation requirements.

## Purpose

This reference provides detailed specifications for all frontmatter fields in agent documentation, ensuring consistency and completeness across all agent definitions.

## Quick Frontmatter Structure

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

## Complete Field Reference

| Field | Type | Required | Description | Validation Rules |
|-------|------|----------|-------------|------------------|
| `name` | string | ✅ | Clear, descriptive agent name | Title case, descriptive, unique |
| `type` | enum | ✅ | Agent classification | One of: verification, implementation, review, utility, specialized |
| `language` | string | ✅ | Programming language(s) | rust, javascript, python, language-agnostic, multiple |
| `purpose` | string | ✅ | One-sentence description | 10-15 words max, crystal clear, immediately understandable |
| `created` | date | ✅ | Creation date | YYYY-MM-DD format |
| `author` | string | ✅ | Creator identifier | "Main Agent", "Team Name", etc. |
| `license` | string | ✅ | License type | "MIT", "Apache-2.0", etc. |
| `metadata.version` | string | ✅ | Semantic version | e.g., "1.0", "2.1.3" |
| `metadata.last_updated` | date | ✅ | Last modification date | YYYY-MM-DD format |
| `metadata.complexity` | enum | ✅ | Complexity level | One of: simple, moderate, complex |
| `metadata.tags` | array | ✅ | Searchable tags | Minimum 2 tags, lowercase-with-hyphens |
| `tools_required` | array | ✅ | Required tools/dependencies | List of tool names |
| `spawned_by` | string | ✅ | Who can spawn this agent | main-agent, sub-agent-name, or both |
| `related_rules` | array | ✅ | Associated rule numbers | e.g., ["Rule 03", "Rule 07"] |
| `status` | enum | ✅ | Current status | One of: active, deprecated, experimental |
| `skills_required` | array | ⚪ Optional | Required skills | Skills from `.agents/skills/` |
| `spawns` | array | ⚪ Optional | Agents this can spawn | List of spawnable agent names |

## Field Descriptions

### `name` (REQUIRED)
**Purpose**: Unique identifier for the agent
**Format**: Title Case (e.g., "Rust Verification Agent")
**Rules**:
- Must be descriptive and clear
- Must be unique across all agents
- Should match filename (kebab-case version)
- Examples: "Rust Verification Agent", "Specification Update Agent"

### `type` (REQUIRED)
**Purpose**: Classification for discovery and filtering
**Valid Values**:
- `verification` - Quality assurance, testing, validation agents
- `implementation` - Code writing, feature development agents
- `review` - Pre-work review, analysis agents
- `utility` - Helper agents, tools, formatters
- `specialized` - Domain-specific agents (database, security, etc.)

### `language` (REQUIRED)
**Purpose**: Programming language specialization
**Valid Values**:
- Specific: `rust`, `javascript`, `typescript`, `python`, `go`, `java`, `csharp`
- Generic: `language-agnostic` - works with any language
- Multiple: `multiple` - supports several languages

### `purpose` (REQUIRED - CRITICAL FOR SELECTION)
**Purpose**: Main Agent uses this for agent selection
**Format**: One sentence, 10-15 words maximum
**Rules**:
- Must be immediately understandable
- Must be specific, not vague
- Main Agent reads ONLY this field for selection
- Bad: "Helps with verification" ❌
- Good: "Runs Rust verification checks including clippy, tests, and formatting" ✅

### `created` (REQUIRED)
**Purpose**: Track when agent was documented
**Format**: YYYY-MM-DD (ISO 8601)
**Example**: `2024-01-15`

### `author` (REQUIRED)
**Purpose**: Attribution and ownership
**Format**: String
**Examples**: "Main Agent", "DevOps Team", "Security Team"

### `license` (REQUIRED)
**Purpose**: Legal licensing information
**Common Values**: "MIT", "Apache-2.0", "GPL-3.0", "Proprietary"

### `metadata.version` (REQUIRED)
**Purpose**: Track documentation versions
**Format**: Semantic versioning (MAJOR.MINOR.PATCH)
**Examples**: "1.0", "2.1.3"
**Rules**:
- Increment PATCH for typo fixes, clarifications
- Increment MINOR for added capabilities
- Increment MAJOR for breaking changes

### `metadata.last_updated` (REQUIRED)
**Purpose**: Track freshness of documentation
**Format**: YYYY-MM-DD
**Rules**:
- MUST update when making changes
- MUST match date of last modification
- Used to identify stale documentation

### `metadata.complexity` (REQUIRED)
**Purpose**: Indicate agent sophistication
**Valid Values**:
- `simple` - Straightforward, single-purpose agent
- `moderate` - Multiple steps, some coordination
- `complex` - Multi-phase workflow, advanced logic

### `metadata.tags` (REQUIRED)
**Purpose**: Enable searchability and discovery
**Format**: Array of lowercase-with-hyphens strings
**Rules**:
- Minimum 2 tags required
- Use lowercase with hyphens
- Be specific and relevant
- Examples: `["verification", "rust", "testing"]`

### `tools_required` (REQUIRED)
**Purpose**: Document dependencies
**Format**: Array of strings
**Examples**: `["cargo", "rustc", "clippy"]`
**Empty if no tools**: `[]`

### `skills_required` (OPTIONAL)
**Purpose**: Link to required skill files
**Format**: Array of skill names from `.agents/skills/`
**Examples**: `["git-workflow", "docker-operations"]`
**Omit if none**: Can be excluded or empty array `[]`

### `spawned_by` (REQUIRED)
**Purpose**: Define spawning authority
**Valid Values**:
- `main-agent` - Only Main Agent can spawn this
- `both` - Both Main Agent and sub-agents can request
- `[specific-agent]` - Only specific agent can spawn

### `spawns` (OPTIONAL)
**Purpose**: Document what this agent can spawn
**Format**: Array of agent names
**Examples**: `["specification-update", "implementation"]`
**Note**: Sub-agents generally should NOT spawn verification agents

### `related_rules` (REQUIRED)
**Purpose**: Cross-reference relevant rules
**Format**: Array of rule identifiers
**Examples**: `["Rule 05", "Rule 08"]`
**Minimum**: Should reference at least one rule

### `status` (REQUIRED)
**Purpose**: Indicate agent lifecycle state
**Valid Values**:
- `active` - Currently in use
- `deprecated` - Being phased out
- `experimental` - Testing/development phase

## Validation Checklist

Before creating or updating agent documentation, verify:

### Completeness
- [ ] All REQUIRED fields present
- [ ] All metadata sub-fields present
- [ ] No missing values in required fields

### Format
- [ ] Dates in YYYY-MM-DD format
- [ ] Arrays properly formatted with brackets
- [ ] Enums use valid values only
- [ ] Version follows semantic versioning

### Quality
- [ ] Purpose is 10-15 words and crystal clear
- [ ] Tags are lowercase with hyphens
- [ ] Name matches filename (title case to kebab-case)
- [ ] Type accurately reflects agent function
- [ ] Related rules are correct

### Clarity
- [ ] Purpose is immediately understandable
- [ ] Purpose is specific, not vague
- [ ] Tags are relevant and searchable
- [ ] Complexity level is accurate

## Common Mistakes

### ❌ Vague Purpose
```yaml
purpose: Helps with verification tasks  # Too vague!
```

### ✅ Clear Purpose
```yaml
purpose: Runs Rust verification including clippy, tests, and formatting
```

### ❌ Wrong Case in Tags
```yaml
tags: [Rust, Verification, Testing]  # Wrong case!
```

### ✅ Correct Tags
```yaml
tags: [rust, verification, testing]
```

### ❌ Missing Metadata Fields
```yaml
metadata:
  version: "1.0"  # Missing last_updated, complexity, tags!
```

### ✅ Complete Metadata
```yaml
metadata:
  version: "1.0"
  last_updated: 2024-01-15
  complexity: "moderate"
  tags: [rust, verification]
```

## Update Guidelines

### When to Update Version

**PATCH increment** (1.0 → 1.0.1):
- Typo fixes
- Clarifications
- Minor wording improvements
- No functional changes

**MINOR increment** (1.0 → 1.1):
- Added capabilities
- New tools or skills
- Enhanced workflow steps
- Backward compatible

**MAJOR increment** (1.0 → 2.0):
- Breaking changes
- Removed capabilities
- Changed requirements
- Incompatible with previous

### When to Update last_updated

Update whenever you modify:
- ✅ Any content in the documentation
- ✅ Frontmatter fields
- ✅ Workflow steps
- ✅ Requirements
- ✅ Examples

### When to Update tags

Add tags when:
- ✅ Agent gains new capabilities
- ✅ New use cases discovered
- ✅ Integration with new tools

Remove tags when:
- ✅ Capabilities removed
- ✅ No longer relevant

## Integration with Rule 10

This reference is extracted from Rule 10 (Agent Documentation and Registry) to:
- ✅ Reduce rule file size
- ✅ Provide comprehensive field documentation
- ✅ Enable easy reference during agent creation
- ✅ Maintain single source of truth

**See Rule 10** for:
- Complete agent documentation workflow
- Duplicate prevention process
- Registry usage patterns
- Enforcement requirements

---

*Created: 2026-01-22*
*Referenced in: Rule 10, section "Frontmatter Fields Reference"*
*Source: Extracted from Rule 10 for better maintainability*
