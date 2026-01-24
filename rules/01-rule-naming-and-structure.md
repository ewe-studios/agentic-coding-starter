# Rule Naming and Structure Policy

## Purpose
This rule establishes the naming conventions and structural requirements for all rules in the `.agents/rules/` directory.

## Requirements

### File Format
- **MUST** be a Markdown file with `.md` extension
- **MUST** use dash (`-`) as the word separator
- **MUST** have a clear, descriptive name that articulates what the rule is about
- **MUST** be prefixed with a two-digit incrementing number (01, 02, 03, etc.)

### Naming Convention
```
NN-rule-name-describing-the-policy.md
```

Where `NN` is a two-digit number (01, 02, 03, etc.) that determines the loading order.

**Format Breakdown:**
- **Numerical Prefix**: Two-digit number (01-99) followed by a dash
- **Rule Name**: Descriptive name using dash separators
- **File Extension**: `.md`

**Loading Order:**
- Rules are loaded in numerical order based on their prefix
- Lower numbers are loaded first (01 loads before 02, which loads before 03, etc.)
- This ensures that foundational rules are loaded before rules that depend on them

**Examples of Good Names:**
- `01-rule-naming-and-structure.md`
- `02-rules-directory-policy.md`
- `03-commit-message-format.md`
- `04-code-review-requirements.md`
- `05-testing-standards.md`

**Examples of Bad Names:**
- `rule1.md` (incorrect prefix format, not descriptive)
- `1-rule-naming.md` (single digit instead of two digits)
- `01_rule_naming.md` (uses underscore instead of dash)
- `01-ruleNaming.md` (uses camelCase instead of dash)
- `01 my rule.md` (contains spaces)
- `rule-naming-and-structure.md` (missing numerical prefix)

### Directory Structure
- **MUST** use a flat structure - all rules in `.agents/rules/` directly
- **MUST NOT** create subdirectories within `.agents/rules/`
- **MUST NOT** nest rules in multiple levels of directories
- Each rule **MUST** be in its own separate file

### Template Policy (CRITICAL)

**MANDATORY REQUIREMENT**: Rules **MUST NOT** embed full templates directly in rule files.

**Template Extraction Rule:**
- ❌ **FORBIDDEN**: Embedding complete templates (skill.md, requirements.md, agent docs, etc.) directly in rule files
- ✅ **REQUIRED**: Extract templates to `.agents/templates/` directory
- ✅ **REQUIRED**: Reference templates from rule files (e.g., "See `.agents/templates/skill-template.md`")
- ✅ **REQUIRED**: Keep only brief structure descriptions in rule files

**Why This Matters:**
1. **Context Window Efficiency**: Templates are often 100-400 lines each
2. **DRY Principle**: Update template once, not in multiple rule files
3. **Faster Loading**: Agents load rules 40-50% faster without embedded templates
4. **Better Maintenance**: Templates centralized and easier to update
5. **Selective Loading**: Agents read templates only when needed

**Template Directory:**
```
.agents/templates/
├── skill-template.md              # For Rule 09 (Skills)
├── learnings-template.md          # For Rule 09 (Skills) and Rule 05 (Orchestration)
├── requirements-template.md       # For Rule 06 (Specifications)
├── tasks-template.md              # For Rule 06 (Specifications)
├── PROGRESS-template.md           # For Rule 06 (Specifications)
├── REPORT-template.md             # For Rule 06 (Specifications)
├── LEARNINGS-template.md          # For Rule 06 (Specifications)
├── VERIFICATION-template.md       # For Rule 06 (Specifications)
└── agent-documentation-template.md  # For Rule 10 (Agent Registry)
```

**What Rules Should Contain:**
- ✅ Brief description of what the template is for
- ✅ Key sections list (bullet points)
- ✅ Reference to template file location
- ✅ Critical requirements about the template
- ❌ NOT the full template content

**Good Example:**
```markdown
## Skill File Format

Every skill MUST follow the skill template structure.

**Template Location**: `.agents/templates/skill-template.md`

**Required Sections:**
- Frontmatter (name, description, approved, tools, files)
- Overview
- When to Use
- Prerequisites
- Step-by-Step Guide
- Examples
- References

**Critical Requirements:**
- Frontmatter must include all required fields
- Usage type must be clearly stated (TEMPLATE/EXECUTABLE/EDUCATIONAL)
- All attached files must be documented

See `.agents/templates/skill-template.md` for complete template.
```

**Bad Example:**
```markdown
## Skill File Format

[300 lines of full template content embedded here]
```

**Enforcement:**
- Any rule with embedded templates (>50 lines of template code) **MUST** be refactored
- Templates **MUST** be extracted to `.agents/templates/`
- Rule **MUST** be updated to reference the template file
- This policy applies to ALL rules (existing and new)

**Correct Structure:**
```
.agents/
├── rules/
│   ├── 01-rule-naming-and-structure.md
│   ├── 02-rules-directory-policy.md
│   ├── 03-commit-message-format.md
│   └── 04-testing-standards.md
└── templates/
    ├── skill-template.md
    ├── requirements-template.md
    └── [other-template].md
```

**Incorrect Structure:**
```
.agents/
└── rules/
    ├── coding/
    │   └── 01-style-guide.md       ❌ NO subdirectories
    ├── process/
    │   └── 02-review-process.md    ❌ NO subdirectories
    ├── all-rules.md                ❌ NO combining multiple rules
    └── rule-naming.md              ❌ Missing numerical prefix
```

## Rationale

### File Naming and Structure
- **Clarity**: Descriptive names make rules easy to find and understand
- **Consistency**: Uniform naming convention improves discoverability
- **Ordering**: Numerical prefixes ensure rules are loaded in a predictable, controlled order
- **Dependencies**: Loading order allows foundational rules to be applied before dependent rules
- **Simplicity**: Flat structure prevents organizational complexity
- **Accessibility**: All rules are immediately visible without navigation

### Template Extraction Policy
- **Context Efficiency**: Reduces rule file sizes by 40-50%, saving AI context window space
- **DRY Principle**: Single source of truth for templates - update once, affects all usage
- **Selective Loading**: Agents load templates only when needed, not every time they read rules
- **Faster Rule Loading**: 2,666 lines removed from rules in optimization (42% reduction)
- **Better Maintenance**: Templates centralized in one location for easy updates
- **Improved Readability**: Rules focus on policies, not template content
- **Reusability**: Templates can be used by any agent, any specification, any skill

## Enforcement

### File Naming Violations
Any rule that violates naming requirements must be:
1. Renamed to follow the naming convention
2. Moved to the flat `.agents/rules/` structure
3. Split into separate files if combining multiple rules

### Template Policy Violations
Any rule with embedded templates must be:
1. **Reviewed for template content** (>50 lines of template code)
2. **Templates extracted** to `.agents/templates/` directory
3. **Rule updated** to reference template file instead
4. **Commit with clear message** explaining the extraction

**Detection:**
```bash
# Check for long embedded templates in rules
for rule in .agents/rules/*.md; do
  templates=$(grep -c '```markdown' "$rule")
  if [ $templates -gt 2 ]; then
    echo "⚠️  $rule may have embedded templates"
  fi
done
```

**Corrective Action:**
1. Identify embedded template sections
2. Extract to appropriate template file in `.agents/templates/`
3. Replace with brief description + template reference
4. Test that reference is clear and accessible
5. Commit changes

---
*Created: 2026-01-11*
*Last Updated: 2026-01-18 (Added Template Policy for context efficiency)*
