# Skills Usage Guide (For Sub-Agents)

## Purpose

This is a **concise guide for sub-agents** who need to use existing skills. For skill creation and review, Main Agent should load Rule 09 instead.

---

## Quick Reference

### Finding Skills

1. **Scan `.agents/skills/` directory**
2. **Read only frontmatter** of `skill.md` files (first 20 lines)
3. **Check `approved: Yes`** before using
4. **Match by name/description** to task needs

```bash
# Efficient scan - frontmatter only
for skill in .agents/skills/*/skill.md; do
  head -n 20 "$skill"
done
```

### Before Using a Skill

1. ✅ Verify `approved: Yes` in frontmatter
2. ✅ Read complete `skill.md`
3. ✅ Read `learnings.md` if it exists
4. ✅ Check **Usage Type** (TEMPLATE/EXECUTABLE/EDUCATIONAL)
5. ✅ Read relevant files from `templates/`, `scripts/`, or `examples/`
6. ✅ Perform clarity check - do you understand all instructions?

---

## Skill Directory Structure

```
.agents/skills/[skill-name]/
├── skill.md        # Required - main doc (always read)
├── learnings.md    # Optional - practical insights (read when using)
├── assets/         # Optional - diagrams, configs, sample data
├── docs/           # Optional - extended user documentation
├── templates/      # Optional - for TEMPLATE skills
├── scripts/        # Optional - for EXECUTABLE skills
└── examples/       # Optional - for EDUCATIONAL skills
```

---

## Three Skill Usage Types

### 1. TEMPLATE Skills (Copy and Customize)

**Identified by**: `Usage Type: TEMPLATE` in skill.md

**What to do**:
```bash
# Copy ALL files from templates/ to project
cp .agents/skills/[skill-name]/templates/*.ts ./src/[destination]/

# Customize the COPIED files
# Import from PROJECT location, NOT .agents/skills/
```

**Rules**:
- ✅ Copy ALL files from `templates/`
- ✅ Customize copied files in project
- ✅ Import from project location
- ❌ **NEVER import from `.agents/skills/` in project code**

### 2. EXECUTABLE Skills (Run as Tools)

**Identified by**: `Usage Type: EXECUTABLE` in skill.md

**What to do**:
```bash
# Run script from scripts/ directory
node .agents/skills/[skill-name]/scripts/run.js --arg value

# Consume output in your project
```

**Rules**:
- ✅ Execute scripts from `scripts/` location
- ✅ Capture and use output
- ❌ Never copy or modify scripts
- ❌ Never import from `.agents/skills/`

### 3. EDUCATIONAL Skills (Learn and Implement)

**Identified by**: `Usage Type: EDUCATIONAL` in skill.md

**What to do**:
```bash
# Install external dependencies listed in skill
npm install [package-name]

# Study examples in examples/, then write FRESH code
```

**Rules**:
- ✅ Install external libraries (NPM, PyPI, Cargo)
- ✅ Study skill examples to learn patterns
- ✅ Write fresh implementation in project
- ❌ **NEVER import from `.agents/skills/` directory**

---

## CRITICAL: Skills Directory Isolation

```
❌ NEVER do this in project code:
   import { something } from '.agents/skills/...'
   require('.agents/skills/...')

✅ ALWAYS do this:
   - TEMPLATE: Copy from templates/, import from project
   - EXECUTABLE: Run from scripts/, use output
   - EDUCATIONAL: Install lib, write fresh code
```

---

## When Skill is Unclear

If you don't understand the skill instructions:

1. **STOP immediately**
2. **Report to Main Agent**:
   ```
   Cannot proceed with skill: [skill-name]

   Clarity Issue: [Specific problem]
   - What's unclear: [Detailed explanation]
   - Why it's blocking: [Impact on implementation]
   - What's needed: [What would make it clear]

   Request: Please review and clarify before I proceed.
   ```
3. **Wait for clarification** before proceeding

---

## When Skill is Unapproved

If skill has `approved: No`:

```
Cannot proceed. Required skill not approved:
  Skill: .agents/skills/[skill-name]/skill.md
  Status: approved: No
  Reason needed: [Explanation]

Awaiting user approval to continue.
```

**NEVER use unapproved skills.**

---

## After Using a Skill

If you discover useful insights while using the skill:

1. ✅ Note the insight
2. ✅ Report to Main Agent
3. ✅ Main Agent will coordinate learnings.md update (requires user approval)

---

## Sub-Agent Checklist

Before using a skill:
- [ ] Skill exists in `.agents/skills/[skill-name]/`
- [ ] Frontmatter has `approved: Yes`
- [ ] Read complete `skill.md`
- [ ] Read `learnings.md` if exists
- [ ] Understand Usage Type (TEMPLATE/EXECUTABLE/EDUCATIONAL)
- [ ] Read relevant subdirectory files (`templates/`, `scripts/`, `examples/`)
- [ ] Instructions are clear (if not, report to Main Agent)

During skill usage:
- [ ] Follow Usage Type rules exactly
- [ ] Never import from `.agents/skills/` in project code
- [ ] Copy ALL files from `templates/` for TEMPLATE skills
- [ ] Run from `scripts/` for EXECUTABLE skills
- [ ] Install external libs for EDUCATIONAL skills

---

## Summary

| Usage Type | Source Directory | Action | Import From |
|------------|------------------|--------|-------------|
| TEMPLATE | `templates/` | Copy ALL files, customize | Project location |
| EXECUTABLE | `scripts/` | Run script, use output | N/A (external tool) |
| EDUCATIONAL | `examples/` | Install lib, write fresh | External package |

**Golden Rule**: `.agents/skills/` is a knowledge base, NOT a code library. Never import from it in project code.

**See**: `.agents/templates/skill-usage-examples.md` for detailed code examples.

---

*Created: 2026-01-19*
*Last Updated: 2026-01-20 (Updated for new directory structure)*
