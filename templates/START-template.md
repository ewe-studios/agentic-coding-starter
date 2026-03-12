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
2. Identify which feature you're working on from the Feature Index
3. Navigate to `features/[feature-name]/start.md` and follow that feature's workflow
4. Read `LEARNINGS.md` (past discoveries and mistakes)
5. **ALWAYS UPDATE LEARNINGS.md** after each completed task/milestone

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
2. Read `../../LEARNINGS.md` (past discoveries and mistakes)
3. Read `./VERIFICATION.md` (verification requirements)
3. Read `./PROGRESS.md` (last progress of this feature if present)
4. Read `.agents/AGENTS.md` to identify your agent type
5. Read your agent file in `.agents/agents/[agent-name].md`
6. Read skills specified in your agent documentation
7. **MANDATORY**: Generate `compacted.md` with all info using `.agents/skills/context-compaction/skill.md`
8. Clear context, reload from `compacted.md` only, start work
9. **Work on ONE item at a time** - one test, one function, one file - finish it completely before next
10. Implement following TDD (test first, then code) - **one test at a time**
11. **Place tests in correct location** - follow language testing skill or project test structure
12. Report to Main Agent when done (DO NOT commit)
13. Wait for verification to pass
14. After commit: delete `compacted.md`, update `./PROGRESS.md`, move to next task
15. **ALWAYS UPDATE ../../LEARNINGS.md** after each completed task/milestone

---

**Workflow:** Feature.md → Learnings → Verification → AGENTS.md → Agent Doc → Skills → **Compact → Clear → Reload** → **ONE ITEM AT A TIME** → Implement → Report → Verify → Commit → Delete compacted.md → Next

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
2. Read `LEARNINGS.md` (past discoveries and mistakes)
3. Read `VERIFICATION.md` (verification requirements)
4. Read `.agents/AGENTS.md` to identify your agent type
5. Read your agent file in `.agents/agents/[agent-name].md`
6. Read skills specified in your agent documentation
7. **MANDATORY**: Generate `compacted.md` with all info using `.agents/skills/context-compaction/skill.md`
8. Clear context, reload from `compacted.md` only, start work
9. **Work on ONE item at a time** - one test, one function, one file - finish it completely before next
10. Implement following TDD (test first, then code) - **one test at a time**
11. **Place tests in correct location** - follow language testing skill or project test structure
12. Report to Main Agent when done (DO NOT commit)
13. Wait for verification to pass
14. After commit: delete `compacted.md`, update `PROGRESS.md`, move to next task
15. **ALWAYS UPDATE LEARNINGS.md** after each completed task/milestone

---

**Workflow:** Requirements → Learnings → Verification → AGENTS.md → Agent Doc → Skills → **Compact → Clear → Reload** → **ONE ITEM AT A TIME** → Implement → Report → Verify → Commit → Delete compacted.md → Next

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
