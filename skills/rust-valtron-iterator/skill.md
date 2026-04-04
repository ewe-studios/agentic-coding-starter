---
name: "Rust Valtron Iterator Creation"
description: "Guide for implementing TaskIterator and StreamIterator in Rust following Valtron's progress-driven execution model"
approved: No
created: 2026-04-04
license: "MIT"
metadata:
  author: "Main Agent"
  version: "1.0"
  last_updated: "2026-04-04"
  tags: [rust, iterator, valtron, taskiterator, streamiterator, progress-driven]
tools: []
files:
  - backends/foundation_core/src/valtron/stream_iterators.rs: "Reference implementations for StreamIterator combinators"
  - backends/foundation_core/src/valtron/task_iterators.rs: "Reference implementations for TaskIterator combinators"
---

# Rust Valtron Iterator Creation

## Overview

This skill documents the correct patterns for implementing `TaskIterator` and `StreamIterator` in Rust within the Valtron framework. The central principle is that `next()` must return immediately after processing ONE item - never loop to consume multiple items before returning.

Valtron's execution model is **progress-driven**: the executor polls iterators frequently to observe intermediate states (Pending, Delayed, Init) and make scheduling decisions. Blocking loops in `next()` defeat this model by consuming items invisibly before the executor can observe progress.

## When to Use

Use these patterns when:
- Implementing custom `TaskIterator` combinators in `backends/foundation_core/src/valtron/task_iterators.rs`
- Implementing custom `StreamIterator` combinators in `backends/foundation_core/src/valtron/stream_iterators.rs`
- Debugging iterator implementations that block or skip progress reporting

## Prerequisites

- Understanding of Rust's `Iterator` trait
- Familiarity with Valtron's `TaskStatus` and `Stream` enums
- Knowledge of the executor model in `backends/foundation_core/src/valtron/executors/`

## Usage Type

**EDUCATIONAL** - Study these patterns and apply them when implementing new iterator combinators. Reference the fixed implementations in the `files:` field for working examples.

## Core Principle

### The Single-Item Return Rule

**`next()` must return immediately after processing ONE item.**

```rust
// CORRECT: Process one item, return immediately
fn next(&mut self) -> Option<Self::Item> {
    let item = self.inner.next()?;
    // Process this single item
    match item {
        Stream::Next(v) => {
            // Transform or filter this ONE value
            Some(Stream::Next(transform(v)))
        }
        _ => Some(item), // Pass through immediately
    }
}
```

```rust
// INCORRECT: Loop consumes multiple items invisibly
fn next(&mut self) -> Option<Self::Item> {
    loop {
        let item = self.inner.next()?;
        match item {
            Stream::Next(v) if should_skip(v) => continue, // BLOCKS!
            _ => return Some(item),
        }
    }
}
```

## Why This Matters

Blocking loops in `next()` defeat Valtron's progress-driven execution model:

| Problem | Consequence |
|---------|-------------|
| **Blocks executor thread** | Executor cannot check timeouts, schedule other tasks, or respond to cancellation |
| **Defeats incremental progress** | No `Pending`/`Delayed` states reported during the loop - all progress invisible |
| **Prevents observability** | Debugging, logging, and metrics cannot see intermediate states |
| **Starves other tasks** | Long-running loops hold the thread, delaying other ready tasks |

## Acceptable Loop Pattern

The **ONLY** acceptable loop is a **single bounded pass** through sources that returns immediately on any meaningful result:

