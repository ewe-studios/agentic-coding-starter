# Skills Creation and Review (Main Agent Only)

## Purpose

This rule is for **Main Agent only** when creating new skills or reviewing skill documents. Sub-agents should load **Rule 11 (Skills Usage)** instead for a concise guide on using existing skills.

**Context Optimization**: This full rule (~900 lines) is only needed when creating/reviewing skills. Sub-agents using skills only need Rule 11 (~150 lines).

---

## Overview

This rule establishes a structured approach for agents to identify knowledge gaps, document required skills, and obtain user approval before using them. This ensures agents have proper guidance for complex tasks while maintaining user oversight and control.

## Core Principles

### 1. Skills as Knowledge Assets
Skills are documented know-how for accomplishing specific technical tasks. They capture:
- How to use specific tools/libraries (e.g., Playwright, Docker, Kubernetes)
- How to implement specific patterns (e.g., authentication flows, caching strategies)
- How to perform specific operations (e.g., database migrations, API integration)
- How to solve specific problems (e.g., performance optimization, security hardening)

### 2. User Approval Required
No skill can be used until the user approves it. This ensures:
- User maintains control over approaches and methodologies
- User can provide feedback or alternative solutions
- Agent doesn't proceed with incorrect or suboptimal approaches
- Critical skills are validated before implementation

### 3. Last Resort Only
Agents **MUST NOT** create skills casually. Skills are created ONLY when:
- ✅ Fundamental understanding is missing for a required task
- ✅ No existing skill covers the need
- ✅ No alternative approach is possible with existing knowledge
- ✅ User hasn't provided clear instructions for alternatives
- ❌ NOT for simple tasks agents should already know
- ❌ NOT for basic programming concepts
- ❌ NOT to avoid thinking or problem-solving

## Skills Directory Structure

### Location
```
.agents/skills/
├── playwright-web-interaction/
│   ├── skill.md (required - canonical documentation)
│   ├── learnings.md (optional - practical learnings from usage)
│   ├── browser-automation.js (executable script with arguments)
│   └── examples/
│       └── login-flow.js (pattern example)
├── kubernetes-deployment/
│   ├── skill.md (required)
│   ├── deploy.sh
│   └── configs/
│       └── deployment.yaml
└── [skill-name]/
    ├── skill.md (required - canonical)
    ├── learnings.md (optional - practical usage insights)
    ├── script1.py (optional)
    └── examples/ (optional)
```

### Naming Convention
- Skills use descriptive kebab-case names: `playwright-web-interaction`, `kubernetes-deployment`
- **NO numeric prefixes** - skills are referenced by name, not number
- Name should clearly describe the skill's purpose

### Three Usage Types for Skills

**1. TEMPLATE SKILLS** (copy ALL files to project and customize):
- skill.md explicitly states "Copy and adapt" or "Template-based"
- Agents copy ALL skill files (templates, helpers, configs) to project
- Agents customize the copied files for specific use case
- Original files in `.agents/skills/` remain untouched
- ✅ Agents modify the COPIED files in project directory
- ❌ **NEVER import from `.agents/skills/` into project code**

**2. EXECUTABLE SKILLS** (run as external tools):
- skill.md provides scripts designed to be executed
- Agents run scripts as external commands and consume output
- Scripts act as tools/utilities that return results
- Example: `node .agents/skills/web-scraper/scraper.js --url https://example.com`
- ✅ Execute scripts, capture output, use results
- ❌ Never modify script code
- ❌ Scripts are not part of project code - they're external tools

**3. EDUCATIONAL SKILLS** (learn pattern, implement in project):
- skill.md teaches concepts and shows example code
- skill.md references external libraries to install (NPM, PyPI, Cargo, etc.)
- Agents learn the pattern and implement fresh code in project
- Example: "Install `jsonwebtoken` from NPM, then implement JWT validation"
- ✅ Install external library as dependency
- ✅ Study skill examples
- ✅ Write fresh implementation in project following the pattern
- ❌ **NEVER import from `.agents/skills/` directory**

