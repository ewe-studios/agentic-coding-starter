---
name: "Rust Valtron Usage"
description: "How to properly use Valtron's TaskIterator/StreamIterator execution model — composable async operations, sync boundaries, and avoiding premature blocking"
approved: No
created: 2026-03-28
license: "MIT"
metadata:
  author: "Main Agent"
  version: "1.0"
  last_updated: "2026-03-28"
  tags: [rust, valtron, async, task-iterator, stream-iterator, foundation-core, execution-model]
tools: []
assets:
  - "bin/platform/src/gen_model_descriptors/mod.rs: Reference implementation of parallel task composition"
  - "backends/foundation_db/src/backends/async_utils.rs: exec_future helper (sync escape hatch)"
  - "infrastructure/foundation_core/src/valtron/: Valtron source code"
---

# Rust Valtron Usage

## Core Philosophy

Valtron is a progress-driven execution engine. Operations produce `TaskIterator`s or `StreamIterator`s that yield incremental results. The execution engine drives progress — **callers decide when and where to synchronize**.

**The fundamental rule:** Do not turn async operations into sync operations at the leaf. Instead, schedule work via `execute()`, return the stream to the caller, and let them decide when to collect results. This preserves composability, parallelism, and progress observability.

Blocking should happen at **boundaries** — the outermost point where a concrete value is actually needed — not inside every individual operation.

## The Execution Pipeline

```
create future/task
    → apply TaskIterator combinators (map_ready, map_pending, filter_ready, etc.)
    → execute() — schedules on thread pool, returns StreamIterator
    → apply StreamIterator combinators (map_done, filter_done, collect, etc.)
    → return stream to caller
    → caller composes multiple streams
    → caller collects at boundary (Iterator::find_map, sync_one, sync_all, etc.)
```

Every step is optional. The key insight: **separating launch from collection** enables parallelism even with heterogeneous types.

## Pattern 1: Methods Return Streams (The Default)

Methods that perform I/O should schedule the work and return the stream. The caller controls when to collect.

```rust
use foundation_core::valtron::{execute, from_future, Stream};

fn get<V: DeserializeOwned + Send + 'static>(
    &self, key: &str,
) -> StorageResult<impl Iterator<Item = Stream<Option<V>, ()>>> {
    let key = key.to_string();
    let conn = Arc::clone(&self.conn);

    let task = from_future(async move {
        let mut stmt = conn.prepare("SELECT value FROM kv_store WHERE key = ?").await?;
        let mut rows = stmt.query([key]).await?;
        match rows.next().await? {
            Some(row) => {
                let value: String = row.get(0)?;
                let deserialized: V = serde_json::from_str(&value)?;
                Ok::<_, BackendError>(Some(deserialized))
            }
            None => Ok(None),
        }
    });

    // Schedule the work — returns immediately, work runs on pool
    let stream = execute(task, None)
        .map_err(|e| StorageError::Backend(format!("Valtron scheduling failed: {e}")))?;
    Ok(stream)
}
```

**Why `StorageResult<impl Iterator<...>>`:** `execute()` returns a `Result` indicating whether the task was successfully scheduled. The `StorageResult` wraps that scheduling result. The stream itself carries the async operation's outcome as `Stream::Next(value)`.

### Caller Composes and Collects at Boundary

```rust
// Launch two independent lookups — both start executing immediately
let user_stream = db.get::<User>("users:alice")?;
let config_stream = db.get::<Config>("app:config")?;

// Both are running in parallel on the thread pool.
// Collect at the boundary — second may already be done.
let user_results = collect_result(user_stream);   // Vec<Option<User>>
let config_results = collect_result(config_stream); // Vec<Option<Config>>
```

## Pattern 2: TaskIterator Composition Before Execute (Parallel Homogeneous Tasks)

When multiple tasks have the same output type, compose them with `execute_collect_all` for parallel execution with synchronized collection.

**Reference implementation:** `bin/platform/src/gen_model_descriptors/mod.rs`

