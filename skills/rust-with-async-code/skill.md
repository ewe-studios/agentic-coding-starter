---
name: "Rust with Async Code"
description: "Writing robust futures and logic in Rust using tokio as the primary runtime, understanding when to use async vs sync code patterns."
approved: No
created: 2026-01-27
license: "MIT"
metadata:
  author: "Main Agent"
  version: "1.0"
  last_updated: "2026-01-27"
  tags:
    - rust
    - tokio
    - async-await
    - futures
tools:
  - Rust
  - Cargo (tokio, async-std)
files:
  - examples/async-best-practices.rs: "Examples of proper async patterns and anti-patterns"
assets:
---
# Async Code in Rust

## Overview

This skill guides the implementation of robust asynchronous code using tokio as the primary runtime. It covers when to use async vs synchronous paths, how futures work internally, and common pitfalls like blocking operations that should be offloaded.

The key principle: **async for I/O-bound operations**, **sync for CPU-intensive computation** (spawn_blocking or parallelize).

## When to Use This Skill

- Implementing asynchronous APIs with tokio runtime
- Writing handlers in async web frameworks
- Designing systems with concurrent task management
- Deciding between synchronous and asynchronous code paths based on workload type

## Prerequisites

- Understanding of Rust ownership, borrowing, lifetimes (from `rust.md`)
- Basic knowledge of futures trait (`Future`, `poll` conceptually)
- Familiarity with tokio runtime concepts: single-threaded vs multi-threaded executors
- Mandatory reading of [`rust.md`](../stacks/rust.md) for async conventions

## Core Concepts

### The Async/Sync Decision Matrix

| Operation Type | Synchronous Approach | Asynchronous (Tokio) |
|----------------|----------------------|---------------------|
| Blocking I/O operations | `std::fs`, blocking sockets | `tokio::net` APIs, non-blocking IO |
| CPU-intensive work | Direct execution in sync context | Spawn via `spawn_blocking` or parallelize with rayon/loom |
| Waiting for timers/events | Poll loops, busy-waiting | Tokio timers: `sleep()`, interval(), select! macro |

### Why tokio is the First Choice

- **Mature ecosystem**: Most async Rust crates are built against tokio
- **Non-blocking by default**: Designed to avoid blocking event loop threads
- **Tooling support**: Strong debugging and observability tools (tokio-console, tracing)
- **Flexibility**: Single-threaded executor for tests (`flavor = "current_thread"`), multi-threaded for production

## Step-by-Step Guide

### Step 1: Add tokio Dependency Correctly

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }

# For CPU-bound work in async contexts (alternative to spawn_blocking)
rayon = "1.8"

# Async runtime alternative for specific needs
async-std = { version = "1.12", optional = true }
```

### Step 2: Use Single-Threaded Executor in Tests

```rust
// MANDATORY pattern for async tests - prevents test isolation issues with global state
#[tokio::test(flavor = "current_thread")]
async fn my_async_test() {
    // Time doesn't advance automatically (use start_paused if needed)
}