**CRITICAL RULE - Skills Directory Isolation**:
- ❌ **NEVER `import/require/use` from `.agents/skills/` in project code**
- ❌ `.agents/skills/` is NOT a code library
- ❌ `.agents/skills/` is NOT part of the project
- ✅ `.agents/skills/` is a knowledge base and tool collection
- ✅ Template files are COPIED to project then modified
- ✅ Executable scripts are RUN as external tools
- ✅ Educational content teaches you to use external libraries

**Critical Rules**:
- ✅ Agents MUST check **Usage Type** in skill.md for each skill
- ✅ Copy ALL files when skill is marked as TEMPLATE (including helpers)
- ✅ Execute scripts and consume output for EXECUTABLE skills
- ✅ Learn patterns and implement fresh for EDUCATIONAL skills
- ✅ Install external dependencies as specified (NPM, PyPI, etc.)
- ❌ NEVER modify original files in `.agents/skills/` directory
- ❌ NEVER import/require code from `.agents/skills/` into project
- ❌ Skills directory must remain completely isolated from project code

**If uncertain about usage**:
1. Check skill.md for explicit usage instructions
2. Look for phrases: "copy and adapt", "execute script", "install library X"
3. If still ambiguous, ask Main Agent for clarification

### skill.md vs learnings.md

**skill.md** (Canonical Documentation):
- ✅ **Primary source of truth** for skill knowledge
- ✅ Formal, structured documentation
- ✅ Research-based, validated information
- ✅ Updated through formal approval process
- ✅ Always read when skill is selected for use
- ✅ Contains: Overview, prerequisites, concepts, step-by-step guides, patterns, pitfalls

**learnings.md** (Practical Insights):
- ✅ **Practical knowledge from actual usage**
- ✅ Discoveries made while applying the skill
- ✅ Edge cases encountered in production
- ✅ Real-world adaptations that worked
- ✅ Updated by agents after using skill
- ✅ **Only read when skill is actively being used** (not during frontmatter scan)
- ✅ Contains: Quick tips, common issues, solutions that worked, anti-patterns

**Key Difference**:
- **skill.md** = What you need to know BEFORE using the skill
- **learnings.md** = What you learn AFTER using the skill

**When agents read each**:
```
Frontmatter scan (discovery):
  ✅ Read: skill.md frontmatter only
  ❌ Skip: learnings.md (not needed yet)

Skill selected for use:
  ✅ Read: skill.md (complete)
  ✅ Read: learnings.md (if exists)
  ✅ Read: relevant scripts

After implementation:
  ✅ Update: learnings.md with new insights
```

## Skill File Format

### Frontmatter (Required)
Every `skill.md` file **MUST** start with frontmatter:

```markdown
---
name: "Playwright Web Interaction"
description: "Guide for using Playwright to automate browser interactions, scraping, and testing"
approved: No
created: 2026-01-13
license: "MIT"
metadata:
  author: "Main Agent"
  version: "1.0"
  last_updated: "2026-01-13"
  tags:
    - web-automation
    - playwright
tools:
  - Playwright
  - TypeScript
files:
  - browser-automation.js: "Executable script for browser automation"
  - scraper-module.ts: "TypeScript module with reusable functions"
---
```

**REQUIRED Fields:**
- **`name`**: Clear, concise skill name (2-6 words)
- **`description`**: 1-2 sentence summary of what skill achieves and when to use it
- **`approved`**: `Yes` or `No` (defaults to `No` until user approves)
- **`created`**: Date skill was created (YYYY-MM-DD)
- **`license`**: License for the skill (MIT, Apache-2.0, GPL-3.0, Proprietary, etc.)
- **`metadata`**: Object with `author`, `version`, `last_updated`, `tags` (array, minimum 2 tags)
- **`tools`**: List of tools/technologies this skill covers

**OPTIONAL Fields:**
- **`files`**: Dictionary of attached files with descriptions (required if files exist)

### Skill Content Structure

See `.agents/templates/skill-template.md` for the complete skill.md template structure.

**Key sections**:
- Overview (2-3 paragraphs)
- When to Use This Skill (clear scope and limitations)
- Prerequisites (knowledge, dependencies, environment)
- **Attached Scripts and Code** (CRITICAL: clearly specify usage type)
- Core Concepts (key concepts needed)
- Step-by-Step Guide (detailed implementation)
- Common Patterns (frequently used patterns)
- Pitfalls to Avoid (common mistakes)
- Examples (real-world scenarios)
- Script Reference (quick lookup table)
- References (documentation links)