```rust
// Each task is a Box<dyn TaskIterator<Ready=Vec<ModelEntry>, Pending=FetchPending, ...>>
let models_dev_task = create_fetch_task(&mut client, "models.dev", URL_A, parse_a)?;
let openrouter_task = create_fetch_task(&mut client, "openrouter", URL_B, parse_b)?;
let ai_gateway_task = create_fetch_task(&mut client, "ai-gateway", URL_C, parse_c)?;

// All three run in parallel — synchronize when all complete
let result_stream = valtron::execute_collect_all(
    vec![models_dev_task, openrouter_task, ai_gateway_task],
    None,
).expect("scheduling succeeded");

// Collect at boundary
for item in result_stream {
    if let Stream::Next(models) = item {
        all_models.extend(models.into_iter().flatten());
    }
}
```

### Pre-Execute Combinators Shape the Task

Apply `TaskIteratorExt` combinators **before** `execute()` to transform the task's output:

```rust
let task = SendRequestTask::new(request, 5, pool, config)
    // Transform Ready: HttpResponse → Vec<ModelEntry>
    .map_ready(move |intro| match intro {
        RequestIntro::Success { stream, .. } => {
            let body = body_reader::collect_string(stream);
            parser(&body, source)
        }
        RequestIntro::Failed(e) => Vec::new(),
    })
    // Transform Pending: HttpPending → FetchPending
    .map_pending(move |p| FetchPending::from_http(p, source));
```

## Pattern 3: Heterogeneous Parallel Execution

When tasks return different types, launch each individually — they still run in parallel:

```rust
// Launch all three — work begins immediately for each
let user_stream = execute(user_task, None)?;
let session_stream = execute(session_task, None)?;
let oauth_stream = execute(oauth_task, None)?;

// Collect at boundary — by the time we finish the first,
// the others may already be done
let user = collect_result(user_stream);       // Vec<Option<User>>
let session = collect_result(session_stream); // Vec<Option<Session>>
let oauth = collect_result(oauth_stream);     // Vec<Option<OAuthState>>
```

## Pattern 4: Rich Pending Types (Per-Method Judgment)

The `Pending` type in `Stream<D, P>` carries progress information. Whether to use a rich type or `()` is a **per-method judgment call** based on whether the caller can do something useful with the progress state.

**Simple get — no meaningful progress, use `()`:**
```rust
fn get(&self, key: &str) -> StorageResult<impl Iterator<Item = Stream<Option<V>, ()>>>
```

**Migration — caller wants progress:**
```rust
#[derive(Debug, Clone)]
pub enum MigrationProgress {
    Applying { current: usize, total: usize, name: String },
    Verifying { migration: String },
}

fn migrate(&self) -> StorageResult<impl Iterator<Item = Stream<MigrationResult, MigrationProgress>>>
```

**HTTP fetch — caller wants connection state:**
```rust
#[derive(Debug, Clone)]
pub enum FetchPending {
    Connecting { source: &'static str },
    AwaitingResponse { source: &'static str },
}

fn fetch(&self, url: &str) -> Result<impl Iterator<Item = Stream<Response, FetchPending>>>
```

**Guideline:** Use `()` unless the operation is long-running or multi-step and the caller would benefit from observability. Don't force richness where it adds no value.

## Sync Boundary Helpers (New Valtron Primitives)

These are the tools for collecting at boundaries. They are the **only** place where blocking should occur.

### `collect_result` — Drain a Stream and Collect All Results

The primary boundary helper. Drains the entire stream, collecting every `Stream::Next` value. Blocks the calling thread until the stream is exhausted.

```rust
/// Blocks until stream is exhausted. Collects ALL `Stream::Next` values.
/// Works for single-value streams (Vec will have one item) and multi-value streams alike.
pub fn collect_result<D, P>(stream: impl Iterator<Item = Stream<D, P>>) -> Vec<D> {
    stream
        .filter_map(|s| match s {
            Stream::Next(v) => Some(v),
            _ => None,
        })
        .collect()
}
```

For multi-value streams (like `list_keys` or `query`), the `Vec` contains all results.

### `collect_one` — Extract First Result from a Stream

For single-value operations (like `get`) where you know the stream produces exactly one `Next`:

```rust
/// Blocks until first `Stream::Next(value)`, returns it. Returns None if stream exhausts.
pub fn collect_one<D, P>(stream: impl Iterator<Item = Stream<D, P>>) -> Option<D> {
    stream.find_map(|s| match s {
        Stream::Next(v) => Some(v),
        _ => None,
    })
}
```

