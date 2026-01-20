# Skills Directory

This directory contains documented skills that agents use to accomplish specific technical tasks.

## Purpose

Skills are detailed guides for:

- Using specific tools/libraries (e.g., Playwright, Docker, Kubernetes)
- Implementing specific patterns (e.g., authentication, caching)
- Performing specific operations (e.g., database migrations, API integration)
- Solving specific problems (e.g., performance optimization)

## Directory Structure

Each skill lives in its own directory with optional subdirectories for assets:

```
.agents/skills/
├── README.md (this file)
├── [skill-name]/
│   ├── skill.md           # REQUIRED - Main documentation
│   ├── learnings.md       # OPTIONAL - Practical insights from usage
│   ├── assets/            # OPTIONAL - Diagrams, configs, sample data
│   ├── docs/              # OPTIONAL - Extended user documentation
│   ├── templates/         # OPTIONAL - For TEMPLATE skills
│   ├── scripts/           # OPTIONAL - For EXECUTABLE skills
│   └── examples/          # OPTIONAL - For EDUCATIONAL skills
```

### Directory Contents

| Directory      | Purpose                                      | When to Include                 |
| -------------- | -------------------------------------------- | ------------------------------- |
| `skill.md`     | Main documentation (REQUIRED)                | Always                          |
| `learnings.md` | Practical insights from usage                | After skill is used             |
| `assets/`      | Diagrams, configs, sample data               | When visual/config aids helpful |
| `docs/`        | Extended documentation, FAQ, troubleshooting | For complex skills              |
| `templates/`   | Code templates (TEMPLATE skills)             | When copying files to project   |
| `scripts/`     | Executable tools (EXECUTABLE skills)         | When running as external tools  |
| `examples/`    | Reference implementations (EDUCATIONAL)      | For learning patterns           |

### Naming Convention

- Skills use **kebab-case**: `playwright-web-interaction`, `kubernetes-deployment`
- **NO numeric prefixes** - skills are referenced by name
- Name clearly describes purpose

### External Documentation

Skills may reference documentation stored elsewhere (e.g., `.agents/docs/[topic]/`). Use the `assets:` field in frontmatter to reference external files.

## Three Usage Types

1. **TEMPLATE**: Copy ALL files from `templates/` to project and customize
2. **EXECUTABLE**: Run scripts from `scripts/` as external tools
3. **EDUCATIONAL**: Study `examples/`, install external libraries, implement fresh

**CRITICAL**: Agents must NEVER `import/require/use` from `.agents/skills/` in project code.

## Skill File Format

Every `skill.md` file must start with frontmatter:

```yaml
---
name: "Skill Name"
description: "Brief description of what this skill achieves"
approved: No # Changed to Yes after user approval
created: YYYY-MM-DD
license: "MIT"
metadata:
  author: "Main Agent"
  version: "1.0"
  last_updated: "YYYY-MM-DD"
  tags: [tag-1, tag-2]
tools: [Tool1, Tool2]
files:
  - templates/client.ts: "Template for API client"
  - scripts/run.sh: "Execution script"
assets:
  - docs/deep-dive.md: "Extended documentation"
  - .agents/docs/topic/guide.md: "External reference doc"
---
```

**Required Fields**: `name`, `description`, `approved`, `created`, `license`, `metadata`, `tools`
**Optional Fields**: `files` (agent-facing), `assets` (user/informational)

## Approval Process

1. **Agent identifies knowledge gap** during specification review
2. **Agent researches** using search tools, documentation, internet
3. **Agent creates skill document** if no alternative exists
4. **Main Agent reviews** for accuracy and necessity
5. **User approves** before skill can be used
6. **Frontmatter updated** to `approved: Yes`
7. **Agent proceeds** with implementation using approved skill

## When to Create Skills

**Create skills ONLY when**:

- ✅ Fundamental understanding is missing for required task
- ✅ No existing skill covers the need
- ✅ No alternative approach possible with existing knowledge
- ✅ User hasn't provided clear alternative instructions

**DO NOT create skills for**:

- ❌ Simple tasks agents should already know
- ❌ Basic programming concepts
- ❌ Tasks solvable with quick research
- ❌ Trivial operations

## Using Skills

### For Agents

**Before starting work**:

1. Scan `.agents/skills/` directory (read frontmatter only)
2. Identify relevant skills by name and description
3. Check if skills are approved (`approved: Yes`)
4. Read full skill content when ready to use

**During work**:

1. Check Usage Type (TEMPLATE/EXECUTABLE/EDUCATIONAL)
2. Apply skill patterns following the rules for that type
3. Update specification frontmatter with skill reference

### For Users

**When new skill is created**:

1. Main Agent will report new skill creation
2. Review the skill document at `.agents/skills/[skill-name]/skill.md`
3. Approve or provide alternative approach
4. Skill is updated to `approved: Yes` upon approval

## Integration with Specifications

Specifications reference skills in frontmatter (one-way relationship):

```yaml
---
status: in-progress
skills:
  - playwright-web-interaction
  - jwt-authentication
---
```

## See Also

- **Rule 09**: Skills Creation and Review - Main Agent creating skills
- **Rule 11**: Skills Usage - Sub-agents using existing skills (concise)
- **Rule 06**: Specifications and Requirements - Skill integration
- **Templates**: `.agents/templates/skill-template.md`, `.agents/templates/skill-usage-examples.md`
