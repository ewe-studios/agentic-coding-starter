# Commit Message Templates and Examples

Complete reference for commit message formatting with templates and real-world examples.

## Purpose

Provides standardized commit message formats for consistency, clarity, and proper co-authorship attribution across all commits.

## Commit Message Format (MANDATORY)

Every commit message **MUST** include:

1. **Brief summary line** (50 characters or less)
2. **Blank line**
3. **Detailed explanation** of what was changed and why
4. **Bullet-point summary** of specific changes
5. **Blank line**
6. **Verification status** (if code changes were made)
7. **Blank line** (if verification section included)
8. **Co-authorship attribution**: `Co-Authored-By: Claude <noreply@anthropic.com>`

---

## Template 1: Non-Code Changes

Use for documentation, configuration, specification updates, etc.

```
Brief summary of change

Detailed explanation of what was changed and why this change
was necessary. Explain the context and reasoning behind the
modification.

Changes made:
- Specific change 1
- Specific change 2
- Specific change 3

Co-Authored-By: Claude <noreply@anthropic.com>
```

---

## Template 2: Code Changes (with Verification)

Use for all code changes after verification passes.

```
Brief summary of change

Detailed explanation of what was changed and why this change
was necessary. Explain the context and reasoning behind the
modification.

Changes made:
- Specific change 1
- Specific change 2
- Specific change 3

Verified by [Language] Verification Agent: All checks passed
- Format: PASS
- Lint: PASS
- Type Check: PASS (if applicable)
- Tests: [N]/[N] PASS
- Build: PASS
- Coverage: [N]%

Co-Authored-By: Claude <noreply@anthropic.com>
```

---

## Real-World Examples

### Example 1: Adding a New Feature with Verification

```bash
git add src/middleware/auth.js
git commit -m "$(cat <<'EOF'
Add authentication middleware for API routes

Implemented JWT-based authentication middleware to secure API
endpoints. This middleware validates JWT tokens and attaches
user information to the request object.

Changes made:
- Created auth.js middleware with token validation
- Added JWT verification using jsonwebtoken library
- Implemented error handling for invalid/expired tokens
- Added user object attachment to req.user
- Wrote comprehensive test suite

Verified by JavaScript Verification Agent: All checks passed
- Format: PASS (prettier)
- Lint: PASS (eslint, 0 warnings)
- Type Check: PASS (tsc)
- Tests: 12/12 PASS
- Build: PASS
- Coverage: 94%

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
git push
```

### Example 2: Bug Fix

```bash
git add src/validators/user-validator.js
git commit -m "$(cat <<'EOF'
Fix email validation regex to support plus addressing

Fixed bug where email addresses with plus signs (user+tag@domain.com)
were incorrectly rejected by the validation logic.

Changes made:
- Updated email regex pattern to include plus sign
- Added test cases for plus addressing
- Updated validation error messages

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
git push
```

### Example 3: Documentation Update

```bash
git add README.md
git commit -m "$(cat <<'EOF'
Update installation instructions for Node 18+

Updated documentation to reflect new Node.js version requirement
and simplified installation steps based on user feedback.

Changes made:
- Changed minimum Node.js version to 18.0.0
- Removed deprecated npm install flags
- Added troubleshooting section for common issues
- Fixed formatting in code examples

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
git push
```

### Example 4: Rust Code Change with Verification

```bash
git add src/auth/token.rs tests/token_tests.rs
git commit -m "$(cat <<'EOF'
Implement JWT token generation and validation

Created robust JWT token handling module with generation,
validation, and refresh token support.

Changes made:
- Implemented TokenManager struct with key management
- Added generate_token() with configurable expiration
- Added validate_token() with signature verification
- Implemented refresh_token() for token renewal
- Added comprehensive error handling
- Wrote test suite with 100% coverage

Verified by Rust Verification Agent: All checks passed
- Format: PASS (rustfmt)
- Lint: PASS (clippy, 0 warnings)
- Tests: 24/24 PASS
- Build: PASS
- Coverage: 100%

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
git push
```

### Example 5: Refactoring

```bash
git add src/utils/helpers.js
git commit -m "$(cat <<'EOF'
Refactor string utilities to use modern ES6 methods

Modernized string utility functions to leverage native ES6
string methods, improving performance and readability.

Changes made:
- Replaced manual string manipulation with ES6 methods
- Removed deprecated lodash dependencies
- Simplified complex functions using destructuring
- Updated function signatures for better type inference
- Maintained backward compatibility

Verified by JavaScript Verification Agent: All checks passed
- Format: PASS (prettier)
- Lint: PASS (eslint, 0 warnings)
- Type Check: PASS (tsc)
- Tests: 18/18 PASS (all existing tests pass)
- Build: PASS
- Coverage: 98%

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
git push
```

### Example 6: Configuration Change

