# Claude Code Autocompact Buffer

## Overview

The **autocompact buffer** is a reserved portion of the context window used by Claude Code CLI for automatic context management and compaction. It ensures smooth operation even as the conversation grows.

## What is the Autocompact Buffer?

From the context usage display:
```
⛝ Autocompact buffer: 77.0k tokens (38.5%)
```

**Size**: 77,000 tokens (38.5% of 200k context window)

**Purpose**: Reserved space for automatic context window management when the conversation becomes too large.

## How Autocompact Works

### Context Window Breakdown

Claude Code manages a 200k token context window as follows:

1. **System Prompt** (~2.7k tokens, 1.4%)
   - Claude's core identity and instructions
   - Tool definitions and capabilities
   - Operating guidelines

2. **System Tools** (~8 tokens, 0.0%)
   - Tool metadata and availability info

3. **Memory Files** (~242 tokens, 0.1%)
   - CLAUDE.md and other auto-loaded files
   - Project-specific context

4. **Messages** (~50k tokens, 25.0%)
   - User messages
   - Claude's responses
   - Tool call results
   - Conversation history

5. **Free Space** (~70k tokens, 35.0%)
   - Available for new messages
   - Room for file reads, tool outputs
   - Working memory

6. **Autocompact Buffer** (~77k tokens, 38.5%)
   - Reserved for context management
   - Used when free space runs low
   - Enables automatic compaction

### When Autocompact Triggers

**Threshold**: When free space drops below a certain threshold (typically when approaching full capacity)

**What Happens**:
1. Claude Code detects context window filling up
2. Autocompact algorithm activates
3. Older, less relevant messages are summarized or removed
4. Important context is preserved (files, key decisions, recent messages)
5. Space is freed up for new operations
6. Conversation continues seamlessly

**User Experience**:
- Transparent to user (happens automatically)
- No manual intervention needed
- Critical context preserved
- Conversation flow maintained

## Why Autocompact Buffer Exists

### Problem Without Autocompact

Without a reserved buffer:
- Context window fills completely
- No space for compaction operations
- Agent must stop and ask user to clear context
- Lost conversation flow
- Manual intervention required

### Solution With Autocompact

With reserved buffer:
- Space always available for compaction
- Automatic pruning of old messages
- Seamless conversation continuation
- No user interruption
- Intelligent context preservation

## Compaction Strategy

Claude Code uses intelligent compaction:

### What Gets Kept (High Priority)
- Recent messages (last ~10-20 exchanges)
- System prompt and tools (always)
- Memory files (always)
- Important decisions and clarifications
- Key code snippets and file contents
- Current task context

### What Gets Compacted (Lower Priority)
- Very old messages (>50-100 exchanges ago)
- Repetitive tool outputs
- Large file reads that are no longer relevant
- Intermediate debugging steps
- Verbose explanations from early conversation

### What Gets Removed (Lowest Priority)
- Duplicate information
- Superseded content
- Exploratory dead-ends
- Resolved issues
- Verbose tool outputs no longer needed

## Current Session Status

**Token Usage**:
- Total context window: 200k tokens
- Currently used: ~53k tokens (26.5%)
- Free space: ~70k tokens (35.0%)
- Autocompact buffer: ~77k tokens (38.5%)

**Health**: Excellent
- Plenty of free space available
- No compaction needed yet
- Buffer remains unused
- Conversation can continue for many more exchanges

**Estimated Remaining Capacity**:
- At current rate: ~50-100 more exchanges before compaction
- With large file reads: ~20-40 more file operations
- With agent spawns: ~10-20 more sub-agent conversations

## Best Practices for Users

### To Maximize Context Window Efficiency

1. **Use Selective File Reading**
   - Read only necessary files
   - Use offset/limit for large files
   - Don't repeatedly read same file

2. **Clear Instructions**
   - Be concise in requests
   - Avoid repetitive questions
   - Ask follow-ups in same message when possible

3. **Trust Autocompact**
   - Don't worry about context limits
   - System handles it automatically
   - Focus on work, not token management

4. **Use Agents Wisely**
   - Spawn sub-agents for isolated tasks
   - Sub-agents have their own context windows
   - Reduces main context usage

### What NOT to Do

❌ **Avoid**:
- Repeatedly reading massive files (>10k lines)
- Asking for full codebase dumps
- Requesting verbose explanations of everything
- Continuous debugging without focus
- Opening 100s of files without purpose

✅ **Instead**:
- Use Grep/Glob to find specific content
- Read targeted sections of large files
- Request summaries when appropriate
- Use agents for deep investigations
- Be strategic about file access

## Technical Details

### Algorithm

Claude Code uses a **sophisticated compaction algorithm** that:
- Analyzes semantic importance of messages
- Preserves causal chains (question → answer → action)
- Maintains project context and decisions
- Keeps recent conversation intact
- Summarizes or removes less relevant history

### Transparency

**Compaction is usually invisible**, but Claude Code may:
- Mention when compaction occurs (rare)
- Summarize what was compacted
- Ask user to restate important lost context (very rare)

### Recovery

If critical context is lost:
- User can re-provide information
- Files can be re-read
- Commands can be re-run
- Sub-agents can be re-spawned with context

## Comparison to Other Systems

### Traditional Chat (No Autocompact)
- Fixed context window
- Hard limit reached → conversation stops
- Manual "clear history" required
- Lost context → start over

### Claude Code (With Autocompact)
- Fixed context window
- Soft limit → automatic compaction
- No manual intervention
- Lost context → automatically recoverable

### Unlimited Context (Theoretical)
- No practical implementation
- Performance degrades with size
- Cost increases linearly
- Searching becomes difficult

**Claude Code's approach balances**:
- Practicality (finite context)
- Usability (automatic management)
- Performance (optimized compaction)
- Cost (efficient token usage)

## Monitoring Context Usage

### Via /context Command

Users can check context at any time:
```bash
/context
```

**Shows**:
- Current token usage
- Free space available
- Autocompact buffer size
- Usage breakdown by category

### Warning Signs (Rare)

If context is nearly full:
- Free space < 10k tokens
- Autocompact buffer partially used
- Claude Code may suggest starting new session
- Or finishing current task before continuing

### Optimal Range

**Healthy**: 20-60% usage (40-120k tokens)
- Plenty of room to work
- No compaction needed
- Good performance

**Caution**: 60-80% usage (120-160k tokens)
- Compaction may trigger soon
- Still functional
- Consider wrapping up complex tasks

**Critical**: >80% usage (>160k tokens)
- Compaction actively happening
- May need new session soon
- Finish current work quickly

## Summary

**Autocompact Buffer**:
- **Size**: 77k tokens (38.5% of context window)
- **Purpose**: Reserved space for automatic context management
- **Function**: Enables seamless conversation continuation without user intervention
- **Strategy**: Intelligent pruning of old/irrelevant content while preserving important context
- **User Impact**: Transparent (user doesn't need to think about it)
- **Current Status**: Unused (plenty of free space available)

**Key Takeaway**: The autocompact buffer is Claude Code's "safety net" that ensures you can continue working without hitting hard context limits. It's always there, working in the background, so you can focus on your tasks instead of token management.

---
*Generated: 2026-01-21*
*Context Window: 200k tokens (claude-sonnet-4.5)*
*Autocompact Buffer: 77k tokens (38.5%)*
*Current Usage: 53k tokens (26.5%)*
