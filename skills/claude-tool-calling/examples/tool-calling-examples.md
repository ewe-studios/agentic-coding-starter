# Claude Tool Calling Examples

This file contains complete working examples of Claude's tool calling syntax. These examples use actual XML that would be interpreted by Claude.

## ⚠️ WARNING

The XML examples in this file will be interpreted as actual tool calls if included in a Claude conversation. They are reference examples only.

## Basic Structure

```
<function_calls>
  <invoke name="tool_name">
    <parameter name="param1">value1</parameter>
    <parameter name="param2">value2</parameter>
  </invoke>
</function_calls>
```

## Example 1: Single Tool Call with Simple Parameters

```
<function_calls>
<invoke name="get_weather">
<parameter name="location">San Francisco, CA</parameter>
<parameter name="unit">celsius</parameter>
</invoke>
</function_calls>
```

**Result**: Calls `get_weather` tool with location="San Francisco, CA" and unit="celsius"

## Example 2: Multiple Parallel Tool Calls

```
<function_calls>
<invoke name="get_weather">
<parameter name="location">San Francisco</parameter>
</invoke>
<invoke name="get_weather">
<parameter name="location">New York</parameter>
</invoke>
<invoke name="get_weather">
<parameter name="location">London</parameter>
</invoke>
</function_calls>
```

**Result**: All three weather calls execute in parallel

## Example 3: JSON Parameters for Complex Types

```
<function_calls>
<invoke name="create_user">
<parameter name="user_data">{"name": "Alice", "email": "alice@example.com", "roles": ["admin", "user"]}</parameter>
<parameter name="options">{"send_welcome_email": true, "create_workspace": false}</parameter>
</invoke>
</function_calls>
```

**Result**: Passes nested objects and arrays as JSON within parameter tags

## Example 4: File Operations

```
<function_calls>
<invoke name="Read">
<parameter name="file_path">/path/to/config.json</parameter>
</invoke>
</function_calls>
```

**Result**: Reads file at specified path

## Example 5: Search Operations

```
<function_calls>
<invoke name="Grep">
<parameter name="pattern">TODO</parameter>
<parameter name="path">src/</parameter>
<parameter name="output_mode">files_with_matches</parameter>
</invoke>
</function_calls>
```

**Result**: Searches for "TODO" in src/ directory

## Example 6: Sequential Operations (Separate Blocks)

First call:
```
<function_calls>
<invoke name="Read">
<parameter name="file_path">/config/settings.json</parameter>
</invoke>
</function_calls>
```

Wait for result, then make second call:
```
<function_calls>
<invoke name="Edit">
<parameter name="file_path">/config/settings.json</parameter>
<parameter name="old_string">{"debug": false}</parameter>
<parameter name="new_string">{"debug": true}</parameter>
</invoke>
</function_calls>
```

**Result**: Read file first, then edit based on contents

## Example 7: Parallel Commands

```
<function_calls>
<invoke name="Bash">
<parameter name="command">cargo test</parameter>
</invoke>
<invoke name="Bash">
<parameter name="command">cargo clippy</parameter>
</invoke>
<invoke name="Bash">
<parameter name="command">cargo fmt --check</parameter>
</invoke>
</function_calls>
```

**Result**: Runs all three cargo commands in parallel

## Key Patterns

### Pattern: Simple Parameters
- One parameter tag per parameter
- Plain text values

### Pattern: JSON Parameters
- Use JSON for arrays, objects
- Properly escape quotes if needed

### Pattern: Parallel Execution
- Multiple invoke tags in ONE function_calls block
- Tools execute simultaneously

### Pattern: Sequential Execution
- Multiple function_calls blocks
- Wait for results between blocks

## Parameter Value Guidelines

### String Values
```
<parameter name="message">Hello, world!</parameter>
```

### Numeric Values
```
<parameter name="count">42</parameter>
<parameter name="temperature">98.6</parameter>
```

### Boolean Values
```
<parameter name="enabled">true</parameter>
<parameter name="debug">false</parameter>
```

### Array Values (use JSON)
```
<parameter name="items">["apple", "banana", "cherry"]</parameter>
```

### Object Values (use JSON)
```
<parameter name="config">{"timeout": 30, "retries": 3}</parameter>
```

### Nested Structures (use JSON)
```
<parameter name="data">{
  "user": {
    "name": "Alice",
    "roles": ["admin", "user"]
  },
  "options": {
    "notify": true
  }
}</parameter>
```

---

*Created: 2026-01-22*
*Reference: Complete working examples of Claude tool calling*
