# Claude Code Memory Files

## Overview

Memory files are special files that Claude Code CLI automatically loads into the context window at the start of a session. They provide project-specific context without requiring explicit Read tool calls.

## Current Memory Files

### 1. CLAUDE.md
**Location**: `/home/darkvoid/Boxxed/@dev/ewe_platform/CLAUDE.md`
**Size**: 242 tokens (0.1% of context window)
**Purpose**: Project configuration and agent instructions

**Content Summary:**

This file serves as a backward compatibility redirect to the new agent configuration system. It contains:

- **Important Notice**: File maintained for backward compatibility only
- **Primary Configuration**: Moved to `.agents/AGENTS.md`
- **Primary Rule**: MUST load `AGENTS.md` before any operations
- **Migration Notice**: References to CLAUDE.md should update to AGENTS.md
- **Quick Start**: Directs to `.agents/AGENTS.md` for current configuration

**Key Points:**
- Instructs agents to load `.agents/AGENTS.md` immediately
- Explains that CLAUDE.md may be deprecated in future
- Provides migration path for workflows
- All new rules go to `.agents/AGENTS.md` and `.agents/rules/*`

**Actual Content:**
```markdown
# Claude AI Agent Configuration

## ⚠️ Important Notice

This file is maintained for **backward compatibility only**.

The primary configuration file for all AI agents (including Claude) has been moved to `AGENTS.md`.

---

## Primary Rule

**MANDATORY:** Before performing any tasks or operations on this project, you **MUST**:

1. **Load `AGENTS.md`** - This is the central configuration file containing all agent rules and guidelines
2. **Follow all instructions** in `.agents/AGENTS.md`, including loading project-specific rules from `.agents/rules/*`

---

## Migration Notice

If you're referencing `CLAUDE.md`:

- Please update your workflows to reference `.agents/AGENTS.md` instead
- `CLAUDE.md` may be deprecated in future versions
- All new rules and updates will be added to `.agents/AGENTS.md` and `.agents/rules/*`

---

## Quick Start

👉 **Go to [`AGENTS.md`](./.agents/AGENTS.md)** to get started.

---

_This file redirects to .agents/AGENTS.md for current agent configuration_
```

## Memory File System

**How Memory Files Work:**
1. Claude Code CLI scans for special files (CLAUDE.md, .claude/*, etc.)
2. Files are loaded automatically at session start
3. Content appears in context window without Read tool calls
4. Updates to memory files require new session to take effect

**Benefits:**
- Immediate project context on session start
- No manual loading required
- Consistent configuration across sessions
- Token-efficient (loaded once, not repeatedly)

**Configuration Location:**
Memory file paths are configured by Claude Code CLI. Common paths:
- `CLAUDE.md` in project root
- `.claude/*` directory
- Custom paths specified in CLI configuration

## Token Usage

**Current Memory Files:**
- CLAUDE.md: 242 tokens (0.1% of 200k context window)

**Total Memory Usage**: 242 tokens

**Impact:**
- Minimal context window usage
- Leaves 199.7k tokens for other operations
- Efficient for project configuration

## Best Practices

**For Memory Files:**
1. Keep content concise and focused
2. Use redirects for large configurations (like this project does)
3. Point to detailed documentation in `.agents/` directory
4. Update memory files when core rules change
5. Test with new session to verify loading

**For This Project:**
- CLAUDE.md redirects to `.agents/AGENTS.md` (keeps memory small)
- Detailed rules loaded selectively from `.agents/rules/*`
- Memory file serves as entry point, not full documentation
- Agents load specific rules based on role (Main, Implementation, Verification)

## Related Documentation

**Primary Configuration:**
- `.agents/AGENTS.md` - Central agent configuration hub
- `.agents/rules/*` - Individual rule files (01-13)
- `.agents/stacks/*` - Language-specific conventions
- `.agents/skills/*` - Reusable skill definitions

**Documentation:**
- `specifications/` - Feature requirements and tasks
- `documentation/` - Module documentation
- `.agents/docs/` - Agent system documentation

## Maintenance

**When to Update Memory Files:**
- Core project rules change
- Agent configuration changes
- Migration to new system
- Critical instructions need emphasis

**Current Status:**
- CLAUDE.md: Stable redirect (no frequent updates needed)
- Acts as entry point to selective rule loading system
- Reduces context usage by pointing to modular rules

---
*Generated: 2026-01-21*
*Memory Files Count: 1*
*Total Memory Usage: 242 tokens (0.1%)*
