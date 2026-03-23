---
# This template has THREE variants depending on specification structure
# Choose the appropriate variant based on has_features flag
---

# VARIANT 1: Feature-Based Specification (has_features: true) - Spec Level

Use this at the specification root level (specifications/NN-spec/start.md)
For feature-based specs that delegate to individual features.

See example: .agents/templates/examples/start-specification-example.md

---
workspace_name: "ewe_platform"
spec_directory: "specifications/[NN-spec-name]"
this_file: "specifications/[NN-spec-name]/start.md"
created: YYYY-MM-DD
---

# Start: [Specification Name]

## Agent Workflow

1. Read `requirements.md` (high-level overview + feature index)
2. **Identify language stack** from requirements.md "Language Stack" section - document languages clearly
3. **Read/generate language skills** - for each language in stack:
   - If skill exists: `.agents/skills/[language]-clean-code/skill.md` → read it
   - If skill missing: launch agent to generate one first, then read it
   - Add this item to the workflow list so future agents remember
4. Read `LEARNINGS.md` (past discoveries and mistakes)
5. Identify which feature you're working on from the Feature Index
6. Navigate to `features/[feature-name]/start.md` and follow that feature's workflow
7. **ALWAYS UPDATE LEARNINGS.md** after each completed task/milestone

---

**Workflow:** Requirements → Select Feature → Feature start.md → Follow feature workflow

---

_Created: YYYY-MM-DD_

================================================================================

# VARIANT 2: Feature-Based Specification (has_features: true) - Feature Level

Use this at the feature level (specifications/NN-spec/features/[feature]/start.md)
For individual features within a feature-based specification.

See example: .agents/templates/examples/start-feature-example.md

---
workspace_name: "ewe_platform"
spec_directory: "specifications/[NN-spec-name]"
feature_directory: "specifications/[NN-spec-name]/features/[feature-name]"
this_file: "specifications/[NN-spec-name]/features/[feature-name]/start.md"
created: YYYY-MM-DD
---

# Start: [Feature Name] Feature

## Agent Workflow

1. Read `feature.md` (detailed requirements + tasks)
2. **Identify language stack** from feature.md or parent requirements.md - document languages clearly
3. **Read/generate language skills** - for each language in stack:
   - If skill exists: `.agents/skills/[language]-clean-code/skill.md` → read it
   - If skill missing: launch agent to generate one first, then read it
   - Add this item to the workflow list so future agents remember
4. Read `../../LEARNINGS.md` (past discoveries and mistakes)
5. Read `./VERIFICATION.md` (verification requirements)
6. Read `./PROGRESS.md` (last progress of this feature if present)
7. Read `.agents/AGENTS.md` to identify your agent type
8. Read your agent file in `.agents/agents/[agent-name].md`
9. Read skills specified in your agent documentation
10. **MANDATORY**: Generate `compacted.md` with all info using `.agents/skills/context-compaction/skill.md`
11. Clear context, reload from `compacted.md` only, start work
12. **Work on ONE item at a time** - one test, one function, one file - finish it completely before next
13. Implement following TDD (test first, then code) - **one test at a time**
14. **Place tests in correct location** - follow language testing skill or project test structure
15. Report to Main Agent when done (DO NOT commit)
16. Wait for verification to pass
17. After commit: delete `compacted.md`, update `./PROGRESS.md`, move to next task
18. **ALWAYS UPDATE ../../LEARNINGS.md** after each completed task/milestone

---

**Workflow:** Requirements → **Language Stack → Skills** → Learnings → Verification → AGENTS.md → Agent Doc → Skills → **Compact → Clear → Reload** → **ONE ITEM AT A TIME** → Implement → Report → Verify → Commit → Delete compacted.md → Next

---

_Created: YYYY-MM-DD_

================================================================================

# VARIANT 3: Simple Specification (has_features: false) - Spec Level Only

Use this at the specification root level for simple specs with no features.
Simple specs have all requirements in requirements.md.

See example: .agents/templates/examples/start-simple-specification-example.md

---
workspace_name: "ewe_platform"
spec_directory: "specifications/[NN-spec-name]"
this_file: "specifications/[NN-spec-name]/start.md"
created: YYYY-MM-DD
---

# Start: [Specification Name]

## Agent Workflow

1. Read `requirements.md` (complete requirements + tasks)
2. **Identify language stack** from requirements.md "Language Stack" section - document languages clearly
3. **Read/generate language skills** - for each language in stack:
   - If skill exists: `.agents/skills/[language]-clean-code/skill.md` → read it
   - If skill missing: launch agent to generate one first, then read it
   - Add this item to the workflow list so future agents remember
4. Read `LEARNINGS.md` (past discoveries and mistakes)
5. Read `VERIFICATION.md` (verification requirements)
6. Read `.agents/AGENTS.md` to identify your agent type
7. Read your agent file in `.agents/agents/[agent-name].md`
8. Read skills specified in your agent documentation
9. **MANDATORY**: Generate `compacted.md` with all info using `.agents/skills/context-compaction/skill.md`
10. Clear context, reload from `compacted.md` only, start work
11. **Work on ONE item at a time** - one test, one function, one file - finish it completely before next
12. Implement following TDD (test first, then code) - **one test at a time**
13. **Place tests in correct location** - follow language testing skill or project test structure
14. Report to Main Agent when done (DO NOT commit)
15. Wait for verification to pass
16. After commit: delete `compacted.md`, update `PROGRESS.md`, move to next task
17. **ALWAYS UPDATE LEARNINGS.md** after each completed task/milestone

---

**Workflow:** Requirements → **Language Stack → Skills** → Learnings → Verification → AGENTS.md → Agent Doc → Skills → **Compact → Clear → Reload** → **ONE ITEM AT A TIME** → Implement → Report → Verify → Commit → Delete compacted.md → Next

---

_Created: YYYY-MM-DD_

================================================================================

## Usage Guide

**When creating start.md files:**

1. **For feature-based specs (has_features: true):**
   - Create VARIANT 1 at spec root: `specifications/NN-spec/start.md`
   - Create VARIANT 2 for each feature: `specifications/NN-spec/features/[feature]/start.md`

2. **For simple specs (has_features: false):**
   - Create VARIANT 3 only at spec root: `specifications/NN-spec/start.md`
   - No feature-level start.md files needed

**Key Differences:**

- **Spec-level (feature-based)**: Short, redirects to feature start.md
- **Feature-level**: Full workflow with compaction, TDD, reporting
- **Simple spec**: Full workflow directly at spec level

**Examples:**
- Spec-level (feature-based): `.agents/templates/examples/start-specification-example.md`
- Feature-level: `.agents/templates/examples/start-feature-example.md`
- Simple spec: `.agents/templates/examples/start-simple-specification-example.md`
