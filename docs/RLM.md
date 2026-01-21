# RLM.md - Recursive Language Models for AI Agents

## Core Principle

**Context rot** degrades LLM performance as input length increases. RLM solves this by treating context as a **variable in an external environment**, not neural network input.

```
Traditional: LLM receives 100K tokens → attention dilutes → accuracy drops
RLM: LLM receives query only → context accessed programmatically → accuracy maintained
```

**The shift:** Context management becomes **programmatic, not neural**.

---

## When to Activate RLM

Use this decision tree at the start of every task:

```
Context > 50K tokens? ─────────────────────── YES → ACTIVATE RLM
    │
    NO
    │
Context > 16K AND task complexity > O(1)? ── YES → CONSIDER RLM
    │
    NO
    │
Files to process > 5? ─────────────────────── YES → ACTIVATE RLM
    │
    NO
    │
Multi-hop or aggregation reasoning needed? ── YES → ACTIVATE RLM
    │
    NO
    │
PROCEED DIRECTLY (no RLM needed)
```

### Task Complexity Guide

| Task Type | Complexity | RLM Benefit |
|-----------|------------|-------------|
| Find one phrase | O(1) | Low - base LLM handles well |
| Classify all entries | O(n) | High - prevents context rot |
| Pair-wise reasoning | O(n²) | Critical - base LLM fails completely |

---

## The Six-Step Protocol

Execute these steps **in order** when RLM is activated:

### 1. ASSESS - Peek at Context

```python
# Don't read everything. Sample first:
print(f"Total size: {len(context)} chars")
print(context[:2000])  # Preview structure
```

**Output:** Understand context type (JSON, text, code) and natural boundaries.

### 2. DECIDE - Select Strategy

| Context Type | Size | Strategy |
|--------------|------|----------|
| Any | < 4K | Direct processing |
| Text | 4K-50K | Grep to narrow, then process |
| Text | > 50K | Partition + Map |
| Multi-document | > 5 docs | Parallel sub-agents |
| Code | Multiple files | Partition by file/concern |

### 3. DECOMPOSE - Partition the Work

Split at **natural boundaries**, not arbitrary byte offsets:
- By file (one sub-agent per file)
- By section (split at headers)
- By record (batch N records per chunk)
- By concern (auth vs UI vs DB)

**Target chunk size:** 4K-10K tokens per sub-task.

### 4. EXECUTE - Run Sub-Agents

**Parallel** if sub-tasks are independent:
```bash
# Launch all sub-agents simultaneously
agent1 "Process chunk 1" > /tmp/r1.txt &
agent2 "Process chunk 2" > /tmp/r2.txt &
agent3 "Process chunk 3" > /tmp/r3.txt &
wait
```

**Sequential** if results depend on each other.

### 5. SYNTHESIZE - Combine Results

| Pattern | When to Use |
|---------|-------------|
| Concatenate | Order matters, no conflicts |
| Merge | Deduplication needed |
| Reduce | Counting, statistics |
| Vote | Classification (majority wins) |

### 6. VERIFY - Check Completeness

- [ ] Original query fully addressed?
- [ ] All partitions contributed results?
- [ ] No obvious gaps in coverage?
- [ ] Spot-check 2-3 claims against source?

---

## Anti-Patterns (Critical DON'Ts)

### ❌ Reading Everything First

**Wrong:** Load all 50 files into context, then think.

**Right:** Peek → Grep → Read only relevant sections → Expand if needed.

### ❌ Sub-Agent Explosion

**Wrong:** Spawn 100 sub-agents for 100 files.

**Right:** Group logically, target 3-10 sub-agents per level, max depth 2.

### ❌ Context Starvation

**Wrong:** Give sub-agent just the function body.

**Right:** Include imports, types, usage context, and relationship to other components.

### ❌ Blind Partitioning

**Wrong:** Split at arbitrary byte boundaries.

**Right:** Split at semantic boundaries (files, sections, records).

### ❌ Ignoring Grep

**Wrong:** Read every file to find error handling code.

**Right:** `grep -r "try\|catch\|except\|error"` → Read only matched sections.

---

## Orchestration Scripts

**Create a bash or Python script** to orchestrate sub-agents. This provides:
- Reproducible execution
- Timeout protection
- Parallel execution control
- Result aggregation

### Bash Template

