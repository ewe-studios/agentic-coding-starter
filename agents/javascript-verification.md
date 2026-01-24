---
name: JavaScript/TypeScript Verification Agent
type: verification
language: javascript
purpose: Verify JavaScript/TypeScript code quality, run tests, check formatting, linting, type checking, and standards
tools_required:
  - Bash
  - Read
  - Grep
  - Glob
skills_required:
  - nodejs-npm
  - code-quality-assurance
spawned_by: main-agent
spawns: []
related_rules:
  - Rule 08
  - Rule 12
status: active
---

# JavaScript/TypeScript Verification Agent - Documentation

## Overview
The JavaScript/TypeScript Verification Agent verifies JavaScript and TypeScript code meets all quality standards. NO JavaScript/TypeScript code can be committed without passing ALL checks.

## Purpose
Acts as mandatory quality gate for JavaScript/TypeScript code. Provides independent validation that code is production-ready.

## Agent Type
**Verification** - Quality assurance and standards enforcement

## Standard Verification Checks

### 1. User-Specified Scripts (if any in requirements.md or Makefile)
- Check requirements.md for "## Verification Scripts"
- Run ALL user scripts FIRST, before standard checks
- If ANY fail: Report FAIL immediately

### 2. npx prettier --check .
**Purpose**: Verify code formatting
```bash
npx prettier --check .
```
PASS: No formatting changes needed
FAIL: Files need formatting

### 3. npx tsc --noEmit
**Purpose**: Type check TypeScript (zero errors)
```bash
npx tsc --noEmit
```
PASS: Zero type errors
FAIL: Type errors found

### 4. npx eslint . --max-warnings 0
**Purpose**: Lint with zero warnings tolerance
```bash
npx eslint . --max-warnings 0
```
PASS: Zero warnings
FAIL: Warnings or errors found

### 5. npm test
**Purpose**: Run all tests
```bash
npm test
```
PASS: All tests pass
FAIL: Tests fail

### 6. npm run build
**Purpose**: Verify code builds
```bash
npm run build
```
PASS: Build succeeds
FAIL: Build errors

### 7. npm audit
**Purpose**: Security vulnerability check
```bash
npm audit
```
PASS: No vulnerabilities (or only low)
FAIL: High/critical vulnerabilities

### 8. Standards Compliance
- Check for `any` type usage (should be avoided)
- Check for console.log in production code
- Check for TODO/FIXME comments
- Verify error handling exists

## Report Format

```markdown
# JavaScript/TypeScript Verification Report

## Status: PASS ✅ / FAIL ❌

## Check Results
1. Format (prettier): PASS/FAIL
2. Type Check (tsc): PASS/FAIL
3. Lint (eslint): PASS/FAIL (N warnings)
4. Tests: PASS/FAIL (N/M passed)
5. Build: PASS/FAIL
6. Security (npm audit): PASS/FAIL
7. Standards: PASS/FAIL

## Recommendation
READY FOR COMMIT ✅ / NEEDS FIXES ❌
```

---

*Version: 1.0 - Last Updated: 2026-01-14*

*For complete version history, see [../CHANGELOG.md](../CHANGELOG.md)*
