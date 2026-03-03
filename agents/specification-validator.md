---
name: "Specification Validator"
description: "Validates that specifications and features meet standards and have all required components"
approved: Yes
created: 2026-03-03
license: "MIT"
metadata:
  author: "Main Agent"
  version: "1.0"
  last_updated: "2026-03-03"
  tags: [specifications, validation, quality, standards]
tools: [Read, Grep, Glob]
files: []
---

# Specification Validator Agent

## Purpose

Validates that specifications and features conform to standards, have all required components, and follow best practices.

**Type**: Automated validation agent
**Trigger**: After specification or feature creation
**Usage**: Spawned automatically by specification creation process

## What This Agent Validates

### Specification-Level Checks

1. **Required Files Exist**:
   - [ ] `requirements.md` - Must exist
   - [ ] `start.md` - Must exist (workflow entry point)
   - [ ] `LEARNINGS.md` - Must exist (permanent learnings)

2. **Front Matter Validation** (`requirements.md`):
   - [ ] `description` field present
   - [ ] `status` field present (draft/in-progress/completed)
   - [ ] `priority` field present (low/medium/high)
   - [ ] `created` date present
   - [ ] `has_features` field present (true/false)
   - [ ] If `has_features: true`, `features` section present

3. **Content Structure** (`requirements.md`):
   - [ ] `# Overview` section exists
   - [ ] `## Goals` section exists
   - [ ] If `has_features: true`, `## Feature Index` exists
   - [ ] `## Success Criteria` section exists

4. **start.md Validation**:
   - [ ] Contains workflow instructions
   - [ ] References other documentation files
   - [ ] Appropriate variant used (simple spec vs feature-based)

### Feature-Level Checks (if has_features: true)

For each feature in `features/` directory:

1. **Required Files Exist**:
   - [ ] `feature.md` - Must exist
   - [ ] `start.md` - Must exist

2. **Front Matter Validation** (`feature.md`):
   - [ ] `workspace_name` field present
   - [ ] `spec_directory` field present
   - [ ] `feature_directory` field present
   - [ ] `status` field present (pending/in-progress/completed)
   - [ ] `priority` field present
   - [ ] `created` date present
   - [ ] `depends_on` field present (array of feature dependencies)
   - [ ] `tasks` section with completion tracking

3. **Content Structure** (`feature.md`):
   - [ ] `# [Feature Name] Feature` heading exists
   - [ ] `## Overview` section exists
   - [ ] `## Dependencies` section exists
   - [ ] `## Requirements` section exists
   - [ ] `## Implementation Phases` section exists
   - [ ] `## Success Criteria` section exists
   - [ ] `## Verification Commands` section exists

4. **Feature start.md Validation**:
   - [ ] Contains feature-specific workflow
   - [ ] References feature.md
   - [ ] Includes implementation phases
   - [ ] Has LEARNINGS.md reminder

### Quality Checks

1. **Documentation Quality**:
   - [ ] No placeholder text (TODO, TBD, etc.)
   - [ ] Code examples have language tags
   - [ ] Links to other files are valid
   - [ ] No broken internal references

2. **Best Practices**:
   - [ ] LEARNINGS.md reminder present in start.md
   - [ ] Success criteria are measurable
   - [ ] Dependencies clearly stated
   - [ ] Implementation phases defined

3. **Naming Conventions**:
   - [ ] Specification directory: `NN-kebab-case-name`
   - [ ] Feature directory: `kebab-case-name`
   - [ ] File names: lowercase with hyphens or UPPERCASE

## Validation Process

### Step 1: Identify Specification Type

```bash
# Read requirements.md front matter
has_features=$(grep "has_features:" requirements.md | cut -d: -f2 | tr -d ' ')

if [ "$has_features" = "true" ]; then
    echo "Feature-based specification"
else
    echo "Simple specification"
fi
```

### Step 2: Validate Required Files

```bash
# Specification-level
required_files=("requirements.md" "start.md" "LEARNINGS.md")

for file in "${required_files[@]}"; do
    if [ ! -f "$file" ]; then
        echo "ERROR: Missing required file: $file"
    fi
done
```

### Step 3: Validate Front Matter

```rust
// Parse YAML front matter
let doc = read_file("requirements.md");
let front_matter = extract_front_matter(doc);

// Check required fields
required_fields = ["description", "status", "priority", "created", "has_features"];
for field in required_fields {
    if !front_matter.contains_key(field) {
        error!("Missing required field: {}", field);
    }
}
```

### Step 4: Validate Features (if applicable)

```bash
if [ "$has_features" = "true" ]; then
    # Find all feature directories
    for feature_dir in features/*/; do
        echo "Validating feature: $feature_dir"

        # Check required files
        if [ ! -f "$feature_dir/feature.md" ]; then
            echo "ERROR: Missing feature.md in $feature_dir"
        fi

        if [ ! -f "$feature_dir/start.md" ]; then
            echo "ERROR: Missing start.md in $feature_dir"
        fi
    done
fi
```