```rust
// ACCEPTABLE: Single bounded pass, returns immediately on result
fn next(&mut self) -> Option<Self::Item> {
    let start_index = self.current_index;
    
    loop {
        // Bounded by number of sources - will complete this pass
        let source = &mut self.sources[self.current_index];
        match source.next() {
            Some(Stream::Next(v)) => {
                // Found meaningful result - return IMMEDIATELY
                self.current_index = (self.current_index + 1) % self.sources.len();
                return Some(Stream::Next(v));
            }
            Some(Stream::Pending(p)) => {
                // Report progress immediately
                self.current_index = (self.current_index + 1) % self.sources.len();
                return Some(Stream::Pending(p));
            }
            None => {
                // Source exhausted, continue to next in SAME pass
                self.current_index = (self.current_index + 1) % self.sources.len();
                if self.current_index == start_index {
                    return None; // Completed full pass, nothing found
                }
            }
            _ => {
                // Ignore/Delayed/Init - continue to next source in same pass
                self.current_index = (self.current_index + 1) % self.sources.len();
            }
        }
    }
}
```

**Key characteristics of acceptable loops:**
1. **Bounded iteration** - Loop runs at most N times (where N = number of sources)
2. **Immediate return on progress** - Returns `Next`, `Pending`, `Delayed`, `Init` immediately when encountered
3. **No invisible consumption** - Only `Ignore` or `None` results trigger continuation
4. **Progress reported** - Intermediate states escape the loop

## Code Examples by Pattern

### 1. Filtering (return `Ignore` vs loop to find match)

**BAD - Blocking Loop:**
```rust
fn next(&mut self) -> Option<Self::Item> {
    loop {
        let item = self.inner.next()?;
        match item {
            Stream::Next(v) if self.predicate(v) => return Some(Stream::Next(v)),
            Stream::Next(_) => continue, // BLOCKS: consumes invisibly
            _ => return Some(item),
        }
    }
}
```

**GOOD - Return Ignore:**
```rust
fn next(&mut self) -> Option<Self::Item> {
    let item = self.inner.next()?;
    match item {
        Stream::Next(v) if self.predicate(v) => Some(Stream::Next(v)),
        Stream::Next(_) => Some(Stream::Ignore), // Yield control back
        _ => Some(item),
    }
}
```

**Reference:** `TFilterReady` in `task_iterators.rs`, `SFilterState` in `stream_iterators.rs`

---

### 2. Accumulating/Fold (yield `Ignore` while accumulating vs loop to collect all)

**BAD - Blocking Loop:**
```rust
fn next(&mut self) -> Option<Self::Item> {
    // BLOCKS: consumes ALL items before returning
    for item in &mut self.inner {
        if let Stream::Next(v) = item {
            self.acc = self.folder(self.acc.take().unwrap_or_default(), v);
        }
    }
    Some(Stream::Next(self.acc.take()))
}
```

**GOOD - Yield Ignore while accumulating:**
```rust
fn next(&mut self) -> Option<Self::Item> {
    if self.done {
        return None;
    }

    match self.inner.next() {
        Some(Stream::Next(v)) => {
            // Accumulate but return Ignore to signal "still working"
            if let Some(acc) = self.acc.take() {
                self.acc = Some(self.folder(acc, v));
            }
            Some(Stream::Ignore)
        }
        Some(Stream::Pending(p)) => Some(Stream::Pending(p)), // Pass through immediately
        Some(Stream::Delayed(d)) => Some(Stream::Delayed(d)),
        Some(Stream::Init) => Some(Stream::Init),
        None => {
            // Inner exhausted - yield final result
            self.done = true;
            self.acc.take().map(Stream::Next)
        }
    }
}
```

**Reference:** `SFold` (lines 2638-2678) in `stream_iterators.rs`, `TFold` (lines 2406-2445) in `task_iterators.rs`

---

### 3. Finding (track `found` state vs loop until found)

**BAD - Blocking Loop:**
```rust
fn next(&mut self) -> Option<Self::Item> {
    loop {
        let item = self.inner.next()?;
        if let Stream::Next(v) = item {
            if self.predicate(v) {
                return Some(Stream::Next(Some(v))); // Found!
            }
            // continue loops invisibly
        }
    }
}
```

