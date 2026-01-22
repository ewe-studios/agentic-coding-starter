# Common Claude Tool Calling Mistakes

This file documents common mistakes when implementing Claude tool calling and how to fix them.

## Mistake 1: Missing function_calls Wrapper

### ❌ WRONG
```
<invoke name="get_weather">
<parameter name="location">SF</parameter>
</invoke>
```

### ✅ CORRECT
```
<function_calls>
<invoke name="get_weather">
<parameter name="location">SF</parameter>
</invoke>
</function_calls>
```

**Why**: Every tool call must be wrapped in function_calls block.

## Mistake 2: Passing All Parameters as JSON

### ❌ WRONG
```
<function_calls>
<invoke name="create_user">
<parameter name="params">{"name": "Alice", "email": "alice@example.com"}</parameter>
</invoke>
</function_calls>
```

### ✅ CORRECT
```
<function_calls>
<invoke name="create_user">
<parameter name="name">Alice</parameter>
<parameter name="email">alice@example.com</parameter>
</invoke>
</function_calls>
```

**Why**: Simple parameters should be individual tags. Only use JSON for complex nested structures.

## Mistake 3: Unclosed Tags

### ❌ WRONG
```
<function_calls>
<invoke name="Read">
<parameter name="file_path">/path/to/file.txt
</invoke>
</function_calls>
```

### ✅ CORRECT
```
<function_calls>
<invoke name="Read">
<parameter name="file_path">/path/to/file.txt</parameter>
</invoke>
</function_calls>
```

**Why**: All XML tags must be properly closed.

## Mistake 4: Wrong Parallel/Sequential Pattern

### ❌ WRONG (Sequential calls in parallel block)
```
<function_calls>
<invoke name="Read">
<parameter name="file_path">/config.json</parameter>
</invoke>
<invoke name="Edit">
<parameter name="file_path">/config.json</parameter>
<parameter name="old_string">{USE_CONTENTS_FROM_READ}</parameter>
<parameter name="new_string">{NEW_VALUE}</parameter>
</invoke>
</function_calls>
```

### ✅ CORRECT (Separate blocks for sequential)
```
<function_calls>
<invoke name="Read">
<parameter name="file_path">/config.json</parameter>
</invoke>
</function_calls>

<!-- Wait for Read result -->

<function_calls>
<invoke name="Edit">
<parameter name="file_path">/config.json</parameter>
<parameter name="old_string">{USE_ACTUAL_CONTENTS}</parameter>
<parameter name="new_string">{NEW_VALUE}</parameter>
</invoke>
</function_calls>
```

**Why**: If second tool needs results from first, use separate blocks.

## Mistake 5: Inconsistent Parameter Names

### ❌ WRONG
```
<function_calls>
<invoke name="get_user">
<parameter name="user_id">123</parameter>
</invoke>
<invoke name="get_user">
<parameter name="id">456</parameter>
</invoke>
</function_calls>
```

### ✅ CORRECT
```
<function_calls>
<invoke name="get_user">
<parameter name="user_id">123</parameter>
</invoke>
<invoke name="get_user">
<parameter name="user_id">456</parameter>
</invoke>
</function_calls>
```

**Why**: Use consistent parameter names as defined in tool schema.

## Mistake 6: Malformed JSON in Parameters

### ❌ WRONG
```
<function_calls>
<invoke name="create_user">
<parameter name="data">{name: "Alice", email: "alice@example.com"}</parameter>
</invoke>
</function_calls>
```

### ✅ CORRECT
```
<function_calls>
<invoke name="create_user">
<parameter name="data">{"name": "Alice", "email": "alice@example.com"}</parameter>
</invoke>
</function_calls>
```

**Why**: JSON requires quoted keys and proper syntax.

## Mistake 7: Missing Required Parameters

### ❌ WRONG
```
<function_calls>
<invoke name="Edit">
<parameter name="file_path">/config.json</parameter>
<parameter name="new_string">Updated value</parameter>
</invoke>
</function_calls>
```

### ✅ CORRECT
```
<function_calls>
<invoke name="Edit">
<parameter name="file_path">/config.json</parameter>
<parameter name="old_string">Old value</parameter>
<parameter name="new_string">Updated value</parameter>
</invoke>
</function_calls>
```

**Why**: Edit requires both old_string and new_string parameters.

## Mistake 8: Wrong Tool Name

### ❌ WRONG
```
<function_calls>
<invoke name="read_file">
<parameter name="path">/config.json</parameter>
</invoke>
</function_calls>
```

### ✅ CORRECT
```
<function_calls>
<invoke name="Read">
<parameter name="file_path">/config.json</parameter>
</invoke>
</function_calls>
```

**Why**: Tool name must exactly match schema (case-sensitive).

## Mistake 9: Nested invoke Tags

### ❌ WRONG
```
<function_calls>
<invoke name="tool1">
  <invoke name="tool2">
    <parameter name="param">value</parameter>
  </invoke>
</invoke>
</function_calls>
```

### ✅ CORRECT
```
<function_calls>
<invoke name="tool1">
<parameter name="param">value1</parameter>
</invoke>
<invoke name="tool2">
<parameter name="param">value2</parameter>
</invoke>
</function_calls>
```

**Why**: invoke tags cannot be nested - use flat structure in function_calls.

## Mistake 10: Whitespace Issues in Parameter Values

### ⚠️ CAREFUL
```
<function_calls>
<invoke name="search">
<parameter name="query">
  multi-line
  value
</parameter>
</invoke>
</function_calls>
```

**Result**: Parameter value will include newlines and leading whitespace.

### ✅ BETTER
```
<function_calls>
<invoke name="search">
<parameter name="query">multi-line value</parameter>
</invoke>
</function_calls>
```

**Why**: Whitespace inside parameter tags is preserved.

## Error Messages

### Common Error: "No such tool available"
- **Cause**: Tool name doesn't match schema
- **Fix**: Check exact tool name spelling and case

### Common Error: "Missing required parameter"
- **Cause**: Required parameter not provided
- **Fix**: Add all required parameters from tool schema

### Common Error: "InputValidationError"
- **Cause**: Parameter value doesn't match expected type
- **Fix**: Check parameter types in schema and use correct format

### Common Error: "Unexpected parameter"
- **Cause**: Parameter name doesn't exist in tool schema
- **Fix**: Check parameter names match schema exactly

## Debugging Checklist

When tool calls fail:

- [ ] Check function_calls wrapper is present
- [ ] Verify tool name matches schema exactly (case-sensitive)
- [ ] Confirm all required parameters are provided
- [ ] Validate parameter names match schema
- [ ] Ensure all tags are properly closed
- [ ] Check JSON syntax if using JSON in parameters
- [ ] Verify sequential vs parallel pattern is correct
- [ ] Look for typos in tool or parameter names

---

*Created: 2026-01-22*
*Reference: Common mistakes in Claude tool calling and how to fix them*