### Step 5: Generate Validation Report

```markdown
# Specification Validation Report

**Specification**: specifications/NN-spec-name
**Validation Date**: YYYY-MM-DD
**Status**: PASS / FAIL

## Required Files
- [x] requirements.md
- [x] start.md
- [x] LEARNINGS.md

## Front Matter
- [x] All required fields present
- [x] Valid status value
- [x] Valid priority value

## Content Structure
- [x] Overview section exists
- [x] Goals section exists
- [x] Success criteria defined

## Features (3 total)
- [x] foundation - All files present
- [x] core-api - All files present
- [ ] advanced - Missing start.md

## Issues Found
1. features/advanced/start.md is missing
2. features/foundation/feature.md missing LEARNINGS.md reminder

## Recommendations
- Add missing start.md to advanced feature
- Update foundation feature.md with LEARNINGS.md reminder

## Overall Assessment
FAIL - 2 issues must be fixed
```

## Agent Workflow

When spawned by specification creation:

1. **Locate Specification**
   ```bash
   spec_dir=$1  # Passed as argument
   cd "$spec_dir"
   ```

2. **Run Validation Checks**
   - File existence checks
   - Front matter validation
   - Content structure validation
   - Feature validation (if applicable)

3. **Generate Report**
   - Create VALIDATION_REPORT.md
   - List all issues found
   - Provide recommendations

4. **Report Results**
   - Print summary to console
   - Exit with code 0 (pass) or 1 (fail)

## Integration with Spec Creation

Update specifications-management skill to spawn validator:

```markdown
### Pattern: Feature-Based Specification

1. User requests complex feature
2. Socratic conversation (10+ questions)
3. Create specifications/NN-feature/requirements.md (overview only)
4. Create features/00-foundation/feature.md (detailed)
5. Create features/01-core/feature.md
6. Set has_features: true
7. Create spec-level start.md (Variant 1 - redirect to features)
8. Create feature-level start.md (Variant 2) for EACH feature
9. **SPAWN SPECIFICATION VALIDATOR AGENT** ← NEW STEP
10. Fix any issues found
11. Get user approval
12. Implement feature-by-feature
```

## Validation Rules

### MUST HAVE (Errors)

- requirements.md exists
- start.md exists
- LEARNINGS.md exists
- Front matter has required fields
- If has_features: true, features/ directory exists
- Each feature has feature.md
- Each feature has start.md

### SHOULD HAVE (Warnings)

- LEARNINGS.md reminder in start.md
- ARCHITECTURE.md for complex features
- Verification commands defined
- Success criteria are measurable
- Code examples have language tags

### NICE TO HAVE (Info)

- VERIFICATION.md exists
- REPORT.md exists
- Templates directory
- Scripts directory

## Example Usage

```bash
# Manual validation
claude-code-agent specification-validator specifications/02-build-http-client

# Automatic validation (spawned by spec creation)
# Happens automatically when creating new spec or feature
```

## Output Format

### Console Output

```
🔍 Validating Specification: 02-build-http-client
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ PASS: requirements.md exists
✅ PASS: start.md exists
✅ PASS: LEARNINGS.md exists
✅ PASS: Front matter valid
✅ PASS: Content structure valid

📁 Validating Features (14 total)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ foundation - All checks passed
✅ connection - All checks passed
⚠️  websocket - Missing LEARNINGS.md reminder
❌ server-sent-events - Missing ARCHITECTURE.md

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 VALIDATION SUMMARY
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Total Checks: 45
Passed: 42
Warnings: 1
Errors: 1

❌ VALIDATION FAILED

Issues to fix:
1. [ERROR] features/server-sent-events/ARCHITECTURE.md missing
2. [WARNING] features/websocket/start.md missing LEARNINGS.md reminder

Recommendations:
- Add ARCHITECTURE.md to server-sent-events feature
- Add LEARNINGS.md reminder to websocket start.md
```

## Configuration

Agent can be configured via `.agents/agents/specification-validator.yaml`:

```yaml
name: specification-validator
enabled: true
auto_spawn: true  # Automatically spawned after spec creation
strict_mode: false  # If true, warnings are errors
checks:
  required_files: true
  front_matter: true
  content_structure: true
  features: true
  learnings_reminder: true
  architecture_for_complex: true
```

## Benefits

1. **Consistency**: Ensures all specs follow same structure
2. **Quality**: Catches missing components early
3. **Best Practices**: Enforces LEARNINGS.md reminders
4. **Automation**: Runs automatically on spec creation
5. **Documentation**: Generates validation reports

## Future Enhancements

- [ ] Validate links to external documents
- [ ] Check code example syntax
- [ ] Validate task completion tracking
- [ ] Ensure feature dependencies are acyclic
- [ ] Validate that completed specs have VERIFICATION.md

---

_Version: 1.0 - Created: 2026-03-03_
_Ensures specifications meet quality standards_
