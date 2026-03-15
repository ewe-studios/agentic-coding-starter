---
workspace_name: "ewe_platform"
spec_directory: "specifications/02-build-http-client"
this_file: "specifications/02-build-http-client/start.md"
created: 2026-02-28
---

# Start: Build HTTP Client

## Agent Workflow

1. Read `requirements.md` (high-level overview + feature index)
2. **Identify language stack** from requirements.md "Language Stack" section - document languages clearly
3. **Read/generate language skills** - for each language in stack:
   - If skill exists: `.agents/skills/[language]-clean-code/skill.md` → read it
   - If skill missing: launch agent to generate one first, then read it
   - Add this item to the workflow list so future agents remember
4. Read `./PROGRESS.md` (last progress of this feature if present)
5. Identify which feature you're working on from the Feature Index
6. Navigate to `features/[feature-name]/start.md` and follow that feature's workflow
7. Read `LEARNINGS.md` (past discoveries and mistakes)

---

**Workflow:** Requirements → **Language Stack → Skills** → Select Feature → Feature start.md → Follow feature workflow

---

_Created: 2026-02-28_