**Usage Type Documentation (CRITICAL)**:

Each skill MUST clearly state its usage type at the beginning of the "Attached Scripts and Code" section:

**TEMPLATE Example**:
```markdown
**Skill Usage Type**: TEMPLATE - Copy all files to project and customize

### Template: api-client.ts
**Usage**: COPY to your project and customize for your API

**Instructions**:
1. Copy: `cp api-client.ts src/clients/your-api-client.ts`
2. Copy helpers: `cp http-helpers.ts src/clients/http-helpers.ts`
3. Customize for your API
4. Import from project: `import { ApiClient } from './clients/your-api-client';`
5. NEVER import from `.agents/skills/` in project code
```

**EXECUTABLE Example**:
```markdown
**Skill Usage Type**: EXECUTABLE - Run scripts as external tools

### Script: scraper.js
**Usage**: EXECUTE as external command

```bash
node scraper.js --url <URL> --selector <CSS_SELECTOR>
```

**Output**: JSON data to stdout or file
```

**EDUCATIONAL Example**:
```markdown
**Skill Usage Type**: EDUCATIONAL - Learn pattern and implement fresh

**External Dependencies**:
- Install: `npm install jsonwebtoken`

### Example: jwt-example.ts
**Usage**: STUDY this example, then IMPLEMENT fresh code using `jsonwebtoken`

Study the pattern, install the library, write your own implementation.
NEVER import from `.agents/skills/` directory.
```

### learnings.md Format

See `.agents/templates/learnings-template.md` for the complete template.

**Key sections**:
- Quick Tips (1-line actionable tips)
- Common Issues Encountered (brief issue + solution)
- Adaptations That Worked (modifications with context)
- Anti-Patterns Discovered (what NOT to do)
- Production Gotchas (unexpected behaviors)
- Performance Insights (metrics-backed recommendations)
- Edge Cases (unusual scenarios)
- Integration Notes (system/tool integration issues)

**Format Guidelines**:
- ✅ Keep entries VERY concise (1-3 lines each)
- ✅ Focus on actionable insights
- ✅ Include context (when/where it matters)
- ❌ No lengthy explanations (belongs in skill.md)
- ❌ No obvious information
- ❌ No duplicating what's in skill.md

## Workflow: Skill Identification and Creation

### Phase 1: Skill Need Identification (Sub-Agent)

When a sub-agent reviews a specification and encounters a knowledge gap:

1. ✅ **Think deeply**: Can this be solved with existing knowledge?
2. ✅ **Check existing skills**: Scan `.agents/skills/` directory frontmatter
3. ✅ **Search for information**: Use search tool to understand the concept
4. ✅ **Consult internet**: Research best practices, official docs, tutorials
5. ✅ **Evaluate alternatives**: Are there other ways to achieve the task?

**Decision Tree**:
```
Do I understand how to accomplish this task?
├─ YES → Proceed with implementation
└─ NO → Can I find existing skill that covers this?
          ├─ YES → Use existing skill (if approved)
          └─ NO → Can I learn from quick research?
                    ├─ YES → Research, understand, proceed
                    └─ NO → Must create new skill
```

### Phase 2: Skill Creation (Sub-Agent)

**ONLY** if skill creation is necessary:

1. ✅ **Research thoroughly**: Official docs, multiple sources, code examples

2. ✅ **Create skill directory**:
   ```bash
   mkdir -p .agents/skills/[skill-name]
   ```

3. ✅ **Create supporting scripts and code** (if applicable):
   - Write executable scripts with clear argument interfaces
   - Design scripts to accept parameters for different use cases
   - Create reusable code modules with exported functions
   - Build working examples demonstrating patterns
   - Test scripts thoroughly

4. ✅ **Write skill.md**:
   - Start with complete frontmatter (`approved: No`)
   - **Clearly state Usage Type** (TEMPLATE/EXECUTABLE/EDUCATIONAL)
   - Include `files` field listing all attached scripts
   - Document overview, prerequisites, concepts
   - Add "Attached Scripts and Code" section explaining each file
   - **Make instructions clear and unambiguous** (will be verified at 2 checkpoints)
   - Provide step-by-step guide referencing scripts
   - Include common patterns and pitfalls
   - Add "Script Reference" table for quick lookup
   - **CRITICAL**: Ensure clarity for Main Agent verification (Checkpoint 1) and Sub-Agent usage (Checkpoint 2)

