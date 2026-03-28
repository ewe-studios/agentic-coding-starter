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

## Code Style: Clear, Simple, Succinct

**Prefer direct, minimal combinator chains.** Each combinator should have a clear purpose:

```rust
// GOOD: Only transform what needs transforming
let task = task
    .map_ready(|v| v * 2)      // transforming Ready
    .filter_ready(|v| v > 10); // filtering Ready

// BAD: Unnecessary map_pending that does nothing
let task = task
    .map_pending(|p| p)        // useless - remove
    .map_ready(|v| v * 2);

// GOOD: Use specific combinators for clarity
let stream = stream.filter_done(|v| v.is_ok());

// BAD: Over-engineered with unnecessary maps
let stream = stream
    .map_done(|v| v)           // useless identity map
    .map_pending(|p| p)        // useless identity map
    .filter_done(|v| v.is_ok());
```

**Rules of thumb:**
- Don't add `map_pending` unless you actually need to transform Pending values
- Don't chain multiple maps when one would do
- Use the most specific combinator (`filter_done` vs `map_done` + conditional)
- If a combinator doesn't change behavior, remove it

## Combinator Usage: Prefer StreamIteratorExt Over Raw Iterator

**Never manually match all `Stream` variants** — use `StreamIteratorExt` combinators that handle pass-through automatically.

### Anti-Pattern: Manual Stream Matching

```rust
// BAD: Verbose, error-prone, manually handles all Stream variants
let mapped = raw_stream.filter_map(|s| match s {
    Stream::Next(Some(json_str)) => match serde_json::from_str::<V>(&json_str) {
        Ok(v) => Some(Stream::Next(Some(v))),
        Err(e) => {
            tracing::error!("Deserialization error: {e}");
            None
        }
    },
    Stream::Next(None) => Some(Stream::Next(None)),
    Stream::Pending(p) => Some(Stream::Pending(p)),      // boilerplate
    Stream::Init => Some(Stream::Init),                   // boilerplate
    Stream::Ignore => Some(Stream::Ignore),               // boilerplate
    Stream::Delayed(d) => Some(Stream::Delayed(d)),       // boilerplate
});
```

**Problems:**
- 15+ lines of boilerplate just to transform `Next`
- Must manually update if `Stream` adds new variants
- Hard to read — logic buried in match arms

### Correct: Use `map_done` Combinator

```rust
// GOOD: map_done automatically passes through Pending, Init, Ignore, Delayed
let mapped = raw_stream.map_done(|opt_json| {
    opt_json.and_then(|json_str| {
        serde_json::from_str::<V>(&json_str)
            .map_err(|e| tracing::error!("Deserialization error: {e}"))
            .ok()
    })
});
```

**Benefits:**
- 5 lines instead of 15+
- Future-proof — new Stream variants handled automatically
- Clear intent — "transform Next values, pass through rest"

### When `map_pending` IS Necessary

Only use `map_pending` when you need to **change the Pending type**:

```rust
// GOOD: Converting Pending type for type compatibility
let stream = raw_stream
    .map_done(|v| v * 2)
    .map_pending(|p| MyPending::from(p));  // Type conversion needed

// BAD: Unnecessary map_pending — Pending type doesn't matter
let stream = raw_stream
    .map_done(|v| v * 2)
    .map_pending(|_| ());  // Useless unless type requires it
```

**Key insight:** At collection boundaries (`find_map`, `collect_result`), only `Stream::Next` values are extracted. Pending is progress information that gets discarded. Don't transform it unless the type signature requires it.

### Common Transformations

| Goal | Use |
|------|-----|
| Transform `Next` values | `map_done(f)` |
| Filter `Next` values | `filter_done(f)` |
| Transform `Pending` values (type change) | `map_pending(f)` |
| Transform both with one function | `map_pending_and_done(f)` |
| Deserialize/parse `Next` | `map_done` + `and_then` |
| Stop on error, preserve it | `map_circuit` |

```rust
// Deserialize JSON in Next values
let stream = stream.map_done(|json_opt| {
    json_opt.and_then(|s| serde_json::from_str(&s).ok())
});

// Convert Result to Option, log errors
let stream = stream.map_done(|result| {
    result.map_err(|e| tracing::error!("Error: {e}")).ok()
});

// Extract field from Next value
let stream = stream.map_done(|user| user.name);
```

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

