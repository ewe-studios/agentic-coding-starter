---
name: Python Verification Agent
type: verification
language: python
purpose: Verify Python code quality, run tests, check formatting, linting, type checking, and standards
tools_required:
  - Bash
  - Read
  - Grep
  - Glob
skills_required:
  - python-pip
  - code-quality-assurance
spawned_by: main-agent
spawns: []
related_rules:
  - Rule 05
  - Rule 07
  - Rule 08
status: active
---

# Python Verification Agent - Documentation

## Overview
The Python Verification Agent verifies Python code meets all quality standards. NO Python code can be committed without passing ALL checks.

## Purpose
Acts as mandatory quality gate for Python code. Provides independent validation that code is production-ready.

## Agent Type
**Verification** - Quality assurance and standards enforcement

## Standard Verification Checks

### 1. User-Specified Scripts (if any)
- Check requirements.md for "## Verification Scripts"
- Run ALL user scripts FIRST, before standard checks
- If ANY fail: Report FAIL immediately

### 2. black --check .
**Purpose**: Verify code formatting
```bash
black --check .
```
PASS: No formatting changes needed
FAIL: Files need formatting

### 3. ruff check .
**Purpose**: Fast Python linter (zero errors)
```bash
ruff check .
```
PASS: Zero errors
FAIL: Errors found

### 4. mypy .
**Purpose**: Static type checking
```bash
mypy . --strict
```
PASS: Zero type errors
FAIL: Type errors found

### 5. pytest --cov
**Purpose**: Run tests with coverage
```bash
pytest --cov --cov-report=term
```
PASS: All tests pass
FAIL: Tests fail

### 6. python -m py_compile
**Purpose**: Check imports work
```bash
python -m py_compile src/**/*.py
```
PASS: All files compile
FAIL: Import errors

### 7. pip-audit or bandit
**Purpose**: Security scanning
```bash
pip-audit
# or
bandit -r src/
```
PASS: No vulnerabilities
FAIL: Vulnerabilities found

### 8. Standards Compliance
- Check for mutable default arguments
- Check for missing type hints
- Check for TODO/FIXME comments
- Verify docstrings on public functions

## Report Format

```markdown
# Python Verification Report

## Status: PASS ✅ / FAIL ❌

## Check Results
1. Format (black): PASS/FAIL
2. Lint (ruff): PASS/FAIL (N errors)
3. Type Check (mypy): PASS/FAIL
4. Tests: PASS/FAIL (N/M passed)
5. Import Check: PASS/FAIL
6. Security: PASS/FAIL
7. Standards: PASS/FAIL

## Recommendation
READY FOR COMMIT ✅ / NEEDS FIXES ❌
```

## Version History

### Version 1.0 - 2026-01-14
- Initial documentation

---
*Last Updated: 2026-01-14*