5. ✅ **Document how agents use scripts**:
   - Clearly define all script arguments and parameters
   - Show execution examples with different argument combinations
   - Document function signatures for importable modules
   - Explain return values and output formats

6. ✅ **Report to Main Agent**:
   ```
   "Created new skill: [skill-name]
   Location: .agents/skills/[skill-name]/skill.md
   Reason: [Why this skill was necessary]
   Research sources: [Links to documentation/resources used]

   Attached files:
   - browser-automation.js: Executable script for automation
   - examples/login-flow.js: Login flow pattern example

   Ready for review and user approval."
   ```

### Phase 3: Skill Review (Main Agent)

When Main Agent receives skill creation report:

1. ✅ **Read the skill document**: Verify frontmatter complete, content comprehensive

2. ✅ **Review attached scripts** (if applicable): Check functionality, safety, best practices

3. ✅ **Validate skill makes sense**: Use search tool to verify accuracy, check approach

4. ✅ **Assess necessity**: Is this skill truly needed?

5. ✅ **Report to user**:
   ```
   "New skill documented: [skill-name]

   Location: .agents/skills/[skill-name]/skill.md
   Purpose: [What the skill achieves]
   Reason needed: [Why sub-agent needed this]

   Attached scripts:
   - browser-automation.js: Executable automation script
   - examples/login-flow.js: Login pattern example

   Please review and approve to proceed.
   To approve, I will update: approved: No → approved: Yes"
   ```

### Phase 4: User Approval

**If Approved**:
- Main Agent updates frontmatter: `approved: Yes`
- Sub-agent proceeds with implementation
- Skill is now available for future use

**If Rejected or Alternative Provided**:
- User provides feedback or alternative approach
- Main Agent communicates alternative to sub-agent
- Sub-agent implements using user's guidance

**If Needs Revision**:
- User provides specific feedback
- Sub-agent updates skill document
- Process returns to Phase 3

## Skill Clarity Verification (MANDATORY)

Skills must be clear and understandable at two critical checkpoints:

### Checkpoint 1: During Requirements Creation (Main Agent)

**When**: After requirements.md is created and skills are listed in frontmatter

**Main Agent MUST**:

1. ✅ **Review each listed skill thoroughly**:
   - Read complete skill.md content
   - Verify usage type is clearly stated (TEMPLATE/EXECUTABLE/EDUCATIONAL)
   - Confirm instructions are clear and unambiguous
   - Validate that examples are understandable

2. ✅ **Verify clarity for each usage type**:
   - **TEMPLATE**: Are ALL files that need copying clearly listed? Are customization instructions clear?
   - **EXECUTABLE**: Are command-line arguments documented? Are return values/output formats specified?
   - **EDUCATIONAL**: Are external dependencies listed? Are installation commands provided?

3. ✅ **Document verification in requirements.md**:
   ```markdown
   ## Skills Clarity Verification

   **Verified by Main Agent**: [Date]

   All listed skills reviewed for clarity:
   - [skill-name]: TEMPLATE - Clear (all files listed, customization documented)
   - [skill-name]: EXECUTABLE - Clear (arguments documented, output specified)
   ```

4. ✅ **If ANY skill is unclear**:
   ```
   Main Agent to User:
   "During requirements review, I found unclear skills:

   Skill: [skill-name]
   Issue: [Specific clarity problem]
   - Missing: [What's not clear]
   - Ambiguous: [What needs clarification]

   Options:
   1. I can update the skill with improvements
   2. You can update the skill directly
   3. We can remove this skill and use alternative

   Requirements cannot proceed until all skills are clear."
   ```

**Main Agent MUST NOT**:
- ❌ Approve requirements with unclear skills
- ❌ Let sub-agents proceed with ambiguous skills
- ❌ Skip skill clarity verification

### Checkpoint 2: Before Skill Usage (Sub-Agent)

