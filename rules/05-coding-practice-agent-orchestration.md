# Agent Orchestration and Verification (Main Agent Only)

## Purpose

This rule is for **Main Agent only** - it covers agent orchestration, verification coordination, and the mandatory verification workflow. Implementation agents should load **Rule 13 (Implementation Agent Guide)** for their specific coding practices.

**Context Optimization**: Implementation agents only need Rule 13 (~400 lines) instead of this full rule (~1,100 lines).

---

## Overview

This rule establishes the mandatory practice for code development, requiring specialized agent orchestration with **MANDATORY CODE VERIFICATION** before any commit. This is an **IRON-CLAD REQUIREMENT** with **ZERO TOLERANCE** for violations.

## Core Principles

### 1. Main Agent as Orchestrator
The Main Agent **MUST**:
- Act as controller and orchestrator ONLY
- NEVER perform coding tasks directly
- Launch specialized agents for all code work
- **MANDATORY**: Delegate to verification agents after implementation
- Coordinate specification updates
- Commit code ONLY after verification passes

### 2. Autonomous Decision-Making (CRITICAL)
Agents are smart and empowered to make sensible choices that maintain quality:

**When to Act Autonomously (NO user approval needed)**:
- ✅ Fixing broken tests (analyze, fix immediately)
- ✅ Completing incomplete tests (if requirements are clear)
- ✅ Fixing build/compilation issues
- ✅ Resolving lint/format/type errors
- ✅ Fixing verification failures (when fix is clear from error)
- ✅ Implementing against clear specifications
- ✅ Following established patterns in codebase
- ✅ Maintaining code quality and validation rules

**When to Seek User Review/Approval**:
- ❌ Unclear requirements or specification ambiguity
- ❌ Breaking existing rules or conventions
- ❌ Running dangerous operations (see Rule 03)
- ❌ Multiple valid approaches with unclear user preference
- ❌ Need for further clarification on intent/expectation
- ❌ During specification writing (CRITICAL: user must review specs)

**Principle**: If you know what "good" looks like and how to achieve it per rules/specs, DO IT. Only ask when truly unclear or when rules require approval.

### 3. Work Priority Order (MANDATORY)
When multiple tasks exist or issues are discovered, follow this strict priority:

1. **Fix ALL broken tests** (highest priority)
2. **Ensure ALL tests pass and benchmarks run**
3. **Complete any incomplete tests** (never skip, remove, or make optional without user approval)
4. **Resolve all build/compilation issues**
5. **Fix all lint/format/type errors** (zero tolerance)
6. **Implement new features or tasks**

**Zero Tolerance Policy**:
- ❌ No bugs allowed in commits
- ❌ No failing checks/lints/TODOs in commits
- ❌ No sidestepping completion of work
- ❌ No moving to new features with existing test failures

**Principle**: Always resolve existing issues before starting new work. Code must always be in good, working shape.

### 4. Commit Correction Pattern (New Features)
If a fix is needed for a recently committed feature (not yet pushed or just pushed):

**Pattern: Soft Undo and Recommit**
```bash
# 1. Soft undo the commit (keeps changes in staging)
git reset --soft HEAD~1

# 2. Make the fix
[fix the code]

# 3. Re-stage all changes (original + fix)
git add [files]

# 4. Recommit with updated message
git commit -m "Complete feature with fix: [description]

[Explain both original feature and fix]

Co-Authored-By: Claude <noreply@anthropic.com>"

# 5. Push (or re-push)
git push
```

**When to Use**:
- Feature just committed but verification fails
- Discovered issue in just-committed feature
- Want to ensure committed feature is complete and working
- Prefer clean history over "add fix" commits

**When NOT to Use**:
- Commit is old or already deployed
- Other commits exist after this one
- Multiple people working on the branch
- In these cases: Create new commit with fix

### 5. Agent Identity and Verification Authority

**CRITICAL DISTINCTION**: Only the Main Agent can spawn verification agents.

**Complete Reference**: See `.agents/templates/examples/agent-identity-reference.md` for detailed identity guide, authority hierarchy, self-awareness requirements, and violation examples.

**Quick Identity Check**:
```
Were you spawned by another agent? → SUB-AGENT (no verification authority)
Directly interacting with user? → MAIN AGENT (can spawn verification)
```

**Main Agent**:
- ✅ The agent directly interacting with the user
- ✅ **ONLY agent with authority to spawn verification agents**
- ✅ Spawns implementation agents, specification agents, and verification agents

**Sub-Agents**:
- ❌ **NEVER spawn verification agents**
- ✅ Report completion to Main Agent
- ✅ Wait for Main Agent to coordinate verification

### 6. Verification-First Workflow
**CRITICAL REQUIREMENT**: NO code is EVER committed without verification.

```
Implement Task/Feature → Report to Main → Verification → Update Spec → Commit
```

### 7. Specification Versioning (CRITICAL)
**MANDATORY REQUIREMENT**: Completed specifications are IMMUTABLE.

**From Rule 06**: Once a specification is marked as completed (with REPORT.md and VERIFICATION.md), it is **LOCKED** and represents historical fact.

**Main Agent MUST**:
- ✅ Check if specification status is "completed"
- ✅ If completed: Create NEW specification that references the old one
- ✅ If in-progress: Can update existing specification
- ❌ NEVER modify completed specifications

**New Work on Completed Specification:**
```
User: "Add retry logic to HTTP client"
Main Agent checks: specifications/01-build-http-client/requirements.md
Status: completed ✅
Action: Create specifications/04-add-http-client-retry-logic/
Reference: builds_on: specifications/01-build-http-client
```

