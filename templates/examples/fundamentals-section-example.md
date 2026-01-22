# Fundamentals Documentation Section Example

This example shows how to structure the "User-Facing Documentation Requirements" section in requirements.md when fundamentals documentation is needed.

## When to Use Fundamentals Documentation

Set `has_fundamentals: true` in requirements.md frontmatter when:
- Implementing new user-facing libraries or APIs
- Creating reusable components users need to understand deeply
- Introducing complex patterns, algorithms, or abstractions
- Building foundational primitives or developer tools
- User needs to make architectural decisions using this code

## Example Section for requirements.md

Add this section to requirements.md when `has_fundamentals: true`:

```markdown
## User-Facing Documentation Requirements (MANDATORY)

**CRITICAL**: Create `fundamentals/` directory with comprehensive user documentation.

### Fundamentals Documentation (REQUIRED)

Create in `specifications/[NN-spec-name]/fundamentals/`:
1. **00-overview.md** - Introduction, quick start, decision trees
2. **01-core-concepts.md** - Fundamental concepts and terminology
3. **02-architecture.md** - System design and component relationships
4. **03-api-reference.md** - Complete API documentation with examples
5. **04-advanced-usage.md** - Advanced patterns and best practices
6. **05-troubleshooting.md** - Common issues and solutions

**Documentation Principles**:
- **Explain WHY** - Design decisions and trade-offs
- **Show internals** - Implementation details with commentary
- **Provide examples** - Compilable, real-world usage
- **Discuss trade-offs** - When to use, when NOT to use
- **Be self-contained** - No external resources needed
```

## Corresponding Tasks in tasks.md

Add these tasks as HIGHEST PRIORITY in tasks.md:

```markdown
### Fundamentals Documentation (HIGHEST PRIORITY - DO FIRST)
- [ ] Write `00-overview.md` - Introduction and selection guide
- [ ] Write `01-core-concepts.md` - Fundamental concepts and terminology
- [ ] Write `02-architecture.md` - System design and component relationships
- [ ] Write `03-api-reference.md` - Complete API documentation with examples
- [ ] Write `04-advanced-usage.md` - Advanced patterns and best practices
- [ ] Write `05-troubleshooting.md` - Common issues and solutions
```

## Critical Implementation Order

**Documentation MUST come BEFORE implementation**:

1. ✅ Read fundamentals list from requirements.md
2. ✅ Create fundamentals/ directory first
3. ✅ Write ALL fundamental documents listed
4. ✅ Mark fundamentals tasks complete
5. ✅ ONLY THEN start implementation coding

## Why This Order Matters

- **Clarifies API design**: Thinking about user documentation helps design better APIs
- **Prevents poor design**: Hard-to-explain APIs indicate design problems
- **Catches flaws early**: Documentation reveals design issues before coding
- **User-centric approach**: Ensures features are designed for users, not just implementation convenience

## Principles for Good Fundamentals Documentation

1. **Explain WHY**: Don't just document what it does—explain why design decisions were made
2. **Show internals**: Include implementation details with clear commentary
3. **Provide examples**: All examples must be compilable and realistic
4. **Discuss trade-offs**: Explain when to use (and when NOT to use) features
5. **Be self-contained**: Users shouldn't need external resources to understand