```bash
#!/bin/bash
# rlm-orchestrate.sh - Parallel sub-agent execution

QUERY="$1"
RESULT_DIR=$(mktemp -d)
TIMEOUT_SEC=300

# PARTITION: Split work (customize this)
CHUNKS=("chunk1.txt" "chunk2.txt" "chunk3.txt")

# MAP: Process chunks in parallel
for i in "${!CHUNKS[@]}"; do
    timeout $TIMEOUT_SEC your-agent-cli "${CHUNKS[$i]}" "$QUERY" \
        > "$RESULT_DIR/result_$i.txt" 2>&1 &
done
wait

# REDUCE: Aggregate results
cat "$RESULT_DIR"/result_*.txt > combined.txt
your-agent-cli "Synthesize these results for: $QUERY" < combined.txt

rm -rf "$RESULT_DIR"
```

### Python Template

```python
#!/usr/bin/env python3
"""RLM orchestration with parallel sub-agents."""

import subprocess
import concurrent.futures
from pathlib import Path

def process_chunk(chunk: str, query: str) -> str:
    """Process a single chunk with timeout."""
    result = subprocess.run(
        ["your-agent-cli", chunk, query],
        capture_output=True, text=True, timeout=300
    )
    return result.stdout

def orchestrate(chunks: list[str], query: str) -> str:
    # MAP: Parallel execution
    with concurrent.futures.ThreadPoolExecutor(max_workers=5) as executor:
        futures = {executor.submit(process_chunk, c, query): c for c in chunks}
        results = [f.result() for f in concurrent.futures.as_completed(futures)]

    # REDUCE: Aggregate
    combined = "\n---\n".join(results)
    return process_chunk(combined, f"Synthesize for: {query}")

if __name__ == "__main__":
    chunks = ["file1.txt", "file2.txt", "file3.txt"]
    print(orchestrate(chunks, "Your query here"))
```

---

## Agent-Specific Implementation

Use these guides for spawning sub-agents with specific CLI tools:

| Tool | Guide | Best For |
|------|-------|----------|
| Claude Code | [CLAUDE-CODE.md](CLAUDE-CODE.md) | Full tool access, web search, complex tasks |
| Codex | [CODEX.md](CODEX.md) | Image/PDF analysis, focused code review |
| Gemini | [GEMINI.md](GEMINI.md) | Cross-validation, alternative perspective |

### Quick Spawn Examples

**Give sub-agents full tool access** - don't restrict capabilities.

```bash
# Claude Code (full tool access)
echo "Analyze auth module" | claude -p > result.txt 2>&1 &

# Codex (full access, with image input)
codex exec -i screenshot.png "Describe this UI" > result.txt 2>&1 &

# Gemini (full access)
gemini "Review this analysis for accuracy" > result.txt 2>&1 &

# Wait for all
wait
```

---

## Quick Reference Card

```
┌─────────────────────────────────────────────────────────────┐
│                    RLM QUICK REFERENCE                      │
├─────────────────────────────────────────────────────────────┤
│ TRIGGERS:                                                   │
│   • Context > 50K tokens                                    │
│   • Files > 5                                               │
│   • Multi-hop reasoning                                     │
│   • Cross-document aggregation                              │
├─────────────────────────────────────────────────────────────┤
│ PROTOCOL:                                                   │
│   1. ASSESS   → Peek, don't read everything                 │
│   2. DECIDE   → Match context + task to strategy            │
│   3. DECOMPOSE → Partition at natural boundaries            │
│   4. EXECUTE  → Parallel if independent                     │
│   5. SYNTHESIZE → Merge, dedupe, resolve conflicts          │
│   6. VERIFY   → Check completeness and accuracy             │
├─────────────────────────────────────────────────────────────┤
│ KEY QUESTIONS:                                              │
│   • Can I grep to narrow scope first?                       │
│   • Are sub-tasks independent (parallel) or dependent?      │
│   • Does each sub-agent have enough context?                │
├─────────────────────────────────────────────────────────────┤
│ AVOID:                                                      │
│   ✗ Reading everything before thinking                      │
│   ✗ Spawning > 10 sub-agents per level                      │
│   ✗ Providing insufficient context to sub-agents            │
│   ✗ Splitting at arbitrary byte boundaries                  │
└─────────────────────────────────────────────────────────────┘
```

---

## Detailed Reference

For comprehensive documentation including:
- Formal architecture and system design
- Benchmark results and cost analysis
- Detailed prompt templates for sub-agents
- Error handling and recovery protocols
- Complete bash script implementations
- Strategy decision flowcharts

See **[RLM-extra.md](RLM-extra.md)**.

---

## Sources

- [Paper (arXiv)](https://arxiv.org/abs/2512.24601) - Zhang, Kraska, Khattab (MIT CSAIL)
- [Blog Post](https://alexzhang13.github.io/blog/2025/rlm/)
- [GitHub Repository](https://github.com/alexzhang13/rlm)

---

*Follow [@jonnyzzz](https://twitter.com/jonnyzzz) on X and [LinkedIn](https://www.linkedin.com/in/jonnyzzz/) for more on AI agents and developer tooling.*