**GOOD - Track found state:**
```rust
fn next(&mut self) -> Option<Self::Item> {
    if self.found {
        return None; // Already found, stop iteration
    }

    match self.inner.next()? {
        Stream::Next(d) => {
            if self.predicate(&d) {
                self.found = true;
                Some(Stream::Next(Some(d)))
            } else {
                Some(Stream::Ignore) // Yield control, executor will poll again
            }
        }
        Stream::Pending(p) => Some(Stream::Pending(p)), // Report progress
        Stream::Delayed(d) => Some(Stream::Delayed(d)),
        Stream::Init => Some(Stream::Init),
        Stream::Ignore => Some(Stream::Ignore),
    }
}
```

**Reference:** `SFind` (lines 2560-2592) in `stream_iterators.rs`, `TFind` (lines 2328-2362) in `task_iterators.rs`

---

### 4. Skip While / Take While (track state vs loop)

**BAD - Blocking Loop:**
```rust
fn next(&mut self) -> Option<Self::Item> {
    loop {
        let item = self.inner.next()?;
        if self.should_skip(&item) {
            continue; // BLOCKS: skips invisibly
        }
        return Some(item);
    }
}
```

**GOOD - Track skipping state:**
```rust
fn next(&mut self) -> Option<Self::Item> {
    let item = self.inner.next()?;

    if !self.done_skipping && self.predicate(&item) {
        // Still skipping, but yield Ignore (not blocking loop)
        return Some(Stream::Ignore);
    }

    // Done skipping - mark state and return item
    self.done_skipping = true;
    Some(item)
}
```

**Reference:** `SSkipWhileState` (lines 2456-2483) in `stream_iterators.rs`, `TSkipWhileState` (lines 2225-2252) in `task_iterators.rs`

---

### 5. Skip/Take with Count (decrement counter vs loop)

**BAD - Blocking Loop:**
```rust
fn next(&mut self) -> Option<Self::Item> {
    while self.to_skip > 0 {
        self.inner.next()?; // BLOCKS: consumes invisibly
        self.to_skip -= 1;
    }
    self.inner.next()
}
```

**GOOD - Decrement counter per call:**
```rust
fn next(&mut self) -> Option<Self::Item> {
    let item = self.inner.next()?;

    if self.to_skip > 0 && self.predicate(&item) {
        self.to_skip -= 1;
        return Some(Stream::Ignore); // Yield control
    }

    Some(item)
}
```

**Reference:** `SSkipState` (lines 2513-2540) in `stream_iterators.rs`, `TSkipState` (lines 2275-2302) in `task_iterators.rs`

---

### 6. All/Any (track result state, yield final at end vs loop to check all)

**BAD - Blocking Loop for All:**
```rust
fn next(&mut self) -> Option<Self::Item> {
    // BLOCKS: checks ALL items before returning
    for item in &mut self.inner {
        if let Stream::Next(v) = item {
            if !self.predicate(v) {
                return Some(Stream::Next(false));
            }
        }
    }
    Some(Stream::Next(true))
}
```

**GOOD - Track state, yield Ignore during check:**
```rust
// ALL - check if all items satisfy predicate
fn next(&mut self) -> Option<Self::Item> {
    if self.done {
        return None;
    }

    // Short-circuit: already found a failure
    if !self.all_true {
        return None;
    }

    match self.inner.next() {
        Some(Stream::Next(v)) => {
            if !self.predicate(v) {
                // Found failure - return immediately
                self.all_true = false;
                self.done = true;
                return Some(Stream::Next(false));
            }
            // Still good, return Ignore (still accumulating)
            Some(Stream::Ignore)
        }
        Some(Stream::Pending(p)) => Some(Stream::Pending(p)),
        Some(Stream::Delayed(d)) => Some(Stream::Delayed(d)),
        Some(Stream::Init) => Some(Stream::Init),
        None => {
            // Exhausted without failure - all true
            self.done = true;
            Some(Stream::Next(true))
        }
    }
}
```

