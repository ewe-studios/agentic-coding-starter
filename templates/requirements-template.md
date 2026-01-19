---
description: Brief one-sentence description
status: in-progress
priority: medium
created: YYYY-MM-DD
author: "Main Agent"
metadata:
  version: "1.0"
  last_updated: YYYY-MM-DD
  estimated_effort: "medium"
  tags:
    - feature
builds_on: []
related_specs: []
---

# [Specification Name] - Requirements

## Overview
Brief summary of what this specification covers and why it's needed.

## Requirements Conversation Summary

### User's Initial Request
[Summary of what user initially asked for]

### Clarifying Questions Asked
1. Question about [topic]
   - Answer: [user's response]
2. Question about [topic]
   - Answer: [user's response]
[... all questions and answers ...]

### Final Requirements Agreement
Based on the conversation, we agreed on:
- [Clear statement of final understanding]
- [All important details confirmed]

## Detailed Requirements

### Functional Requirements
1. [Requirement 1]
2. [Requirement 2]

### Non-Functional Requirements
1. [Performance requirements]
2. [Security requirements]

### Technical Specifications
- **Technology Stack:** [Technologies to be used]
- **Dependencies:** [Required libraries/tools]
- **Integration Points:** [How this integrates]

### Success Criteria
- [ ] Criterion 1
- [ ] Criterion 2

## Module Documentation References

This specification modifies the following modules:

### [Module Name]
- **Documentation**: `documentation/[module]/doc.md`
- **Purpose**: [Brief summary]
- **Changes Needed**: [What will be changed]

**CRITICAL**: Agents MUST read module documentation BEFORE making changes.

## Agent Rules Reference

**MANDATORY**: All agents working on this specification MUST load the rules listed below.

**Rules Location**: `.agents/rules/`
**Stacks Location**: `.agents/stacks/`
**Skills Location**: `.agents/skills/`

### All Agents (Mandatory)

Load these rules from `.agents/rules/`:

| Rule | File | Purpose |
|------|------|---------|
| 01 | `.agents/rules/01-rule-naming-and-structure.md` | File naming conventions |
| 02 | `.agents/rules/02-rules-directory-policy.md` | Directory policies |
| 03 | `.agents/rules/03-dangerous-operations-safety.md` | Dangerous operations safety |
| 04 | `.agents/rules/04-work-commit-and-push-rules.md` | Work commit and push rules |

### By Agent Role

Load additional rules from `.agents/rules/` based on your role:

| Agent Type | Additional Rules to Load |
|------------|--------------------------|
| **Review Agent** | `.agents/rules/06-specifications-and-requirements.md` |
| **Implementation Agent** | `.agents/rules/13-implementation-agent-guide.md`, `.agents/rules/11-skills-usage.md` (if skills listed below), stack file |
| **Verification Agent** | `.agents/rules/08-verification-workflow-complete-guide.md`, stack file |
| **Documentation Agent** | `.agents/rules/06-specifications-and-requirements.md` |

### Stack Files

Load from `.agents/stacks/`:

- **Language**: [language] → `.agents/stacks/[language].md`

### Skills Referenced

Load from `.agents/skills/` if applicable:

[List any skills that agents should use, or "None" if not applicable]

---

## Important Notes for Agents

### Before Starting Work
- **MUST LOAD** rules listed in "Agent Rules Reference" section above
- **MUST READ** both requirements.md and tasks.md
- **MUST VERIFY** completion status by searching codebase
- **MUST UPDATE** tasks.md to reflect actual status
- **MUST ADD** new tasks BEFORE starting work

### Verification Requirements
Agents **MUST**:
1. Search codebase for relevant implementations
2. Verify code exists and works as specified
3. Update task status based on findings
4. Mark completed only when fully verified

---
*Created: [Date]*
*Last Updated: [Date]*
