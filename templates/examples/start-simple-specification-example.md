---
workspace_name: "ewe_platform"
spec_directory: "specifications/01-fix-rust-lints-checks-styling"
this_file: "specifications/01-fix-rust-lints-checks-styling/start.md"
created: 2026-02-28
---

# Start: Fix Rust Lints, Checks, and Styling

## Agent Workflow

1. Read `requirements.md` (complete requirements + tasks)
2. Read `../../LEARNINGS.md` (past discoveries and mistakes)
3. Read `./PROGRESS.md` (last progress of this feature if present)
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
12. Wait for verification to pass
13. After commit: delete `compacted.md`, update `PROGRESS.md`, move to next task

---

**Workflow:** Requirements → Learnings → Verification → AGENTS.md → Agent Doc → Skills → **Compact → Clear → Reload** → **ONE ITEM AT A TIME** → Implement → Report → Verify → Commit → Delete compacted.md → Next

---

_Created: 2026-02-28_
