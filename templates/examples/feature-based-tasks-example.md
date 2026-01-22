# Feature-Based Tasks.md Example

This example shows how to structure the main `tasks.md` file when using feature-based specifications.

## Key Points

- Main `tasks.md` is CONCISE (1-2KB)
- Shows feature priority order with status
- Shows feature task counts (e.g., "Tasks: 12")
- Shows dependencies between features
- Does NOT contain individual task checkboxes (those go in feature tasks.md files)
- Does NOT contain implementation notes (those go in feature tasks.md files)

## Example: HTTP Client Feature Progress

```markdown
---
completed: 1
uncompleted: 4
created: 2026-01-18
features:
  - foundation
  - connection
  - request-response
  - task-iterator
  - public-api
---

# HTTP Client - Feature Progress

## Feature Priority Order

1. [x] **foundation** - Error types and DNS resolution
2. [ ] **connection** - URL parsing, TCP, TLS (depends on: foundation)
3. [ ] **request-response** - Request builder, response types (depends on: connection)
4. [ ] **task-iterator** - TaskIterator, executors (depends on: request-response)
5. [ ] **public-api** - User-facing API, integration (depends on: task-iterator)

## Notes
- Complete features in order due to dependencies
- Each feature has its own tasks.md with detailed checkboxes
```

## What This Achieves

1. **Context Efficiency**: Main Agent can quickly see overall progress without loading 15KB+ files
2. **Clear Dependencies**: Shows which features must complete before others
3. **Feature Tracking**: Simple checkbox format for high-level progress
4. **Detail Separation**: Individual task details live in feature-specific files
