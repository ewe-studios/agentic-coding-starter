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
  - file1.ext: "Brief description"
  - file2.ext: "Brief description"
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

## Attached Scripts and Code

**IMPORTANT**: Clearly specify the skill's usage type and how agents should use the files.

### Skill Usage Type

Choose ONE of the following:

**TEMPLATE** - Copy all files to project and customize
**EXECUTABLE** - Run scripts as external tools
**EDUCATIONAL** - Learn pattern and implement fresh

### File Documentation

For each file, document:
- **Purpose**: What the file does
- **Language**: Programming language
- **Usage**: How agents should use it

**Example for TEMPLATE**:
```
### Template: api-client.ts
**Purpose**: Main API client implementation template
**Language**: TypeScript
**Usage**: COPY to your project and customize

Instructions:
1. Copy to project: `cp api-client.ts src/clients/your-api-client.ts`
2. Customize for your specific API
3. Import from project location (NOT from .agents/skills/)
```

**Example for EXECUTABLE**:
```
### Script: scraper.js
**Purpose**: Web scraping utility
**Language**: JavaScript
**Usage**: EXECUTE as external command

Usage: `node scraper.js --url <URL> --selector <SELECTOR>`

Arguments:
- --url: Target URL (required)
- --selector: CSS selector (required)
```

**Example for EDUCATIONAL**:
```
### Example: jwt-example.ts
**Purpose**: Demonstrates JWT authentication pattern
**Language**: TypeScript
**Usage**: STUDY this example, implement fresh in project

External Dependencies:
- npm install jsonwebtoken
- npm install @types/jsonwebtoken --save-dev
```

## Core Concepts
Key concepts needed to understand this skill:
- Concept 1: Explanation
- Concept 2: Explanation
- Concept 3: Explanation

## Step-by-Step Guide

### Step 1: [First Step Name]
Detailed explanation with code examples.

### Step 2: [Second Step Name]
Detailed explanation with code examples.

[Continue for all steps...]

## Common Patterns
Frequently used patterns when applying this skill:
- Pattern 1: When and how to use
- Pattern 2: When and how to use

## Pitfalls to Avoid
Common mistakes and how to avoid them:
- Pitfall 1: What to avoid and why
- Pitfall 2: What to avoid and why

## Examples

### Example 1: [Scenario Name]
```[language]
// Code example
```

**How agents use this**:
1. Understand the pattern
2. Apply to specific use case
3. Do NOT modify examples without approval

### Example 2: [Another Scenario]
```[language]
// Code example
```

## Script Reference

| Script | Language | Usage Type | Purpose | How to Use |
|--------|----------|------------|---------|------------|
| file1.ext | Lang | TYPE | Purpose | Brief instructions |
| file2.ext | Lang | TYPE | Purpose | Brief instructions |

## References
- Official documentation links
- Tutorials used
- Stack Overflow discussions
- Blog posts or guides

---
*Created: [Date]*
*Last Updated: [Date]*
