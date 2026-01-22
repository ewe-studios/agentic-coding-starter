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

## Usage

These examples are referenced throughout Rule 06 to:
- ✅ Reduce the main rule file size (saved 129 lines)
- ✅ Make examples reusable across documentation
- ✅ Allow independent updates to examples
- ✅ Improve searchability and organization
- ✅ Follow DRY (Don't Repeat Yourself) principle

## Maintenance

When updating these examples:
1. Update the specific example file
2. Verify references in Rule 06 are still accurate
3. Check if related template files need updates
4. Commit with clear message explaining changes

---

*Created: 2026-01-22*
*Purpose: DRY refactoring of Rule 06 inline examples*