### Anti-Pattern 3: Using loop {} in StreamIterator::next() or TaskIterator::next_status

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
| `filter_state(f)` | Filter based on full `TaskStatus` |
| `stream_collect()` | Collect all Ready values into Vec |
| `flatten_ready()` | Flatten Ready values that are `IntoIterator` |
| `flatten_pending()` | Flatten Pending values that are `IntoIterator` |
| `flat_map_ready(f)` | Map + flatten Ready in one operation |
| `flat_map_pending(f)` | Map + flatten Pending in one operation |
| `map_state(f)` | Transform any `TaskStatus` variant |
| `inspect_state(f)` | Side-effect on any `TaskStatus` |
| `map_circuit(f)` | Short-circuit on condition — return error and stop, or continue |
| `map_iter(f)` | Flatten Ready into inner iterator |
| `split_collector(pred, size)` | Fork into observer + continuation |
| `split_collect_one(pred)` | Fork on first match |
| `split_collect_until(pred, size)` | Fork until predicate signals close |
| `split_collect_until_map(f, size)` | Fork with transformation until close |
| `split_collector_map(f, size)` | Fork with transformation |
| `take(n)` / `take_state(n, f)` | Take first n items (Ready / any state) |
| `take_all(n)` | Take first n items of any state |
| `take_while(f)` / `take_while_state(f)` | Take while predicate holds |
| `take_while_any(f)` | Take while predicate holds on any state |
| `skip(n)` / `skip_state(n, f)` | Skip first n items (Ready / any state) |
| `skip_all(n)` | Skip first n items of any state |
| `skip_while(f)` / `skip_while_state(f)` | Skip while predicate holds |
| `skip_while_any(f)` | Skip while predicate holds on any state |
| `enumerate()` | Add index to each item |
| `find(f)` | Find first item matching predicate |
| `find_map(f)` | Find first item mapping to Some |
| `fold(init, f)` | Fold/accumulate values |
| `all(f)` | Check if all Ready items satisfy predicate |
| `any(f)` | Check if any Ready item satisfies predicate |
| `count()` | Count Ready items |
| `count_all()` | Count all items (any state) |

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

// Flat map Ready values
let task = task.flat_map_ready(|vec| vec.into_iter().map(TaskStatus::Ready));

// Split stream to observe first match while continuing
let (observer, continuation) = task.split_collect_one(|item| item.is_success());
```

**Why `map_circuit` for error handling:**

When a task should fail immediately on error, you need to communicate that error to the caller while stopping iteration. Without `map_circuit`, you'd need wrapper enums or lose the error:

```rust
// WITHOUT map_circuit — verbose, creates wrapper states
enum ResultWithDone<T> {
    Continue(T),
    Done(T),  // Need a separate variant just to signal "stop with this value"
}

let task = task.map_ready(|result| match result {
    Err(e) => ResultWithDone::Done(Err(e)),  // Custom enum needed
    Ok(v) => ResultWithDone::Continue(Ok(v)),
});