**GOOD - Any (short-circuit on first true):**
```rust
fn next(&mut self) -> Option<Self::Item> {
    if self.done {
        return None;
    }

    // Short-circuit: already found a match
    if self.any_true {
        return None;
    }

    match self.inner.next() {
        Some(Stream::Next(v)) => {
            if self.predicate(v) {
                // Found match - return immediately
                self.any_true = true;
                self.done = true;
                Some(Stream::Next(true))
            } else {
                Some(Stream::Ignore)
            }
        }
        Some(Stream::Pending(p)) => Some(Stream::Pending(p)),
        Some(Stream::Delayed(d)) => Some(Stream::Delayed(d)),
        None => {
            // Exhausted without match
            self.done = true;
            Some(Stream::Next(false))
        }
    }
}
```

**Reference:** `SAll` (lines 2688-2730), `SAny` (lines 2734-2776) in `stream_iterators.rs`; `TAll` (lines 2455-2498), `TAny` (lines 2502-2545) in `task_iterators.rs`

---

### 7. FlatMap (the acceptable nested iterator exception)

FlatMap **drains inner iterator before polling outer** - this is the acceptable nested iterator pattern.

```rust
fn next(&mut self) -> Option<Self::Item> {
    // First drain current inner iterator
    if let Some(ref mut inner) = self.current_inner {
        if let Some(item) = inner.next() {
            return Some(item); // Return inner items one at a time
        }
        // Inner exhausted
        self.current_inner = None;
    }

    // Poll outer for next item
    match self.outer.next() {
        Some(item) => match item {
            Stream::Next(inner) => {
                // Create new inner iterator
                self.current_inner = Some(self.mapper(inner));
                // Return Ignore to signal "setting up next inner"
                Some(Stream::Ignore)
            }
            Stream::Pending(p) => Some(Stream::Pending(p)),
            Stream::Delayed(d) => Some(Stream::Delayed(d)),
            Stream::Init => Some(Stream::Init),
            Stream::Ignore => Some(Stream::Ignore),
        },
        None => None,
    }
}
```

