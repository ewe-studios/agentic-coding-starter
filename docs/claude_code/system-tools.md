# Claude Code System Tools Reference

## Overview

This document provides a complete reference of all tools available to Claude in the Claude Code CLI environment. These tools enable file operations, command execution, agent spawning, and more.

## Tool Categories

### 1. Agent Orchestration Tools

#### **Task**
**Purpose**: Launch specialized agents for complex, multi-step tasks

**Available Agent Types:**
- `Bash` - Command execution specialist for git, npm, etc.
- `general-purpose` - Multi-step research and code search
- `statusline-setup` - Configure status line settings
- `Explore` - Fast codebase exploration (quick/medium/thorough)
- `Plan` - Software architect for implementation planning
- `claude-code-guide` - Help with Claude Code/API questions

**Key Parameters:**
- `subagent_type` - Type of agent to spawn (required)
- `prompt` - Task for the agent (required)
- `description` - 3-5 word summary (required)
- `model` - Optional: sonnet/opus/haiku
- `run_in_background` - Run agent asynchronously
- `resume` - Resume previous agent by ID

**Usage Notes:**
- Launch multiple agents concurrently when possible
- Provide clear, detailed prompts
- Agents with context access see full conversation history
- Background agents write to output_file for checking later

#### **TaskOutput**
**Purpose**: Retrieve output from running or completed tasks

**Parameters:**
- `task_id` - Task identifier (required)
- `block` - Wait for completion (default: true)
- `timeout` - Max wait time in ms (default: 30000)

#### **Skill**
**Purpose**: Execute predefined skills from `.agents/skills/`

**Parameters:**
- `skill` - Skill name (required)
- `args` - Optional arguments

**Usage Notes:**
- Check if skill already running before invoking
- Skills provide specialized domain capabilities

---

### 2. File System Tools

#### **Read**
**Purpose**: Read files from the filesystem

**Parameters:**
- `file_path` - Absolute path to file (required)
- `offset` - Starting line number (optional)
- `limit` - Number of lines to read (optional)

**Capabilities:**
- Default: Read up to 2000 lines
- Can read images (PNG, JPG) visually
- Can read PDFs page by page
- Can read Jupyter notebooks (.ipynb)
- Lines >2000 chars are truncated
- Returns cat -n format with line numbers

**Usage Notes:**
- ALWAYS prefer Read over cat/head/tail commands
- Read multiple files in parallel when possible
- Must read file before using Edit or Write on it

#### **Write**
**Purpose**: Write or overwrite files

**Parameters:**
- `file_path` - Absolute path (required)
- `content` - File content (required)

**Usage Notes:**
- MUST read file first if it exists
- ALWAYS prefer editing over writing new files
- Overwrites existing files completely
- Never create documentation files proactively

#### **Edit**
**Purpose**: Perform exact string replacements in files

**Parameters:**
- `file_path` - Absolute path (required)
- `old_string` - Text to replace (required)
- `new_string` - Replacement text (required)
- `replace_all` - Replace all occurrences (default: false)

**Usage Notes:**
- MUST read file with Read tool first
- Preserve exact indentation from Read output
- old_string must be unique unless using replace_all
- Fails if old_string not found or ambiguous
- Use replace_all for renaming variables

#### **NotebookEdit**
**Purpose**: Edit Jupyter notebook cells

**Parameters:**
- `notebook_path` - Absolute path to .ipynb (required)
- `new_source` - Cell content (required)
- `cell_id` - Cell identifier (optional)
- `cell_type` - code/markdown (optional)
- `edit_mode` - replace/insert/delete (default: replace)

#### **Glob**
**Purpose**: Fast file pattern matching

**Parameters:**
- `pattern` - Glob pattern like "**/*.js" (required)
- `path` - Directory to search (optional, defaults to cwd)

**Usage Notes:**
- ALWAYS use Glob over find/ls commands
- Returns files sorted by modification time
- Works with any codebase size
- Use Task tool for open-ended searches

#### **Grep**
**Purpose**: Powerful content search using ripgrep

**Parameters:**
- `pattern` - Regex pattern to search (required)
- `path` - File/directory to search (optional)
- `output_mode` - content/files_with_matches/count
- `glob` - Filter files by pattern (e.g., "*.js")
- `type` - Filter by file type (js, py, rust, etc.)
- `-i` - Case insensitive search
- `-A/-B/-C` - Context lines after/before/around
- `-n` - Show line numbers (default: true)
- `multiline` - Enable multiline matching
- `head_limit` - Limit output lines
- `offset` - Skip first N lines

**Usage Notes:**
- ALWAYS use Grep over grep/rg commands
- Use Task tool for multi-round searches
- Literal braces need escaping in patterns

---

### 3. Command Execution Tools

#### **Bash**
**Purpose**: Execute bash commands with optional timeout

**Parameters:**
- `command` - Command to execute (required)
- `description` - What the command does (recommended)
- `timeout` - Timeout in ms (max: 600000, default: 120000)
- `run_in_background` - Run asynchronously

**Key Usage Guidelines:**

**DO Use Bash For:**
- git operations (status, diff, commit, push, pull)
- npm/cargo/pip package management
- docker commands
- build systems (make, cmake, etc.)
- Process management

