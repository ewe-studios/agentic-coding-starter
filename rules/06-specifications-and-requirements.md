# Specifications and Requirements Management

## Purpose
This rule establishes a mandatory requirements-gathering and specification-tracking system that ensures all work begins with a documented conversation between the main agent and user, creating a clear record of requirements and tasks in the `specifications/` directory.

## Rule

### Requirements-First Development
Before **ANY** work begins on new features, enhancements, or significant changes, the main agent **MUST**:

1. **Engage in a conversation** with the user about requirements
2. **Document the requirements** in a specification directory
3. **Create a task list** for tracking work progress
4. **Have agents read specifications** before starting work
5. **Verify and update status** as work progresses

### No Exceptions
- **NO coding** without documented requirements
- **NO starting work** without a specification
- **NO skipping** the requirements conversation
- This applies to **ALL significant development work**

## Directory Structure

### Overview
```
specifications/
├── Spec.md                          # Master index of all specifications
├── 01-specification-name/
│   ├── requirements.md              # (MANDATORY) Requirements and conversation summary
│   ├── tasks.md                     # (MANDATORY) Task list with checkboxes
│   ├── PROGRESS.md                  # (MANDATORY) Mid-work progress report
│   ├── FINAL_REPORT.md              # (MANDATORY) Comprehensive completion summary
│   ├── VERIFICATION_SIGNOFF.md      # (MANDATORY) Official verification report
│   └── LEARNINGS.md                 # (MANDATORY) Lessons learned and insights
├── 02-another-specification/
│   ├── requirements.md
│   ├── tasks.md
│   ├── PROGRESS.md
│   ├── FINAL_REPORT.md
│   ├── VERIFICATION_SIGNOFF.md
│   └── LEARNINGS.md
├── 03-yet-another-specification/
│   ├── requirements.md
│   ├── tasks.md
│   ├── PROGRESS.md
│   ├── FINAL_REPORT.md
│   ├── VERIFICATION_SIGNOFF.md
│   └── LEARNINGS.md
└── ...

documentation/
├── module-1/
│   ├── doc.md                       # (MANDATORY) Detailed module documentation
│   ├── diagrams/                    # (OPTIONAL) Architecture diagrams
│   └── assets/                      # (OPTIONAL) Additional documentation assets
├── module-2/
│   ├── doc.md
│   ├── diagrams/
│   └── assets/
└── ...
```

**CRITICAL**: The `documentation/` directory exists at the project root level, parallel to `specifications/`, NOT inside any specification directory.

### Naming Convention
- Each specification gets its own numbered directory
- Format: `NN-descriptive-name/` where NN is a two-digit number (01, 02, 03, etc.)
- Use dash separators for multi-word names
- Name should clearly describe what the specification is for

**Examples:**
- ✅ `01-build-http-client/`
- ✅ `02-implement-user-authentication/`
- ✅ `03-add-database-migrations/`
- ❌ `http-client/` (missing number prefix)
- ❌ `01_http_client/` (wrong separator)
- ❌ `1-http-client/` (single digit instead of two)

## Spec.md File (Master Index)

### Purpose
The `Spec.md` file serves as the central index and dashboard for all specifications.

### Required Contents
1. **Introduction**: Explanation of what specifications are and how they work
2. **Specifications List**: Links to all specification directories
3. **Status Dashboard**: Breakdown of completed vs pending specifications

### Example Spec.md Structure
```markdown
# Project Specifications

## Overview
This directory contains all project specifications and requirements. Each specification represents a significant feature, enhancement, or change to the project.

## How Specifications Work
1. **Requirements-First**: Before work begins, main agent discusses requirements with user
2. **Documentation**: Requirements and tasks are documented in numbered specification directories
3. **Agent Reading**: Agents MUST read both requirements.md and tasks.md before starting work
4. **Status Verification**: Agents MUST verify completion status by searching the codebase
5. **Task Updates**: Agents MUST update tasks.md as work progresses
6. **Status Accuracy**: Agents MUST ensure status reflects actual implementation

## All Specifications

### [01: Build HTTP Client](./01-build-http-client/)
**Status:** ✅ Completed
**Description:** RESTful HTTP client with request/response handling

### [02: Implement User Authentication](./02-implement-user-authentication/)
**Status:** 🔄 In Progress
**Description:** JWT-based authentication system with role management

### [03: Add Database Migrations](./03-add-database-migrations/)
**Status:** ⏳ Pending
**Description:** Database migration system for schema version control

## Status Dashboard

### Summary
- **Total Specifications:** 3
- **Completed:** 1 (33%)
- **In Progress:** 1 (33%)
- **Pending:** 1 (33%)

### Completed ✅
- 01: Build HTTP Client

### In Progress 🔄
- 02: Implement User Authentication

### Pending ⏳
- 03: Add Database Migrations

---
*Last updated: 2026-01-11*
```

## requirements.md File

### Purpose
Documents the detailed requirements from the conversation between main agent and user.

### File Structure
```markdown
---
description: Brief one-sentence description of what this specification is for
status: completed | uncompleted
---

# [Specification Name] - Requirements

## Overview
Brief summary of what this specification covers and why it's needed.

## Requirements Conversation Summary

### User Request
[Summary of what the user initially requested]

### Clarifying Questions
[Questions the agent asked to understand requirements better]

### User Responses
[User's answers and additional context provided]

### Final Requirements Agreement
[What was agreed upon as the final set of requirements]

## Detailed Requirements

### Functional Requirements
1. [Requirement 1]
2. [Requirement 2]
3. [Requirement 3]

### Non-Functional Requirements
1. [Performance requirements]
2. [Security requirements]
3. [Compatibility requirements]

### Technical Specifications
- **Technology Stack:** [Technologies to be used]
- **Dependencies:** [Required libraries/tools]
- **Integration Points:** [How this integrates with existing code]

### Success Criteria
- [ ] Criterion 1
- [ ] Criterion 2
- [ ] Criterion 3

## Important Notes for Agents

### Before Starting Work
- **MUST READ** both this requirements.md and tasks.md files
- **MUST VERIFY** completion status by searching the codebase
- **MUST UPDATE** tasks.md to reflect actual implementation status
- **MUST ADD** new tasks to tasks.md BEFORE starting work on them

### Verification Requirements
Agents cannot rely solely on the status field or task checkboxes. They **MUST**:
1. Search the codebase for relevant files and implementations
2. Verify that code actually exists and works as specified
3. Update task status based on actual findings, not assumptions
4. Mark tasks as completed only when fully verified in codebase

### During Work
- Update tasks.md as you complete each task
- Add new tasks if you discover additional work needed
- Keep frontmatter counts accurate in tasks.md
- Update tools list as new tools are used

---
*Created: [Date]*
*Last Updated: [Date]*
```

### Example requirements.md
```markdown
---
description: Build RESTful HTTP client with request/response handling and error management
status: completed
---

# HTTP Client - Requirements

## Overview
This specification covers the implementation of a robust HTTP client library for making RESTful API calls with comprehensive error handling, request/response transformation, and middleware support.

## Requirements Conversation Summary

### User Request
User requested a HTTP client that can handle common REST operations with support for authentication headers, request retries, and response parsing.

### Clarifying Questions
Agent asked:
- What HTTP methods need to be supported?
- Should it include built-in authentication handling?
- What level of error handling is required?
- Should it support request/response interceptors?

### User Responses
User confirmed:
- Support for GET, POST, PUT, PATCH, DELETE methods
- Built-in support for Bearer token authentication
- Automatic retry on network failures (configurable)
- Request and response interceptor middleware
- TypeScript support with full type definitions

### Final Requirements Agreement
Build a TypeScript HTTP client class with method chaining, middleware support, automatic retries, and comprehensive error handling.

## Detailed Requirements

### Functional Requirements
1. Support all standard HTTP methods (GET, POST, PUT, PATCH, DELETE)
2. Automatic JSON request/response parsing
3. Configurable base URL and default headers
4. Request and response interceptor middleware
5. Automatic retry on network failures (with configurable attempts)
6. Bearer token authentication helper methods
7. Query parameter building and encoding
8. Custom error classes for different failure types

### Non-Functional Requirements
1. **Performance:** Minimal overhead over native fetch
2. **Security:** No credential leakage in error messages
3. **Compatibility:** Works in Node.js 18+ and modern browsers
4. **Type Safety:** Full TypeScript type definitions

### Technical Specifications
- **Technology Stack:** TypeScript, native fetch API
- **Dependencies:** None (uses built-in fetch)
- **Integration Points:** Used throughout application for all API calls

### Success Criteria
- [x] All HTTP methods implemented and tested
- [x] Middleware system works correctly
- [x] Retry logic handles failures properly
- [x] TypeScript types are accurate and helpful
- [x] Error handling covers all edge cases
- [x] Documentation is complete

## Important Notes for Agents

### Before Starting Work
- **MUST READ** both this requirements.md and tasks.md files
- **MUST VERIFY** completion status by searching the codebase
- **MUST UPDATE** tasks.md to reflect actual implementation status
- **MUST ADD** new tasks to tasks.md BEFORE starting work on them

### Verification Requirements
Search for:
- `src/http-client.ts` or similar file
- Test files for HTTP client
- Integration usage in other parts of codebase
- Documentation files

---
*Created: 2026-01-11*
*Last Updated: 2026-01-11*
```

## PROGRESS.md File

### Purpose
Documents progress at the midpoint of specification work, providing a snapshot of what has been accomplished and what remains. This helps track momentum, identify blockers early, and communicate status to users.

### When to Create
Create PROGRESS.md when:
- You've completed approximately 40-60% of the tasks
- You're switching between major phases of work
- Significant progress has been made and it's a good checkpoint
- User asks for a progress update

### Required Contents
```markdown
# [Specification Name] - Progress Report

## Overall Status: [X%] Complete

### Completed Work
[List of completed tasks, features, or phases]

### Current Status
[What you're currently working on]

### Remaining Work
[What still needs to be done]

### Blockers/Issues
[Any problems encountered or blockers that need resolution]

### Statistics
[Metrics like files modified, lines changed, warnings fixed, etc.]

### Next Steps
[Immediate next actions]

---
*Progress Report Created: [Date and Time]*
```

### Example PROGRESS.md
```markdown
# Rust Lints Fix - Progress Report

## Overall Status: 60% Complete

### Completed Work (Phases 1-4)
- ✅ Full clippy analysis completed
- ✅ Fixed 2 cast_possible_truncation warnings
- ✅ Fixed 3 unnecessary_debug_formatting warnings
- ✅ Fixed 4 redundant continue expressions
- ✅ Fixed 1 match_same_arms warning
- ✅ All formatting corrections applied

### Current Status
Working on Phase 5: Adding comprehensive documentation sections
- In progress: Adding # Errors and # Panics documentation
- Currently on: ewe_channels crate

### Remaining Work
- Phase 5: Documentation (18 # Errors, 3 # Panics sections to add)
- Phase 6: Code quality improvements (7 needless pass-by-value fixes)
- Phase 7: Numeric literal separators
- Phase 8: Final verification

### Blockers/Issues
- foundation_wasm has 113 warnings but was excluded from scope
- No blockers for current work

### Statistics
- Tasks Completed: 17/28 (60%)
- Files Modified: 6
- Warnings Fixed: 12
- Commits Created: 4

### Next Steps
1. Complete documentation additions for ewe_channels
2. Add documentation for watch_utils
3. Fix needless pass-by-value warnings in foundation_macros

---
*Progress Report Created: 2026-01-14 14:30*
```