**Why this is acceptable:**
- Inner iterator is itself a `StreamIterator` that reports `Pending`/`Delayed`
- Each `inner.next()` call returns immediately (doesn't block loop)
- Outer progress (`Pending`/`Delayed`) passes through immediately
- The nesting is explicit in the type system

**Reference:** `MapIter` (lines 2117-2156), `MapIterDone` (lines 2064-2109), `MapIterPending` (lines 2013-2056) in `stream_iterators.rs`

---

### 8. Multi-Source Round-Robin (single pass, sort indices descending before swap_remove)

```rust
fn next(&mut self) -> Option<Self::Item> {
    if self.done {
        return None;
    }

    let start_index = self.current_index;
    let mut exhausted_indices = Vec::new();

    // Single bounded pass through all sources
    loop {
        let idx = self.current_index;
        let source = &mut self.sources[idx];

        match source.next() {
            Some(Stream::Next(v)) => {
                // Found value - return immediately
                self.collected.push(v);
                self.current_index = (self.current_index + 1) % self.sources.len();
                return Some(Stream::Pending(self.collected.len()));
            }
            Some(Stream::Pending(p)) => {
                // Report progress immediately
                self.current_index = (self.current_index + 1) % self.sources.len();
                return Some(Stream::Pending(p));
            }
            Some(Stream::Delayed(d)) => {
                self.current_index = (self.current_index + 1) % self.sources.len();
                return Some(Stream::Delayed(d));
            }
            Some(Stream::Init) => {
                self.current_index = (self.current_index + 1) % self.sources.len();
                return Some(Stream::Init);
            }
            Some(Stream::Ignore) => {
                // Continue to next source in same pass
                self.current_index = (self.current_index + 1) % self.sources.len();
            }
            None => {
                // Source exhausted - mark for removal AFTER pass
                exhausted_indices.push(idx);
                self.current_index = (self.current_index + 1) % self.sources.len();
            }
        }

        // Check if we completed a full pass
        if self.current_index == start_index {
            break;
        }
    }

    // Remove exhausted sources AFTER the pass
    // CRITICAL: Sort descending so swap_remove doesn't invalidate indices
    exhausted_indices.sort_by(|a, b| b.cmp(a));
    for idx in exhausted_indices {
        self.sources.swap_remove(idx);
    }

    // Adjust current_index if it now points beyond new length
    if !self.sources.is_empty() && self.current_index >= self.sources.len() {
        self.current_index = 0;
    }

    if self.sources.is_empty() {
        self.done = true;
        if self.collected.is_empty() {
            return None;
        }
        return Some(Stream::Next(std::mem::take(&mut self.collected)));
    }

    Some(Stream::Pending(self.collected.len()))
}
```

**Key patterns:**
1. **Single bounded pass** - Loop runs at most `sources.len()` times
2. **Immediate return on progress** - `Next`, `Pending`, `Delayed`, `Init` all escape immediately
3. **Deferred removal** - Exhausted sources removed AFTER pass completes
4. **Sort descending before swap_remove** - Prevents index invalidation
5. **Adjust current_index after removal** - Handles index shift from swap_remove

**Reference:** `CollectNextFromStreams` in `stream_iterators.rs`

---

## Checklist for Iterator Implementation

Before submitting an iterator implementation, verify:

- [ ] **No blocking `loop {}`** - `next()` returns after processing one item
- [ ] **State tracking** - Uses struct fields (`found`, `done`, `acc`) to track progress across calls
- [ ] **Immediate progress reporting** - `Pending`, `Delayed`, `Init` pass through without delay
- [ ] **Proper termination** - Returns `None` when exhausted, sets `done` flag when appropriate
- [ ] **Ignore for intermediate** - Returns `Stream::Ignore` (or `TaskStatus::Ignore`) while working, not `continue`
- [ ] **Short-circuit logic** - For `find`/`all`/`any`, stops polling after result determined
- [ ] **Multi-source safety** - If removing sources, sort indices descending before `swap_remove`

## Summary Table: Do's and Don'ts

| Pattern | DO | DON'T |
|---------|-----|-------|
| **Filtering** | Return `Ignore` for filtered items | `loop { continue }` to find match |
| **Accumulating** | Yield `Ignore` while accumulating, final result at end | Loop to collect all before returning |
| **Finding** | Track `found: bool`, return `Ignore` while searching | Loop until found |
| **Skip/Take While** | Track `done_skipping: bool` state | `while` loop to skip |
| **Skip/Take Count** | Decrement counter per `next()` call | `while to_skip > 0 { next() }` |
| **All/Any** | Track result state, yield `Ignore` during check | Loop to check all items |
| **FlatMap** | Drain inner iterator, poll outer only when inner exhausted | N/A (nested iterator is acceptable) |
| **Multi-Source** | Single bounded pass, return immediately on progress | Nested loops consuming sources |
| **Source Removal** | Sort indices descending, then `swap_remove` | `swap_remove` in ascending order |
| **State Management** | Use struct fields to track across calls | Local variables that reset each call |

## References

### Fixed Implementations (StreamIterator)
- `SAll` - lines 2688-2730
- `SAny` - lines 2734-2776
- `SFold` - lines 2638-2678
- `SCountAll` - lines 2809-2845
- `SFind` / `SFindMap` - lines 2560-2630
- `SSkipState` / `SSkipWhileState` - lines 2456-2540
- `CollectNextFromStreams` - multi-source round-robin
- `MapIter` / `MapIterDone` / `MapIterPending` - lines 2013-2156 (flatmap pattern)

### Fixed Implementations (TaskIterator)
- `TAll` - lines 2455-2498
- `TAny` - lines 2502-2545
- `TFold` - lines 2406-2445
- `TCount` / `TCountAll` - lines 2549-2610
- `TFind` / `TFindMap` - lines 2328-2402
- `TSkipState` / `TSkipWhileState` - lines 2225-2302

---

_Version: 1.0 - Created: 2026-04-04_