```bash
git add .eslintrc.json
git commit -m "$(cat <<'EOF'
Update ESLint config to enforce stricter TypeScript rules

Enhanced linting configuration to catch more potential issues
in TypeScript code and enforce team coding standards.

Changes made:
- Enabled @typescript-eslint/strict-boolean-expressions
- Added @typescript-eslint/no-floating-promises
- Configured explicit-function-return-type for public APIs
- Updated max-line-length to 100 characters
- Added custom rules for import ordering

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
git push
```

### Example 7: Specification Update

```bash
git add specifications/01-http-client/tasks.md
git commit -m "$(cat <<'EOF'
Update tasks.md: mark HTTP client connection pooling complete

Marked connection pooling tasks as complete after successful
implementation and verification.

Changes made:
- Marked 3 connection pooling tasks as [x]
- Updated frontmatter: completed: 12, uncompleted: 3
- Updated completion_percentage to 80%

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
git push
```

### Example 8: Multiple File Changes

```bash
git add src/api/routes.ts src/api/controllers.ts tests/api_tests.ts
git commit -m "$(cat <<'EOF'
Add RESTful API endpoints for user management

Implemented complete CRUD API for user management with proper
authentication, validation, and error handling.

Changes made:
- Created routes.ts with 5 RESTful endpoints
- Implemented controllers with business logic
- Added input validation middleware
- Implemented proper error responses
- Added comprehensive test coverage
- Updated API documentation

Verified by TypeScript Verification Agent: All checks passed
- Format: PASS (prettier)
- Lint: PASS (eslint, 0 warnings)
- Type Check: PASS (tsc, strict mode)
- Tests: 35/35 PASS
- Build: PASS
- Coverage: 96%

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
git push
```

---

## Key Points

### Summary Line (First Line)
- ✅ 50 characters or less
- ✅ Imperative mood ("Add" not "Added" or "Adds")
- ✅ No period at the end
- ✅ Clearly describes what the commit does

**Good Examples**:
- ✅ "Add user authentication middleware"
- ✅ "Fix memory leak in connection pool"
- ✅ "Update dependencies to latest versions"

**Bad Examples**:
- ❌ "Added authentication" (past tense)
- ❌ "Fixes a bug where the application crashed when..." (too long)
- ❌ "Updates." (not descriptive)

### Detailed Explanation
- ✅ Explain WHY the change was made
- ✅ Provide context and reasoning
- ✅ Keep paragraphs focused (3-5 sentences)
- ✅ Write for someone who doesn't have context

### Changes Made Section
- ✅ Bullet points for each significant change
- ✅ Specific and concrete
- ✅ Focus on WHAT was changed
- ✅ Ordered logically (not chronologically)

### Verification Section (Code Changes Only)
- ✅ Include language/tool used
- ✅ List all check types
- ✅ Show pass/fail status
- ✅ Include test count and coverage
- ✅ Only include if code was changed

### Co-Authorship
- ✅ Always include `Co-Authored-By: Claude <noreply@anthropic.com>`
- ✅ Last line of commit message
- ✅ Exactly this format (GitHub recognizes it)

---

## Common Mistakes

### ❌ Missing Verification Status (Code Changes)
```
Add authentication middleware

Implemented JWT authentication.

Changes made:
- Created middleware

Co-Authored-By: Claude <noreply@anthropic.com>
```
**Problem**: Code change but no verification status!

### ❌ Vague Summary
```
Fixed stuff

Fixed some issues.

Changes made:
- Fixed things

Co-Authored-By: Claude <noreply@anthropic.com>
```
**Problem**: Not specific, no context!

### ❌ Missing Co-Authorship
```
Add authentication middleware

Implemented JWT authentication.

Changes made:
- Created middleware
```
**Problem**: Missing `Co-Authored-By` line!

### ❌ No Details
```
Add auth

Co-Authored-By: Claude <noreply@anthropic.com>
```
**Problem**: No explanation, no changes listed!

---

## Using HEREDOC for Multi-Line Messages

Always use HEREDOC for proper formatting:

```bash
git commit -m "$(cat <<'EOF'
Your multi-line
commit message
here
EOF
)"
```

**Why HEREDOC?**:
- ✅ Preserves line breaks
- ✅ Handles special characters
- ✅ Easier to read and maintain
- ✅ Prevents quoting issues

---

## Integration with Rules

- **Rule 03 (Dangerous Operations)**: Commit after every change
- **Rule 04 (Work Commit and Push)**: Immediate commit and automatic push
- **Rule 05 (Agent Orchestration)**: Verification before commit
- **Rule 08 (Verification Workflow)**: Complete verification details

---

*Created: 2026-01-22*
*Referenced in: Rule 04, section "Commit Message Format"*
*Source: Extracted from Rule 04 for better maintainability*