## FINAL_REPORT.md File

### Purpose
Provides a comprehensive summary of all work completed for the specification. This serves as the official record of achievements, statistics, and final state. It's the "mission accomplished" document that details everything that was done.

### When to Create
Create FINAL_REPORT.md when:
- **All tasks are 100% complete**
- **Before marking the specification as completed**
- **Before running final verification**
- You're ready to present final results to the user

### Required Contents
```markdown
# [Specification Name] - Final Report

## Mission Accomplished! 🎉

### Overall Status: [X%] Complete

## Work Completed ([X]/[Y] tasks)

### [Phase/Category 1] ✅
[Detailed list of completed work in this phase]

### [Phase/Category 2] ✅
[Detailed list of completed work in this phase]

## Detailed Accomplishments

### [Category 1]
[Specific details, code examples, before/after comparisons]

### [Category 2]
[Specific details, code examples, before/after comparisons]

## Commits Created
[List of all commits with hashes and descriptions]

## Remaining Work (if any)
[Any tasks that were descoped or deferred]

## Statistics

| Metric | Value |
|--------|-------|
| Total Work Items | X |
| Files Modified | Y |
| Commits | Z |
| [Other Metrics] | ... |

## Verification Results
[Results from clippy, tests, builds, etc.]

## Impact
**Before:**
[State before the work]

**After:**
[State after the work]

## Recommendation
[Status assessment and next steps]

---
*Final Report Created: [Date and Time]*
```

### Example FINAL_REPORT.md
```markdown
# Rust Lints Fix - Final Report

## Mission Accomplished! 🎉

### Overall Status: **96% Complete**

All targeted crates have **ZERO clippy warnings**:
- ✅ foundation_nostd
- ✅ foundation_macros
- ✅ ewe_watch_utils
- ✅ ewe_channels
- ✅ template-macro
- ✅ bin/platform

## Work Completed (27/28 tasks)

### Phase 1: Discovery and Assessment ✅
- Comprehensive clippy analysis
- Detailed categorization and prioritization
- Assessment document created

### Phase 2: Critical Warnings ✅
- Fixed 2 cast_possible_truncation warnings (foundation_nostd)
- Fixed 2 unnecessary_wraps warnings (foundation_macros, channels)
- Fixed similar_names warnings (channels)

[... continues with all phases ...]

## Detailed Accomplishments

### Documentation Added (21 total)
**# Panics sections (3):**
1. `foundation_nostd::raw_parts::into_vec` - Panics on usize overflow
2. `ewe_channels::broadcast::has_pending_messages` - Panics on invalid receiver state
3. `ewe_channels::broadcast::broadcast` - Panics if message queue fails

[... continues with detailed accomplishments ...]

## Statistics

| Metric | Value |
|--------|-------|
| **Total Warnings Fixed** | 80+ |
| **Files Modified** | 15+ |
| **Crates Fixed** | 6 |
| **Documentation Added** | 21 sections |
| **Code Quality Improvements** | 20+ changes |
| **Commits** | 11 |
| **Task Completion** | 27/28 (96%) |

## Impact

**Before:**
- 80+ clippy warnings
- Missing documentation
- Unsafe casts
- Poor code patterns

**After:**
- 0 warnings in target crates
- Complete documentation
- Safe conversions with try_from
- Idiomatic Rust patterns

## Recommendation

**Status: READY FOR MERGE** ✅

---
*Final Report Created: 2026-01-14 18:45*
```

## VERIFICATION_SIGNOFF.md File

### Purpose
Contains the official verification report from the Rust Verification Agent (or equivalent verification process). This is the formal sign-off that the specification work meets all requirements and quality standards. It provides independent validation that the work is complete and correct.

### When to Create
Create VERIFICATION_SIGNOFF.md when:
- **All work is complete and ready for verification**
- **After running the Rust Verification Agent (or verification process)**
- **Before marking the specification as officially completed**
- You need formal approval to merge/ship the work

### Required Contents
```markdown
# 🎯 [VERIFICATION TYPE] AGENT - FINAL SIGN-OFF REPORT

## Executive Summary

**Specification**: [Specification Name]
**Verification Date**: [Date]
**Verification Agent**: [Agent Name/Type]
**Overall Status**: **[APPROVED / APPROVED WITH NOTES / REJECTED]** [Icon]

---

## ✅ Verification Results

### 1. [Verification Check 1] - **[PASS/FAIL]** [Icon]
**Result**: [Details]
**Status**: [Assessment]

### 2. [Verification Check 2] - **[PASS/FAIL]** [Icon]
**Result**: [Details]
**Status**: [Assessment]

[... more verification checks ...]

---

## 📊 [Quality Assessment Category]

### [Subcategory] - **[Rating]** [Icon]
[Details of assessment]

---

## 🎯 Specification Compliance

### Requirements Met - **[X%]** [Icon]

| Requirement | Status |
|-------------|--------|
| [Requirement 1] | [Icon] [STATUS] |
| [Requirement 2] | [Icon] [STATUS] |

---

## ⚠️ Issues Found

### Critical Issues: **[NONE/COUNT]** [Icon]
[List of critical issues if any]

### Minor Issues:
[List of minor issues if any]

---

## 🏆 Final Verdict

### **[APPROVED/APPROVED WITH NOTES/REJECTED]** [Icon]

### Compliance Rating: **[X]/10** [Stars]

### Recommendation: **[READY FOR MERGE / NEEDS WORK / etc.]** [Icon]

### Verification Confidence: **[X%]** [Icon]

---

## 📝 Verification Checklist

- [x] [Check 1] - [Icon] [RESULT]
- [x] [Check 2] - [Icon] [RESULT]
[... more checks ...]

---

## 🚀 Sign-Off

**Verified By**: [Agent Name]
**Date**: [Date]
**Specification**: [Spec Number and Name]
**Status**: [Icon] **[FINAL STATUS]**
**Confidence**: [Percentage]

**This code is [APPROVED/NOT APPROVED] for [merge/production/etc.].**

---
*Verification Report Created: [Date and Time]*
```

### Example VERIFICATION_SIGNOFF.md
```markdown
# 🎯 RUST VERIFICATION AGENT - FINAL SIGN-OFF REPORT

## Executive Summary

**Specification**: Fix Rust Lints, Checks, and Styling (Specification 01)
**Verification Date**: 2026-01-14
**Verification Agent**: Rust Verification Agent
**Overall Status**: **APPROVED WITH NOTES** ⚠️

---

## ✅ Verification Results

### 1. Clippy Linting - **PASS** ✅
**Result**: **ZERO warnings** on all in-scope crates
**Status**: ✅ **EXCELLENT** - All clippy warnings successfully resolved

### 2. Build Compilation - **PASS** ✅
**Result**: All crates compile successfully
**Status**: ✅ **EXCELLENT** - Clean compilation with zero errors

[... continues with all verification checks ...]

## 🏆 Final Verdict

### **APPROVED WITH NOTES** ⚠️

### Compliance Rating: **9.5/10** ⭐⭐⭐⭐⭐

### Recommendation: **READY FOR MERGE** ✅

This code is **production-ready** for the in-scope crates.

---
*Verification Report Created: 2026-01-14 17:30*
```

## LEARNINGS.md File

### Purpose
Captures lessons learned, insights gained, and knowledge acquired during the specification work. This document helps future agents and developers avoid pitfalls, understand design decisions, and benefit from the experience gained during implementation.

### When to Create
Create LEARNINGS.md when:
- **Work is complete or substantially complete**
- **During or after creating FINAL_REPORT.md**
- **Before closing out the specification**
- You've encountered interesting challenges or gained valuable insights

### Required Contents
```markdown
# [Specification Name] - Learnings

## Overview
Brief summary of what this specification taught us.

## Key Insights

### Technical Insights
[Technical discoveries, patterns learned, best practices identified]

### Process Insights
[What worked well in the workflow, what could be improved]

### Tool Insights
[Learnings about tools used, their strengths/limitations]

## Challenges and Solutions

### Challenge 1: [Name]
**Problem**: [Description of the problem]
**Solution**: [How it was solved]
**Learning**: [What we learned from this]

### Challenge 2: [Name]
**Problem**: [Description]
**Solution**: [How it was solved]
**Learning**: [What we learned]

## Best Practices Discovered
[List of best practices learned during this work]

## Anti-Patterns to Avoid
[Things that didn't work well or should be avoided]

## Recommendations for Future Work

### Similar Specifications
[Advice for anyone working on similar specifications]

### Follow-Up Work
[Suggestions for future improvements or related work]

### Process Improvements
[Ideas for improving the specification/development process]

## Knowledge Gained

### About the Codebase
[New understanding of the codebase structure, patterns, conventions]

### About the Tools
[Insights about tools, their usage, configuration]

### About the Domain
[Domain-specific knowledge gained]

## Documentation Improvements Needed
[Areas where documentation could be improved based on this work]

## Technical Debt Identified
[Any technical debt discovered during this work]

## Success Factors
[What made this specification successful]

---
*Learnings Documented: [Date]*
```

### Example LEARNINGS.md
```markdown
# Rust Lints Fix - Learnings

## Overview
This specification taught us the importance of systematic code quality improvements, the power of Rust's linting tools, and effective strategies for tackling technical debt in a large codebase.

## Key Insights

### Technical Insights
1. **Clippy is extremely thorough** - Even in well-written code, clippy finds subtle improvements
2. **Documentation lints are valuable** - `# Errors` and `# Panics` sections significantly improve API clarity
3. **Type safety conversions** - `try_from()` is always safer than `as` casts for numeric conversions
4. **Pass-by-reference vs pass-by-value** - Rust makes it easy to identify unnecessary ownership transfers

### Process Insights
1. **Phase-based approach works well** - Breaking into 8 phases kept work organized
2. **Verification agent is invaluable** - Caught scope issues before wasting time
3. **Progress reports help** - Mid-work snapshots keep user informed and maintain momentum
4. **Exclude non-compiling code** - Focus on what works first, fix compilation issues separately

### Tool Insights
1. **cargo clippy is fast** - Even on large codebases, clippy runs quickly
2. **Grep patterns help** - Using ripgrep to find unwrap() calls was very effective
3. **Git submodules need care** - Learned to keep specifications out of submodules

## Challenges and Solutions

