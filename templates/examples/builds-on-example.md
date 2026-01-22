# Builds On Example

This example shows how to create a new specification that builds upon a completed specification.

## When to Use `builds_on`

Use the `builds_on` field in requirements.md frontmatter when:
- Creating a new specification that enhances/extends a completed specification
- Adding features to work that was previously completed and verified
- Creating a follow-up specification that depends on previous work

## Key Principle

**Once a specification is completed** (status: completed, with FINAL_REPORT.md and VERIFICATION_SIGNOFF.md), it becomes **immutable** and represents historical fact.

Any new additions or changes MUST become a new specification that references the old one.

## Example Frontmatter

When creating a new specification that builds upon a completed specification:

```yaml
---
description: Add advanced caching layer with Redis support
status: in-progress
priority: high
created: 2026-01-22
author: Main Agent
metadata:
  version: "1.0"
  last_updated: 2026-01-22
  estimated_effort: medium
  tags:
    - caching
    - redis
    - enhancement
  stack_files:
    - .agents/stacks/rust.md
  skills: []
builds_on:
  - specifications/01-basic-caching-layer
related_specs:
  - specifications/03-database-optimization
has_features: false
has_fundamentals: false
---
```

## Field Descriptions

- **`builds_on`**: Array of parent specifications that this builds upon
  - Use relative paths from project root: `specifications/NN-spec-name`
  - Creates lineage chain for traceability
  - Multiple parents allowed if building on several completed specs

- **`related_specs`**: Array of related specifications (context only)
  - These are related but not dependencies
  - Provides additional context for understanding

## Why This Matters

### Historical Record
- Preserves complete history of requirements and implementations
- Know exactly what was done, when, and why

### Traceability
- Clear lineage showing how features evolved over time
- Can trace back through parent specifications

### Audit Trail
- Immutable record of what was completed
- New work clearly separated from old work

### No Confusion
- Prevents mixing old and new requirements
- Each specification has single clear purpose

## Workflow

When user requests additions to completed specification:

1. **Main Agent creates NEW specification** (next available number)
2. **New requirements.md includes `builds_on` field** referencing old spec
3. **New specification explains** how it builds upon the old one
4. **Old specification remains untouched** (historical record)

## Exception

Specifications that are **NOT completed** can be modified directly:
- Status is NOT "completed"
- No FINAL_REPORT.md exists
- Work is still ongoing
- In this case, update the existing specification rather than creating a new one