**When**: Sub-agent is about to use a skill for implementation

**Sub-Agent MUST**:

1. ✅ **Perform initial clarity check**:
   - Read skill.md completely
   - Read learnings.md if it exists
   - Verify understanding of usage type
   - Confirm all instructions are clear

2. ✅ **Validate understanding**:
   ```
   Self-check questions:
   - Do I understand what type of skill this is?
   - Do I know exactly which files to copy/execute/study?
   - Are the steps clear and unambiguous?
   - Are there any confusing or contradictory instructions?
   ```

3. ✅ **If skill is CLEAR**: Proceed with usage

4. ✅ **If skill is UNCLEAR**: Report to Main Agent immediately
   ```
   Sub-agent to Main Agent:
   "Cannot proceed with skill: [skill-name]

   Clarity Issue: [Specific problem]
   - What's unclear: [Detailed explanation]
   - Why it's blocking: [Impact on implementation]
   - What's needed: [What would make it clear]

   Request: Please review and clarify before I proceed."
   ```

**Why Two Checkpoints**:
- **Checkpoint 1** (Requirements): Catch obvious clarity issues early
- **Checkpoint 2** (Implementation): Catch issues that only appear during actual usage
- Skills may change between requirements creation and implementation
- Different perspectives reveal different clarity issues

### Phase 5: Skill Usage (Sub-Agent)

Once skill is approved AND verified as clear:

1. ✅ **Reference skill in specification**: Update frontmatter with skill list

2. ✅ **Apply skill during implementation**:
   - Check skill usage type first
   - Follow documented patterns

**Type 1: TEMPLATE Skills** (Copy all files and customize)
```bash
# Step 1: Copy ALL skill files to project
cp .agents/skills/rest-api-client/api-client.ts ./src/clients/product-api.ts
cp .agents/skills/rest-api-client/http-helpers.ts ./src/clients/http-helpers.ts

# Step 2: Customize the COPIED files in project

# Step 3: Use in project (import from project location)
import { ProductApi } from './clients/product-api';
```

**CRITICAL for TEMPLATE**:
- ✅ Copy ALL files (templates, helpers, everything)
- ✅ Customize copied files in project
- ❌ **NEVER import from `.agents/skills/` in project code**

**Type 2: EXECUTABLE Skills** (Run as external tools)
```bash
# Execute script from .agents/skills/ location
node .agents/skills/web-scraper/scraper.js \
  --url "https://example.com/products" \
  --output ./data/products.json

# Use output in project code
import fs from 'fs';
const products = JSON.parse(fs.readFileSync('./data/products.json'));
```

**CRITICAL for EXECUTABLE**:
- ✅ Execute scripts from `.agents/skills/` location
- ✅ Consume script output in project code
- ❌ Never copy or modify executable scripts

**Type 3: EDUCATIONAL Skills** (Learn and implement fresh)
```bash
# Step 1: Install external dependencies
npm install jsonwebtoken bcrypt

# Step 2: Study the skill examples
# Read: .agents/skills/auth/jwt-example.ts

# Step 3: Implement FRESH code in project
import jwt from 'jsonwebtoken';  // From NPM, NOT .agents/skills/

export class AuthService {
  generateToken(userId: string): string {
    return jwt.sign({ userId }, this.secret, { expiresIn: '24h' });
  }
}
```

**CRITICAL for EDUCATIONAL**:
- ✅ Install external libraries (NPM, PyPI, Cargo)
- ✅ Study skill examples to learn patterns
- ✅ Write fresh implementation in project
- ❌ **NEVER import from `.agents/skills/` directory**

## Skill Updates and Maintenance

### When to Update

**skill.md** (requires user approval):
- ✅ Fundamental approach changes
- ✅ Better pattern discovered
- ✅ Official documentation updated
- ✅ Scripts added or modified significantly
- ❌ NOT for one-off issues (use learnings.md)
- ❌ NOT for project-specific adaptations

**learnings.md** (requires user approval):
- ✅ Discovered new edge case
- ✅ Found better workaround
- ✅ Encountered production issue
- ✅ Learned performance optimization

**Scripts** (requires user approval):
- ✅ Bug fixes
- ✅ Improvements
- ✅ New functionality