### Challenge 1: Compilation Errors Blocking Progress
**Problem**: foundation_core and infrastructure had compilation errors that prevented clippy from running
**Solution**: Excluded them from scope and focused only on compiling crates
**Learning**: Don't let broken code block improvements to working code - scope appropriately

### Challenge 2: Git Submodule Complexity
**Problem**: Initially tried to put specifications in .agents submodule, caused git errors
**Solution**: Moved specifications to specifications/ in main repo
**Learning**: Keep specifications close to the code they document, not in submodules

### Challenge 3: Numeric Literal Separators in WASM
**Problem**: Adding underscores to numeric literals broke WASM compilation
**Solution**: Skipped numeric literal changes in WASM crates
**Learning**: Some lint fixes aren't universal - platform-specific considerations matter

## Best Practices Discovered

1. **Always run clippy with pedantic lints** - Catches subtle issues
2. **Fix critical warnings first** - Prioritize by severity and impact
3. **Group similar fixes together** - Makes commits cleaner and review easier
4. **Document as you go** - Add # Errors and # Panics while code is fresh in mind
5. **Verify zero warnings** - Don't stop until clippy is completely happy
6. **Use specification-driven development** - Requirements-first approach prevents scope creep

## Anti-Patterns to Avoid

1. **Don't fix warnings in non-compiling code** - Waste of time, fix compilation first
2. **Don't batch unrelated fixes** - Makes commits hard to review
3. **Don't assume documentation is accurate** - Always verify with review agent
4. **Don't skip verification** - Running clippy at the end is essential
5. **Don't make platform-breaking changes** - Test that fixes work for all targets

## Recommendations for Future Work

### Similar Specifications
- Start with review agent to verify scope
- Break work into clear phases
- Commit after each logical group of fixes
- Create progress reports at 50% mark
- Run verification agent for final sign-off

### Follow-Up Work
- Create separate specification for foundation_wasm (113 warnings)
- Fix compilation errors in foundation_core
- Fix build script issues in infrastructure
- Consider adding more comprehensive documentation

### Process Improvements
- Make PROGRESS.md, FINAL_REPORT.md, VERIFICATION_SIGNOFF.md, and LEARNINGS.md mandatory
- Always use review agent before starting implementation
- Create verification checklist template for Rust projects

## Knowledge Gained

### About the Codebase
- ewe_channels has custom executor implementation
- foundation_nostd uses raw pointer conversions
- WASM code has different linting requirements
- Most code is already high quality, just needed polish

### About the Tools
- cargo clippy has dozens of useful lints
- cargo fmt is reliable and consistent
- ripgrep is perfect for finding code patterns
- git submodules require careful handling

### About the Domain
- Rust's type system catches many errors at compile time
- Documentation lints improve API usability significantly
- Performance-critical code may need lint exceptions
- WASM has different constraints than native code

## Documentation Improvements Needed
- Add more examples to public API documentation
- Document error conditions more thoroughly
- Explain panic conditions clearly
- Add architectural documentation for complex modules

## Technical Debt Identified
- foundation_wasm needs comprehensive linting pass
- foundation_core has compilation errors to fix
- infrastructure build scripts need repair
- Some modules could benefit from more tests

## Success Factors
1. **Clear scope** - Excluding non-compiling code allowed focus
2. **Systematic approach** - 8 phases kept work organized
3. **Verification agent** - Caught issues early
4. **Progress tracking** - PROGRESS.md kept momentum visible
5. **User communication** - Regular updates maintained alignment
6. **Final verification** - Rust Verification Agent provided confidence

---
*Learnings Documented: 2026-01-14*
```

## Module Documentation System (MANDATORY)

### Purpose
The `documentation/` directory provides living, detailed documentation of individual code modules. This ensures agents have a clear, up-to-date understanding of what each module implements, imports, exports, and does **BEFORE** making any changes.

### Why Module Documentation Is Critical

**Problem Without Module Documentation:**
- Agents waste time using Grep/Glob to understand what code does
- Agents miss critical context about module purpose and design
- Agents make changes without understanding full impact
- Documentation in code comments alone isn't comprehensive enough
- No central place to understand module architecture and relationships

**Solution With Module Documentation:**
- Agents read `documentation/[module]/doc.md` first for immediate understanding
- Clear documentation of what module implements, imports, calls, and does
- Architecture diagrams and visual aids when needed
- Line number references to key implementations
- Faster onboarding and safer changes
- Living documentation that stays up-to-date with code

### Context Window Management for Large Documentation

**CRITICAL OPTIMIZATION**: Documentation files can become large (10KB+), which wastes valuable context window space if loaded unnecessarily.

**Main Agent Responsibility:**

When documentation is **too large** to load without wasting context:

1. **Main Agent DOES NOT load large documentation** into its own context
2. **Main Agent delegates to sub-agents** who work with the module
3. **Sub-agents ARE REQUIRED to**:
   - Load the documentation relevant to their work
   - Keep documentation up-to-date as they make changes
   - Update documentation when code changes affect it
   - Report documentation updates to Main Agent

**When Documentation is "Too Large":**
- **Threshold**: Documentation > 8-10KB or 1500+ lines
- **Main Agent Action**: Reference the path, delegate reading to sub-agents
- **Sub-Agent Action**: Load and maintain documentation

**Main Agent Delegation Pattern:**
```
Main Agent identifies module needs work
  ↓
Main Agent sees documentation/[module]/doc.md exists and is large
  ↓
Main Agent DOES NOT load documentation (context optimization)
  ↓
Main Agent spawns Implementation Agent with instruction:
  "Read documentation/[module]/doc.md first, keep it updated"
  ↓
Implementation Agent loads documentation
  ↓
Implementation Agent makes changes to code
  ↓
Implementation Agent updates documentation/[module]/doc.md
  ↓
Implementation Agent reports back with documentation status
```

**Why This Matters:**
- **Context Efficiency**: Main Agent preserves context for orchestration
- **Responsibility Distribution**: Sub-agents maintain docs for modules they work on
- **Documentation Freshness**: Agents updating code also update docs
- **No Documentation Drift**: Updates happen at implementation time

**Main Agent Must Still:**
- ✅ Know documentation exists (reference path in requirements.md)
- ✅ Instruct sub-agents to read and update documentation
- ✅ Verify sub-agents report documentation status
- ❌ NOT load large documentation files into own context

**Sub-Agents Must:**
- ✅ Load relevant documentation before making changes (if size permits)
- ✅ Use Grep, Glob, and Read tools to analyze code details
- ✅ Rely on tools (Grep/ripgrep) for finding specific implementations
- ✅ If documentation too large for sub-agent context: Skip reading, use tools only
- ✅ Report documentation status to Main Agent after changes
- ✅ Keep documentation synchronized with code
- ❌ NOT skip documentation updates "to save time"

**When Documentation is Too Large for Sub-Agent:**

If sub-agent finds documentation is also too large (>8-10KB), sub-agent **MUST**:

1. **Skip Loading Documentation**: Don't waste context on large doc
2. **Rely Entirely on Tools**: Use Grep, Glob, Read to understand code directly
3. **Make Code Changes**: Complete implementation work
4. **Report to Main Agent**:
   ```
   "Changes completed. Documentation at documentation/[module]/doc.md
    is too large (XXkb) for my context. Requesting Documentation Agent
    to update documentation based on my changes."
   ```
5. **Main Agent Response**:
   - Spawns Documentation Agent
   - Documentation Agent reviews code changes
   - Documentation Agent updates documentation
   - Documentation Agent reports completion

**Tool Usage Strategy for Sub-Agents:**

**If documentation is reasonable size (<8-10KB):**
- **Documentation provides**: High-level overview, architecture, module structure
- **Tools provide**: Specific line numbers, exact implementations, current state
- **Pattern**: Read documentation → Use Grep/Glob to find specifics → Make changes → Update documentation

**If documentation is too large (>8-10KB):**
- **Tools provide everything**: Architecture discovery, implementation finding, current state
- **Pattern**: Use Grep/Glob exclusively → Make changes → Report to Main Agent → Main Agent spawns Documentation Agent

### When Module Documentation Is Created

**Initial Creation (After requirements.md Completed):**

Once `specifications/[NN-spec-name]/requirements.md` is documented and user has approved requirements, Main Agent **MUST**:

1. **Identify Affected Modules**: Determine which modules (existing or new) will be modified/created
2. **Spawn Documentation Agent(s)**: Launch specialized documentation agent(s) to create/update module documentation
3. **Parallel Execution When Possible**: If modules are independent, spawn multiple documentation agents in parallel
4. **Sequential When Dependencies Exist**: If modules are highly interdependent, spawn single agent for related modules

**Documentation Agent Responsibilities:**

Documentation agents **MUST**:
1. Read the specification `requirements.md` to understand what needs to be implemented
2. For **NEW modules**: Create new `documentation/[module-name]/doc.md` with initial structure
3. For **EXISTING modules**:
   - Read current `documentation/[module-name]/doc.md` (if exists)
   - Analyze actual module code
   - Compare documentation vs reality
   - Update documentation to match current state
   - Flag discrepancies to Main Agent
4. Document with extreme factual accuracy (see doc.md structure below)
5. Reference documentation path in `requirements.md` (Main Agent will do this after agent reports)

**CRITICAL ASSUMPTION RULE:**

**NEVER assume existing module documentation is complete or accurate.**

Before starting ANY work on a specification:
- Main Agent **MUST** spawn documentation agent to review/update relevant module documentation
- Documentation agent **MUST** verify documentation matches actual code
- Documentation agent **MUST** report discrepancies to Main Agent
- If documentation doesn't match code: STOP ALL WORK, update documentation FIRST

### Documentation Agent Must STOP If Mismatch Found

If documentation agent discovers that `documentation/[module]/doc.md` does NOT match actual module code:

1. **STOP immediately** - do not proceed with implementation
2. **Report to Main Agent** with detailed mismatch findings:
   - What documentation claims
   - What code actually does
   - Specific line numbers and functions affected
   - Severity of mismatch (minor vs critical)
3. **Main Agent MUST**:
   - Halt all specification work
   - Spawn documentation agent(s) to update documentation FIRST
   - Wait for documentation to be corrected and verified
   - ONLY THEN resume specification implementation

**Why This Matters:**
- Prevents implementation based on false assumptions
- Ensures agents always work with accurate information
- Maintains documentation as single source of truth
- Catches documentation drift before it causes problems

### Module Documentation Directory Structure

```
documentation/
├── module-name/
│   ├── doc.md                 # (MANDATORY) Main documentation file
│   ├── diagrams/              # (OPTIONAL) Visual architecture diagrams
│   │   ├── architecture.svg
│   │   ├── flow-diagram.svg
│   │   └── class-diagram.svg
│   └── assets/                # (OPTIONAL) Additional assets
│       ├── screenshots/
│       ├── examples/
│       └── references/
└── ...
```

**Naming Convention:**
- Module directory name should match actual module/package/crate name when possible
- Use lowercase with dashes for multi-word names: `user-auth`, `http-client`, `data-validator`
- Keep names concise but descriptive

### doc.md File Structure

Every `documentation/[module]/doc.md` **MUST** contain:

```markdown
---
module: [Exact module/package/crate name]
language: [rust|javascript|typescript|python|go|etc]
status: [active|deprecated|experimental|planning]
last_updated: [YYYY-MM-DD]
maintainer: [Primary responsible agent or team]
related_specs:
  - specifications/NN-spec-name
  - specifications/MM-spec-name
