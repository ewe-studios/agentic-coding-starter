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

## Claude Code-Specific Behaviors

### Behavior 1: Tool Results Format

When Claude calls tools, it expects results in this format:

```xml
<function_results>
<result>
<name>tool_name</name>
<output>Tool output here</output>
</result>
</function_results>
```

For multiple tool results (parallel calls):

```xml
<function_results>
<result>
<name>get_weather</name>
<output>{"temperature": 72, "condition": "sunny"}</output>
</result>
<result>
<name>get_weather</name>
<output>{"temperature": 45, "condition": "rainy"}</output>
</result>
<result>
<name>get_weather</name>
<output>{"temperature": 18, "condition": "cloudy"}</output>
</result>
</function_calls>

### Behavior 2: Error Handling in Results

When a tool fails, return error in the output:

```xml
<function_results>
<result>
<name>Read</name>
<output><tool_use_error>File does not exist.</tool_use_error></output>
</result>
</function_results>
```

### Behavior 3: Conversation Flow

Complete request-response cycle:

**User Message**: "Read the config file and show me the database settings"

**Claude Response** (includes tool call):
```
I'll read the config file for you.

<function_calls>
<invoke name="Read">
<parameter name="file_path">/config/database.json</parameter>
</invoke>
</function_calls>
```

**System Returns** (tool results):
```xml
<function_results>
<result>
<name>Read</name>
<output>{"host": "localhost", "port": 5432, "database": "myapp"}</output>
</result>
</function_results>
```

**Claude Response** (after receiving results):
```
Based on the config file, here are your database settings:
- Host: localhost
- Port: 5432
- Database: myapp
```

### Behavior 4: Tool Schema Definition

When calling Claude API, define tools using JSON Schema:

```json
{
  "name": "get_weather",
  "description": "Get the current weather in a given location",
  "input_schema": {
    "type": "object",
    "properties": {
      "location": {
        "type": "string",
        "description": "The city and state, e.g. San Francisco, CA"
      },
      "unit": {
        "type": "string",
        "enum": ["celsius", "fahrenheit"],
        "description": "The unit of temperature"
      }
    },
    "required": ["location"]
  }
}
```

### Behavior 5: Stop Reasons

Claude API returns `stop_reason` to indicate why generation stopped:

- `"end_turn"` - Normal completion
- `"tool_use"` - Claude wants to call tools
- `"max_tokens"` - Hit token limit
- `"stop_sequence"` - Hit custom stop sequence

Check for `"tool_use"` to know when to execute tools.

### Behavior 6: Thinking Tags (Claude 3.5+)

Claude may use thinking tags for reasoning:

```xml
<thinking>
I need to read the file first to see its contents before editing.
</thinking>

<function_calls>
<invoke name="Read">
<parameter name="file_path">/config.json</parameter>
</invoke>
</function_calls>
```

These are Claude's internal thoughts - don't execute as tool calls.

### Behavior 7: Text Before and After Tool Calls

Claude can include explanatory text:

```
Let me check those files for you.

<function_calls>
<invoke name="Read">
<parameter name="file_path">/file1.txt</parameter>
</invoke>
<invoke name="Read">
<parameter name="file_path">/file2.txt</parameter>
</invoke>
</function_calls>

I'll analyze the contents once I receive them.
```

Parse the function_calls block, but preserve surrounding text for context.

### Behavior 8: Tool Use Continuation

After receiving tool results, Claude automatically continues:

1. You send message with tool results
2. Claude processes results
3. Claude may call more tools OR respond to user
4. Continue loop until Claude responds without tool calls

### Behavior 9: Parallel vs Sequential Detection

**Parallel** - Claude puts multiple invoke tags in ONE function_calls block:
```xml
<function_calls>
<invoke name="tool1">...</invoke>
<invoke name="tool2">...</invoke>
</function_calls>
```
Execute all tools simultaneously, return all results together.

**Sequential** - Claude makes second call after receiving first results:
```xml
<!-- First turn -->
<function_calls>
<invoke name="Read">...</invoke>
</function_calls>

<!-- System returns results -->

<!-- Second turn (after processing results) -->
<function_calls>
<invoke name="Edit">...</invoke>
</function_calls>
```

### Behavior 10: Empty Parameters

Some tools may have optional parameters. Omit the parameter tag entirely:

```xml
<function_calls>
<invoke name="list_files">
<parameter name="directory">/home</parameter>
<!-- No 'recursive' parameter means use default -->
</invoke>
</function_calls>
```

Don't use empty tags: `<parameter name="recursive"></parameter>`

---

*Updated: 2026-01-22*
*Added: Claude Code-specific behaviors and API integration patterns*