### `sync_collect_one` — Execute Task, Return Single Value

The single-value counterpart to `sync_one`. Returns `Result<T::Ready>`, not `Result<Vec<T::Ready>>`:

```rust
/// Execute a single task, block until first result. Returns the value directly.
pub fn sync_collect_one<T>(task: T) -> GenericResult<T::Ready>
where
    T: TaskIterator + Send + 'static,
    T::Ready: Send + 'static,
    T::Pending: Send + 'static,
    T::Spawner: ExecutionAction + Send + 'static,
{
    let stream = execute(task, None)?;
    collect_one(stream).ok_or_else(|| /* error */)
}
```

### `sync_one` — Execute Task and Block for All Results

For tasks that produce multiple values:

```rust
/// Execute a single task and block until complete. Collects ALL results.
pub fn sync_one<T>(task: T) -> GenericResult<Vec<T::Ready>>
where
    T: TaskIterator + Send + 'static,
    T::Ready: Send + 'static,
    T::Pending: Send + 'static,
    T::Spawner: ExecutionAction + Send + 'static,
{
    let stream = execute(task, None)?;
    Ok(collect_result(stream))
}
```

### `sync_all` — Execute Multiple Tasks, Block Until All Complete

```rust
/// Execute multiple homogeneous tasks in parallel, block until all complete.
pub fn sync_all<T>(tasks: Vec<T>) -> GenericResult<Vec<T::Ready>>
where
    T: TaskIterator + Send + 'static,
    T::Ready: Send + 'static,
    T::Pending: Send + 'static,
    T::Spawner: ExecutionAction + Send + 'static,
{
    let stream = execute_collect_all(tasks, None)?;
    // execute_collect_all already buffers and collects all results internally.
    // It yields Stream::Pending(count) while in flight, then a single
    // Stream::Next(Vec<T::Ready>) when all complete.
    stream
        .find_map(|s| match s {
            Stream::Next(v) => Some(v),
            _ => None,
        })
        .ok_or_else(|| /* GenericError: no results produced */)
}
```

### Between vs. At Boundaries

**Between operations**, use `StreamIteratorExt` combinators — they preserve Valtron's execution model without blocking:

```rust
// BETWEEN operations — StreamIteratorExt (non-blocking, preserves Stream protocol)
let transformed_stream = stream
    .map_done(|user| user.name)
    .filter_done(|name| !name.is_empty());
```

**At boundaries**, use standard `Iterator` methods — these block the thread and extract raw values:

```rust
// AT BOUNDARY — standard Iterator (blocking, extracts values)
let names: Vec<String> = transformed_stream
    .filter_map(|s| match s { Stream::Next(v) => Some(v), _ => None })
    .collect();
```

## The `!Send` Constraint

Many database crates (Turso, libsql) return row iterators that are `!Send`. These cannot cross the Valtron execution boundary. **Consume them fully inside the async block**, collecting into `Vec<T>` before the future returns.

```rust
// CORRECT: Collect inside async block — Vec<String> is Send
let task = from_future(async move {
    let mut rows = stmt.query([]).await?;
    let mut keys = Vec::new();
    while let Some(row) = rows.next().await? {
        keys.push(row.get::<String>(0)?);
    }
    Ok::<_, BackendError>(keys)
});
let stream = execute(task, None)?;

// WRONG: turso::Rows is !Send — this won't compile
let task = from_future(async move {
    stmt.query([]).await  // Returns Rows which is !Send
});
```

## The `Send + 'static` Requirement

All data captured by async blocks must be `Send + 'static` for Valtron scheduling:

```rust
fn query(&self, sql: &str) -> StorageResult<impl Iterator<Item = Stream<Vec<Row>, ()>>> {
    let sql = sql.to_string();          // &str → owned String
    let conn = Arc::clone(&self.conn);   // Arc clone, not borrow

    let task = from_future(async move {
        // sql and conn are moved in — both Send + 'static
        conn.prepare(&sql).await?.query([]).await
    });
    let stream = execute(task, None)?;
    Ok(stream)
}
```

## Turbo-Fish for Async Block Error Types

When the compiler can't infer the error type in an async block, annotate explicitly:

