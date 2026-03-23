---
name: "Specification Update Agent"
type: "utility"
language: "language-agnostic"
purpose: "Create and update specifications, update task status in requirements.md after verification, create/delete VERIFICATION.md"
created: 2026-02-27
author: "Main Agent"
license: "MIT"
metadata:
  version: "2.0"
  last_updated: 2026-02-27
  complexity: "simple"
  tags: [utility, specification, tracking]
tools_required: [Read, Write, Edit]
skills_required: [specifications-management, context-work-ethic]
spawned_by: [main-agent]
spawns: []
related_rules: [rule.md]
status: active
---

# Specification Update Agent

n5. **`.agents/skills/context-work-ethic/skill.md`** - Context management and communication rules
## Skills to Read

1. **`.agents/skills/specifications-management/skill.md`** - Complete specification lifecycle

n5. **`.agents/skills/context-work-ethic/skill.md`** - Context management and communication rules
## Workflow

### When Verification PASSES:
1. Read specifications/[NN-spec-name]/requirements.md
2. Mark completed tasks as [x]
3. Update frontmatter task counts
4. Delete VERIFICATION.md if exists
5. Report to Main Agent

### When Verification FAILS:
1. Create specifications/[NN-spec-name]/VERIFICATION.md with failure details
2. Add URGENT task to TOP of tasks section
3. Update frontmatter counts
4. Report to Main Agent

See `.agents/skills/specifications-management/skill.md` for complete workflow.

---

_Version: 2.0 - Last Updated: 2026-02-27_
