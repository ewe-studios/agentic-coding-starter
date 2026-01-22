# Cross-Reference Links for requirements.md

This example shows the mandatory cross-reference links that must be included in every `requirements.md` file for self-containment.

## Purpose

These links ensure that anyone reading `requirements.md` can immediately navigate to related tracking and verification documents without searching.

## Top Link (After Frontmatter, Before Overview)

Place this immediately after the frontmatter block, before the Overview section:

```markdown
> **Specification Tracking**: See [tasks.md](./tasks.md) for task progress and [learnings.md](./learnings.md) for implementation insights.
```

## Bottom Link (After Final Verification Checklist)

Place this at the very end of the requirements.md file, after all content:

```markdown
> **Verification**: See [verification.md](./verification.md) or [VERIFICATION_SIGNOFF.md](./VERIFICATION_SIGNOFF.md) for complete verification results.
```

## Why These Links Matter

**Without cross-reference links:**
- Readers don't know where to find task progress
- No clear path to verification results
- Must manually search for related documents
- Breaks the self-contained specification principle

**With cross-reference links:**
- Clear navigation to all related documents
- Immediate access to progress and verification
- Self-contained specification that guides readers
- Professional documentation structure

## Validation

Before committing `requirements.md`, verify:
- ✅ Top link present after frontmatter
- ✅ Bottom link present at end of file
- ✅ Links point to correct relative paths
- ✅ Both links use proper markdown blockquote format (`>`)

---

*Created: 2026-01-22*
*Referenced in: Rule 06, section "Requirements.md Self-Containment"*