// WITH map_circuit — clear, succinct, no wrapper types
let task = task.map_circuit(|status| match status {
    TaskStatus::Ready(Err(e)) => TaskShortCircuit::ReturnAndStop(TaskStatus::Ready(Err(e))),
    _ => TaskShortCircuit::Continue(status),
});
```

The key insight: `ReturnAndStop(value)` returns the value AND stops — the caller receives the error via `Stream::Next(error)` and knows iteration is done. This preserves the error without inventing wrapper types.

Use `map_circuit` **before `execute()`** when:
- The task should terminate early on error
- You want to propagate the error to the stream consumer
- You need to stop immediately without creating wrapper enums

### Post-Execute Combinators (StreamIteratorExt)

Applied **after** `execute()` — transforms the stream. Use these **between** operations, not at boundaries:

| Combinator | Purpose |
|------------|---------|
| `map_done(f)` | Transform Next values |
| `map_pending(f)` | Transform Pending values |
| `map_pending_and_done(f)` | Transform both Pending and Next with single function |
| `map_delayed(f)` | Transform Delayed durations |
| `filter_done(f)` | Filter Next values (filtered → Ignore) |
| `filter_state(f)` | Filter based on full `Stream` state |
| `collect()` | Accumulate all Next values, yield as single Vec |
| `flatten_next()` | Flatten Next values that are `IntoIterator` |
| `flatten_pending()` | Flatten Pending values that are `IntoIterator` |
| `flat_map_next(f)` | Map + flatten Next in one operation |
| `flat_map_pending(f)` | Map + flatten Pending in one operation |
| `map_state(f)` | Transform any `Stream` variant |
| `map_iter(f)` | Flatten Next into inner iterator |
| `inspect_state(f)` | Side-effect on any `Stream` state |
| `map_circuit(f)` | Short-circuit on condition — return value and stop, or continue |
| `split_collector(pred, size)` | Fork into observer + continuation |
| `split_collect_one(pred)` | Fork on first match |
| `split_collect_until(pred, size)` | Fork until predicate signals close |
| `split_collector_map(f, size)` | Fork with transformation |
| `split_collect_one_map(f, size)` | Fork with transformation on first match |
| `take(n)` / `take_state(n, f)` | Take first n items (Next / any state) |
| `take_all(n)` | Take first n items of any state |
| `take_while(f)` / `take_while_state(f)` | Take while predicate holds |
| `take_while_any(f)` | Take while predicate holds on any state |
| `skip(n)` / `skip_state(n, f)` | Skip first n items (Next / any state) |
| `skip_all(n)` | Skip first n items of any state |
| `skip_while(f)` / `skip_while_state(f)` | Skip while predicate holds |
| `skip_while_any(f)` | Skip while predicate holds on any state |
| `enumerate()` | Add index to each item |
| `find(f)` | Find first item matching predicate |
| `find_map(f)` | Find first item mapping to Some |
| `fold(init, f)` | Fold/accumulate values |
| `all(f)` | Check if all Next items satisfy predicate |
| `any(f)` | Check if any Next item satisfies predicate |
| `count()` | Count Next items |
| `count_all()` | Count all items (any state) |

**Quick examples:**

```rust
// Transform Next values
let stream = stream.map_done(|v| v.to_string());

// Filter Next values (filtered items become Ignore)
let stream = stream.filter_done(|v| !v.is_empty());

// Stop immediately when seeing an error, preserving it for the caller
let stream = stream.map_circuit(|item| match item {
    Stream::Next(Err(e)) => ShortCircuit::ReturnAndStop(Stream::Next(Err(e))),
    _ => ShortCircuit::Continue(item),
});

// Chain combinators for clean pipelines
let stream = stream
    .filter_done(|v| v.is_ok())
    .map_done(|v| v.unwrap());

// Flat map Next values
let stream = stream.flat_map_next(|vec| vec.into_iter().map(Stream::Next));

// Split stream to observe first match while continuing
let (observer, continuation) = stream.split_collect_one(|item| matches!(item, Stream::Next(v) if v > 10));
```

**Why `map_circuit` after `execute()`:**

When consuming a stream, you often want to stop on error while preserving the error for the caller. Without `map_circuit`, you'd need to return `None` and lose the error, or wrap results in custom enums:

```rust
// WITHOUT map_circuit — error is lost, caller can't distinguish
let stream = stream.filter_done(|v| v.is_ok());  // Errors become Ignore, swallowed

// OR: verbose wrapper enum
enum StreamValue<T> {
    More(T),
    Last(T),  // Extra variant just to say "stop with this"
}

// WITH map_circuit — error preserved, iteration stops cleanly
let stream = stream.map_circuit(|item| match item {
    Stream::Next(Err(e)) => ShortCircuit::ReturnAndStop(Stream::Next(Err(e))),
    Stream::Next(Ok(v)) => ShortCircuit::Continue(Stream::Next(Ok(v))),
    _ => ShortCircuit::Stop,
});
```

Use `map_circuit` **after `execute()`** when:
- The stream should terminate early on error
- You want to propagate errors to the final consumer
- You need clean error handling without wrapper enums

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
