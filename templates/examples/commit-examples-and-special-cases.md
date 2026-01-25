# Commit Message Examples and Special Cases

**Reference**: This document contains detailed examples for Rule 04 (Work Commit and Push Rules)

---

## Commit Message Examples

### Good Example: Feature Addition with Verification

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
- Wrote comprehensive test suite

Verified by JavaScript Verification Agent: All checks passed
- Format: PASS (prettier)
- Lint: PASS (eslint, 0 warnings)
- Tests: 12/12 PASS
- Build: PASS

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
git push
```

---

## Bad Practice Examples ❌

### Example 1: Batching multiple unrelated changes

```bash
# Made changes to auth.js, user-validator.js, and README.md
git add .
git commit -m "Updated files"
git push

❌ Multiple unrelated changes in one commit
❌ Non-descriptive commit message
❌ No detailed explanation
❌ No bullet points
❌ Missing co-authorship
```

### Example 2: Committing incomplete work

```bash
# Started implementing feature, only 30% done
git add partial-implementation.js
git commit -m "Add user feature"
git push

❌ Task/feature not complete
❌ Verification not run
❌ Breaks atomicity principle
```

### Example 3: Asking for approval

```bash
git add src/feature.js
git commit -m "Add feature"
# Agent asks: "Should I push this to remote?"

❌ Never ask for approval to push
❌ Push should be automatic after commit
```

### Example 4: Not pushing automatically

```bash
git add src/feature.js
git commit -m "Add feature"
git status
# Agent stops here without pushing

❌ Must automatically push after commit
❌ Workflow is incomplete without push
```

### Example 5: Using force push

```bash
git add src/feature.js
git commit -m "Add feature"
git push --force

❌ Force push is absolutely forbidden
❌ Destructive operation violates safety requirements
```

### Example 6: Committing code without verification (CRITICAL VIOLATION)

```bash
# Implementation agent completes work
git add src/payment/processor.js
git commit -m "$(cat <<'EOF'
Add payment processing module

Implemented payment processing with Stripe integration.

Changes made:
- Created PaymentProcessor class
- Added Stripe API integration
- Implemented error handling

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
git push

❌ CRITICAL VIOLATION: Code committed without verification
❌ No verification agent was delegated to
❌ Tests might be failing
❌ Code might not compile
❌ This violates Rule 05 (ZERO TOLERANCE)
```

---

## Special Cases Reference

### Merge Conflicts

If `git push` fails due to merge conflicts:

```bash
git push
# Error: Updates were rejected because remote contains work...

# Proper resolution:
git pull              # Pull remote changes
# Resolve conflicts if any
git add [files]       # Stage resolved files
git commit -m "..."   # Commit merge resolution (if needed)
git status            # Verify
git push              # Push again (automatic)
```

**Never use `--force` to override conflicts.**

### Branch Protection Rules

If remote has branch protection requiring reviews:

```bash
git push
# Error: Protected branch requires review...

# This is expected behavior
# Agent should report: "Changes committed and push attempted.
# Remote branch requires pull request review per repository settings."
```

**Do not attempt to bypass branch protection rules.**

### First Push to New Branch

```bash
git checkout -b new-feature-branch
# Make changes
git add [files]
git commit -m "..."
git status
git push -u origin new-feature-branch  # Use -u for first push to new branch
```

**The `-u` flag is allowed for setting upstream branch.**

### Network Issues

If `git push` fails due to network issues:

```bash
git push
# Error: Could not resolve host / Connection timeout

# Agent should:
# 1. Report the network error to user
# 2. Note that changes are committed locally and safe
# 3. Do not retry indefinitely
# 4. Do not use --force
```

---

## Main Agent Push Verification Scenarios

### Scenario 1: Sub-agent reports completion without mentioning push

```
Sub-agent: "Task completed. Files changed: [list]. Implementation done."

Main Agent MUST:
1. Check: "Did you git push?"
2. If not mentioned, verify git status
3. If unpushed commits detected, remind: "You must git push per Rule 04"
4. Wait for push confirmation
```

### Scenario 2: Sub-agent commits but doesn't push

```
Sub-agent: "Changes committed successfully."

Main Agent MUST:
1. Immediately ask: "Did you git push?"
2. If no: "You must git push immediately per Rule 04"
3. If yes: "Confirm push with git status output"
4. Verify before proceeding
```

### Scenario 3: Sub-agent says "push failed"

```
Sub-agent: "Commit succeeded but push failed due to [error]"

Main Agent MUST:
1. Review error (network issue? merge conflict?)
2. If recoverable error:
   - Guide sub-agent to resolve
   - Ensure push succeeds before proceeding
3. If unrecoverable:
   - Report to user
   - Note: Changes are safe locally
```

---

_Reference document for Rule 04_
_Last Updated: 2026-01-25_
