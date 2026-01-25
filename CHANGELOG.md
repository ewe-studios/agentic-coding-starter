# .agents Directory - Change Log

This file contains the complete version history for all files in the `.agents/` directory. Individual files no longer contain version history sections to reduce context size.

**Format**: Each entry lists the file path, version, date, and changes.

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

*Last Updated: 2026-01-24*