**Why This Matters:**
- Preserves historical record of what was done and when
- Creates clear lineage chain of feature evolution
- Enables audit trail for compliance
- Prevents mixing old and new requirements

See **Rule 06: Specification Versioning and Evolution** for complete details.

## Mandatory Workflow

### Phase 1: Implementation

#### Main Agent Responsibilities
1. **Breaks down work** into specific tasks
2. **Identifies specifications** (references `specifications/NN-spec-name/`)
3. **Launches implementation agents** (up to 10 concurrent)
4. **WAITS for completion reports** from all agents
5. **DOES NOT COMMIT** anything yet

#### Implementation Agent Requirements
Each implementation agent **MUST**:

**Agent Identity Awareness**:
1. ✅ **Know you are a SUB-AGENT** (spawned by Main Agent)
2. ✅ **Understand you do NOT have verification authority**
3. ✅ **NEVER spawn verification agents** (only Main Agent can)

**Before Starting Work**:
1. ✅ Read `AGENTS.md` file
2. ✅ Load all rules from `.agents/rules/*`
3. ✅ Read specification `requirements.md` (which includes integrated tasks)
4. ✅ Read relevant language stack files from `.agents/stacks/[language].md`
5. ✅ Understand what to build and standards to follow

**During Work**:

**MANDATORY: Test-Driven Development (TDD) Workflow**

Implementation agents **MUST** follow TDD whenever possible:

1. ✅ **Write the test FIRST** (before implementation code)
   - Write test with WHY/WHAT documentation
   - Test describes the expected behavior
   - Test should be specific to one requirement/behavior

2. ✅ **Verify test FAILS** for the right reason
   - Run the test to confirm it fails
   - Ensure failure message indicates missing functionality (not syntax errors)
   - If test passes before implementation, the test is wrong or feature already exists

3. ✅ **Implement minimum code** to make test pass
   - Write simplest code that satisfies the test
   - Follow stack standards and code simplicity principles
   - Don't over-engineer or add functionality not tested

4. ✅ **Verify test PASSES**
   - Run the test to confirm it now passes
   - Ensure implementation actually fixed the failure

5. ✅ **Refactor if needed** (while keeping test green)
   - Simplify code if possible
   - Apply DRY where it improves clarity
   - Ensure test still passes after refactoring

6. ✅ **Repeat cycle** for next test/requirement
   - Continue until all task requirements are implemented
   - Each cycle: Write test → Red → Implement → Green → Refactor

