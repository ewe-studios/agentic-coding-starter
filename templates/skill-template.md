---
name: "[Skill Name]"
description: "1-2 sentence summary of what skill achieves and when to use it"
approved: No
created: YYYY-MM-DD
license: "MIT"
metadata:
  author: "Main Agent"
  version: "1.0"
  last_updated: "YYYY-MM-DD"
  tags:
    - tag-1
    - tag-2
    - tag-3
tools:
  - Tool 1
  - Tool 2
files:
  - templates/file1.ext: "Brief description"
  - scripts/run.sh: "Brief description"
  - examples/example1.ext: "Brief description"
assets:
  - docs/deep-dive.md: "Extended documentation for users"
  - assets/diagrams/flow.png: "Architecture diagram"
---

# [Skill Name]

## Overview

Brief overview of what this skill is about (2-3 paragraphs).

## When to Use This Skill

- List specific scenarios where this skill applies
- Be clear about scope and limitations
- Include use cases

## Prerequisites

- Knowledge required before using this skill
- Dependencies that must be installed
- Environment setup needed

## Skill Usage Type

**Choose ONE**: TEMPLATE | EXECUTABLE | EDUCATIONAL

---

### For TEMPLATE Skills:

**Skill Usage Type**: TEMPLATE - Copy all files to project and customize

**Files to Copy** (from `templates/` directory):
| File | Purpose | Copy To |
|------|---------|---------|
| `templates/client.ts` | Main implementation | `src/clients/your-client.ts` |
| `templates/helpers.ts` | Helper functions | `src/clients/helpers.ts` |

**Instructions**:
1. Copy ALL files from `templates/` to your project
2. Customize copied files for your use case
3. Import from PROJECT location, NOT from `.agents/skills/`

---

### For EXECUTABLE Skills:

**Skill Usage Type**: EXECUTABLE - Run scripts as external tools

**Available Scripts** (in `scripts/` directory):
| Script | Purpose | Usage |
|--------|---------|-------|
| `scripts/run.sh` | Main execution | `./run.sh --arg value` |

**Arguments**:
- `--arg1`: Description (required)
- `--arg2`: Description (optional, default: X)

**Output**: Description of output format

---

### For EDUCATIONAL Skills:

**Skill Usage Type**: EDUCATIONAL - Learn pattern and implement fresh

**External Dependencies**:
```bash
npm install package-name
# or
cargo add package-name
```

**Examples to Study** (in `examples/` directory):
| File | What It Demonstrates |
|------|---------------------|
| `examples/basic.rs` | Core pattern implementation |
| `examples/advanced.rs` | Extended patterns |

**Instructions**:
1. Study the examples to understand the pattern
2. Install the external dependencies
3. Implement FRESH code in your project
4. NEVER import from `.agents/skills/`

---

## Core Concepts

Key concepts needed to understand this skill:

- **Concept 1**: Explanation
- **Concept 2**: Explanation
- **Concept 3**: Explanation

## Step-by-Step Guide

### Step 1: [First Step Name]

Detailed explanation with code examples.

### Step 2: [Second Step Name]

Detailed explanation with code examples.

[Continue for all steps...]

## Common Patterns

Frequently used patterns when applying this skill:

- **Pattern 1**: When and how to use
- **Pattern 2**: When and how to use

## Pitfalls to Avoid

Common mistakes and how to avoid them:

- **Pitfall 1**: What to avoid and why
- **Pitfall 2**: What to avoid and why

## Examples

### Example 1: [Scenario Name]

```[language]
// Code example
```

### Example 2: [Another Scenario]

```[language]
// Code example
```

## File Reference

| File | Location | Type | Purpose |
|------|----------|------|---------|
| file1.ext | `templates/` | TEMPLATE | Purpose |
| run.sh | `scripts/` | EXECUTABLE | Purpose |
| example1.ext | `examples/` | EDUCATIONAL | Purpose |

## Additional Resources

**In `docs/` directory** (optional - for user reference):
- `deep-dive.md` - Extended explanation
- `troubleshooting.md` - Common issues
- `faq.md` - Frequently asked questions

**In `assets/` directory** (optional - supporting files):
- `diagrams/` - Architecture diagrams
- `configs/` - Configuration templates
- `data/` - Sample data files

## References

- Official documentation links
- Tutorials used
- Stack Overflow discussions
- Blog posts or guides

---

*Created: [Date]*
*Last Updated: [Date]*
