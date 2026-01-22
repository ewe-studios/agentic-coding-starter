# Specification Examples

This directory contains extracted examples from Rule 06 (Specifications and Requirements Management) to improve maintainability and follow DRY principles.

## Available Examples

### 1. Feature-Based Specifications

#### [feature-based-tasks-example.md](./feature-based-tasks-example.md)
Example of how to structure the main `tasks.md` file when using feature-based specifications.
- Shows feature priority order
- Demonstrates task counts and dependencies
- Referenced in Rule 06, line 279

#### [feature-frontmatter-examples.md](./feature-frontmatter-examples.md)
Complete frontmatter examples for feature files:
- `feature.md` frontmatter with status, dependencies, effort
- `tasks.md` frontmatter with completion tracking
- Referenced in Rule 06, line 283

### 2. Specification Versioning

#### [builds-on-example.md](./builds-on-example.md)
Example showing how to create new specifications that build upon completed specifications.
- When to use `builds_on` field
- Proper frontmatter structure
- Specification versioning principles
- Referenced in Rule 06, line 348

### 3. Documentation Requirements

#### [fundamentals-section-example.md](./fundamentals-section-example.md)
Example of "User-Facing Documentation Requirements" section for specifications that need fundamentals documentation.
- When to set `has_fundamentals: true`
- Structure for requirements.md section
- Corresponding tasks.md entries
- Documentation principles
- Referenced in Rule 06, line 1127

### 4. Git Workflow

#### [git-workflow-examples.md](./git-workflow-examples.md)
Complete examples of proper git commit and push practices:
- Atomic commits during implementation
- Final commit after completion
- Commit message formats
- Safety rules and best practices
- Referenced in Rule 06, line 1307

### 5. Self-Containment Requirements

#### [cross-reference-links-example.md](./cross-reference-links-example.md)
Example of mandatory cross-reference links for requirements.md:
- Top link after frontmatter (to tasks.md and learnings.md)
- Bottom link at end (to verification.md)
- Why these links matter
- Validation checklist
- Referenced in Rule 06, section "Requirements.md Self-Containment"

#### [enhanced-frontmatter-example.md](./enhanced-frontmatter-example.md)
Example of enhanced frontmatter for requirements.md (Rule 06 v6.0):
- `metadata.stack_files` array
- `metadata.skills` array
- `has_features` boolean
- `has_fundamentals` boolean
- Migration from old format
- Validation checklist
- Referenced in Rule 06, section "Requirements.md Self-Containment"

### 6. Completion and Verification

#### [completion-verification-section-example.md](./completion-verification-section-example.md)
Complete "MANDATORY Completion and Verification Requirements" section:
- Task completion verification (100% required)
- Code/implementation verification
- Documentation verification
- Quality verification (zero tolerance)
- Specification tracking verification
- Verification issue resolution
- Validation script example
- Referenced in Rule 06, section "Mandatory 100% Completion Verification"

#### [validation-commands-example.md](./validation-commands-example.md)
Exact bash commands for specification completion validation:
- Task validation commands
- File existence checks
- Quality validation (build, test, lint)
- Frontmatter validation
- Documentation quality checks
- Complete validation script
- Common validation failures
- Referenced in Rule 06, section "Validation Before Marking Complete"

### 7. Agent Orchestration (Rule 05)

#### [agent-identity-reference.md](./agent-identity-reference.md)
Complete reference for MAIN AGENT vs SUB-AGENT distinction:
- Quick identity check flowchart
- Authority hierarchy and verification spawning rules
- Self-awareness requirements
- Common violations and corrections
- Referenced in Rule 05 and Rule 08, section "Agent Identity and Verification Authority"

#### [workflow-success-example.md](./workflow-success-example.md)
Complete successful workflow from user request to commit:
- Step-by-step breakdown of ideal path
- Agent interactions and authority respect
- Quality gates and success factors
- Time savings from first-attempt pass
- Referenced in Rule 05, section "Complete Workflow Examples"

#### [workflow-failure-example.md](./workflow-failure-example.md)
Failed verification with fix cycle and recovery:
- Complete workflow with failures and fixes
- Verification failure handling
- Fix cycle process
- Re-verification and commit
- Comparison of with vs without verification
- Referenced in Rule 05, section "Complete Workflow Examples"

#### [test-documentation-examples.md](./test-documentation-examples.md)
Comprehensive test documentation guide (WHY/WHAT/IMPORTANCE):
- Language-specific examples (Rust, TypeScript, Python, Go, Java, C#)
- Format requirements and validation checklist
- DO and DON'T guidelines
- Quick reference card
- Referenced in Rule 05, section "Test Documentation Requirements"

### 8. Agent Documentation (Rule 10)

#### [agent-frontmatter-reference.md](./agent-frontmatter-reference.md)
Complete frontmatter field reference for agent documentation:
- All required and optional fields with descriptions
- Validation rules and format requirements
- Field-by-field explanations
- Update guidelines and versioning
- Common mistakes and corrections
- Referenced in Rule 10, section "Frontmatter Fields Reference"

### 9. Commit Messages (Rule 04)

#### [commit-message-templates.md](./commit-message-templates.md)
Comprehensive commit message guide with templates and examples:
- Mandatory format requirements
- Two standard templates (code and non-code)
- 8+ real-world examples (feature, bug fix, refactor, docs, config)
- Language-specific examples (Rust, TypeScript, JavaScript)
- Common mistakes and corrections
- HEREDOC usage guide
- Referenced in Rule 04, section "Commit Message Format"

## Usage

These examples are referenced throughout Rules 04, 05, 06, 08, and 10 to:
- ✅ Reduce the main rule files size (saved 700+ lines across all rules)
- ✅ Make examples reusable across documentation
- ✅ Allow independent updates to examples
- ✅ Improve searchability and organization
- ✅ Follow DRY (Don't Repeat Yourself) principle
- ✅ Provide copy-paste templates for common needs
- ✅ Consolidate related concepts in single files

## Summary Statistics

**Total Templates/Examples**: 11 files
**Total Lines Extracted**: 5,000+ lines
**Rules Consolidated**: 5 major rules (04, 05, 06, 08, 10)
**Total Reduction**: 400+ lines from rules (~13% average)
**Context Optimization**: Significant improvement for agent loading

## Maintenance

When updating these examples:
1. Update the specific example file
2. Verify references in Rule 06 are still accurate
3. Check if related template files need updates
4. Commit with clear message explaining changes

---

*Created: 2026-01-22*
*Purpose: DRY refactoring of Rule 06 inline examples*