### Skill Update Workflow

**For skill.md or script changes**:
1. Agent identifies need for update
2. Agent creates/modifies files
3. Agent reports to Main Agent (if Sub-Agent)
4. Main Agent reviews changes
5. Main Agent reports to user
6. User approves or requests changes
7. Main Agent confirms update

**For learnings.md updates**:
1. Agent discovers practical insight
2. Agent updates learnings.md (append to section)
3. Agent reports to Main Agent
4. Main Agent reviews and reports to user
5. User approves
6. Learning is retained

### Update Approval Requirements

**MANDATORY approval needed for**:
- ✅ Any change to skill.md content
- ✅ Any change to scripts or code
- ✅ Any change to learnings.md
- ✅ Adding/removing files

**Approval process**:
1. Agent makes changes
2. Main Agent reviews (validates correctness)
3. User reviews and approves
4. Changes are finalized

**Main Agent MUST validate**:
- Information is accurate (verify with search)
- Changes improve the skill
- No introduction of errors
- Code is functional and tested (for scripts)
- No hardcoded secrets
- Follows best practices

## Specification Integration

Specifications **MUST** list required skills in frontmatter:

```markdown
---
status: in-progress
priority: high
skills:
  - playwright-web-interaction
  - jwt-authentication
tools:
  - TypeScript
---
```

**Direction of reference**:
- ✅ Specifications reference skills (one-way relationship)
- ✅ Skills are independent documents
- ❌ Skills do NOT track which specs use them

## Skill Scanning (Agents)

### Efficient Skill Discovery

Agents **MUST**:

1. ✅ **Scan only frontmatter** initially:
   ```bash
   for skill in .agents/skills/*/skill.md; do
     head -n 15 "$skill"  # Read just frontmatter
   done
   ```

2. ✅ **Match by name and description**: Use to identify relevant skills

3. ✅ **Check approval status**: Only consider skills with `approved: Yes`

4. ✅ **Read full content only when using**: Once selected, read complete document

## Enforcement and Violations

### Sub-Agent Requirements

Sub-agents **MUST**:
- ✅ Think deeply before creating skills
- ✅ Exhaust research and alternatives first
- ✅ Create comprehensive, accurate skill documents
- ✅ Test all scripts before reporting
- ✅ Clearly mark skill with **Usage Type**
- ✅ Check Usage Type before using any skill
- ✅ **Perform clarity check before using any skill** (Checkpoint 2)
- ✅ **Report unclear skills to Main Agent immediately**
- ✅ Copy ALL files for TEMPLATE skills
- ✅ Execute EXECUTABLE scripts as external tools
- ✅ Study EDUCATIONAL skills and implement fresh
- ✅ Install external dependencies for EDUCATIONAL skills
- ✅ Import from PROJECT locations, NEVER from `.agents/skills/`
- ✅ Report skill creation to Main Agent
- ✅ Never use unapproved skills
- ✅ Update specifications with skill references
- ✅ Update learnings.md after using skill (with approval)

Sub-agents **MUST NOT**:
- ❌ Create skills for trivial or known tasks
- ❌ Skip research when creating skills
- ❌ Use skills before user approval
- ❌ Proceed with unapproved skills without reporting
- ❌ Read learnings.md during frontmatter scan
- ❌ Update skill.md or scripts without approval
- ❌ **Proceed with unclear skills without reporting** (CRITICAL)
- ❌ **Import from `.agents/skills/` in project code** (CRITICAL)
- ❌ Copy only some files from TEMPLATE skills (must copy ALL)
- ❌ Modify EXECUTABLE scripts (run as external tools only)
- ❌ Import skill example code for EDUCATIONAL skills
- ❌ Modify original files in `.agents/skills/` directory

### Main Agent Requirements

Main Agent **MUST**:
- ✅ Review all skill documents thoroughly
- ✅ Review all attached scripts for safety
- ✅ Verify skill has clear **Usage Type**
- ✅ **Verify ALL skills for clarity during requirements** (Checkpoint 1)
- ✅ **Document skill clarity verification in requirements.md**
- ✅ **Block requirements if any skill is unclear**
- ✅ **Report unclear skills to user with specific issues**
- ✅ **Relay sub-agent clarity concerns to user** (Checkpoint 2)
- ✅ Validate skill accuracy using search/internet
- ✅ Report new skills to user for approval
- ✅ Report skill updates to user for approval
- ✅ Update frontmatter when user approves
- ✅ Block implementation until approval received

