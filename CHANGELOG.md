# .agents Directory - Change Log

This file contains the version history for the `.agents/` directory.

---

## 2026-11-11 - Story-Driven Specifications

**Skills:** story-driven-specification (new), specifications-management (version 5.0)

**WHAT:** Added the skill `story-driven-specification`. Each feature file became one
self-contained story: stories and use cases, a bounded context with entities and verbs, a
law worksheet with numbered invariants, must have / must do / must not lists, Given / When /
Then acceptance scenarios, a technical breakdown with fault tables, and a table from each
law and scenario to the test that proves it.

**CHANGED:** `specifications-management` made this layout mandatory for every `feature.md`.
`templates/FEATURE-template.md` got the new sections. The main agent and the specification
update agent load the new skill.

## 2026-03-03 - Skills Optimization & TDD Consolidation (Version 6.1.0)

### Part 1: Rust Clean Code Skills Optimization

**Skills:** rust-clean-code/* (testing, implementation, async, directory-and-configuration)

**WHAT:** Comprehensive optimization of all rust-clean-code sub-skills.

**RESULT:** Combined reduction from 2,282 lines to 886 lines total (**61.2% smaller**)

| Skill | Before | After | Reduction |
|-------|--------|-------|-----------|
| **testing/skill.md** | 1,595 | 328 | 79.4% |
| **implementation/skill.md** | 1,400 | 230 | 83.6% |
| **async/skill.md** | 458 | 458 | 0% (no duplicates) |
| **directory-and-configuration/skill.md** | 424 | 424 | 0% (no duplicates) |
| **TOTAL** | **3,877** | **1,440** | **62.9%** |

**WHY:**
- Large skills hard for agents to remember
- Examples mixed with principles
- Token waste loading unused examples
- Target: 75-80% token savings

**CHANGES BY SKILL:**

### 1. testing/skill.md (1,595 → 328 lines)

**Optimizations:**
- Phase 1: Extracted 9 large code examples (1,595 → 902)
- Phase 2: Moved configs/trees/commands (902 → 379)
- Phase 3: Removed learning log (379 → 328)
- Phase 4: Optimized decision guide (table → numbered list)

**13 Example Files Created:**
1. test-organization.md ⭐ MUST READ
2. three-test-validations.md
3. integration-theater-vs-real-testing.md
4. common-pitfalls.md
5. http-testing-with-project-types.md
6. tcp-testing-stdlib.md
7. docker-for-testing.md
8. testcontainers-examples.md
9. test-dependency-decisions.md
10. feature-gated-tests.md
11. property-based-testing-basics.md
12. intro-to-property-based-testing.md
13. running-tests.md

### 2. implementation/skill.md (1,400 → 230 lines)

**Optimizations:**
- Extracted 44 large code blocks into examples
- Kept core principles inline
- Created numbered "read when needed" guide

**4 New Example Files Created:**
1. dependency-hierarchy.md - Project → Stdlib → External pattern
2. no-std-support.md - Supporting both std and no_std
3. performance-tips.md - Optimization patterns and benchmarking
4. trait-patterns.md - Trait implementation best practices

**Existing Examples (kept):**
- documentation-patterns.md
- error-handling-guide.md
- security-guide.md
- iterator-patterns.md
- basic-template.md

### 3. async/skill.md (458 lines - no change)

**Review:** No duplicate content found. Test isolation section is specific to async testing (appropriate).

### 4. directory-and-configuration/skill.md (424 lines - no change)

**Review:** No duplicate content found. Only mentions tests/ directory as part of project structure (appropriate).

---

## FINAL STRUCTURE

**All skills now follow pattern:**
- ⭐ Core Principles (always apply) - Small inline examples
- 📖 Read When Needed (numbered list) - Links to example files
- 🔗 Related Skills

**Token Savings:**
- **testing:** 75-80% for typical workflows
- **implementation:** 70-75% for typical workflows
- **Overall:** ~60% average across all skills

---

## HOW TO USE

### For Agents:
1. Read skill.md for core principles (200-400 lines)
2. Scan numbered "read when needed" list
3. Load only relevant example files
4. Fresh context for implementation

### For Maintainers:
- Update examples without touching principles
- Add new examples easily
- Clear separation of concerns
- Modular documentation

---

### Part 2: Test-Driven Development (TDD) Skill Optimization

**Skill:** test-driven-development

**WHAT:** Streamlined TDD skill from 480 lines to 253 lines (47.3% reduction).

**CHANGES:**
- Extracted TDD workflow examples → `tdd-workflow-examples.md` (Rust, TypeScript, Python)
- Extracted common patterns → `tdd-patterns.md` (features, edge cases, refactoring)
- Extracted documentation standards → `test-documentation.md` (WHY/WHAT templates)
- Kept core TDD cycle and principles inline
- Created numbered "read when needed" guide

**3 Example Files Created:**
1. tdd-workflow-examples.md - Complete cycle examples (6 steps per language)
2. tdd-patterns.md - Common patterns (multiple requirements, edge cases, etc.)
3. test-documentation.md - WHY/WHAT documentation standards

**Result:**
- **47.3% smaller** (480 → 253 lines)
- **No tables** - using numbered lists
- **3 focused examples** covering all TDD scenarios
- **Consistent pattern** with other optimized skills

---

### Part 3: TDD Duplicate Removal from Rust Testing Skill

**WHAT:** Removed duplicate TDD workflow from rust-clean-code/testing and added reference to TDD skill.

**CHANGES:**
- Removed "Write One Test at a Time" section (4 lines)
- Added reference: "For TDD workflow (test-first, one at a time), see: Test-Driven Development"
- Kept Rust-specific testing practices (test organization, cargo commands, etc.)
- Clear separation: TDD skill = workflow, Rust testing = language-specific patterns

**Result:**
- **No duplicate content** between TDD and Rust testing skills
- **Clear navigation** - Rust testing references TDD for workflow
- **Single source of truth** - TDD principles in one place
- **Rust testing focused** on language-specific testing patterns only

---

## COMBINED IMPACT (All Optimizations)

| Skill | Before | After | Reduction | Status |
|-------|--------|-------|-----------|--------|
| rust-clean-code/testing | 1,595 | 324 | 79.7% | ✅ Optimized + TDD reference |
| rust-clean-code/implementation | 1,400 | 230 | 83.6% | ✅ Optimized |
| rust-clean-code/async | 458 | 452 | 1.3% | ✅ Table removed |
| rust-clean-code/skill.md | 104 | 99 | 4.8% | ✅ Table removed |
| **test-driven-development** | **480** | **253** | **47.3%** | ✅ **Optimized** |
| **TOTAL** | **4,037** | **1,358** | **66.4%** | ✅ |

**Overall token savings: 65-70% for typical workflows!**

---

## 2026-02-27 - MAJOR ARCHITECTURE MIGRATION (Version 6.0.0)

### System-Wide Transformation: Rules → Skills

**BREAKING CHANGE**: Complete migration from rules-based to skill-based architecture

**What Changed:**
- ✅ Deleted 15 individual rule files (01-15)
- ✅ Created 21 focused skills covering all workflows
- ✅ Single entry point: `AGENTS.md` with agent registry
- ✅ Agent documentation in `agents/` directory (10 files)
- ✅ Skills in `skills/` directory (organized by domain)
- ✅ Deleted `stacks/` directory (4 files) - replaced by language skills
- ✅ Deleted `rule.md` - merged into `AGENTS.md`
- ✅ Simplified all templates - minimal frontmatter

**Skills Created:**
1. main-agent-orchestration - Main Agent workflow
2. implementation-practices - Implementation best practices
3. test-driven-development - TDD workflow (ONE test at a time)
4. learning-documentation - LEARNINGS.md patterns
5. code-verification - Complete verification workflow
6. git-workflow - Commit and push requirements
7. skills-management - Creating and using skills
8. agent-documentation - Agent documentation standards
9. context-compaction - compacted.md approach (90% token reduction)
10. specifications-management - Specification structure
11. dangerous-operations - Safety protocols
12. language-standards - Cross-language patterns
13+ Language skills: rust-clean-code, python-clean-code, etc.

**Context Optimization:**
- ❌ Deleted `machine_prompt.md` approach
- ❌ Deleted `COMPACT_CONTEXT.md` approach
- ✅ New unified approach: `compacted.md` (single temporary file)
- ✅ 90% token reduction through pipe-delimited compression
- ❌ Deleted Python generation scripts (generate_machine_prompt.py, generate_compact_context.py)

**Workflow Changes:**
- ✅ New `START-template.md` with 13-step workflow
- ✅ Mandatory: ONE item at a time (one test, one function, one file)
- ✅ Mandatory: TDD with test-first approach
- ✅ Mandatory: Retrieval-led reasoning (read code first, not assumptions)
- ✅ Mandatory: Context compaction for all agents

**Template Updates:**
- ✅ All templates simplified with minimal frontmatter
- ✅ Removed verbose section headers in YAML
- ✅ Added frontmatter where missing
- ❌ Deleted obsolete templates (COMPACT_CONTEXT, machine_prompt_example, compact_context_example)
- ❌ Deleted TEMPLATE_IMPROVEMENTS.md
- ❌ Deleted MAKEFILE-SPEC-template

**Files Deleted (Total: 26 files):**
- 15 rule files (.agents/rules/01-15.md)
- 1 rule.md
- 4 stack files (.agents/stacks/*.md)
- 1 context-optimization skill
- 3 template files
- 2 Python generation scripts

**Result:**
- System reduced from ~8,700 lines to ~4,500 lines
- Clearer separation: AGENTS.md = entry, agents/ = docs, skills/ = procedures
- Zero duplication: skills referenced, not repeated
- Better organization: 2 layers instead of 5
- Token efficiency: 90% reduction with compacted.md

**Migration Guide:**
- Old: Read Rules 01-15 → New: Read AGENTS.md, identify agent, read agent file, load specified skills
- Old: Rules referenced by number → New: Skills referenced by name
- Old: machine_prompt.md + COMPACT_CONTEXT.md → New: compacted.md only
- Old: Stacks directory → New: Language skills in skills/ directory

---

## Archive: Pre-6.0 History (Rules-Based System)

The entries below document the old rules-based architecture (prior to Feb 27, 2026).
This system has been replaced by the skill-based architecture described above.

---

## 2026-02-01

### AGENTS.md - Version 5.4.0
- Updated: Main Agent generates initial COMPACT_CONTEXT.md before spawning sub-agents
- Clarified: Sub-agents receive and maintain COMPACT_CONTEXT.md during work
- Clarified: Main Agent handles cleanup after verification
- Added: Clear handoff model for COMPACT_CONTEXT.md lifecycle
- Benefit: Clear ownership and lifecycle management for context optimization

### AGENTS.md - Version 5.4.0
- **NAVIGATION**: Converted all plain text rule references to markdown links
- Updated: "By Role" table with clickable links to all rules
- Updated: "Rules Reference" table with full navigable links
- Enhanced: "Spawning Sub-Agents" section with linked references
- Updated: "Critical Reminders" section with links to rules and sections
- Enhanced: "Directory Structure" with navigable directory links
- Stats: +44 insertions, -28 deletions
- Benefit: Easy one-click navigation throughout AGENTS.md, better discoverability

### rules/14-machine-optimized-prompts.md - Version 1.1
- **MAJOR**: Added "machine_prompt.md Lifecycle and Usage" section (~200 lines)
- Added: 8-step generation and context workflow
- Clarified: Dual file maintenance (human + machine versions)
- Clarified: Human files remain source of truth, never deleted
- Clarified: Machine files generated, synced, used by agents, never hand-edited
- Added: Synchronization workflow (edit human → regen machine → clear → reload → commit both)
- Added: Integration with Rule 15 (machine_prompt content embeds in COMPACT_CONTEXT.md)
- Benefit: Complete clarity on machine_prompt.md lifecycle and relationship with human files

### rules/15-instruction-compaction.md - Version 1.1
- **MAJOR**: Added "Ephemeral Nature of COMPACT_CONTEXT.md" section (~150 lines)
- Clarified: Lifecycle is per-task, regenerated on updates, deleted on completion
- Clarified: NEVER accumulates history - current task only
- Added: Size limit guidance (500-800 tokens maximum)
- **MAJOR**: Added "machine_prompt.md Integration" section
- Clarified: COMPACT_CONTEXT.md embeds machine_prompt content (MACHINE_PROMPT_CONTENT section)
- Clarified: Self-contained after context clear - no need to re-read machine_prompt.md
- **MAJOR**: Added "PROGRESS.md Lifecycle" section
- Clarified: PROGRESS.md rewritten from scratch per task (no cumulative history)
- Added: Pattern documentation (Create → Update → CLEAR → Rewrite for next)
- Updated: Generation workflow to include machine_prompt embedding (Step 3b)
- Added: extract_task_from_machine_prompt() function example
- Updated: Example to show MACHINE_PROMPT_CONTENT section
- Stats: +575 insertions, -50 deletions
- Benefit: Complete clarity on COMPACT_CONTEXT.md and PROGRESS.md lifecycles

### templates/COMPACT_CONTEXT-template.md - Version 1.1
- Added: MACHINE_PROMPT_CONTENT section for embedded requirements
- Added: Comprehensive agent instructions for using embedded content
- Benefit: Template guides agents to create self-contained compact context

### templates/PROGRESS-template.md - Version 1.2
- Enhanced: Header with lifecycle and relationship diagram
- Added: Clear documentation of ephemeral nature
- Benefit: Agents understand PROGRESS.md is rewritten per task

### templates/requirements-template.md - Version 3.3
- Added: context_optimization frontmatter section
- Added: Token Optimization and Context Management section
- Documented: Rule 14 and Rule 15 integration
- Benefit: Specifications include optimization guidance

### templates/feature-template.md - Version 1.3
- Added: context_optimization frontmatter section
- Added: Token Optimization and Context Management section
- Documented: COMPACT_CONTEXT.md and PROGRESS.md lifecycle details
- Benefit: Features include complete optimization workflow

### rules/14-machine-optimized-prompts.md - Initial Creation
- **NEW RULE**: Machine-optimized prompts for 58% token savings
- Core principle: Sub-agents read machine_prompt.md (NOT requirements.md)
- Added: Automatic generation protocol from requirements.md
- Added: Compact format specification (pipe-delimited, minimal tokens)
- Added: When to use guidelines (sub-agents, not Main Agent)
- Benefit: Massive token reduction for sub-agent context loading

### rules/15-instruction-compaction.md - Initial Creation
- **NEW RULE**: Instruction compaction for 97% context reduction
- Core principle: COMPACT_CONTEXT.md for radically compressed task context
- Added: Complete compaction protocol (generate → clear → reload)
- Added: Compact format specification (ultra-minimal, reference-based)
- Added: When to compact guidelines (before tasks, after updates, at thresholds)
- Added: Context preservation strategy
- Benefit: Eliminates context exhaustion, enables indefinite work sessions

### AGENTS.md - Version 5.3.0
- Added: Rule 14 as mandatory for ALL agents
- Added: Rule 15 as mandatory for ALL sub-agents
- Updated: "By Role" table to include Rules 14 and 15
- Updated: "Critical Reminders" section with context optimization points
- Updated: "Spawning Sub-Agents" prompt to include Rules 14 and 15
- Benefit: Context optimization built into agent workflow from start

### rules/05-coding-practice-agent-orchestration.md - Version 1.2
- Added: Main Agent responsibility to generate initial COMPACT_CONTEXT.md
- Added: Main Agent handles cleanup after verification
- Benefit: Clear Main Agent ownership of COMPACT_CONTEXT.md lifecycle

### rules/13-implementation-agent-guide.md - Version 1.2
- Added: Context compaction to "Before Starting Work" (steps 11-15)
- Added: COMPACT_CONTEXT.md generation and reload workflow
- Clarified: Sub-agents receive (not generate) initial COMPACT_CONTEXT.md
- Added: Maintenance during work session
- Benefit: Sub-agents know when and how to compact context

### rules/06-specifications-and-requirements.md - Version 2.2
- Added: COMPACT_CONTEXT.md to "Work-in-Progress Files"
- Added: context_optimization to frontmatter specification
- Updated: Directory structure examples to include COMPACT_CONTEXT.md
- Benefit: Specifications recognize and guide context optimization files

### rules/08-verification-workflow-complete-guide.md - Version 1.2
- Added: Automated verification scripts requirement
- Added: Guidelines for user-specified verification script support
- Added: Script discoverability protocol
- Benefit: Verification agents can use project-specific verification workflows

### AGENTS.md - Version 5.2.0
- **MAJOR**: Added retrieval-led reasoning as core principle
- Added: Comprehensive "Retrieval-Led Reasoning (MANDATORY)" section
- Clarified: Read codebase FIRST before assumptions
- Clarified: Use Grep/Glob/Read tools to understand patterns
- Clarified: Trust project rules over general best practices
- Added: Enforcement requirements before any implementation
- Added: Examples of retrieval-led vs pretraining-led reasoning
- Stats: ~60 lines added
- Benefit: Ensures agents follow project-specific patterns, not generic assumptions

---

## 2026-01-25

### rules/07-language-conventions-and-standards.md - Version 1.1
- **CRITICAL FIX**: Corrected typo in Summary section (line 681)
- Changed: "Never document language stack in requirements" → "Never skip documenting language stack in requirements"
- Reason: Original text contradicted rule's intent which REQUIRES language stack documentation
- Benefit: Removes logical inconsistency, clarifies mandatory requirement

### AGENTS.md - Version 5.1.1
- **CLARIFICATION**: Updated "By Role" table to explicitly show Rule 07 inclusion via stack files
- Updated: Implementation Agent - "stack file (includes Rule 07)"
- Updated: Verification Agent - "stack file (includes Rule 07)"
- Updated: Any Sub-Agent - "relevant stack (includes Rule 07 if applicable)"
- Updated: Rules Reference table - Rule 07 purpose changed to "Language conventions (embedded in stack files)"
- Benefit: Makes clear that Rule 07 is implicitly loaded via stack files, not separately
- Rationale: Stack files contain language-specific standards, making separate Rule 07 loading redundant

### CHANGELOG.md - Version 1.1
- Added: Today's changes for Rule 07 typo fix and AGENTS.md clarification
- Benefit: Complete audit trail of all rule updates

---

## 2026-01-25

### templates/requirements-template.md - Version 3.0
- **MAJOR**: Complete rewrite based on working Spec 02 example
- Reduced from ~450 lines to ~200 lines (56% reduction)
- Removed bloat: Deprecated sections, redundant agent instructions, verbose examples
- Clear separation: Simple specs vs Feature-based specs structure
- Streamlined frontmatter: Removed redundant fields, clearer comments
- Feature-based section: Overview, Known Issues, Feature Index, Success Criteria only
- Simple spec section: Complete requirements with tasks inline
- Removed: Old "Agent Reminders", "Role-Specific Rules", deprecated "Skills" sections (now in files_required)
- Benefit: Clean template that matches actual usage pattern

### specifications/02-build-http-client/requirements.md - Version 4.2
- **CRITICAL FIX**: Removed `implementation_agent` section from files_required (feature-based specs don't have this)
- Fixed: `has_fundamentals: false` → `true` (HTTP client needs user documentation)
- Added: Clear note explaining implementation agents load feature.md files directly
- Updated: Success criteria to include fundamentals documentation requirements
- Benefit: Correct files_required structure per Rule 06

### templates/requirements-template.md - Version 3.2
- **CRITICAL FIX**: Removed `implementation_agent` section from default files_required (since default is has_features: true)
- Fixed: `has_fundamentals: false` → `true` (DEFAULT: true unless user says no)
- Added: Clear comments explaining structure differs based on has_features
- Added: Commented example showing implementation_agent section only for has_features: false
- Benefit: Template now correctly matches Rule 06 requirements

### rules/06-specifications-and-requirements.md - Version 2.1
- Clarified: files_required Frontmatter section with CRITICAL note about structure differences
- Updated: has_features: false example unchanged (includes implementation_agent)
- Updated: has_features: true example removes implementation_agent section entirely
- Added: Clear NOTE explaining why implementation_agent not included for feature-based specs
- Benefit: Crystal clear guidance on files_required structure
- **MAJOR**: Restructured to match feature-based best practices
- Reduced from 543 lines to 200 lines (63% reduction)
- Removed: User-Facing API details (moved to public-api feature), File Structure (unnecessary), Tasks list (in features), Total Tasks counter (bloat), Additional Tasks section, Guidelines (feature-specific), Old mandatory rules, Skills section
- Updated: files_required to match template, feature status tracking, cleaner frontmatter
- Kept: Overview, Known Issues, Feature Index, Requirements Summary, Success Criteria (spec-wide only)
- Created backup: requirements-old-v3.md
- Benefit: True high-level overview, agents load only relevant features

### rules/04-work-commit-and-push-rules.md - Version 2.0
- **BREAKING**: Redefined "atomic commits" from file-level to task/feature-level
- Changed: Commits happen ONLY after task/feature completion + full verification (not after every file change)
- Added: Branch management workflow - auto-create branch from spec name when on main/master
- Updated: Core Principles section renamed from "Immediate Commit" to "Task/Feature Commit Requirement"
- Updated: Complete workflow shows task/feature-based commits
- Updated: "No Exceptions" section clarified (no batching tasks, no incomplete work)
- Updated: Rationale section emphasizes logical units and clean history
- Updated: Safety Guarantees section reflects task/feature atomicity
- Updated: Violations section aligned with new commit strategy
- Updated: Examples section replaced file-level with task/feature examples
- Updated: Summary section reflects complete workflow change
- Lines changed: ~150 lines
- Benefit: Clean git history, easy rollback, reduced commit noise

### rules/05-coding-practice-agent-orchestration.md - Version 1.1
- Updated: Verification-First Workflow diagram to show "Implement Task/Feature" instead of "Implementation"
- Aligned terminology with Rule 04 task/feature-level commits
- Lines changed: ~5 lines

### rules/06-specifications-and-requirements.md - Version 2.0
- **MAJOR**: Added "Requirements.md Content Structure" section (~50 lines)
  - Formalized simple specs (has_features: false) - requirements.md contains COMPLETE details
  - Formalized feature-based specs (has_features: true - DEFAULT) - requirements.md is HIGH-LEVEL OVERVIEW ONLY
  - Clear guidelines on what to include/exclude for each type
- Updated: Directory Structure section with clear comments and separation
  - Simple Specification section: "Use ONLY for trivial specs (1-3 simple tasks)"
  - Feature-Based Specification section: "Use for all non-trivial work (DEFAULT)"
- Updated: "When to Use Features" section to emphasize DEFAULT behavior
  - Added decision rule: "When in doubt, default to `has_features: true`"
- Lines changed: ~95 lines
- Benefit: Context optimization - agents load overview + specific feature, not all features

### rules/08-verification-workflow-complete-guide.md - Version 1.1
- Updated: Core Principle workflow diagram to show "Task/Feature Complete" instead of "Implementation"
- Updated: CRITICAL RULES text to clarify commits after task/feature complete + verification
- Aligned terminology with Rule 04
- Lines changed: ~10 lines

### rules/13-implementation-agent-guide.md - Version 1.1
- Updated: "Before Starting Work" section step 6 to clarify file loading based on has_features
- Added: Clear distinction between has_features: false (read requirements.md only) and has_features: true (read requirements.md + specific feature.md)
- Lines changed: ~8 lines
- Benefit: Clear guidance for agents on which files to load

### templates/requirements-template.md - Version 2.0
- **MAJOR**: Complete restructuring for feature-based specifications
- Updated: Frontmatter has_features default changed from false to true with comment
- Updated: Frontmatter files_required section with clear comments about has_features differences
  - Added comments explaining implementation_agent file loading for both cases
  - Clarified: IF has_features: false vs IF has_features: true file loading
- Added: Header note explaining specification structure differences
- Added: "FOR SIMPLE SPECS" section marker with skip instructions
- Added: "IF has_features: true - FEATURE-BASED SPEC SECTIONS" with complete structure:
  - Known Issues/Limitations section
  - High-Level Architecture section
  - Feature Index table with status tracking (⬜ Pending | 🔄 In Progress | ✅ Complete)
- Updated: Features section with agent instructions about loading specific features only
- Updated: Success Criteria section split into separate subsections for simple vs feature-based
- Updated: Task/Feature tracking notes to reflect commit after verification approach
- Lines changed: ~170 lines
- Benefit: Clear template for both simple and feature-based specifications

### templates/feature-template.md - Version 1.2
- Added: Rule 11 (skills-usage) to implementation_agent rules
- Added: `../fundamentals/*` to files section (if parent spec has_fundamentals: true)
- Updated: Comments for conditional file inclusion
- Benefit: Complete files_required structure for feature-based development

### specifications/02-build-http-client/features - Batch Update
- Updated 9 pending features: auth-helpers, compression, cookie-jar, middleware, proxy-support, public-api, request-response, task-iterator, websocket
- Added: Rule 11 (skills-usage) to implementation_agent rules in all pending features
- Completed features (foundation, connection, tls-verification, valtron-utilities) left unchanged per immutability principle
- Benefit: All features now have complete rule references

### templates/feature-template.md - Version 1.1
- Updated: Task tracking note to clarify "Mark tasks as `[x]` after completing AND verifying"
- Added: "Commit after task completion + verification pass (Rule 04)"
- Added: Note that each feature manages its own task tracking
- Lines changed: ~5 lines

### templates/PROGRESS-template.md - Version 1.1
- Updated: Header to include "Commit Strategy: Update this file during work. Commit happens AFTER task/feature verification passes"
- Updated: Status options to include "Awaiting Verification"
- Added: "Progress This Session" section with status indicators (✅ Completed, 🔄 In Progress, ⏳ Ready for Verification)
- Lines changed: ~15 lines

### templates/VERIFICATION-template.md - Version 1.1
- Added: Header note explaining spec-wide verification for feature-based specs
- Added: "Specification Type" field to Executive Summary (Simple / Feature-Based)
- Lines changed: ~5 lines

### Documentation Files Created
- Created: templates/examples/commit-examples-and-special-cases.md - Extracted examples from Rule 04
- Note: Other summary files (RULE_UPDATES_SUMMARY.md, QUICK_REFERENCE.md, etc.) were consolidated into this CHANGELOG and removed per best practices

### Summary of Changes
**Theme**: Task/Feature-Level Commits + Features-First + Requirements.md Structure Formalization

**Key Changes**:
1. Commits now happen after task/feature completion + verification (not per file)
2. Automatic branch creation from spec name when starting work on main/master
3. Default to `has_features: true` for all non-trivial specifications
4. Requirements.md structure formalized: simple specs have complete details, feature-based specs are overview only
5. Clear file loading instructions based on has_features flag

**Impact**:
- Clean git history (one commit = one complete feature)
- Context optimization (agents load specific features, not all)
- Better organization (features scale to large specifications)
- Backward compatible (existing specs unaffected)

**Files Modified**: 9 files (~420 lines in rules, ~200 lines in templates)

---

## 2026-01-24

### AGENTS.md - Version 5.1.0
- Updated agent references to use requirements.md (tasks integrated into requirements.md)
- Emphasized files_required frontmatter as source of truth for agent context

### agents/implementation.md - Version 1.3
- Fixed remaining tasks.md references in "Works With" section, examples, and violation examples
- Changed all references to use requirements.md task status instead of tasks.md
- Ensured consistency across all examples and documentation

### agents/implementation.md - Version 1.2
- Added explicit requirement to report completed tasks to Main Agent
- Updated completion report format to include "Completed Tasks" section
- Added workflow steps showing Specification Update Agent marks tasks complete
- Clarified that implementation agent only reports task completion, doesn't update requirements.md directly
- Added note in self-review about identifying completed tasks

### agents/implementation.md - Version 1.1
- Updated to use requirements.md as single source (tasks now integrated)
- Changed references from tasks.md to requirements.md task status
- Updated learnings.md references to LEARNINGS.md (uppercase per Rule 06)
- Added explicit step to load files_required.implementation_agent context

### agents/review.md - Version 1.1
- Updated to use requirements.md as single source (tasks now integrated)
- Added explicit step to load files_required.review_agent context
- Removed references to separate tasks.md file

### agents/rust-verification.md - Version 1.1
- Updated to use requirements.md as single source (tasks now integrated)
- Added explicit step to load files_required.verification_agent context
- Emphasized requirements.md contains all necessary context

### agents/specification-update.md - Version 1.1
- Updated to work with requirements.md (tasks now integrated)
- Changed references from verification.md to VERIFICATION.md (uppercase per Rule 06)
- Updated to modify tasks section within requirements.md instead of separate tasks.md

---

## 2026-01-19

### AGENTS.md - Version 5.0.0
- Selective rule loading for context optimization

---

## 2026-01-14

### agents/documentation.md - Version 2.0
- Added comprehensive asset creation requirements
- OpenAPI specifications for APIs
- JSON schemas for data models
- Examples and configuration templates
- Architecture diagrams mandatory
- Asset quality standards defined

### agents/documentation.md - Version 1.0
- Initial documentation

### agents/implementation.md - Version 1.0
- Initial documentation
- TDD workflow
- Test documentation requirements
- Self-review requirements
- Learning documentation
- Module documentation verification
- Critical identity rules for SUB-AGENT

### agents/javascript-verification.md - Version 1.0
- Initial documentation

### agents/python-verification.md - Version 1.0
- Initial documentation

### agents/review.md - Version 1.0
- Initial documentation

### agents/rust-verification.md - Version 1.0
- Initial documentation
- Comprehensive Rust verification workflow
- User-specified scripts support
- Standards compliance checking
- Complete report generation

### agents/specification-update.md - Version 1.0
- Initial documentation

---

## Changelog Format

**Entry Structure**:
```
### [file_path] - Version [version]
- Change 1
- Change 2
- Change N
```

**Guidelines**:
- Entries are organized by date (newest first)
- Each file update gets its own entry
- Multiple updates on same date are listed separately
- Version numbers follow semantic versioning where applicable
- Changes are listed as bullet points
- Use past tense for change descriptions

---

*Last Updated: 2026-02-01*
