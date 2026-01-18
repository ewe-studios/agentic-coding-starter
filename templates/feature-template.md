---
feature: feature-name
description: Brief one-sentence description of what this feature implements
status: pending
depends_on: []
estimated_effort: medium
created: YYYY-MM-DD
last_updated: YYYY-MM-DD
---

# [Feature Name]

## Overview

Brief summary of what this feature implements and its purpose within the larger specification.

## Dependencies

This feature depends on:
- `[other-feature]` - Why this dependency exists

This feature is required by:
- `[dependent-feature]` - Why this feature is needed

## Requirements

### Functional Requirements

1. **Requirement 1**
   - Detail about the requirement
   - Expected behavior

2. **Requirement 2**
   - Detail about the requirement
   - Expected behavior

### Technical Requirements

- **Pattern to follow**: Description of required patterns
- **Types to create**: List of types/structs to implement
- **Integrations**: What this connects to

## Implementation Details

### Key Structures

```rust
// Example structure - or reference templates/
pub struct ExampleStruct {
    // fields
}
```

### Key Functions

| Function | Purpose | Location |
|----------|---------|----------|
| `function_name()` | What it does | `file.rs` |

## Templates

See `templates/` directory for:
- `example-struct.rs` - Base structure template
- `example-impl.rs` - Implementation template

## Success Criteria

- [ ] Criterion 1 - specific and verifiable
- [ ] Criterion 2 - specific and verifiable
- [ ] All unit tests pass
- [ ] Code passes `cargo fmt` and `cargo clippy`

## Verification Commands

```bash
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test --package [package] -- [test_filter]
cargo build --package [package]
```

## Notes for Agents

### Before Starting
- **MUST READ** parent specification's requirements.md
- **MUST VERIFY** dependent features are complete
- **MUST READ** any templates referenced

### Implementation Guidelines
- Follow existing patterns in codebase
- Use types from dependent features
- Update tasks.md as work progresses

---
*Created: YYYY-MM-DD*
*Last Updated: YYYY-MM-DD*