Main Agent **MUST NOT**:
- ❌ Approve skills without user consent
- ❌ Skip validation of skill content
- ❌ Allow sub-agents to use unapproved skills
- ❌ Allow skill updates without user approval
- ❌ **Skip skill clarity verification during requirements** (CRITICAL)
- ❌ **Approve requirements with unclear skills** (CRITICAL)
- ❌ **Ignore sub-agent clarity concerns** (CRITICAL)
- ❌ Allow sub-agents to import from `.agents/skills/` in project

### Critical Violations

**Serious violations**:
- Using unapproved skills
- Creating skills for trivial tasks
- Skipping research
- Not reporting to Main Agent
- Updating without approval
- **Skipping skill clarity verification** (CRITICAL - Main Agent)
- **Approving requirements with unclear skills** (CRITICAL - Main Agent)
- **Proceeding with unclear skills** (CRITICAL - Sub-Agent)
- **Importing from `.agents/skills/` in project code** (CRITICAL - breaks isolation)
- **Partial copying of TEMPLATE skills** (must copy ALL)
- **Modifying EXECUTABLE scripts** (breaks reusability)
- **Importing EDUCATIONAL examples** (should implement fresh)
- **Modifying original files in `.agents/skills/`** (corrupts shared resources)

**If agent needs unapproved skill**:
```
Sub-agent: "Cannot proceed. Required skill not approved:
  Skill: .agents/skills/kubernetes-deployment/skill.md
  Status: approved: No
  Reason needed: [Explanation]

Awaiting user approval to continue."
```

## Examples

### Example 1: Using TEMPLATE Skill

**Scenario**: Create custom API client from template

```markdown
# Task: Create REST API client for product service

1. Find skill: "rest-api-client" - approved: Yes
2. Read skill.md: States "TEMPLATE - Copy all files to project"
3. Files listed:
   - api-client.ts (template)
   - http-helpers.ts (helper)
   - retry-logic.ts (helper)

# Execution:
# Step 1: Copy ALL files
cp .agents/skills/rest-api-client/api-client.ts ./src/clients/product-api.ts
cp .agents/skills/rest-api-client/http-helpers.ts ./src/clients/http-helpers.ts
cp .agents/skills/rest-api-client/retry-logic.ts ./src/clients/retry-logic.ts

# Step 2: Customize COPIED files for product API

# Step 3: Import from project locations
import { ProductApi } from './clients/product-api';
import { handleError } from './clients/http-helpers';

# Result: Type-safe API client, no imports from .agents/skills/
```

### Example 2: Using EXECUTABLE Skill

**Scenario**: Scrape product data from website

```markdown
# Task: Extract product information

1. Find skill: "web-scraper" - approved: Yes
2. Read skill.md: States "EXECUTABLE - Run as external tool"
3. Script: scraper.js with --url and --selector arguments

# Execution:
node .agents/skills/web-scraper/scraper.js \
  --url "https://example.com/products" \
  --selector ".product-item" \
  --output ./data/products.json

# In project code:
import fs from 'fs';
const products = JSON.parse(fs.readFileSync('./data/products.json'));

# Result: Data extracted, script remains external tool
```

## Integration with Other Rules

- **Rule 05 (Agent Orchestration)**: Skills identified during "Before Starting Work" phase
- **Rule 06 (Specifications)**: Specifications frontmatter includes `skills` field
- **Rule 07 (Language Conventions)**: Skills may be stack-specific
- **Rule 11 (Skills Usage)**: Sub-agents load this concise guide for using skills

## Benefits

**For Agents**:
- ✅ Clear guidance for complex tasks
- ✅ Reusable knowledge across projects
- ✅ Structured approach to learning
- ✅ Reduced errors from misunderstanding

**For Users**:
- ✅ Control over methodologies used
- ✅ Visibility into agent knowledge gaps
- ✅ Opportunity to provide better approaches
- ✅ Building reusable knowledge base