// With time-dependent behavior:
#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn test_with_manual_time_advance() {
    let before_sleep = tokio::time::Instant::now();
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Manually advance time if needed for deterministic tests
}
```

### Step 3: Offload Blocking Work to `spawn_blocking`

```rust
// BAD - Blocks the event loop thread!
async fn process_data_sync(data: &[u8]) -> Vec<u8> {
    std::process::data_processing(data) // CPU-intensive, blocks everything else!

#[tokio::test]
async fn test_with_spawn_blocking() {
    let result = tokio::task::spawn_blocking(|| {
        expensive_computation()
    })
    .await
    .expect("Task failed");

    assert!(result == expected);
}
```

### Step 4: Use Non-Blocking I/O APIs

```rust
use tokio::net::{TcpListener, TcpStream};

async fn handle_client(mut stream: TcpStream) -> Result<()> {
    // Good - non-blocking connect with timeout
    let addr = "127.0.0.1:8080";
    match tokio::time::timeout(Duration::from_secs(5), stream.connect(addr)).await {
        Ok(stream) => { /* successful connection */ }
        Err(_) => return Err(Error::ConnectionTimeout),
    }

    // Good - read with timeout
    let mut buf = vec![0u8; 1024];
    match tokio::time::timeout(Duration::from_secs(30), stream.read(&mut buf)).await {
        Ok(n) if *n > 0 => { /* successful read */ }
        _ => return Err(Error::ReadTimeout),
    }

    // Good - write with timeout
    let data = b"response";
    tokio::time::timeout(Duration::from_secs(10), stream.write_all(data)).await?;
}
```

### Step 5: Use Channels for Message Passing

```rust
use tokio::sync::{mpsc, broadcast, oneshot};

// Unbounded channel for producer-consumer patterns
let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

tokio::spawn(async move {
    while let Some(msg) = rx.recv().await {  // Blocks only when empty!
        process_message(msg).await;
    }
});

// Bounded channel with backpressure
let (tx, mut rx) = mpsc::channel(100);  // Max 100 messages in flight

tokio::spawn(async move {
    while let Some(msg) = rx.recv().await {  // Blocks when full!
        process_message(msg).await;
    }
});
```

## Common Patterns

### Pattern 1: Task Spawning with Join Handles

```rust
async fn parallel_processing(items: Vec<Item>) -> Result<Vec<Result>> {
    use futures::future::join_all;

    let handles = items.into_iter().map(|item| async move {
        process_item(item).await
    });

    // Run up to N tasks concurrently (N determined by executor)
    join_all(handles).await

// Using rayon for true parallelism across CPU cores:
use rayon::prelude::*;

let results: Vec<_> = items.into_par_iter()
    .map(|item| process_item(item))
    .collect();
```

### Pattern 2: Select! Macro for Multiple Futures

```rust
async fn wait_for_multiple<F1, F2>(
    fut1: F1,
    fut2: F2) -> Result<(OutcomeA, OutcomeB)>
where
    F1: Future<Output = A> + Unpin,
    F2: Future<Output = B> + Unpin,
{
    use futures::future::{select, Either};

    let (a_result, remaining_fut) = select(fut1, fut2).await;

    match a_result {
        Either::Left(a) => Ok((a, remaining_fut.await)),  // F1 finished first
        Either::Right(b) => Err(Error::UnexpectedB),      // Shouldn't happen here!
    }
}
```

### Pattern 3: Stream Processing

```rust
use tokio_stream::{StreamExt, StreamMap};

async fn process_events() -> Result<()> {
    let mut stream = event_source().await?;

    while let Some(event) = stream.next().await {  // Non-blocking when no events!
        match handle_event(event).await {
            Ok(_) => continue,
            Err(e) => log::error!("Failed to process: {}", e),
        }
    }

    Ok(())
}
```

## Pitfalls to Avoid

### ❌ Bad: Blocking the Event Loop with `std` I/O in async code
```rust
// BAD - Blocks all other tasks!
async fn read_file_sync(path: &str) -> Vec<u8> {
    std::fs::read_to_string(path).unwrap()  // This blocks!

#[tokio::test]
async fn test_async_with_blocker() { /* ... */ }
```

**✓ Good:** Use tokio file APIs or spawn_blocking.

### ❌ Bad: Using `std::thread` instead of `spawn_blocking`
```rust
// BAD - Creates OS threads, not task-local resources!
tokio::task::spawn(|| {
    std::thread::sleep(Duration::from_secs(1));  // Doesn't integrate with tokio runtime!

#[tokio::test]
async fn test_with_std_thread() { /* ... */ }
```

**✓ Good:** Use `tokio::task::spawn_blocking` for CPU-bound work.

### ❌ Bad: Not understanding async test isolation
```rust
// BAD - Shared mutable state causes flaky tests!
static SHARED_STATE: std::sync::Arc<std::sync::Mutex<usize>> = ...;

#[tokio::test]
async fn concurrent_tests() {
    // Multiple parallel runs of this same function will race on SHARED_STATE!

#[tokio::test(flavor = "current_thread")]
async fn isolated_async_test() {  // GOOD - Each test gets its own runtime!
```

### ❌ Bad: Forgetting to await futures
```rust
// BAD - Future is never polled, code won't execute as written
let result = process().await;  // If this line was missing...

#[tokio::test]
async fn example() {
    let future = expensive_operation();
    handle_result(future);  // BUG: This doesn't await!
}
```

**✓ Good:** Always `?` or `.await` futures.

## Examples

```rust
use tokio::sync::mpsc;
use std::time::Duration;

/// # Async HTTP Handler Pattern
async fn fetch_and_process(url: &str) -> Result<Response> {
    // Step 1: Non-blocking timeout for connection
    let stream = tokio::net::TcpStream::connect(url)
        .await
        .context("Failed to connect")?;

    // Step 2: Read with separate timeout (per-request basis)
    let mut buf = vec![0u8; 4096];
    match tokio::time::timeout(Duration::from_secs(30), stream.read(&mut buf)).await {
        Ok(n) if *n > 0 => { /* success */ }
        _ => return Err(Error::ReadTimeout),
    }

    // Step 3: Process with spawn_blocking for CPU work
    let processed = tokio::task::spawn_blocking(|| {
        parse_response(buf)
    }).await?;

    match processed {
        Ok(data) if data.is_valid() => Ok(Response::new(data)),
        Err(e) | _ => return Err(Error::ParseError),
    }
}

/// # Channel-Based Task Coordinator
async fn task_coordinator(num_workers: usize, tasks: Vec<Task>) -> Result<()> {
    let (tx, mut rx) = mpsc::channel::<TaskResult>(num_workers * 2);

    // Spawn worker pool
    for i in 0..num_workers {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            while let Some(task) = rx.recv().await {  // Blocks when empty, not full!
                match process_task(&task).await {
                    Ok(result) => drop(tx_clone.send(Ok((i, result)))),
                    Err(e) => drop(tx_clone.send(Err((i, e)))),
                }
            }
        });
    }

    // Send tasks to pool
    for task in tasks.into_iter() {  // Non-blocking send when channel not full!
        tx.send(task).await?;
    };

    Ok(())
}
```

## References

- [`rust.md`](../stacks/rust.md) - Comprehensive Rust async conventions (MANDATORY reading)
- The `tokio` crate documentation: https://docs.rs/tokio/latest/
- "Tokio for Real World Applications" book: https://tokio.rs/tokio/tutorial
- Futures API reference: https://docs.rs/futures/latest/

---

*Created: 2026-01-27*