```rust
exec_future(async move {
    conn.execute_batch(&sql).await?;
    Ok::<_, turso::Error>(true)  // Turbo-fish needed
})?;
```

## When Sync Blocking IS Acceptable

Some operations genuinely need to complete before anything else can proceed. Use `sync_one` or `exec_future` for:

- **One-shot initialization** — creating a DB connection, loading a model
- **Migrations** — must complete before the application starts serving
- **CLI tools** — where the entire program is sequential by nature

Even here, consider whether multiple initializations could run in parallel (e.g., connect to DB AND load config simultaneously).

## Anti-Patterns

### Anti-Pattern 1: Blocking at the Leaf (exec_future as Default)

```rust
// BAD: Every method blocks immediately — no parallelism possible
fn get(&self, key: &str) -> StorageResult<Option<V>> {
    exec_future(async move { /* ... */ })  // Blocks here!
}

fn set(&self, key: &str, value: &V) -> StorageResult<()> {
    exec_future(async move { /* ... */ })  // Blocks here!
}

// Caller has no choice — two sequential blocking calls
let user = db.get("user:1")?;      // blocks
let config = db.get("config")?;    // blocks (could have been parallel)
```

### Anti-Pattern 2: Blocking in the Middle of a Chain

```rust
// BAD: Blocking mid-pipeline kills composability
let partial = collect_result(stream_a);  // blocks!
let transformed = transform(partial);
let final_stream = execute(make_task(transformed), None)?;
// The first block prevented us from overlapping work
```

### Anti-Pattern 3: Using loop {} in StreamIterator::next()

```rust
// BAD: Blocks the executor thread
impl StreamIterator for MyStream {
    fn next(&mut self) -> Stream<D, P> {
        loop {
            if let Some(val) = self.try_get() {
                return Stream::Next(val);
            }
        }
    }
}

// GOOD: Return control to the executor
impl StreamIterator for MyStream {
    fn next(&mut self) -> Stream<D, P> {
        match self.try_get() {
            Some(val) => Stream::Next(val),
            None if self.done => Stream::Init, // signal completion
            None => Stream::Ignore,            // yield back to executor
        }
    }
}
```

## Spawner Type: Prefer `BoxedSendExecutionAction` over `NoAction`

When implementing `TaskIterator`, the `Spawner` associated type controls whether the task can spawn sub-tasks during execution. **`NoAction` should be rarely used** — it prevents the task from ever spawning work.

**Default choice:** Use `BoxedSendExecutionAction` for the `Spawner` type. This allows tasks to spawn sub-work if needed and is compatible with all executor functions.

```rust
use foundation_core::valtron::{BoxedSendExecutionAction, TaskIterator, TaskStatus};

// PREFERRED: Allows spawning sub-tasks
impl TaskIterator for MyTask {
    type Ready = MyResult;
    type Pending = MyProgress;
    type Spawner = BoxedSendExecutionAction;

    fn next_status(&mut self) -> Option<TaskStatus<Self::Ready, Self::Pending, Self::Spawner>> {
        // ...
    }
}

// AVOID: Only use when you are certain the task will never need to spawn
impl TaskIterator for SimplePureComputeTask {
    type Ready = u32;
    type Pending = ();
    type Spawner = NoAction;  // Rarely appropriate
    // ...
}
```

**When `NoAction` is acceptable:** Trivial tasks that are pure computation with no possibility of needing to delegate work (e.g., test fixtures, simple wrappers around `from_future`).

## Valtron Quick Reference

### Core Types

| Type | Purpose |
|------|---------|
| `TaskStatus<D, P, S>` | Raw task state: Ready, Pending, Delayed, Init, Spawn, Ignore |
| `Stream<D, P>` | Stream state: Next, Pending, Delayed, Init, Ignore |
| `TaskIterator` | Trait for producing `TaskStatus` — input to `execute()` |
| `StreamIterator` | Trait for producing `Stream` — output from `execute()` |

### Executor Functions

