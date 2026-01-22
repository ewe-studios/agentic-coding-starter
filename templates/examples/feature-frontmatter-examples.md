# Feature Frontmatter Examples

This document shows the required frontmatter for feature files in a feature-based specification.

## 1. Feature.md Frontmatter

Used in `features/[feature-name]/feature.md` files.

### Example

```yaml
---
feature: feature-name
description: Brief one-sentence description
status: pending | in-progress | completed
depends_on:
  - other-feature-name
estimated_effort: small | medium | large
---
```

### Field Descriptions

- **`feature`**: The name of this feature (lowercase with dashes)
- **`description`**: One-sentence summary of what this feature does
- **`status`**: Current state (pending | in-progress | completed)
- **`depends_on`**: Array of other feature names this feature depends on (empty array if none)
- **`estimated_effort`**: Size estimate (small | medium | large)

## 2. Feature Tasks.md Frontmatter

Used in `features/[feature-name]/tasks.md` files.

### Example

```yaml
---
feature: feature-name
completed: 0
uncompleted: 5
last_updated: 2026-01-18
---
```

### Field Descriptions

- **`feature`**: The name of this feature (must match feature.md)
- **`completed`**: Count of `[x]` completed tasks in this file
- **`uncompleted`**: Count of `[ ]` pending tasks in this file
- **`last_updated`**: Date of last update (YYYY-MM-DD)

## Usage

When creating feature-based specifications:

1. Main `requirements.md` references feature names and shows high-level overview
2. Main `tasks.md` lists features with priority order (see `feature-based-tasks-example.md`)
3. Each feature directory contains:
   - `feature.md` - Detailed requirements with frontmatter as shown above
   - `tasks.md` - Individual task checkboxes with frontmatter as shown above
   - `templates/` - (Optional) Feature-specific templates