**For Projects**:
- ✅ Documented institutional knowledge
- ✅ Consistency across implementations
- ✅ Faster onboarding for new work
- ✅ Traceable decisions and approaches

## Summary

**Core Workflow**:
```
Need identified → Research → Check existing skills →
Create if necessary → Main Agent review → User approval →
Update approved: Yes →
Checkpoint 1: Main Agent verifies clarity during requirements →
Document verification in requirements.md →
Checkpoint 2: Sub-Agent verifies clarity before usage →
Implementation proceeds →
Update learnings.md with insights
```

**Key Principles**:
- Skills are last resort (think deeply first)
- User approval is mandatory (creation AND updates)
- Research thoroughly before creating
- Scan efficiently (frontmatter only, skip learnings.md)
- Never proceed with unapproved skills
- skill.md is canonical (formal documentation)
- learnings.md captures practical insights (read when using)
- **Two mandatory clarity checkpoints** (Requirements + Before Usage)
- **Three Usage Types**: TEMPLATE (copy all), EXECUTABLE (run as tool), EDUCATIONAL (learn and implement)
- **`.agents/skills/` is ISOLATED** - never import from it in project code

**Three Usage Types**:
1. **TEMPLATE**: Copy ALL files (template + helpers) to project, customize copied files
2. **EXECUTABLE**: Run scripts from `.agents/skills/` as external tools
3. **EDUCATIONAL**: Study examples, install external libraries, implement fresh

**CRITICAL - Skills Directory Isolation**:
- ❌ **NEVER `import/require/use` from `.agents/skills/` in project code**
- ✅ `.agents/skills/` is knowledge base + tool collection, NOT a code library
- ✅ TEMPLATE: Copy to project, then import from project locations
- ✅ EXECUTABLE: Run as external tools
- ✅ EDUCATIONAL: Implement fresh using external libraries

**Critical Rules**:
- ❌ No skill usage without approval
- ❌ No casual skill creation
- ❌ No skill updates without approval
- ❌ Do NOT read learnings.md during frontmatter scan
- ❌ **NEVER skip clarity verification** (both checkpoints mandatory)
- ❌ **NEVER proceed with unclear skills**
- ❌ **NEVER import from `.agents/skills/` in project code** (CRITICAL)
- ❌ **NEVER partially copy TEMPLATE skills** (must copy ALL)
- ❌ **NEVER modify EXECUTABLE scripts** (run as external tools)
- ❌ **NEVER import EDUCATIONAL examples** (implement fresh)
- ❌ **NEVER modify original files in `.agents/skills/`**
- ✅ Always report to Main Agent
- ✅ Always update specifications with skill references
- ✅ Always validate before approval
- ✅ Always research thoroughly
- ✅ Always test scripts before submitting
- ✅ Update learnings.md after gaining insights (with approval)
- ✅ **Checkpoint 1: Main Agent verifies clarity during requirements**
- ✅ **Checkpoint 2: Sub-Agent verifies clarity before usage**
- ✅ **Report unclear skills immediately to user**
- ✅ **Check Usage Type before using any skill**
- ✅ **TEMPLATE: Copy ALL files to project and customize**
- ✅ **EXECUTABLE: Run scripts as external tools**
- ✅ **EDUCATIONAL: Install external libs, implement fresh**

**Documentation Hierarchy**:
1. **skill.md** = Canonical truth (what to know BEFORE using)
2. **learnings.md** = Practical wisdom (what you learn AFTER using)
3. **Files** = Implementation support (how to DO it)
   - **TEMPLATE**: Copy ALL to project and customize
   - **EXECUTABLE**: Run as external tool
   - **EDUCATIONAL**: Learn pattern, implement with external libraries

**Template References**:
- Complete skill.md structure: `.agents/templates/skill-template.md`
- Complete learnings.md structure: `.agents/templates/learnings-template.md`

---
*Created: 2026-01-13*
*Last Updated: 2026-01-19 (Split into Rule 09 for creation and Rule 11 for usage)*

---

## Related Rules

- **Rule 11 (Skills Usage)**: Concise guide for sub-agents using existing skills
- **Rule 06 (Specifications)**: How specifications reference skills
- **Rule 05 (Agent Orchestration)**: How skills are identified during work