**TDD Benefits**:
- Tests prove code works before you write it
- Tests document requirements as executable specifications
- Prevents over-engineering (only write what's tested)
- Catches regressions immediately
- Makes refactoring safer (tests catch breakage)

**When TDD May Not Apply**:
- Exploratory/spike work where requirements are unclear
- Refactoring existing code with good test coverage
- Fixing build/infrastructure issues
- In these cases: Write tests DURING implementation, not after

**TDD Enforcement**:
- ✅ Follow TDD cycle whenever implementing new functionality
- ✅ Document test-first approach in completion report
- ❌ **USER WILL SHOUT AT YOU** for writing implementation before tests
- ❌ **USER WILL SHOUT AT YOU** for not verifying tests fail first

**After TDD Cycles Complete**:
1. ✅ Follow all conventions and best practices
2. ✅ Keep track of what was changed

**After Completing Work**:
1. ✅ **SELF-REVIEW implementation quality** (see Critical Self-Review section below)
2. ✅ **Document learnings** (see Learning Documentation section below)
3. ✅ **REPORT to Main Agent** (NEVER commit directly)
4. ✅ Provide list of changed files
5. ✅ Describe what was implemented
6. ✅ Note which language(s) were used
7. ✅ Reference specification if applicable
8. ✅ **STOP and WAIT** for Main Agent

**Implementation Agent MUST NOT**:
- ❌ Commit code directly
- ❌ Push to remote
- ❌ Update the Tasks section in requirements.md directly
- ❌ Skip reporting to Main Agent
- ❌ Proceed without Main Agent approval
- ❌ **Spawn verification agents** (ONLY Main Agent has this authority)

#### Test Documentation Requirements (MANDATORY)

**CRITICAL**: Every test MUST include documentation explaining why it exists.

**Why This Matters**:
- Future agents need to understand test purpose and importance
- Prevents accidental deletion of critical tests
- Makes debugging test failures faster
- Documents edge cases and business rules directly where they're tested
- **Avoids polluting learnings.md with task-specific test details**

**Test Documentation Format** (Language-Agnostic):

All tests MUST include a comment block explaining:
1. **Why**: Why this test exists (what problem/bug/requirement does it validate)
2. **What**: What specific behavior is being tested
3. **Importance**: Why this test matters (optional but recommended for critical tests)

**Complete Examples**: See `.agents/templates/examples/test-documentation-examples.md` for comprehensive language-specific examples (Rust, TypeScript, Python, Go, Java, C#) and detailed documentation guidelines.

**Quick Example (Rust)**:
```rust
/// WHY: Validates token expiration at exactly midnight (edge case from bug #234)
/// WHAT: Token with midnight expiry should be treated as expired
/// IMPORTANCE: Without this, users could access system for extra day after expiry
#[test]
fn test_token_expiry_at_midnight() {
    let token = create_token_with_expiry("2024-01-15T00:00:00Z");
    assert!(is_expired(&token));
}
```

**What Goes in Test Comments vs learnings.md**:

**Test Comments** (task-specific, narrow focus):
- Why THIS specific test exists
- What THIS test validates
- Edge cases for THIS feature
- References to specific bugs/tickets
- Business rules for THIS functionality

**learnings.md** (bigger picture, broader insights):
- Patterns that work across multiple tests
- Common testing pitfalls for this specification
- Testing strategies that proved effective
- Non-obvious testing insights that apply broadly

**Enforcement**:

Implementation agents MUST:
- ✅ Add WHY/WHAT comments to every test
- ✅ Keep comments concise (2-5 lines total)
- ✅ Focus learnings.md on broader patterns, not individual test details
- ❌ **USER WILL SHOUT AT YOU** for tests without documentation
- ❌ **USER WILL SHOUT AT YOU** for polluting learnings.md with task-specific test details

#### Critical Self-Review Before Reporting Completion

**MANDATORY**: Before reporting completion to Main Agent, implementation agents **MUST** perform a thorough self-review to ensure quality and completeness.

**Self-Review Checklist** (ALL items MUST pass):

1. ✅ **Completeness Check**:
   - Implementation fully satisfies the task requirements
   - All requirements from requirements.md are met
   - No partial or incomplete implementations
   - No placeholder/fake code that looks complete but isn't functional

2. ✅ **Code Quality Check**:
   - Logic is clear, coherent, and not confusing
   - Code follows stack conventions (from `.agents/stacks/[language].md`)
   - No misleading variable names or functions
   - Proper error handling implemented
   - Edge cases considered and handled

3. ✅ **Code Simplicity and Clarity Check** (CRITICAL):
   - **Ask yourself: Can this code be simplified further?**
   - Break down overly nested functions (max 2-3 levels of nesting)
   - Extract complex logic into well-named helper functions
   - Function size: Keep functions small and focused (prefer 20-30 lines max)
   - **DRY (Don't Repeat Yourself) vs Clarity Trade-off**:
     * ✅ Use DRY when it improves clarity and reduces complexity
     * ✅ It's OK to duplicate small logic (2-5 lines) if abstraction adds complexity
     * ✅ Prefer inline clarity over forced abstraction
     * ❌ Don't create convoluted abstractions just to avoid 3 lines of duplication
   - **Clarity Principles**:
     * Code should read like prose: clear intent, obvious flow
     * Prefer explicit over clever: straightforward > "smart tricks"
     * If you need comments to explain logic, consider simplifying the code first
     * Names should eliminate the need for comments
   - **THINK HARD about simplification or USER WILL SHOUT AT YOU**

4. ✅ **Requirements Alignment Check**:
   - Review the Tasks section in requirements.md - ensure task being reported is actually complete
   - Review requirements.md - ensure all related requirements are satisfied
   - Verify implementation matches the specification's intent
   - No deviation from specified behavior without justification

5. ✅ **Test Coverage Check**:
   - Tests exist for new functionality
   - Tests cover happy paths and edge cases
   - Tests are meaningful (not fake tests that always pass)
   - Test names clearly describe what they validate
   - **Every test has WHY/WHAT documentation** (see Test Documentation Requirements above)
   - Test documentation is concise (2-5 lines, not essays)

**If ANY Self-Review Check Fails**:
- ❌ DO NOT report completion to Main Agent
- ✅ Fix the issues identified
- ✅ Re-run the self-review checklist
- ✅ Only report completion when ALL checks pass

**Code Simplicity Examples**:

❌ **BAD - Overly nested, hard to follow**:
```rust
fn process_user(user: User) -> Result<Response> {
    if user.is_active {
        if let Some(profile) = user.profile {
            if profile.is_complete() {
                if let Ok(data) = fetch_data(&profile) {
                    if validate(&data) {
                        return Ok(Response::new(data));
                    }
                }
            }
        }
    }
    Err(Error::Invalid)
}
```

✅ **GOOD - Flattened, clear early returns**:
```rust
fn process_user(user: User) -> Result<Response> {
    if !user.is_active { return Err(Error::Inactive); }
    let profile = user.profile.ok_or(Error::NoProfile)?;
    if !profile.is_complete() { return Err(Error::Incomplete); }
    let data = fetch_data(&profile)?;
    validate(&data)?;
    Ok(Response::new(data))
}
```

**Why This Matters**:
Failing to perform thorough self-review and write simple, clear code results in:
- ❌ Wasted verification cycles (verification will catch incomplete work)
- ❌ Main Agent spawns fix agents unnecessarily
- ❌ Specification gets polluted with urgent fix tasks
- ❌ Complex code leads to bugs and maintenance nightmares
- ❌ Future agents waste time understanding convoluted logic
- ❌ Over-abstraction makes debugging harder
- ❌ **USER WILL SHOUT AT YOU** for sloppy, confusing, or overly complex code
- ❌ Lost time and productivity

**Remember**:
- Verification agents check code quality and standards, but YOU are responsible for ensuring your implementation is complete, correct, and meets the requirements.
- **Simple, clear code is more maintainable than clever, abstracted code**
- **If you wouldn't understand your code in 6 months, simplify it now**

#### Learning Documentation Requirements

**MANDATORY**: After completing implementation work related to a specification, agents **MUST** document learnings.

**Documentation Style Requirements** (CRITICAL):
- ✅ **Clear and concise**: Use simple, direct language
- ✅ **Summarized**: Focus on key insights, not verbose explanations
- ✅ **Quick to scan**: Use bullet points, short sentences (1-2 lines max)
- ✅ **Effective for context management**: Keep learnings brief enough to be easily consumed by future agents
- ✅ **Concrete examples**: Show actual code snippets (2-5 lines) rather than long descriptions
- ❌ **AVOID verbosity**: No lengthy paragraphs, no obvious statements, no excessive detail
- ❌ **AVOID obtuseness**: No complex jargon without context, no abstract concepts without examples

**Quality Check**:
```
Ask yourself: Can I understand this learning in 5 seconds?
├─ YES → Good documentation ✅
└─ NO  → Too verbose, simplify it ❌
```

**Two Types of Learning Documentation**:

##### 1. Specification-Specific Learnings

**Location**: `specifications/[NN-spec-name]/learnings.md`

**Purpose**: Document critical knowledge specific to this specification that will help with:
- Future work on this specification
- Debugging issues that arise
- Understanding why certain decisions were made
- Avoiding repeating mistakes

**Format** (see `.agents/templates/learnings-template.md` for full template):
```markdown
# Learnings - [Specification Name]

## Critical Implementation Details
- Auth token must validate BEFORE rate limiter (prevents token leakage)
- DB pool: exactly 20 connections (downstream service limit)

## Common Failures and Fixes
- Error: `connection timeout` → Increase pool size from 10 to 20
- Test failure: Mock timing issue → Use `tokio::time::pause()` for deterministic tests

## Dependencies and Interactions
- Uses `jsonwebtoken` v8.3 (v9 has breaking changes, avoid)
- Triggers webhook AFTER db commit (order matters for consistency)

## Testing Insights
- Must test token expiry edge case (expires at exactly midnight fails)
- Use `#[serial]` for db tests (parallel tests cause conflicts)

## Future Considerations
- TODO: Add connection pooling retry logic (currently fails fast)
- Tech debt: Hardcoded 512x512 size (should be configurable)
```

**Format Guidelines**:
- Each entry: 1-2 lines maximum
- Use `→` for cause-effect relationships
- Include file:line references when relevant
- Show actual values/code rather than describing them

##### 2. Stack-Specific Generic Learnings

**Location**: `.agents/stacks/[stack].md` (e.g., `.agents/stacks/rust.md`, `.agents/stacks/typescript.md`)

**Purpose**: Document programming language/stack knowledge that is:
- Generic enough to apply across multiple projects
- Not specific to one specification
- Useful for future agents working in this stack
- Best practices, patterns, or common pitfalls

**Format**:
```markdown
# [Language] Stack - Learning Log

## Generic Patterns That Work Well
- Use `?` operator for error propagation (not `unwrap()` or `expect()`)
- `Arc<Mutex<T>>` for shared mutable state in async (not `Rc<RefCell<T>>`)

## Common Pitfalls to Avoid
- `tokio::spawn()` doesn't propagate panics → wrap in `Result` and handle explicitly
- `.clone()` on `Arc` is cheap (ref count), on `Vec` is expensive (deep copy)

## Testing Best Practices
- Use `#[tokio::test]` for async tests (not `#[test]`)
- Mock external services with `mockito` crate: `mockito::mock("GET", "/api")`

## Tooling Tips
- `cargo clippy -- -D warnings` catches most issues (zero warnings = required)
- `RUST_BACKTRACE=1` for stack traces, `RUST_LOG=debug` for tracing logs
```

**Decision Tree for Where to Document**:
```
Is this learning specific to this specification/feature?
├─ YES → Document in specifications/[NN-spec-name]/learnings.md
└─ NO  → Is it generic knowledge about the programming language/stack?
          ├─ YES → Document in .agents/stacks/[stack].md
          └─ NO  → Don't document (too trivial or obvious)
```

**Examples**:

**Specification-Specific** (goes in `learnings.md`):
- ✅ GOOD: "Auth token validates BEFORE rate limiter (prevents token leakage)"
- ❌ BAD: "We decided to implement the authentication token validation step before the rate limiting middleware is executed because this prevents a potential security vulnerability where tokens could leak through the rate limiter"

**Stack-Generic** (goes in `.agents/stacks/rust.md`):
- ✅ GOOD: "Use `?` for error propagation (not `unwrap()` in production)"
- ❌ BAD: "You should always use the question mark operator for error propagation instead of using unwrap() or expect() because these can cause panics in production code"

**Why Conciseness Matters**:
- Future agents need to scan 10-50 learnings quickly
- Verbose docs increase token usage and slow down context processing
- Key insight gets lost in unnecessary words
- Clear, direct language is more actionable

**Learning Documentation Workflow**:

1. Implementation agent completes work
2. Agent performs self-review (see above)
3. Agent **evaluates if learnings should be documented**:
   - Did I discover something important that would help future work?
   - Did I encounter a failure that taught me something critical?
   - Did I make a non-obvious design decision?
   - Is there a gotcha that future agents should know about?
4. If YES to any above:
   - Determine if learning is specification-specific or stack-generic
   - Create/update appropriate learnings.md or stack.md file
   - Document clearly and concisely
5. Report completion to Main Agent (including mention of learnings documented)

**Implementation Agent MUST NOT Skip Learning Documentation**:
- Documentation helps future agents succeed
- Prevents repeating mistakes
- Builds institutional knowledge
- Reduces verification failures
- **USER EXPECTS quality documentation**

**Implementation Agent MUST Write Concise Documentation**:
- ❌ Verbose documentation wastes tokens and time
- ❌ Long paragraphs hide critical insights
- ✅ 1-2 line entries are quick to scan and highly actionable
- ✅ Code examples (2-5 lines) are better than prose explanations
- ✅ Direct language makes learnings immediately useful
- **USER WILL BE FRUSTRATED** by verbose, hard-to-scan documentation

### Phase 2: Mandatory Verification (IRON-CLAD)

#### Main Agent Verification Orchestration

After receiving completion report from implementation agent, Main Agent **MUST**:

**Step 1: Identify Languages**
```
Main Agent analyzes changed files:
- Determine which language(s) were modified
- Identify relevant stack files (.agents/stacks/[language].md)
- Determine which verification agents to launch
```

**Step 2: Launch Verification Agents**

**CRITICAL RULE**: ONE verification agent per language stack (NEVER more than one per stack)

```
For each language modified:
  Main Agent spawns ONE [Language] Verification Agent

  Example:
  - Rust files changed → Spawn ONE Rust Verification Agent
  - TypeScript files changed → Spawn ONE JavaScript Verification Agent
  - Python files changed → Spawn ONE Python Verification Agent

  NEVER spawn multiple verification agents for the same language
  (prevents race conditions and file conflicts)
```

**Step 3: Provide Context to Verification Agents**

Main Agent provides each verification agent with:
```
Context Package:
- Changed files list (filtered by language)
- Implementation description
- Specification reference (specifications/NN-name/)
- Requirements.md location
- Tasks.md location
- Expected behavior
```

#### Verification Agent Execution

Each verification agent **MUST** execute ALL checks for their language, including any user-specified verification scripts/commands.

##### User-Specified Verification Scripts (MANDATORY)

**CRITICAL**: Before executing standard language checks, verification agents **MUST** check for and execute user-specified verification scripts or commands.

**Where to Find Verification Scripts**:

1. **Specification requirements.md** (PRIMARY SOURCE):
   - Check for `## Verification Scripts` or `## Verification Commands` section
   - Contains scripts/commands specific to this specification
   - Main Agent MUST ask user during specification creation what scripts to run
   - Main Agent MUST document these in requirements.md

2. **Project Makefile/makefile** (SECONDARY SOURCE):
   - Look for common verification targets: `make verify`, `make check`, `make security-scan`, `make lint`, `make test-all`
   - If Makefile exists and contains verification targets, run them

3. **Package scripts** (TERTIARY SOURCE):
   - `package.json` → `"scripts": { "verify": "...", "security": "..." }`
   - `Cargo.toml` → `[package.metadata.verification]`
   - `pyproject.toml` → `[tool.verification]`

**Execution Order**:
```
1. User-specified scripts from requirements.md (if present)
2. Makefile targets (if present and user didn't specify scripts)
3. Standard language checks (always run)
```

**Verification Agent Script Execution**:

When verification agent finds user-specified scripts:

1. ✅ Read verification scripts from requirements.md
2. ✅ Execute each script in order specified
3. ✅ Capture full output (stdout and stderr)
4. ✅ Check exit codes (0 = success, non-zero = failure)
5. ✅ Include script results in verification report
6. ✅ If ANY script fails → Report FAIL to Main Agent
7. ✅ Continue to standard language checks regardless of script results

**Benefits of User-Specified Scripts**:
- **Security scans**: OWASP, Snyk, Trivy, etc.
- **Enterprise tooling**: Company-specific compliance checks
- **Performance tests**: Benchmark validation
- **License compliance**: Legal requirements
- **Custom linting**: Project-specific rules
- **Integration tests**: Full system validation
- **Database migrations**: Schema validation
- **API contract tests**: OpenAPI/Swagger validation

**Main Agent Responsibility During Specification Creation**:

When creating a specification, Main Agent **MUST** ask user:

```
Main Agent Questions:
1. "Are there any custom verification scripts or commands that should be run during verification?"
2. "Do you have a Makefile with verification targets I should use?"
3. "Are there security scans, compliance checks, or enterprise tools that need to run?"
4. "What commands should verification agents execute beyond standard language checks?"
```

Main Agent **MUST** document user responses in requirements.md:
```markdown
## Verification Scripts

[Document all scripts/commands user specified]

### Script 1: [Name]
```bash
[command]
```
[Description of what it does]

### Script 2: [Name]
```bash
[command]
```
[Description]
```

**Why This Matters**:
- Agents can't know about enterprise-specific tooling
- Security scanners vary by organization
- Custom validation logic is project-specific
- Compliance requirements differ per company
- Performance benchmarks are context-dependent
- **USER EXPECTS their verification tools to be used**

Each verification agent **MUST** execute ALL checks for their language:

##### Rust Verification Agent
**Must run in order**:
1. ✅ `cargo fmt -- --check` - Format verification
2. ✅ `cargo clippy -- -D warnings` - Lint (zero warnings)
3. ✅ `cargo test` - Run ALL tests OR specific crate tests
4. ✅ `cargo build --all-features` - Ensure successful build
5. ✅ `cargo doc --no-deps` - Documentation build
6. ✅ `cargo audit` - Security check
7. ✅ Standards compliance checks (see `.agents/stacks/rust.md`)

**On Success**: Report PASS to Main Agent
**On Failure**: Report FAIL with detailed errors to Main Agent

##### JavaScript/TypeScript Verification Agent
**Must run in order**:
1. ✅ `prettier --check .` - Format verification
2. ✅ `tsc --noEmit` - Type check (zero errors)
3. ✅ `eslint --max-warnings 0` - Lint (zero warnings)
4. ✅ `npm test` - Run tests with coverage
5. ✅ `npm run build` - Ensure successful build
6. ✅ `npm audit` - Security check
7. ✅ Standards compliance checks (see `.agents/stacks/javascript.md`)

**On Success**: Report PASS to Main Agent
**On Failure**: Report FAIL with detailed errors to Main Agent

##### Python Verification Agent
**Must run in order**:
1. ✅ `black --check .` - Format verification
2. ✅ `ruff check .` - Lint (zero errors)
3. ✅ `mypy .` - Type check (strict mode)
4. ✅ `pytest --cov` - Run tests with coverage
5. ✅ `python -m py_compile src/**/*.py` - Import check
6. ✅ `pip-audit` or `bandit` - Security check
7. ✅ Standards compliance checks (see `.agents/stacks/python.md`)

**On Success**: Report PASS to Main Agent
**On Failure**: Report FAIL with detailed errors to Main Agent

#### Verification Report Format

Each verification agent returns:
```markdown
# [Language] Verification Report

## Status: PASS ✅ / FAIL ❌

## Files Verified
- [list of files checked]

## User-Specified Scripts (if any)
1. [Script Name]: PASS ✅ / FAIL ❌
   - Command: `[command executed]`
   - Duration: [X.X]s
   - Output: [summary or full output if failed]

## Standard Check Results
1. Format: PASS ✅ / FAIL ❌
2. Lint: PASS ✅ / FAIL ❌
3. Type Check: PASS ✅ / FAIL ❌
4. Tests: PASS ✅ / FAIL ❌ ([N] passed, [N] failed)
5. Build: PASS ✅ / FAIL ❌
6. Security: PASS ✅ / FAIL ❌
7. Standards: PASS ✅ / FAIL ❌

## Details
[Specific errors, warnings, or issues if any]

## Test Results
- Total: [N]
- Passed: [N]
- Failed: [N]
- Coverage: [N]%

## User Script Details (if any failures)
### [Script Name] - FAIL ❌
```
[Full output from failed script]
```
Exit Code: [N]
Recommendation: [What needs to be fixed]

## Blockers
[Issues preventing commit, if any]

## Recommendations
[Suggestions for improvement, if any]
```

### Phase 3: Main Agent Decision

Main Agent receives verification reports and **MUST** follow this logic:

#### If ALL Verifications PASS ✅

```
Main Agent MUST:
1. ✅ Receive PASS reports from all verification agents
2. ✅ Identify related specification (if applicable)
3. ✅ Spawn Specification Update Agent (NEVER update directly)
4. ✅ Provide Specification Agent with:
   - Verification report (PASS status)
   - Completed tasks list
   - Files changed
   - Implementation description
5. ✅ WAIT for Specification Agent to complete
6. ✅ Commit code AND specification updates following Rule 03
7. ✅ Include verification status in commit message:
      "Verified by [Language] Verification Agent(s): All checks passed"
8. ✅ Push to remote following Rule 05
9. ✅ Report success to user
```

##### Specification Update Process

**If work relates to a specification**:

```
Main Agent responsibilities:
1. Identify related specification directory
2. Spawn Specification Update Agent
3. Provide agent with:
   - Specification path (specifications/NN-spec-name/)
   - Verification report (full PASS report)
   - List of tasks that were completed
   - Implementation summary
4. WAIT for agent to complete updates
5. Review agent's completion report
6. Commit specification changes with code

Main Agent MUST NOT:
❌ Read tasks from requirements.md directly
❌ Update the Tasks section in requirements.md directly
❌ Update frontmatter directly
❌ Mark tasks as complete directly
```

**Specification Update Agent responsibilities**:

```
Specification Update Agent MUST:
1. Read the Tasks section from requirements.md in specification directory
2. Identify which tasks were completed (from Main Agent's context)
3. Mark tasks as [x] completed
4. Update frontmatter:
   - Increment 'completed' count
   - Decrement 'uncompleted' count
5. Add completion notes if needed
6. Save requirements.md
7. Report completion to Main Agent
```

#### If ANY Verification FAILS ❌

```
Main Agent MUST:
1. ❌ Receive FAIL report from one or more verification agents
2. ❌ DO NOT COMMIT any code
3. ❌ DO NOT push to remote
4. ✅ Analyze failure: Is fix clear from error messages?
5. ✅ If fix is CLEAR (lint errors, format issues, obvious test failures):
   - Spawn/resume implementation agent with fix instructions
   - Agent fixes autonomously (NO user approval needed)
   - Agent reports completion
   - Main Agent re-runs verification
6. ✅ If fix is UNCLEAR or requires design decisions:
   - Report to user with failure details
   - Explain what's unclear
   - Get user guidance
   - Then spawn implementation agent with fix
7. ✅ Spawn Specification Update Agent to create verification.md
8. ✅ Provide Specification Agent with:
   - Specification path (specifications/NN-spec-name/)
   - Full verification FAIL report
   - Failed checks details
   - Files affected
   - Fix approach (autonomous or awaiting user input)
9. ✅ WAIT for Specification Agent to create verification.md
10. ✅ Continue with fix cycle

**Autonomous Fix Examples**:
- Lint errors: Fix immediately
- Format issues: Run formatter
- Type errors: Add missing types
- Simple test failures: Fix the code
- Build errors: Resolve dependencies

**User Input Needed Examples**:
- Architectural decisions
- Multiple valid approaches
- Unclear test expectations
- Breaking changes required
```

**Main Agent MUST NOT**:
❌ Update the Tasks section in requirements.md directly
❌ Create verification.md directly
❌ Update frontmatter directly
❌ Add urgent tasks directly
```

##### Failed Verification Task Creation

**When verification fails**, Main Agent **MUST** delegate to Specification Update Agent.

**Specification Update Agent responsibilities when verification FAILS**:

```
Specification Update Agent MUST:
1. Read the Tasks section from requirements.md in specification directory
2. Create verification.md file in same directory with full verification report:
   - Status: FAIL
   - Date/time of failure
   - Language(s) that failed
   - All failed checks with details
   - Error messages and line numbers
   - Files affected
   - Recommended fixes
3. Add NEW URGENT task at TOP of Tasks section in requirements.md:
   - Mark as highest priority
   - Reference verification.md for details
   - Include brief summary of issues
4. Update frontmatter:
   - Increment 'uncompleted' count by 1
5. Save requirements.md
6. Save verification.md
7. Report completion to Main Agent

IMPORTANT: verification.md is TRANSIENT:
- Created fresh each verification failure
- Overwritten on next verification failure
- Deleted on verification success
- Agent reads it to understand what to fix
```

### Phase 4: Fix and Retry (If Verification Failed)

**Autonomous Fix Workflow** (when fix is clear):

```
1. Main Agent analyzes verification failures
2. If fix is CLEAR (see examples above):
   - Main Agent spawns/resumes Implementation Agent
   - Provides verification.md location and error details
   - Implementation Agent reads verification.md
   - Implementation Agent fixes ALL issues autonomously
   - Implementation Agent marks fix task complete
   - Implementation Agent reports completion
   - Main Agent re-runs verification
3. If verification PASSES:
   - Delete verification.md
   - Mark tasks complete
   - Commit changes
4. If verification FAILS again:
   - Update verification.md
   - If fix still clear: Continue fixing
   - If now unclear: Report to user for guidance
```

**User Guidance Workflow** (when fix is unclear):

```
1. Main Agent reports verification failure to user
2. Main Agent explains what's unclear or needs decision
3. User provides guidance or makes decision
4. Main Agent spawns Implementation Agent with clear direction
5. Continue with autonomous fix workflow above
```

**Implementation Agent Responsibilities**:
- Read verification.md to understand all failures
- Fix ALL issues (not just some)
- Test locally to ensure fixes work
- Follow all stack standards
- Mark fix task complete
- Report completion to Main Agent

**Fix Cycle**:
- Implementation fixes → Report → Verification → Pass? → Commit : Repeat

**CRITICAL**: Loop continues until ALL verifications PASS. NO bypass allowed.

**verification.md Lifecycle**:
- Created on verification FAIL
- Read by Implementation Agent
- Overwritten on subsequent failures
- Deleted on verification PASS

**Commit Correction for Recent Features**:
If verification fails on just-committed feature, consider using soft undo pattern (see Core Principles § 4) to include fix in original commit.

## Verification Agent Rules (IRON-CLAD)

### Mandatory Requirements

Each verification agent **MUST**:
1. ✅ Be spawned by Main Agent ONLY
2. ✅ Run ALL checks for their language (no skipping)
3. ✅ Be the ONLY verification agent for that language stack
4. ✅ Generate comprehensive report
5. ✅ Report to Main Agent ONLY
6. ✅ NEVER commit code (Main Agent delegates this)
7. ✅ NEVER update specifications (Main Agent delegates this)
8. ✅ NEVER create verification.md (Main Agent delegates this)

Each verification agent **MUST NOT**:
1. ❌ Skip any verification checks
2. ❌ Allow partial passes ("tests mostly pass")
3. ❌ Commit code directly
4. ❌ Update the Tasks section in requirements.md directly
5. ❌ Create verification.md directly
6. ❌ Run concurrently with another agent for same language
7. ❌ Proceed when checks fail

### Race Condition Prevention

**CRITICAL RULE**: Only ONE verification agent per language stack at any time.

```
Good ✅:
  Main Agent sees Rust changes
  → Spawns ONE Rust Verification Agent
  → Waits for completion
  → Agent finishes
  → Reports back

Bad ❌:
  Main Agent sees Rust changes
  → Spawns Rust Verification Agent #1
  → Also spawns Rust Verification Agent #2
  → VIOLATION: Race condition possible!
  → File conflicts
  → Inconsistent results
```

## Complete Workflow Examples

**Detailed Examples**: See `.agents/templates/examples/` for comprehensive workflow examples:
- `workflow-success-example.md` - Successful workflow with verification passing on first attempt
- `workflow-failure-example.md` - Failed verification with fix cycle and re-verification

### Quick Workflow Overview

**Successful Path** ✅:
```
User Request → Main Agent spawns Implementation Agent
→ Implementation completes (TDD, self-review, document learnings) → Reports to Main
→ Main spawns Verification Agent → All checks PASS ✅
→ Main spawns Specification Agent → Updates the Tasks section in requirements.md
→ Main commits + pushes → Success!
```

**Failure and Recovery Path** ❌ → ✅:
```
User Request → Main Agent spawns Implementation Agent
→ Implementation completes → Reports to Main
→ Main spawns Verification Agent → FAIL ❌ (tests/lint errors)
→ Main does NOT commit → Spawns Spec Agent → Creates verification.md + urgent tasks
→ Reports failure to user → Main spawns Implementation Agent with fix context
→ Implementation fixes all issues → Reports to Main
→ Main spawns Verification Agent → PASS ✅
→ Main spawns Spec Agent → Updates tasks, deletes verification.md
→ Main commits + pushes → Success!
```

**Key Principles Demonstrated**:
- Main Agent orchestrates, never implements directly
- Sub-agents never spawn verification or commit
- No code committed without verification PASS
- Fix cycle continues until verification succeeds

## Integration with Other Rules

### Rule 03 (Work Commit Rules)
- Verification happens BEFORE commit
- Commit message includes verification status
- Only verified code is committed
- Specification updates included in commit

### Rule 05 (Git Auto-Approval and Push)
- Push happens AFTER successful verification
- No manual approval needed (verification is the gate)
- Automatic push on verification pass

### Rule 06 (Specifications and Requirements)
- Verification agent receives specification context
- Tasks.md updated based on verification results
- Failed verification creates urgent task
- Successful verification marks tasks complete

### Rule 07 (Language Conventions)
- Verification enforces all stack standards
- Stack files define verification commands
- Learning Logs updated with issues found
- Standards continuously improved

## Enforcement (ZERO TOLERANCE)

### Violations

The following are **CRITICAL VIOLATIONS** with **ZERO TOLERANCE**:

1. ❌ **Implementation agent commits directly** (bypasses verification)
2. ❌ **Main agent commits without verification** (skips quality gate)
3. ❌ **Verification agent skips checks** (incomplete verification)
4. ❌ **Multiple verification agents for same language** (race condition)
5. ❌ **Committing code with failed verification** (quality breach)
6. ❌ **Not updating specification on verification failure** (lost tracking)
7. ❌ **Sub-agent spawns verification agent** (violates authority hierarchy)

### Consequences

Any violation results in:
1. **IMMEDIATE REVERT** of any committed code
2. **STOP ALL WORK** until violation is corrected
3. **DOCUMENT in Learning Log** (violation details)
4. **REPORT to user** (transparency)
5. **RE-RUN proper workflow** (correct process)

### User Impact

Violations have severe consequences:
- ❌ **Broken builds** in production
- ❌ **Failed tests** discovered too late
- ❌ **Race conditions** from concurrent verifications
- ❌ **Lost work** from reverts
- ❌ **Wasted time** fixing avoidable issues
- ❌ **User frustration** and lost trust

**THE USER WILL BE VERY UPSET** if this workflow is not followed!

## Summary

**Core Workflow**:
```
Implement (TDD: Test → Red → Code → Green → Refactor) → Self-Review → Document Learnings → Report → Verify (Scripts + Checks) → Fix Autonomously if Clear → Update Spec → Commit → Push
```

**Key Principles**:
1. ✅ Agents are smart: Make sensible decisions that maintain quality
2. ✅ **Work Priority**: Fix tests → Pass all checks → Complete features
3. ✅ **Zero Tolerance**: No bugs, failures, incomplete tests, or sidestepping
4. ✅ **Autonomous Fixes**: Fix clear issues (lint, format, simple bugs) without asking
5. ✅ **Seek Approval Only**: When unclear, breaking rules, or dangerous operations
6. ✅ **Commit Correction**: Use soft undo for fixes to just-committed features

**Workflow Rules**:
1. ✅ Implementation agents NEVER commit directly
2. ✅ TDD MANDATORY: Write tests FIRST, verify failure, then implement
3. ✅ Self-review before reporting completion
4. ✅ Document learnings (spec-specific or stack-generic)
5. ✅ Main Agent delegates to verification ALWAYS
6. ✅ ONE verification agent per language stack
7. ✅ ALL checks must PASS before commit
8. ✅ Fix cycle continues until passing (autonomous when clear)
9. ✅ ONLY Main Agent spawns verification agents

**Verification Requirements**:
- ✅ Main Agent asks about verification scripts during spec creation
- ✅ Agents check requirements.md, Makefile, package.json for scripts
- ✅ User scripts run BEFORE standard checks
- ✅ Any failed script fails entire verification

**Self-Review Checklist**:
- ✅ TDD followed: Tests first, verified failing, then implementation
- ✅ Completeness: No partial/fake code
- ✅ Code quality: Clear logic, error handling
- ✅ Simplicity: Can it be simpler? Max 2-3 nesting levels
- ✅ DRY vs Clarity: OK to duplicate 2-5 lines if abstraction adds complexity
- ✅ Requirements alignment: Matches requirements.md
- ✅ Test coverage: WHY/WHAT docs on every test (2-5 lines)

**Learning Documentation**:
- ✅ Specification-specific → `specifications/NN-name/learnings.md`
- ✅ Stack-generic → `.agents/stacks/[stack].md`
- ✅ Concise: 1-2 lines max, 5-second scan test
- ✅ Test details go in test comments, not learnings.md

**Zero Tolerance**:
- ❌ No bypassing verification
- ❌ No skipping self-review or TDD
- ❌ No implementing before tests
- ❌ No incomplete/fake code
- ❌ No tests without WHY/WHAT docs
- ❌ No overly complex code
- ❌ No skipping fixes when needed
- ❌ No asking user for clearly fixable issues

**Result**: **100% VERIFIED, COMPLETE, SIMPLE, WELL-DOCUMENTED, TEST-DRIVEN CODE** with smart autonomous agents who fix issues without unnecessary user interruption.

---
*Created: 2026-01-11*
*Last Updated: 2026-01-19 (Split into Rule 05 for orchestration and Rule 13 for implementation)*

---

## Related Rules

- **Rule 13 (Implementation Agent Guide)**: Concise guide for implementation agents (TDD, self-review, learnings)
- **Rule 08 (Verification Workflow)**: Details for verification agents
- **Rule 06 (Specifications)**: How specifications work with verification