**DON'T Use Bash For:**
- File reading (use Read tool)
- File writing (use Write tool)
- File editing (use Edit tool)
- File search (use Glob tool)
- Content search (use Grep tool)

**Command Chaining:**
- Use && for sequential dependent commands
- Use ; for sequential independent commands
- Make multiple parallel Bash calls for independent operations
- ALWAYS quote paths with spaces

**Git Safety Protocol:**
- NEVER use --force flags
- NEVER use --no-verify
- NEVER use git commit --amend without explicit request
- NEVER use git reset --hard without approval
- ALWAYS commit immediately after changes
- ALWAYS push automatically after commit

**Special Git Operations:**
- Creating commits: Use HEREDOC for multi-line messages
- Creating PRs: Use gh pr create with proper formatting
- Include Co-Authored-By: Claude <noreply@anthropic.com>

---

### 4. Planning and User Interaction Tools

#### **EnterPlanMode**
**Purpose**: Transition into plan mode for implementation planning

**When to Use:**
- New feature implementation with multiple approaches
- Code modifications affecting existing behavior
- Architectural decisions needed
- Multi-file changes (>2-3 files)
- Unclear requirements needing exploration

**When NOT to Use:**
- Single-line or few-line fixes
- Tasks with very specific instructions
- Pure research tasks (use Task tool with explore agent)

#### **ExitPlanMode**
**Purpose**: Signal completion of plan and request user approval

**Parameters:**
- `allowedPrompts` - Prompt-based permissions needed
- `pushToRemote` - Push to Claude.ai session

**Usage Notes:**
- Only use after writing plan to plan file
- Do NOT use for research tasks
- Inherently requests user approval

#### **AskUserQuestion**
**Purpose**: Ask user questions during execution

**Parameters:**
- `questions` - Array of 1-4 questions (required)
  - `question` - The question text
  - `header` - Short label (max 12 chars)
  - `options` - 2-4 answer choices
  - `multiSelect` - Allow multiple selections

**Usage Notes:**
- Users can always select "Other" for custom input
- Use in plan mode to clarify requirements BEFORE finalizing plan
- Do NOT ask "Is my plan ready?" (use ExitPlanMode)

#### **TodoWrite**
**Purpose**: Create and manage structured task lists

**Parameters:**
- `todos` - Array of todo items
  - `content` - Imperative form ("Run tests")
  - `activeForm` - Present continuous ("Running tests")
  - `status` - pending/in_progress/completed

**When to Use:**
- Complex multi-step tasks (3+ steps)
- User provides multiple tasks
- Non-trivial tasks requiring organization

**When NOT to Use:**
- Single straightforward task
- Trivial tasks (<3 steps)
- Purely conversational requests

**Requirements:**
- Update in real-time as work progresses
- Mark complete IMMEDIATELY after finishing
- Exactly ONE task in_progress at a time
- Only mark completed when FULLY accomplished

---

### 5. Web and Knowledge Tools

#### **WebFetch**
**Purpose**: Fetch and analyze web content

**Parameters:**
- `url` - URL to fetch (required)
- `prompt` - What to extract from content (required)

**Usage Notes:**
- HTTP upgraded to HTTPS automatically
- 15-minute self-cleaning cache
- Handles redirects (will inform and provide redirect URL)
- Prefer gh CLI for GitHub URLs
- Prefer MCP web fetch if available

#### **WebSearch**
**Purpose**: Search the web for current information

**Parameters:**
- `query` - Search query (required)
- `allowed_domains` - Include only these domains
- `blocked_domains` - Exclude these domains

**Usage Notes:**
- CRITICAL: MUST include Sources section in response
- Use correct year (2026) in queries
- Only available in US
- Format sources as markdown links

---

### 6. Process Management Tools

#### **KillShell**
**Purpose**: Terminate background bash shells

**Parameters:**
- `shell_id` - Shell identifier to kill (required)

**Usage Notes:**
- Use /tasks command to find shell IDs
- For long-running background processes

---

## Tool Selection Guidelines

### Priority Order for Common Tasks

**File Reading:**
1. Read tool (NOT cat/head/tail)

**File Writing:**
1. Edit tool (if file exists)
2. Write tool (for new files)
3. NEVER echo/sed/awk

**File Finding:**
1. Glob tool (NOT find/ls)

**Content Search:**
1. Grep tool (NOT grep/rg command)

**Command Execution:**
1. Bash tool (for git, npm, docker, etc.)
2. Multiple parallel calls for independent operations
3. Chain with && for sequential dependencies

**Complex Tasks:**
1. Task tool to spawn specialized agents
2. Launch multiple agents concurrently
3. Resume agents with resume parameter

**User Interaction:**
1. AskUserQuestion for clarifications
2. EnterPlanMode for planning approval
3. TodoWrite for task tracking

## Tool Usage Statistics

**Current Session:**
- Total tool calls: ~12
- Read operations: 7 (loading rules and config)
- Bash operations: 0
- Write operations: 0
- Agent spawns: 0

## Notes

- Tools are provided by Claude Code CLI infrastructure
- Tool descriptions embedded in system prompt
- Some tools have parameter validation and requirements
- Failed tool calls return error messages with details
- Large outputs (>30k chars) are truncated

---
*Generated: 2026-01-21*
*Claude Code CLI Version: Latest*
*Tool Count: 16 primary tools*