---

# [Module Name] - Documentation

## Overview
[2-3 sentence summary of what this module does and why it exists]

## Purpose and Responsibility
[Detailed explanation of module's purpose, primary responsibilities, and role in the system]

## Module Location
- **Path**: `[exact file path or directory]`
- **Entry Point**: `[main file or entry point]`
- **Language**: [language and version]
- **Package Manager**: [cargo/npm/pip/go mod/etc]

## What It Implements

### Core Functionality
1. **[Feature/Function Name]** (Line [NNN-MMM])
   - What: [What this feature does]
   - Why: [Why it exists, what problem it solves]
   - How: [Brief explanation of how it works]
   - Key Functions: `function_name()`, `another_function()`

2. **[Another Feature]** (Line [NNN-MMM])
   - What: [Description]
   - Why: [Rationale]
   - How: [Implementation approach]
   - Key Functions: `foo()`, `bar()`

### Public API
**Exported Functions:**
- `function_name(args) -> return_type` (Line NNN): [Purpose and usage]
- `another_function(args) -> return_type` (Line MMM): [Purpose and usage]

**Exported Types/Classes:**
- `TypeName` (Line NNN): [Purpose and fields]
- `ClassName` (Line MMM): [Purpose and methods]

**Exported Constants:**
- `CONSTANT_NAME` (Line NNN): [Purpose and value]

## What It Imports

### External Dependencies
- `dependency-name` (v1.2.3): [Why this dependency is used, what features]
- `another-dependency` (v2.0.0): [Purpose]

### Internal Dependencies
- `internal/module/path`: [What is imported and why]
  - `function_a()`: Used for [purpose]
  - `Type_B`: Used for [purpose]

## What It Calls

### External Function Calls
- **Database Operations**: Calls `db.query()`, `db.insert()` (Lines NNN-MMM)
  - Purpose: [Why these calls are made]
  - Context: [When and how they're used]

- **API Calls**: Calls `http.request()` (Lines NNN-MMM)
  - Purpose: [What APIs are called and why]
  - Context: [Request/response handling]

### Internal Function Calls
- **Helper Functions**: `helper.validate()`, `helper.transform()` (Lines NNN-MMM)
  - Purpose: [What helper functions do]

- **Utility Functions**: `util.format()`, `util.parse()` (Lines NNN-MMM)
  - Purpose: [Utility function usage]

## What It Does (Step-by-Step)

### Primary Workflows

#### Workflow 1: [Workflow Name]
1. **Input**: [What triggers this workflow]
2. **Processing Steps**:
   - Step 1: [Action] (Line NNN)
   - Step 2: [Action] (Line MMM)
   - Step 3: [Action] (Line PPP)
3. **Output**: [What the workflow produces]
4. **Error Handling**: [How errors are handled]

#### Workflow 2: [Another Workflow]
[Same structure as above]

### Edge Cases and Special Handling
- **Case 1**: [Description] (Lines NNN-MMM)
  - Condition: [When this occurs]
  - Handling: [How it's handled]

- **Case 2**: [Description] (Lines NNN-MMM)
  - Condition: [When this occurs]
  - Handling: [How it's handled]

## Architecture

### Design Patterns Used
- **Pattern Name**: [How and why this pattern is used]
- **Another Pattern**: [Explanation]

### Module Structure
```
module-directory/
├── main_file.ext         # [Purpose]
├── submodule_a.ext       # [Purpose]
├── submodule_b.ext       # [Purpose]
├── tests/                # [Test organization]
└── docs/                 # [Additional docs]
```

### Diagrams
[Include or reference architecture diagrams if they exist in diagrams/ directory]

## Key Implementation Details

### Performance Considerations
- [Any performance-critical code or optimization notes]
- Line references: [NNN-MMM]

### Security Considerations
- [Any security-relevant implementations]
- Line references: [NNN-MMM]

### Concurrency/Async Handling
- [How concurrency is handled]
- Line references: [NNN-MMM]

## Tests

### Test Coverage
- **Unit Tests**: `[test file path]`
  - Coverage: [XX%]
  - Key test cases: [List important tests]

- **Integration Tests**: `[test file path]`
  - Coverage: [XX%]
  - Key test cases: [List important tests]

### Testing Strategy
- [Explanation of how module is tested]
- [Any special testing considerations]

## Dependencies and Relationships

### Depends On
- **Module A**: [Why this dependency exists]
- **Module B**: [Purpose of dependency]

### Used By
- **Module C**: [How this module is used]
- **Module D**: [Purpose of usage]

### Sibling Modules
- **Module E**: [Related module and relationship]

## Configuration

### Environment Variables
- `ENV_VAR_NAME`: [Purpose and default value]
- `ANOTHER_VAR`: [Purpose and default value]

### Configuration Files
- `config.json`: [What is configured]
- `.env`: [Environment-specific settings]

## Known Issues and Limitations

### Current Limitations
1. **[Limitation Name]**: [Description and workaround]
2. **[Another Limitation]**: [Description]

### Known Bugs
- **[Bug Description]**: [Impact and status] (Issue #NNN)

### Technical Debt
- **[Debt Item]**: [Description and plan for resolution]

## Future Improvements

### Planned Enhancements
- **[Enhancement Name]**: [Description and priority]
- **[Another Enhancement]**: [Description and timeline]

### Refactoring Opportunities
- **[Refactor Item]**: [Why and when it should be done]

## Related Documentation

### Specifications
- [Link to related specifications]

### External Resources
- [Links to relevant documentation, RFCs, articles]

### Related Modules
- [Links to related module documentation]

## Version History

### [Version X.Y.Z] - YYYY-MM-DD
- [Major changes in this version]
- [Breaking changes if any]
- [Bug fixes]

### [Version X.Y.Z-1] - YYYY-MM-DD
- [Previous version changes]

---
*Last Updated: [Date]*
*Documentation Version: [Version]*
```

### Referencing Module Documentation in requirements.md

After documentation agent(s) complete module documentation, Main Agent **MUST** update `specifications/[NN-spec-name]/requirements.md` to reference the module documentation:

**Add to requirements.md:**
```markdown
## Module Documentation References

This specification modifies the following modules:

### [Module Name 1]
- **Documentation**: `documentation/[module-1]/doc.md`
- **Purpose**: [Brief summary]
- **Changes Needed**: [What will be changed in this module]

### [Module Name 2]
- **Documentation**: `documentation/[module-2]/doc.md`
- **Purpose**: [Brief summary]
- **Changes Needed**: [What will be changed in this module]

**CRITICAL**: Agents MUST read module documentation BEFORE making changes.
```

This creates a clear connection between specifications and the modules they affect.

### Implementation Agent Workflow With Module Documentation

When implementation agent is spawned for a specification, they **MUST**:

1. **Read Specification Files**:
   - `specifications/[NN-spec-name]/requirements.md`
   - `specifications/[NN-spec-name]/tasks.md`

2. **Read Module Documentation** (if referenced in requirements.md):
   - Locate module documentation paths in requirements.md
   - Read **ALL** `documentation/[module]/doc.md` files for affected modules
   - Understand what module currently does BEFORE making changes
   - Verify understanding matches actual code (spot check key functions)

3. **Report Discrepancies Immediately**:
   - If module documentation doesn't match actual code: STOP
   - Report mismatch to Main Agent with details
   - Wait for Main Agent to resolve (spawn documentation agent to fix)

4. **Implement Changes**:
   - Make changes with full context of module's current state
   - Update module documentation as part of implementation if structure changes
   - Keep documentation synchronized with code changes

5. **Report Completion**:
   - Notify Main Agent of completed work
   - Note if module documentation needs updates due to changes

### Documentation Agent Workflow

Documentation agents are specialized agents spawned by Main Agent to create/update module documentation.

**Agent Type**: Documentation Agent (should be documented in `.agents/agents/documentation.md`)

**When Spawned**:
- After requirements.md is completed (before implementation)
- When module documentation needs verification/update
- When documentation mismatch is reported
- When new modules are created

**Responsibilities**:
1. **Read Specification**: Understand what will be implemented
2. **Analyze Module Code**:
   - Use Glob to find module files
   - Use Grep to search for key functions/types
   - Read key files to understand implementation
3. **Create/Update doc.md**:
   - Fill in all sections with factual accuracy
   - Include line number references
   - Document all imports, exports, calls, and workflows
   - Add diagrams if helpful (in diagrams/ directory)
4. **Verify Accuracy**: Cross-reference documentation against actual code
5. **Report to Main Agent**:
   - Completion status
   - Path to documentation
   - Any discrepancies found
   - Recommendation (GO/STOP/UPDATE)

**Documentation Agent MUST NOT**:
- ❌ Make assumptions about code without reading it
- ❌ Copy stale documentation without verification
- ❌ Document planned features (only document what EXISTS)
- ❌ Skip line number references
- ❌ Assume documentation is complete

### Parallelization Strategy

**When to Spawn Multiple Documentation Agents (Parallel):**
- Modules are independent with minimal interdependencies
- Different modules in different directories/packages
- No shared state or tight coupling

**When to Spawn Single Documentation Agent (Sequential):**
- Modules are highly interdependent
- Changes in one module affect others
- Shared types/interfaces across modules
- Need to document relationships between modules

Main Agent makes the decision based on specification requirements.

### Documentation Maintenance

**When to Update Module Documentation**:
1. **Before Work Begins**: Verify documentation matches reality
2. **During Implementation**: Update if module structure changes significantly
3. **After Verification Passes**: Ensure documentation reflects final state
4. **When Bugs Found**: Document bug fixes and why they were needed
5. **When Refactoring**: Update to reflect new architecture

**Who Updates**:
- **Documentation Agent**: For major reviews/updates (spawned by Main Agent)
- **Implementation Agent**: For minor updates during implementation (if structure changes)
- **Main Agent**: Never updates directly, always delegates to agents

### Integration with Specification Workflow

Updated workflow with module documentation:

```
1. User Requests Feature
   ↓
2. Main Agent Conversation with User
   ↓
3. Create Specification Directory
   ↓
4. Document Requirements (requirements.md)
   ↓
4.5 **SPAWN DOCUMENTATION AGENT(S)** (NEW STEP)
   ├─ Main Agent identifies affected modules
   ├─ Spawns documentation agent(s) for each module
   ├─ Documentation agents create/update documentation/[module]/doc.md
   ├─ Documentation agents verify accuracy
   ├─ Documentation agents report completion or discrepancies
   ├─ If discrepancies: STOP, fix documentation, then continue
   ├─ Main Agent updates requirements.md with module doc references
   └─ Main Agent commits module documentation
   ↓
5. Create Task List (tasks.md)
   ↓
6. Update Spec.md Master Index
   ↓
7. Commit Specification Files
   ↓
8. LAUNCH REVIEW AGENT (reads requirements.md, tasks.md, AND module docs)
   ↓
9. Launch Implementation Agents
   ├─ Agents read requirements.md
   ├─ Agents read tasks.md
   ├─ **Agents read module documentation** (NEW)
   ├─ Agents verify module docs match reality
   ├─ If mismatch: STOP, report to Main Agent
   ├─ Agents implement with full context
   └─ Agents update module docs if structure changes
   ↓
10-17. [Continue with existing workflow: PROGRESS.md, FINAL_REPORT.md, etc.]
```

### Enforcement - Zero Tolerance

**Violations:**
- ❌ **Starting implementation without module documentation** (if module exists)
- ❌ **Not verifying module documentation accuracy before work**
- ❌ **Assuming module documentation is complete/correct without verification**
- ❌ **Proceeding when documentation doesn't match code**
- ❌ **Not updating module documentation when structure changes**
- ❌ **Creating vague or incomplete module documentation**
- ❌ **Documenting planned features instead of existing code**
- ❌ **Omitting line number references for key implementations**

**Consequences:**
- **Wasted time**: Implementing based on false assumptions
- **Bugs introduced**: Misunderstanding current code leads to breaking changes
- **Confusion**: Future agents can't understand what code does
- **Documentation drift**: Docs become useless when they don't match reality

**THE USER WILL BE VERY UPSET** if agents work without reading/verifying module documentation!

### Examples

**Example 1: Creating Module Documentation for New Module**

```
Specification: 03-user-authentication
Requirements: Implement JWT-based authentication system

Main Agent workflow:
1. Requirements documented in specifications/03-user-authentication/requirements.md
2. Main Agent identifies: NEW module "user-auth" will be created
3. Main Agent spawns Documentation Agent with context:
   - Specification: 03-user-authentication
   - Module: user-auth (NEW)
   - Task: Create initial documentation structure

Documentation Agent:
1. Creates documentation/user-auth/ directory
2. Creates documentation/user-auth/doc.md with:
   - Frontmatter (status: planning)
   - Overview: "Handles JWT token generation and validation"
   - Module Location: src/auth/ (planned)
   - Sections left as placeholders (module doesn't exist yet)
   - Note: "Module not yet implemented, documentation will be updated during implementation"
3. Reports to Main Agent: "Initial documentation structure created"

Main Agent:
1. Updates requirements.md with reference:
   ```markdown
   ## Module Documentation References
   ### User Auth
   - **Documentation**: `documentation/user-auth/doc.md`
   - **Purpose**: JWT authentication (new module)
   - **Changes Needed**: Full implementation from scratch
   ```
2. Commits documentation
3. Continues with task list creation

Implementation Agent (later):
1. Reads requirements.md, sees module doc reference
2. Reads documentation/user-auth/doc.md (sees it's planning status)
3. Implements authentication module
4. Updates documentation/user-auth/doc.md as implementation progresses:
   - Changes status: planning → active
   - Fills in all sections with actual implementation details
   - Adds line numbers for key functions
   - Documents actual imports, exports, calls
5. Reports completion to Main Agent

✅ Documentation created before implementation
✅ Documentation updated during implementation
✅ Living documentation stays synchronized with code
```

**Example 2: Verifying Existing Module Documentation**

```
Specification: 05-add-caching-layer
Requirements: Add Redis caching to API responses

Main Agent workflow:
1. Requirements documented
2. Main Agent identifies: EXISTING module "http-client" will be modified
3. Main Agent spawns Documentation Agent:
   - Specification: 05-add-caching-layer
   - Module: http-client (EXISTING)
   - Task: Verify documentation matches current code

Documentation Agent:
1. Reads documentation/http-client/doc.md (created 2 months ago)
2. Globs for http-client files: src/http/*.ts
3. Greps for key functions mentioned in doc.md
4. Reads main implementation files
5. **DISCOVERS MISMATCH**:
   - doc.md says: "Uses axios library for requests"
   - Code actually uses: native fetch API (axios was removed)
   - doc.md line numbers are outdated (code was refactored)
   - doc.md missing: recently added retry logic
6. Reports to Main Agent: "STOP - Documentation does not match code"

Documentation Agent Report:
---
STATUS: STOP
REASON: Documentation out of date, does not match current code

MISMATCHES FOUND:
- Documentation claims axios library is used (Line 45 reference)
  Reality: Code uses native fetch API (axios removed in commit abc123)

- Documentation line numbers outdated
  Example: Claims makeRequest() is at Line 120
  Reality: makeRequest() is at Line 89

- Documentation missing retry logic
  Reality: Retry logic added in commit def456

RECOMMENDATION: Update documentation before proceeding with caching implementation

Main Agent actions:
1. STOPS all work on specification (does not spawn implementation agents)
2. Spawns Documentation Agent again with task: "Update documentation to match reality"
3. Documentation Agent:
   - Updates all sections
   - Corrects library references
   - Updates line numbers
   - Documents retry logic
   - Verifies everything matches code
   - Reports: "Documentation updated and verified"
4. Main Agent commits updated documentation
5. Main Agent updates requirements.md with correct module doc reference
6. NOW Main Agent continues with implementation

✅ Caught documentation drift BEFORE implementation
✅ Prevented implementation based on false assumptions
✅ Ensured agents have accurate information
✅ No time wasted on wrong approach
```

**Example 3: Documentation Mismatch Found by Implementation Agent**

```
Specification: 07-add-payment-webhooks
Implementation Agent spawned to add webhook handling

Implementation Agent workflow:
1. Reads specifications/07-add-payment-webhooks/requirements.md
2. Sees module reference: documentation/payment-processor/doc.md
3. Reads documentation/payment-processor/doc.md
4. Documentation says: "Payment validation in validatePayment() at Line 234"
5. Implementation Agent checks code:
   - Greps for validatePayment
   - Finds it at Line 312 (not 234)
   - Reads function - logic is different than documented
6. **STOPS IMMEDIATELY**
7. Reports to Main Agent:

Implementation Agent Report:
---
I cannot proceed with webhook implementation.

ISSUE: Module documentation does not match actual code.

DISCREPANCY:
- documentation/payment-processor/doc.md claims:
  * validatePayment() at Line 234
  * Function validates amount and currency

- Actual code shows:
  * validatePayment() at Line 312
  * Function validates amount, currency, AND merchant ID
  * Additional validation logic not documented

This affects my implementation of webhooks because I need to understand the full validation process.

REQUEST: Please update payment-processor documentation before I continue.

Main Agent actions:
1. Acknowledges the issue
2. Spawns Documentation Agent to update documentation/payment-processor/doc.md
3. Documentation Agent:
   - Corrects line number
   - Documents full validation logic including merchant ID check
   - Verifies all other sections
   - Reports: "Documentation corrected"
4. Main Agent commits updated documentation
5. Main Agent resumes Implementation Agent: "Documentation updated, please continue"
6. Implementation Agent:
   - Re-reads documentation (now correct)
   - Implements webhooks with accurate understanding
   - Successfully completes work

✅ Implementation Agent caught mismatch during work
✅ Stopped immediately instead of proceeding with false assumptions
✅ Main Agent resolved issue by updating documentation
✅ Work resumed with accurate information
✅ Final implementation is correct
```

## tasks.md File

### Purpose
Tracks all tasks required to complete the specification using markdown checkboxes.

### File Structure with Frontmatter
```markdown
---
completed: 5
uncompleted: 3
tools:
  - TypeScript
  - Jest
  - ESLint
  - Prettier
---

# [Specification Name] - Tasks

## Task List

### Implementation Tasks
- [x] Create base HttpClient class
- [x] Implement GET method
- [x] Implement POST method
- [x] Implement PUT method
- [x] Implement DELETE method
- [ ] Add request interceptor middleware
- [ ] Add response interceptor middleware
- [ ] Implement retry logic

### Testing Tasks
- [x] Write unit tests for GET method
- [ ] Write unit tests for POST method
- [ ] Write integration tests

### Documentation Tasks
- [x] Write API documentation
- [ ] Add usage examples
- [ ] Create migration guide

## Notes
- Retry logic needs configuration for max attempts and backoff strategy
- Middleware system should support async middleware functions
- Consider adding request timeout configuration

---
*Last Updated: 2026-01-11*
```

### Frontmatter Fields

#### Required Fields
- **completed**: Total count of completed tasks (checkbox count with `[x]`)
- **uncompleted**: Total count of uncompleted tasks (checkbox count with `[ ]`)
- **tools**: List of tools, skills, and MCP tools required or used

#### Counting Rules
- Count must match actual number of checkboxes in the file
- Update counts every time task status changes
- Use search/count to verify accuracy

### Checkbox Format
- Uncompleted task: `- [ ] Task description`
- Completed task: `- [x] Task description`
- Use consistent spacing (dash, space, bracket, space/x, bracket, space, description)

### Task Management Rules
1. **Before starting work**: Add task to tasks.md
2. **During work**: Keep task as `[ ]` uncompleted
3. **After completing**: Change to `[x]` completed
4. **Update frontmatter**: Adjust completed/uncompleted counts
5. **Update tools**: Add any new tools used

## Pre-Work Review Agent Requirement

### MANDATORY REVIEW AGENT REQUIREMENT

Before **ANY** agent starts work on tasks within a specification, a **review agent MUST be launched first**. This is a **HARD REQUIREMENT** with **NO EXCEPTIONS**.

#### Review Agent Purpose

The review agent's role is to:

1. **Read Specification Files Thoroughly**
   - Read the `requirements.md` file completely
   - Read the `tasks.md` file completely
   - Understand all requirements and expected outcomes

2. **Analyze Current Codebase Implementation**
   - Search the codebase using Glob and Grep tools
   - Locate all relevant files and implementations
   - Analyze code quality and completeness
   - Verify actual implementation state

3. **Compare Reality vs Documentation**
   - Compare codebase reality against requirements.md
   - Compare actual implementation against tasks.md status
   - Verify completion claims in checkboxes
   - Identify any discrepancies or inconsistencies

4. **Verify Task Status Accuracy**
   - Check each task marked as `[x]` completed
   - Verify completed tasks are actually implemented in code
   - Check each task marked as `[ ]` pending
   - Verify pending tasks are truly not yet implemented
   - Identify tasks incorrectly marked as complete
   - Identify tasks incorrectly marked as pending

5. **Identify Issues and Blockers**
   - Find unclear or ambiguous requirements
   - Identify incomplete or vague specifications
   - Detect inconsistencies between requirements and tasks
   - Spot missing information that needs user clarification
   - Flag potential technical blockers

6. **Assess Work Readiness**
   - Determine if specifications are clear enough to proceed
   - Verify all necessary information is available
   - Check if tasks are well-defined and actionable
   - Assess if work can begin without further user input

#### STOP WORK IF

The review agent **MUST STOP ALL WORK** and report to the main agent if:

- ❌ **Inconsistencies found**: Tasks marked complete but code doesn't exist
- ❌ **Inconsistencies found**: Tasks marked pending but code already exists
- ❌ **Requirements unclear**: Specifications are vague or ambiguous
- ❌ **Requirements incomplete**: Critical information is missing
- ❌ **Tasks need refinement**: Task descriptions are unclear or not actionable
- ❌ **User input required**: Decisions need to be made by the user
- ❌ **Conflicting information**: Requirements contradict each other
- ❌ **Technical blockers**: Dependencies or prerequisites are missing

#### Report to Main Agent

The review agent **MUST** report back with:

1. **Current Implementation State**
   - What's actually implemented in the codebase
   - What files and components exist
   - What functionality is working
   - What's missing or incomplete

2. **Verified Task Status**
   - Accurate completion status for each task
   - Tasks that need status corrections
   - Discrepancies between documentation and reality
   - Recommended status updates

3. **Inconsistencies Found**
   - Specific tasks incorrectly marked
   - Missing implementations claimed as complete
   - Code existing for "pending" tasks
   - Any conflicts or contradictions

4. **Readiness Assessment**
   - **GO**: Work can proceed (all clear, no issues)
   - **STOP**: Work cannot proceed (issues require resolution)
   - **CLARIFY**: User input needed before proceeding
   - List of blockers or clarifications needed

5. **Recommendations**
   - Suggested corrections to tasks.md
   - Recommended requirement clarifications
   - Priority order for implementing tasks
   - Technical approach suggestions

#### Enforcement - Zero Tolerance

**This requirement has ZERO TOLERANCE for violations.**

- ❌ **FORBIDDEN**: Starting work without running review agent first
- ❌ **FORBIDDEN**: Skipping review agent "to save time"
- ❌ **FORBIDDEN**: Assuming specifications are accurate without verification
- ❌ **FORBIDDEN**: Trusting task status without codebase verification
- ❌ **FORBIDDEN**: Proceeding when review agent reports STOP or CLARIFY

**VIOLATION CONSEQUENCES:**

Any agent that violates this requirement will:
1. Be immediately stopped
2. Have their work discarded
3. Require the review agent to be run properly
4. Report the violation to the user

#### Integration with Workflow

The review agent requirement is **Step 0** - it happens **BEFORE** any implementation work:

```
Step 0: LAUNCH REVIEW AGENT (MANDATORY)
   ├─ Review agent reads requirements.md and tasks.md
   ├─ Review agent searches and analyzes codebase
   ├─ Review agent verifies task completion status
   ├─ Review agent identifies inconsistencies
   ├─ Review agent reports back with readiness assessment
   └─ Main agent evaluates report
      ↓
      IF report says "GO":
         → Proceed to Step 1 (launch implementation agents)
      IF report says "STOP" or "CLARIFY":
         → DO NOT PROCEED
         → Address issues or get user input
         → May need to run review agent again
```

## Workflow

### Complete Requirements-to-Implementation Workflow

```
1. User Requests Feature
   ↓
2. Main Agent Conversation with User
   ├─ Ask clarifying questions
   ├─ Understand full requirements
   ├─ Confirm technical approach
   └─ Get user agreement
   ↓
3. Create Specification Directory
   ├─ Determine next number (e.g., 04)
   ├─ Create directory: specifications/04-feature-name/
   └─ Create both requirements.md and tasks.md files
   ↓
4. Document Requirements
   ├─ Fill in requirements.md frontmatter
   ├─ Write conversation summary
   ├─ List detailed requirements
   └─ Include agent notes
   ↓
4.5 Create/Update Module Documentation (MANDATORY - NEW STEP)
   ├─ Main Agent identifies which modules (existing or new) are affected
   ├─ For each module:
   │  ├─ Spawn Documentation Agent (parallel if independent, sequential if interdependent)
   │  ├─ Documentation Agent analyzes existing code (if module exists)
   │  ├─ Documentation Agent creates/updates documentation/[module]/doc.md
   │  ├─ Documentation Agent verifies accuracy against actual code
   │  ├─ Documentation Agent reports completion or STOP if mismatch found
   │  └─ If STOP: Fix documentation first, then continue
   ├─ Main Agent updates requirements.md with module documentation references
   ├─ Main Agent commits module documentation
   └─ Ensures all module docs are accurate before proceeding
   ↓
5. Create Task List
   ├─ Fill in tasks.md frontmatter
   ├─ Break down work into tasks
   ├─ List all tools needed
   └─ All tasks start as [ ] uncompleted
   ↓
6. Update Spec.md Master Index
   ├─ Add new specification to list
   ├─ Update status dashboard counts
   └─ Link to new specification directory
   ↓
7. Commit Specification Files
   ├─ git add specifications/
   ├─ git commit (following Rule 03)
   ├─ git push (following Rule 05)
   └─ Verify success
   ↓
8. LAUNCH REVIEW AGENT (MANDATORY - NEW STEP)
   ├─ Review agent reads requirements.md thoroughly
   ├─ Review agent reads tasks.md thoroughly
   ├─ Review agent searches codebase for implementations
   ├─ Review agent verifies task completion accuracy
   ├─ Review agent identifies inconsistencies
   ├─ Review agent reports readiness: GO / STOP / CLARIFY
   └─ Main agent evaluates review report
      ↓
      IF GO: Continue to Step 9
      IF STOP/CLARIFY: Address issues, get user input, re-review if needed
   ↓
9. Launch Specialized Agents (Rule 04)
   ├─ Agents MUST read requirements.md
   ├─ Agents MUST read tasks.md
   ├─ Agents MUST read review agent's report
   ├─ **Agents MUST read module documentation** (if referenced in requirements.md)
   ├─ **Agents MUST verify module docs match reality** (spot check)
   ├─ **If mismatch found: STOP, report to Main Agent**
   ├─ Agents work on tasks based on verified status
   ├─ Agents follow review agent's recommendations
   └─ Agents update module docs if structure changes during implementation
   ↓
10. Agent Updates During Work
    ├─ Add new tasks BEFORE starting work on them
    ├─ Update task checkboxes as work completes
    ├─ Update frontmatter counts
    ├─ Update tools list
    └─ Commit changes to tasks.md after updates
   ↓
11. Create PROGRESS.md (MANDATORY - NEW)
    ├─ Create at ~40-60% completion or mid-work checkpoint
    ├─ Document completed work, current status, remaining work
    ├─ Include statistics and any blockers
    ├─ Commit PROGRESS.md
    └─ Provides user with mid-work visibility
   ↓
12. Complete All Implementation Work
    ├─ Finish all remaining tasks
    ├─ Update tasks.md to reflect completion
    ├─ Search codebase to verify all tasks truly complete
    └─ Ensure all code works and tests pass
   ↓
13. Create FINAL_REPORT.md (MANDATORY - NEW)
    ├─ Create when all tasks are 100% complete
    ├─ Document comprehensive summary of all work
    ├─ Include detailed statistics and achievements
    ├─ List all commits created
    ├─ Document before/after impact
    ├─ Commit FINAL_REPORT.md
    └─ Official record of completion
   ↓
14. Create LEARNINGS.md (MANDATORY - NEW)
    ├─ Create when work is substantially complete
    ├─ Document lessons learned and insights gained
    ├─ Capture challenges, solutions, and best practices
    ├─ Include recommendations for future work
    ├─ Identify technical debt discovered
    ├─ Commit LEARNINGS.md
    └─ Knowledge transfer for future work
   ↓
15. Run Final Verification (MANDATORY)
    ├─ Launch verification agent (e.g., Rust Verification Agent)
    ├─ Verification agent checks all requirements met
    ├─ Verification agent validates code quality
    ├─ Verification agent provides sign-off report
    └─ Receive GO/STOP/CLARIFY assessment
   ↓
16. Create VERIFICATION_SIGNOFF.md (MANDATORY - NEW)
    ├─ Create after verification agent completes
    ├─ Contains official verification report
    ├─ Includes compliance rating and confidence level
    ├─ Documents all verification checks performed
    ├─ Provides final APPROVED/REJECTED status
    ├─ Commit VERIFICATION_SIGNOFF.md
    └─ Official sign-off for merge/production
   ↓
17. Final Status Updates
    ├─ Update requirements.md: status: uncompleted → completed
    ├─ Update tasks.md: ensure all checkboxes and counts accurate
    ├─ Update Spec.md master index with completion status
    ├─ Commit all final updates
    └─ Final commit and push
```

## Verification Requirements

### Critical: Verify Actual Implementation
Agents **MUST NOT** trust the status field or task checkboxes blindly. Instead, agents **MUST**:

1. **Search the Codebase**
   - Use Glob to find relevant files
   - Use Grep to search for specific implementations
   - Read key files to verify functionality exists

2. **Verify Implementation Quality**
   - Code exists and is not just a stub
   - Tests are present and passing
   - Documentation is complete
   - Integration points work correctly

3. **Update Status Accordingly**
   - Mark tasks as `[x]` only if truly implemented
   - Change tasks back to `[ ]` if implementation is missing or incomplete
   - Update frontmatter counts to match reality
   - Update requirements.md status field if specification is complete

4. **Report Discrepancies**
   - If status says "completed" but implementation is missing, report this
   - If tasks are marked done but code doesn't exist, correct it
   - User should know if specifications are inaccurate

### Example Verification Process
```
Agent reads specification 02-user-authentication/tasks.md
Task says: "[x] Implement JWT token generation"

Agent verification steps:
1. Glob for: "**/*auth*.ts" or "**/*jwt*.ts"
2. Grep for: "generateToken|createToken|signJWT"
3. Read found files to verify implementation
4. Check tests exist and pass
5. If implementation exists and works:
   → Keep as [x] completed
6. If implementation missing or broken:
   → Change to [ ] uncompleted
   → Update frontmatter count
   → Add note about what needs fixing
```

## Examples

### Good Practice ✅

**Example 1: Starting New Specification with Review Agent**
```
User: "I need a caching layer for API responses"

Main Agent:
1. "Let me clarify some requirements..."
   - What caching strategy? (LRU, TTL-based, etc.)
   - Where should cache be stored? (memory, Redis, etc.)
   - What should be cached? (all responses, specific endpoints?)
   - What's the invalidation strategy?

2. User provides answers

3. Agent creates:
   - specifications/05-api-response-caching/
   - requirements.md with full conversation summary
   - tasks.md with all identified tasks
   - Updates Spec.md with new specification

4. Agent commits specification files

5. Agent launches REVIEW AGENT FIRST (MANDATORY)
   - Review agent reads requirements.md and tasks.md
   - Review agent searches codebase for any existing caching code
   - Review agent verifies no conflicting implementations exist
   - Review agent confirms all requirements are clear
   - Review agent reports: "GO - specifications clear, no blockers"

6. Agent launches specialized agents to implement
   - Each agent reads requirements.md and tasks.md first
   - Each agent reads review agent's report
   - Agents verify status by searching codebase
   - Agents update tasks.md as they work

✅ Requirements documented before work began
✅ Full conversation captured
✅ Clear task list created
✅ Review agent verified readiness BEFORE implementation
✅ Agents have clear direction
✅ No wasted effort on unclear requirements
```

**Example 2: Review Agent Finding Inconsistencies**
```
Main agent assigned to work on specification 03-database-migrations

Main agent launches REVIEW AGENT FIRST (Step 0):

Review Agent process:
1. Reads requirements.md thoroughly
2. Reads tasks.md (shows 5 tasks marked [x] completed, 2 tasks marked [ ] pending)
3. Searches codebase:
   - Globs for "**/migrations/*.ts"
   - Greps for "migrate|migration|schema"
   - Reads migration files to verify implementation
4. FINDS INCONSISTENCIES:
   - Task "Create migration CLI command" marked [x] but no CLI file exists
   - Task "Add rollback functionality" marked [x] but code is incomplete/stub only
   - Task "Write migration docs" marked [ ] but docs/migrations.md already exists
5. Reports to main agent: "STOP - Found 3 inconsistencies between tasks and code"

Review Agent Report:
---
STATUS: STOP
REASON: Task status does not match codebase reality

INCONSISTENCIES FOUND:
- ❌ Task "Create migration CLI command" marked complete but src/cli/migrate.ts does not exist
- ❌ Task "Add rollback functionality" marked complete but implementation is stub only
- ✅ Task "Write migration docs" marked pending but docs/migrations.md exists and is complete

RECOMMENDED CORRECTIONS:
- Change "Create migration CLI command" from [x] to [ ]
- Change "Add rollback functionality" from [x] to [ ]
- Change "Write migration docs" from [ ] to [x]

READINESS: Cannot proceed until tasks.md is corrected to reflect reality

Main Agent actions:
1. Does NOT launch implementation agents (review said STOP)
2. Corrects tasks.md based on review agent findings:
   - Updates 3 task statuses
   - Updates frontmatter: completed: 5 → 3, uncompleted: 2 → 4
   - Adds note explaining corrections
3. Commits the corrections
4. NOW launches implementation agents to work on actual pending tasks

✅ Review agent caught inaccurate status BEFORE work started
✅ Prevented wasted effort on wrong tasks
✅ Corrected documentation to match reality
✅ Implementation agents get accurate picture
✅ No confusion about what's actually done
```

**Example 3: Adding Tasks During Work**
```
Agent working on specification 04-file-upload-system

Agent discovers additional work needed:
1. Opens tasks.md
2. Adds new tasks BEFORE starting work:
   - [ ] Add file size validation
   - [ ] Implement virus scanning integration
   - [ ] Add progress tracking events
3. Updates frontmatter: uncompleted: 8 → 11
4. Commits tasks.md update
5. Begins implementation of new tasks

✅ Added tasks before starting work
✅ Updated counts immediately
✅ Clear record of scope expansion
```

**Example 4: Review Agent Requesting Clarification**
```
Main agent working on specification 06-payment-integration

Main agent launches REVIEW AGENT FIRST:

Review Agent process:
1. Reads requirements.md thoroughly
2. Reads tasks.md
3. Searches codebase for existing payment code
4. IDENTIFIES UNCLEAR REQUIREMENTS:
   - Requirements mention "payment processing" but don't specify which provider (Stripe, PayPal, etc.)
   - Tasks include "Handle payment failures" but no retry strategy defined
   - Requirements say "secure payment handling" but no PCI compliance requirements listed
   - No mention of webhook handling or event processing
5. Reports to main agent: "CLARIFY - Requirements need user input"

Review Agent Report:
---
STATUS: CLARIFY
REASON: Critical information missing, user input required

CLARIFICATION NEEDED:
1. Which payment provider should be used? (Stripe, PayPal, Square, etc.)
2. What should happen on payment failure? (retry strategy, user notification)
3. What PCI compliance level is required?
4. Should webhook events be processed? If so, which events?
5. Are recurring payments needed or one-time only?
6. What currencies need to be supported?

RECOMMENDATION: Do not proceed with implementation until user clarifies these points

Main Agent actions:
1. Does NOT launch implementation agents (review said CLARIFY)
2. Reports to user with questions from review agent
3. Has conversation with user to get answers
4. Updates requirements.md with clarified information
5. Updates tasks.md with new tasks based on clarifications
6. Commits updated specification
7. Runs review agent AGAIN to verify clarity
8. Review agent now reports "GO"
9. NOW launches implementation agents

✅ Review agent caught ambiguous requirements BEFORE implementation
✅ Prevented implementation of wrong solution
✅ Got user input on critical decisions
✅ Specifications now clear and actionable
✅ No wasted development effort
```

**Example 5: Updating Spec.md Dashboard**
```
Agent completes specification 02-user-authentication

Agent process:
1. Verifies all tasks are truly complete (by searching codebase)
2. Updates requirements.md: status: uncompleted → completed
3. Updates Spec.md:
   - Changes status to ✅ Completed
   - Updates dashboard counts
   - Moves from "In Progress" to "Completed" section
4. Commits all changes
5. Reports completion to user

✅ Verified completion before marking done
✅ Updated all relevant files
✅ Dashboard reflects accurate status
```

### Bad Practice ❌

**Example 1: Starting Work Without Review Agent**
```
User: "Implement user authentication"

Main Agent:
Creates specification with requirements.md and tasks.md
Commits specification files
Immediately launches implementation agents WITHOUT review agent

Implementation agents start coding based on tasks.md

Problems:
- tasks.md shows some tasks as [x] completed
- Agents assume those are actually done
- Agents start working on "pending" tasks
- Later discover "completed" tasks aren't actually complete
- Wasted 3 hours implementing features that depend on missing code
- Had to stop and backtrack
- User frustrated by delays and confusion

❌ CRITICAL VIOLATION: Skipped mandatory review agent
❌ No verification of task status before starting
❌ Assumed documentation was accurate
❌ Wasted significant development time
❌ User experience negatively impacted
❌ Could have been prevented by running review agent first
```

**Example 2: Starting Work Without Requirements**
```
User: "Add user authentication"

Main Agent:
Launches agents immediately to implement authentication

❌ No requirements conversation
❌ No specification directory created
❌ No documented requirements
❌ No review agent (because no spec exists to review!)
❌ Agents don't know what to implement
❌ No task tracking
❌ User expectations may not be met
```

**Example 3: Proceeding When Review Agent Says STOP**
```
Main agent working on specification 07-email-system

Review Agent runs and reports:
"STATUS: STOP - Found 5 tasks marked complete but implementations are missing"

Main Agent IGNORES report and launches implementation agents anyway

Implementation agents:
- Read tasks.md and see tasks marked [x] complete
- Assume those are done and skip them
- Work on remaining tasks
- Build features that depend on "completed" code
- Features break because dependencies don't exist
- Spend hours debugging "why isn't X working?"
- Eventually realize the completed tasks weren't actually done

User is upset: "Why did you waste time? Why didn't you verify first?"

❌ CRITICAL VIOLATION: Ignored review agent STOP directive
❌ Proceeded despite knowing status was inaccurate
❌ Caused massive waste of development time
❌ Built features on false assumptions
❌ User trust damaged
❌ This is exactly what review agent is designed to prevent
```

**Example 4: Trusting Status Without Verification**
```
Agent reads specification 05-api-caching
tasks.md shows: "[x] Implement Redis cache adapter"

Agent assumes it's done and moves to next task

But actually:
- No Redis adapter exists in codebase
- Previous agent marked it done incorrectly
- Implementation is missing

❌ Didn't verify actual implementation
❌ Assumed checkbox was accurate
❌ Failed to search codebase
❌ Left incomplete work as "completed"
```

**Example 3: Not Adding Tasks Before Work**
```
Agent working on specification 06-payment-integration

Agent starts implementing Stripe integration without adding task

Later marks task as complete in tasks.md retroactively

❌ Started work without task documented
❌ No record of work scope before implementation
❌ Tasks should be added BEFORE work begins
```

**Example 4: Not Updating Counts**
```
Agent completes 3 tasks in specification 07-email-system

Agent updates checkboxes:
- [ ] Task 1 → [x] Task 1
- [ ] Task 2 → [x] Task 2
- [ ] Task 3 → [x] Task 3

But doesn't update frontmatter counts

Frontmatter still shows:
---
completed: 2
uncompleted: 8
---

❌ Counts don't match actual checkboxes
❌ Dashboard will show wrong progress
❌ Frontmatter must be updated with checkboxes
```

**Example 5: Vague Requirements Documentation**
```
requirements.md content:
---
description: Add authentication
status: uncompleted
---

# Authentication

User wants authentication.

Will implement JWT.

❌ No conversation summary
❌ No detailed requirements
❌ No technical specifications
❌ No success criteria
❌ No agent notes
❌ Too vague to be useful
```

## Rationale

### Why Requirements-First Development
1. **Clear Direction**: Agents know exactly what to implement
2. **User Alignment**: Ensures work meets user expectations
3. **Scope Control**: Prevents scope creep and unnecessary work
4. **Better Planning**: Can estimate effort and identify dependencies
5. **Documentation**: Creates permanent record of decisions
6. **Onboarding**: New agents/developers can understand project evolution

### Why Conversation Documentation
1. **Context Preservation**: Future agents understand the "why" not just "what"
2. **Decision Record**: Captures reasoning behind technical choices
3. **Clarification History**: Shows what questions were asked and answered
4. **Requirement Changes**: Can see how requirements evolved
5. **Knowledge Transfer**: Helps humans understand agent's understanding

### Why Task Tracking
1. **Progress Visibility**: User can see what's done and what remains
2. **Work Planning**: Agents can pick up where others left off
3. **Scope Management**: Clear list of what's in and out of scope
4. **Accountability**: Clear record of completed work
5. **Estimation**: Can gauge project completion percentage

### Why Verification is Critical
1. **Accuracy**: Status must reflect reality, not assumptions
2. **Trust**: User can rely on status information being correct
3. **Quality**: Ensures work is actually done, not just marked done
4. **Debugging**: Prevents confusion about what's implemented
5. **Handoffs**: Next agent gets accurate picture of state

### Why Frontmatter in Files
1. **Quick Reading**: Can see status without reading full file
2. **Machine Readable**: Tools can parse frontmatter for dashboards
3. **Metadata Separation**: Keeps metadata distinct from content
4. **Standard Format**: Uses established YAML frontmatter convention
5. **Efficiency**: Agents can scan multiple files quickly

### Why Master Index (Spec.md)
1. **Central Dashboard**: Single place to see all specifications
2. **Quick Navigation**: Links to all specification directories
3. **Status Overview**: Bird's-eye view of project progress
4. **Discoverability**: Easy to find specifications
5. **Progress Tracking**: User can monitor overall completion

## Enforcement

### Mandatory Compliance
All agents **MUST**:
- Never begin significant work without documented requirements
- Create specification directory before starting implementation
- Document requirements conversation thoroughly
- Create comprehensive task list before work begins
- **Launch review agent BEFORE any implementation work begins**
- **Read review agent's report before proceeding**
- **Stop work if review agent reports STOP or CLARIFY**
- Read both requirements.md and tasks.md before starting work
- Verify status by searching codebase, not trusting checkboxes
- Update tasks.md as work progresses
- Update frontmatter counts whenever task status changes
- Add new tasks BEFORE starting work on them
- **Create PROGRESS.md at mid-work checkpoint (~40-60% completion)**
- **Create FINAL_REPORT.md when all work is complete**
- **Create LEARNINGS.md to document lessons learned**
- **Run final verification before marking specification complete**
- **Create VERIFICATION_SIGNOFF.md with verification results**
- Commit specification changes following Rule 03 and Rule 05

### Violations

Any of the following constitutes a serious violation:
- **Starting implementation without running review agent first (CRITICAL)**
- **Ignoring review agent's STOP or CLARIFY directive (CRITICAL)**
- **Proceeding when review agent identifies blockers (CRITICAL)**
- **Starting implementation without module documentation (CRITICAL - NEW)**
- **Not verifying module documentation accuracy before work (CRITICAL - NEW)**
- **Proceeding when module docs don't match code (CRITICAL - NEW)**
- **Assuming module documentation is accurate without verification (CRITICAL - NEW)**
- **Completing specification without creating PROGRESS.md (VIOLATION)**
- **Completing specification without creating FINAL_REPORT.md (VIOLATION)**
- **Completing specification without creating LEARNINGS.md (VIOLATION)**
- **Completing specification without creating VERIFICATION_SIGNOFF.md (VIOLATION)**
- **Marking specification complete without running verification (VIOLATION)**
- Starting implementation without documented requirements
- Not creating specification directory and files
- Skipping requirements conversation with user
- Trusting task status without verifying in codebase
- Not updating tasks.md during work
- Not updating frontmatter counts
- Starting work on tasks not yet added to tasks.md
- Incomplete or vague requirements documentation
- Not updating Spec.md master index
- Not creating module documentation for new modules
- Not updating module documentation when structure changes

### User Impact
Violations have serious consequences:
- **User frustration**: Work doesn't meet expectations
- **Wasted effort**: Implementation may be wrong or unnecessary
- **Lost context**: Future agents don't understand requirements
- **False progress**: Status shows completion when work is incomplete
- **Confusion**: User can't understand what's been done
- **Rework**: May need to redo work due to misunderstanding
- **Time waste**: Building on false assumptions wastes hours of development time
- **Trust erosion**: User loses confidence in agent reliability
- **Breaking changes**: Misunderstanding modules leads to bugs and broken functionality
- **Documentation drift**: Module docs become useless when they don't match reality

**Review Agent Violations Are Especially Costly:**
- Skipping review agent leads to hours of wasted implementation effort
- Building on incorrect assumptions causes compound errors
- Discovering issues late in development requires extensive rework
- User becomes frustrated watching agents work on wrong things
- Review agent could have prevented all of this in minutes

**Module Documentation Violations Are Equally Costly:**
- Implementing without understanding current module state causes breaking changes
- False assumptions about module behavior lead to bugs
- Time wasted searching code instead of reading clear documentation
- Documentation drift makes future work exponentially harder
- Agents repeat mistakes that documentation could have prevented

**THE USER WILL BE UPSET** if work proceeds without proper requirements documentation, status verification, mandatory review agent execution, **or accurate module documentation**!

### Corrective Action

When a violation occurs:
1. **Stop immediately** if work has started without requirements or review agent
2. **Launch review agent** if it was skipped (CRITICAL)
3. **Read and act on review agent report** (MANDATORY)
4. **Do not proceed** if review agent reports STOP or CLARIFY
5. **Create/verify module documentation** if missing or unverified (CRITICAL - NEW)
6. **Spawn documentation agent** to verify/update module docs if needed (NEW)
7. **Do not proceed** if module docs don't match reality (NEW)
8. **Create specification** if missing
9. **Document requirements** by having conversation with user
10. **Create task list** before proceeding
11. **Verify status** by searching codebase if relying on checkboxes
12. **Update files** to reflect accurate status
13. **Commit changes** following proper git workflow
14. **Re-run review agent** if specifications were updated
15. **Re-verify module documentation** if code was changed
16. **Only proceed with implementation** when review agent reports GO and module docs are accurate

## Special Cases

### Small Bug Fixes
Very small bug fixes (single line changes) may not require full specification:
- Use judgment: if it takes longer to document than fix, proceed with fix
- Still commit with detailed message per Rule 03
- Consider adding to existing specification if related to one

### Urgent Hotfixes
For critical production issues:
- Fix the issue immediately
- Document requirements retroactively
- Create specification documenting what was done and why

### Research Tasks
For research/exploration tasks without implementation:
- Create specification with research questions
- Document findings in requirements.md
- Use tasks.md to track research activities

### Documentation-Only Changes
For pure documentation updates:
- May not need full specification
- Use judgment based on scope
- Major documentation overhauls should get specification

## Integration with Other Rules

### Works With Rule 03 (Work Commit Rules)
- Specification files follow commit-after-every-change rule
- Each specification update gets its own commit
- Commit messages explain what was added/changed in specifications

### Works With Rule 04 (Agent Orchestration)
- Main agent creates specifications before launching specialized agents
- Specialized agents MUST read specifications before working
- Agents report back with task completion status
- Main agent updates Spec.md based on agent reports

### Works With Rule 05 (Git Auto-Approval and Push)
- Specification changes are automatically pushed
- No approval needed for specification updates
- Each specification commit is pushed immediately

## Summary

**Core Principle**: Never start significant work without documented requirements and a clear task list. Always launch a review agent to verify specifications before implementation. Never trust checkboxes blindly. Always create all mandatory documentation files. **Always create/verify module documentation before implementation.**

**Key Points**:
- ✅ Requirements conversation comes first
- ✅ Document everything in specification directory
- ✅ Create comprehensive task list before work begins
- ✅ **Create/verify module documentation after requirements (MANDATORY - NEW)**
- ✅ **Spawn documentation agent(s) to create/update module docs**
- ✅ **Never assume module documentation is accurate**
- ✅ **Verify module docs match actual code**
- ✅ **Reference module docs in requirements.md**
- ✅ **Launch review agent BEFORE implementation (MANDATORY)**
- ✅ **Act on review agent's report (GO/STOP/CLARIFY)**
- ✅ Agents read specifications before working
- ✅ **Agents read module documentation before making changes**
- ✅ **Agents verify module docs match reality (spot check)**
- ✅ **Agents STOP if module docs don't match code**
- ✅ Verify status by searching codebase
- ✅ Update tasks.md as work progresses
- ✅ **Update module documentation if structure changes during implementation**
- ✅ **Create PROGRESS.md at mid-work checkpoint (MANDATORY)**
- ✅ **Create FINAL_REPORT.md when work is complete (MANDATORY)**
- ✅ **Create LEARNINGS.md to capture insights (MANDATORY)**
- ✅ **Run final verification before completion (MANDATORY)**
- ✅ **Create VERIFICATION_SIGNOFF.md with verification report (MANDATORY)**
- ✅ Keep Spec.md master index current
- ❌ **Never skip module documentation creation/verification (CRITICAL VIOLATION - NEW)**
- ❌ **Never assume module docs are accurate without verification (CRITICAL - NEW)**
- ❌ **Never proceed when module docs don't match code (CRITICAL - NEW)**
- ❌ **Never skip review agent requirement (CRITICAL VIOLATION)**
- ❌ **Never ignore review agent's STOP or CLARIFY (CRITICAL VIOLATION)**
- ❌ **Never skip mandatory documentation files (VIOLATION)**
- ❌ Never start work without documented requirements
- ❌ Never trust status without verification
- ❌ Never add tasks retroactively
- ❌ Never skip updating frontmatter counts

**Mandatory Files for Every Specification:**
Every specification **MUST** have these 6 files:
1. **requirements.md** - Requirements and conversation summary (created at start)
2. **tasks.md** - Task list with checkboxes (created at start)
3. **PROGRESS.md** - Mid-work progress report (created at ~50% completion)
4. **FINAL_REPORT.md** - Comprehensive completion summary (created when work is done)
5. **LEARNINGS.md** - Lessons learned and insights (created at completion)
6. **VERIFICATION_SIGNOFF.md** - Official verification report (created after verification)

**Module Documentation System (NEW):**
Every affected module **MUST** have accurate documentation:
1. **documentation/[module]/doc.md** - Detailed module documentation with:
   - What the module implements (all features and functions)
   - What it imports (external and internal dependencies)
   - What it calls (external and internal function calls)
   - What it does (step-by-step workflows)
   - Line number references for key implementations
   - Architecture diagrams when helpful
2. **Created/verified after requirements.md, before implementation**
3. **Updated when module structure changes**
4. **Referenced in requirements.md** so agents know to read it
5. **Verified to match actual code** (never assume it's accurate)

**Review Agent Is Mandatory:**
The review agent requirement is non-negotiable. It saves hours of wasted effort by:
- Verifying task status accuracy before implementation
- Identifying unclear requirements that need clarification
- Catching inconsistencies between documentation and code
- Preventing work based on false assumptions
- Ensuring specifications are actionable and complete

**Module Documentation Is Mandatory:**
The module documentation requirement is non-negotiable. It saves hours of wasted effort by:
- Providing immediate understanding of what module currently does
- Preventing breaking changes from misunderstanding module behavior
- Catching documentation drift before implementation begins
- Giving agents clear context instead of forcing them to grep/glob through code
- Ensuring all agents have same accurate understanding of module state

**Documentation Files Are Mandatory:**
The new documentation files (PROGRESS.md, FINAL_REPORT.md, LEARNINGS.md, VERIFICATION_SIGNOFF.md) are non-negotiable. They provide:
- **PROGRESS.md**: Mid-work visibility for users, helps track momentum and identify blockers
- **FINAL_REPORT.md**: Official record of achievements, comprehensive completion documentation
- **LEARNINGS.md**: Knowledge transfer, helps future work avoid pitfalls and build on insights
- **VERIFICATION_SIGNOFF.md**: Formal quality assurance, independent validation of completeness

**Remember**: The user will be upset if work proceeds without proper requirements, without status verification, without running the mandatory review agent first, **without creating/verifying module documentation**, or without creating all mandatory documentation files!

---
*Created: 2026-01-11*
*Last Updated: 2026-01-14*
*Version: 2.0 (Added Module Documentation System)*
