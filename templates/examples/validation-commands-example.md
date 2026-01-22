# Validation Commands for Specification Completion

This example provides the exact bash commands Main Agent should use to validate specification completion before marking status as "completed".

## Purpose

These commands provide objective, automated validation that all completion requirements are met. They prevent subjective "looks done" assessments.

## Complete Validation Workflow

### 1. Task Validation

Verify all tasks are complete:

```bash
cd specifications/[NN-spec-name]/

# Must return 0 (no unchecked tasks)
grep -c "^- \[ \]" tasks.md

# Must return total task count (all checked)
grep -c "^- \[x\]" tasks.md
```

**Expected output**:
- First command: `0` (zero uncompleted tasks)
- Second command: `N` (where N is total tasks)

**If first command returns > 0**:
- ❌ Specification NOT complete
- ❌ Cannot mark as completed
- ✅ Fix incomplete tasks first

### 2. File Existence Validation

Verify all mandatory files exist:

```bash
cd specifications/[NN-spec-name]/

# Must all exist (returns 0 if all found)
ls tasks.md learnings.md progress.md verification.md requirements.md

# If has_fundamentals: true in frontmatter
ls fundamentals/*.md
```

**Expected output**:
- All files listed without error
- No "No such file or directory" errors

**Common missing files**:
- `learnings.md` - often forgotten
- `progress.md` - skipped "because it's small"
- `verification.md` - assumed verbal approval counts

### 3. Quality Validation

Verify build, tests, and linter pass:

```bash
# Rust example (adjust for other languages)

# Build validation
cargo build
# Expected: "Finished dev [unoptimized + debuginfo] target(s)"

# Test validation
cargo test
# Expected: "test result: ok. X passed; 0 failed"

# Linter validation (ZERO warnings)
cargo clippy -- -D warnings
# Expected: "Finished dev [unoptimized + debuginfo] target(s)"
# NO warnings output

# Formatter validation
cargo fmt -- --check
# Expected: No output (code is formatted)
```

**Language-Specific Commands**:

**TypeScript/JavaScript**:
```bash
npm run build     # Must succeed
npm test          # Must show 100% pass
npm run lint      # Must show 0 errors, 0 warnings
npm run format    # Must show no changes needed
```

**Python**:
```bash
python -m build                    # Must succeed
pytest                             # Must show 100% pass
ruff check . --select=ALL          # Must show 0 warnings
black --check .                    # Must show no formatting needed
```

**Go**:
```bash
go build ./...              # Must succeed
go test ./...               # Must show PASS
golangci-lint run           # Must show 0 issues
gofmt -l .                  # Must return nothing
```

### 4. Frontmatter Validation

Verify frontmatter accuracy:

```bash
cd specifications/[NN-spec-name]/

# Extract frontmatter values (requires yq or manual check)
grep "^uncompleted:" tasks.md          # Must be 0
grep "^completion_percentage:" tasks.md # Must be 100
grep "^has_fundamentals:" requirements.md  # Must match reality
grep "^status:" requirements.md        # Should still be in-progress until ALL checks pass
```

**Manual Checks**:
- `tasks.md` frontmatter:
  - `completed`: matches `grep -c "^- \[x\]" tasks.md`
  - `uncompleted`: is `0`
  - `completion_percentage`: is `100`
- `requirements.md` frontmatter:
  - `has_fundamentals`: matches if `fundamentals/` exists
  - `metadata.stack_files`: lists correct stack files
  - `metadata.skills`: lists used skills or `[]`

### 5. Documentation Quality Validation

Verify fundamentals documentation (if `has_fundamentals: true`):

```bash
cd specifications/[NN-spec-name]/

# Check all fundamentals exist
EXPECTED_DOCS=$(grep "^- \[ \] \`fundamentals/" tasks.md | wc -l)
ACTUAL_DOCS=$(ls fundamentals/*.md 2>/dev/null | wc -l)

if [ "$EXPECTED_DOCS" -ne "$ACTUAL_DOCS" ]; then
  echo "❌ FAIL: Missing fundamentals documents"
  exit 1
fi

# Check for stub/placeholder docs (file size too small)
for doc in fundamentals/*.md; do
  SIZE=$(wc -c < "$doc")
  if [ "$SIZE" -lt 500 ]; then
    echo "❌ FAIL: $doc appears to be a stub (too small)"
    exit 1
  fi
done

# Check for broken links in docs
if command -v markdown-link-check &> /dev/null; then
  markdown-link-check fundamentals/*.md
fi
```

## Complete Validation Script

Use this comprehensive script for validation:

```bash
#!/bin/bash
# Complete specification validation script
# Usage: ./validate-spec.sh specifications/01-example-spec

set -e

SPEC_DIR="$1"
if [ -z "$SPEC_DIR" ]; then
  echo "Usage: $0 <specification-directory>"
  exit 1
fi

cd "$SPEC_DIR"

echo "=== Validating Specification: $(basename $SPEC_DIR) ==="

# 1. Task Validation
echo -n "Checking tasks completion... "
UNCOMPLETED=$(grep -c "^- \[ \]" tasks.md || echo "0")
if [ "$UNCOMPLETED" -ne 0 ]; then
  echo "❌ FAIL: $UNCOMPLETED tasks incomplete"
  exit 1
fi
echo "✅ PASS"

# 2. File Existence
echo -n "Checking mandatory files... "
for file in tasks.md learnings.md progress.md requirements.md; do
  if [ ! -f "$file" ]; then
    echo "❌ FAIL: Missing $file"
    exit 1
  fi
done

# Check for verification file (either name is ok)
if [ ! -f "verification.md" ] && [ ! -f "VERIFICATION_SIGNOFF.md" ]; then
  echo "❌ FAIL: Missing verification.md or VERIFICATION_SIGNOFF.md"
  exit 1
fi
echo "✅ PASS"

# 3. Fundamentals (if required)
HAS_FUNDAMENTALS=$(grep "^has_fundamentals: true" requirements.md || echo "")
if [ -n "$HAS_FUNDAMENTALS" ]; then
  echo -n "Checking fundamentals documentation... "
  if [ ! -d "fundamentals" ]; then
    echo "❌ FAIL: has_fundamentals: true but no fundamentals/ directory"
    exit 1
  fi

  FUND_COUNT=$(ls fundamentals/*.md 2>/dev/null | wc -l)
  if [ "$FUND_COUNT" -eq 0 ]; then
    echo "❌ FAIL: fundamentals/ directory empty"
    exit 1
  fi
  echo "✅ PASS ($FUND_COUNT documents)"
fi

# 4. Build and Test (language-specific)
echo -n "Checking build... "
if [ -f "../../Cargo.toml" ]; then
  (cd ../.. && cargo build --quiet 2>&1) || {
    echo "❌ FAIL: Build failed"
    exit 1
  }
elif [ -f "../../package.json" ]; then
  (cd ../.. && npm run build 2>&1) || {
    echo "❌ FAIL: Build failed"
    exit 1
  }
fi
echo "✅ PASS"

echo -n "Checking tests... "
if [ -f "../../Cargo.toml" ]; then
  (cd ../.. && cargo test --quiet 2>&1) | grep -q "test result: ok" || {
    echo "❌ FAIL: Tests failed"
    exit 1
  }
elif [ -f "../../package.json" ]; then
  (cd ../.. && npm test 2>&1) | grep -q "pass" || {
    echo "❌ FAIL: Tests failed"
    exit 1
  }
fi
echo "✅ PASS"

echo -n "Checking linter (zero warnings)... "
if [ -f "../../Cargo.toml" ]; then
  CLIPPY_OUTPUT=$(cd ../.. && cargo clippy -- -D warnings 2>&1)
  if echo "$CLIPPY_OUTPUT" | grep -q "warning:"; then
    echo "❌ FAIL: Clippy warnings present"
    echo "$CLIPPY_OUTPUT"
    exit 1
  fi
elif [ -f "../../package.json" ]; then
  (cd ../.. && npm run lint 2>&1) | grep -q "0 errors, 0 warnings" || {
    echo "❌ FAIL: Linter errors/warnings present"
    exit 1
  }
fi
echo "✅ PASS"

# 5. Frontmatter Validation
echo -n "Checking frontmatter accuracy... "
COMPLETED_COUNT=$(grep "^completed:" tasks.md | awk '{print $2}')
ACTUAL_COMPLETED=$(grep -c "^- \[x\]" tasks.md)
if [ "$COMPLETED_COUNT" -ne "$ACTUAL_COMPLETED" ]; then
  echo "❌ FAIL: completed count mismatch (frontmatter: $COMPLETED_COUNT, actual: $ACTUAL_COMPLETED)"
  exit 1
fi

UNCOMPLETED_COUNT=$(grep "^uncompleted:" tasks.md | awk '{print $2}')
if [ "$UNCOMPLETED_COUNT" -ne 0 ]; then
  echo "❌ FAIL: uncompleted count is $UNCOMPLETED_COUNT (must be 0)"
  exit 1
fi
echo "✅ PASS"

echo ""
echo "🎉 SUCCESS: All validation checks passed!"
echo "This specification is ready to be marked as completed."
```

## Usage Instructions

**For Main Agent**:
1. Run validation script before marking complete
2. If ANY check fails, status remains `in-progress`
3. Fix all failures
4. Re-run validation
5. Only mark `completed` after 100% PASS

**Save script as**: `.agents/scripts/validate-spec.sh`

**Run with**: `bash .agents/scripts/validate-spec.sh specifications/01-example-spec`

## Common Validation Failures

### Tasks Not Complete
```bash
$ grep -c "^- \[ \]" tasks.md
3  # ❌ FAIL - 3 tasks still incomplete
```
**Fix**: Complete all tasks, mark them `[x]`, update frontmatter

### Missing Files
```bash
$ ls learnings.md
ls: learnings.md: No such file or directory  # ❌ FAIL
```
**Fix**: Create learnings.md using template

### Linter Warnings
```bash
$ cargo clippy -- -D warnings
warning: unused variable `x`  # ❌ FAIL
```
**Fix**: Fix all clippy warnings, get clean output

### Test Failures
```bash
$ cargo test
test result: FAILED. 10 passed; 2 failed  # ❌ FAIL
```
**Fix**: Fix failing tests until 100% pass

---

*Created: 2026-01-22*
*Referenced in: Rule 06, section "Validation Before Marking Complete"*