| Function | Purpose | Returns |
|----------|---------|---------|
| `execute(task, wait)` | Schedule task, return stream | `GenericResult<DrivenStreamIterator<T>>` |
| `execute_collect_all(tasks, wait)` | Parallel exec, collect all | `GenericResult<CollectAllStream<T>>` |
| `execute_map_all(tasks, f, wait)` | Parallel exec, map when done | `GenericResult<MapAllDoneStream<T, F, O>>` |
| `execute_as_task(task, wait)` | Schedule, return raw TaskStatus | `GenericResult<DrivenRecvIterator<T>>` |
| `send(task)` | Fire and forget | `GenericResult<()>` |
| `from_future(future)` | Wrap Future as TaskIterator | `FutureTask<F>` |

### Pre-Execute Combinators (TaskIteratorExt)

Applied **before** `execute()` — transforms the task itself:

| Combinator | Purpose |
|------------|---------|
| `map_ready(f)` | Transform Ready values |
| `map_pending(f)` | Transform Pending values |
| `filter_ready(f)` | Filter Ready values (filtered → Ignore) |
| `stream_collect()` | Collect all Ready values into Vec |
| `flatten_ready()` | Flatten Ready values that are IntoIterator |
| `map_circuit(f)` | Short-circuit on condition — return error and stop, or continue |

**Quick examples:**

```rust
// Transform Ready values before execution
let task = task.map_ready(|v| v * 2);

// Filter out unwanted results (filtered items become Ignore)
let task = task.filter_ready(|v| v > 10);

// Stop immediately when seeing an error, returning it
let task = task.map_circuit(|status| match status {
    TaskStatus::Ready(Err(e)) => TaskShortCircuit::ReturnAndStop(TaskStatus::Ready(Err(e))),
    _ => TaskShortCircuit::Continue(status),
});
```

### Post-Execute Combinators (StreamIteratorExt)

Applied **after** `execute()` — transforms the stream. Use these **between** operations, not at boundaries:

| Combinator | Purpose |
|------------|---------|
| `map_done(f)` | Transform Next values |
| `map_pending(f)` | Transform Pending values |
| `filter_done(f)` | Filter Next values |
| `collect()` | Accumulate all Next values, yield as single Vec |
| `split_collector(pred, size)` | Fork stream into observer + continuation |
| `map_circuit(f)` | Short-circuit on condition — return value and stop, or continue |

**Quick examples:**

```rust
// Transform Next values
let stream = stream.map_done(|v| v.to_string());

// Filter Next values (filtered items become Ignore)
let stream = stream.filter_done(|v| !v.is_empty());

// Stop immediately when seeing an error, preserving it
let stream = stream.map_circuit(|item| match item {
    Stream::Next(Err(e)) => ShortCircuit::ReturnAndStop(Stream::Next(Err(e))),
    _ => ShortCircuit::Continue(item),
});

// Chain combinators for clean pipelines
let stream = stream
    .filter_done(|v| v.is_ok())
    .map_done(|v| v.unwrap());
```

### Boundary Collection (Standard Iterator — Use at Sync Points Only)

| Method | Usage |
|--------|-------|
| `find_map(\|s\| match s { Stream::Next(v) => Some(v), _ => None })` | Extract first result |
| `.filter_map(...).collect::<Vec<_>>()` | Collect all results |
| `collect_result(stream)` | Drain stream, collect all `Next` values into `Vec<D>` |
| `collect_one(stream)` | Drain stream until first `Next`, return `Option<D>` |
| `sync_collect_one(task)` | Execute + return single value as `Result<T::Ready>` |
| `sync_one(task)` | Execute + collect all results as `Result<Vec<T::Ready>>` |
| `sync_all(tasks)` | Execute all in parallel + collect all results |

## Decision Flowchart

**Is this operation I/O (database, HTTP, file download)?**
- No → Keep it synchronous. No Valtron needed.
- Yes → Continue:

**Does the caller need to compose this with other operations?**
- Yes → Return a stream (`execute()` + return the iterator). Always the default for library/trait methods.
- No → Is this a one-shot initialization or CLI tool?
  - Yes → `sync_one()` or `exec_future()` is acceptable.
  - No → Return a stream anyway. The caller might compose later.

**Does the caller benefit from progress reporting?**
- Yes → Use a rich `Pending` type (e.g., `MigrationProgress`, `FetchPending`).
- No → Use `Pending = ()`.

---

_Created: 2026-03-28_
_Reference implementations: bin/platform/src/gen_model_descriptors/mod.rs, backends/foundation_db/src/backends/_
