---
name: "Cross-Target Crate Layout"
description: "Keep shared/ target-agnostic; move native-only code to native/"
license: "MIT"
tags:
  - rust
  - wasm
  - cfg
  - module-layout
  - anti-pattern
---

# Cross-Target Crate Layout

## The rule

A crate that builds for native and for WebAssembly (wasm) puts its code in
three modules:

| Module    | Builds on            | Holds                                  |
|-----------|----------------------|----------------------------------------|
| `shared/` | every target         | code that is correct on every target   |
| `native/` | native only          | code that needs an operating system    |
| `wasm/`   | wasm only            | code that needs the browser or host    |

`src/lib.rs` declares them:

```rust
// Cross-platform modules (native + wasm).
pub mod shared;

// Native-only modules (TCP server, HTTP/2 and HTTP/3 transports).
#[cfg(not(target_family = "wasm"))]
pub mod native;

// Browser-only modules.
#[cfg(target_family = "wasm")]
pub mod wasm;
```

`shared/` compiles on every target from the same source. That is its purpose.
If an item inside `shared/` carries a target marker, the item is in the wrong
module.

---

## Bad code

```rust
// src/shared/router/dispatch.rs — BAD

/// One entry is added for each value.
#[cfg(not(target_family = "wasm"))]
pub(crate) fn extend_handler_headers(
    headers: &mut HeaderMap,
    values: &HeaderValues,
) {
    // Only called by the HTTP/2 dispatch below, which needs foundation_netio's
    // native HTTP/2 backend.
}
```

Why this is bad:

- The function is not "wasm unsafe". It is native-only because its caller is
  native-only. The marker records the caller, not a property of the function.
- The marker hides the real defect: native-only code lives in a cross-target
  file.
- The marker spreads. The caller needs a marker. The caller's caller needs a
  marker. One native-only line becomes nineteen markers in `shared/`.
- The next edit in `shared/` sees a missing function on wasm and adds the
  twentieth marker instead of moving the code.
- The file no longer shows which code runs on which target. A reader must
  evaluate every marker to know.

A second example: `src/shared/vfs/search.rs` uses `std::process::Command` and
`BufRead` to run `rg` and `fd`. The whole `CliSearcher` is wrapped in
`#[cfg(not(target_family = "wasm"))]` — eighteen markers — because it runs
native command-line tools. It belongs in `native/vfs/search.rs`.

---

## Good code

Move the item and its callers to `native/`. Remove the markers.

```rust
// src/shared/router/dispatch.rs — target-agnostic, no markers
use crate::shared::router::headers::is_framing_header;

// src/native/dispatch_h2.rs — native-only, no markers
use crate::shared::router::dispatch::route_request;

/// Adds one header entry for each value.
pub(crate) fn extend_handler_headers(/* … */) { /* … */ }

/// Runs the HTTP/2 request/response loop.
pub(crate) async fn run_streaming_h2(/* … */) { /* … */ }
```

```rust
// src/shared/router/mod.rs
pub(crate) mod dispatch;          // target-agnostic

// src/native/mod.rs
pub(crate) mod dispatch_h2;       // native-only
```

Move the point where the two worlds meet to the crate root or to the entry
point that is already target-specific. Do not make `shared/` call `native/`.
The native entry point calls into `shared/`.

---

## The two kinds of marker

Not every marker in `shared/` is wrong. Separate them.

### ❌ Deleting marker (wrong location)

The marker removes the item on wasm. Only native code uses it.

```rust
#[cfg(not(target_family = "wasm"))]
pub fn start_tcp_listener(/* … */) { /* … */ }
```

**Fix:** move the item to `native/`.

### ✅ Strategy marker (correct, stays)

Both targets have the item, and each target has its own correct
implementation. The marker selects a body; it does not delete the item.

```rust
// src/shared/password_hash.rs — correct
#[cfg(target_family = "wasm")]
pub fn hash_password(password: &str) -> Result<String, Error> {
    webcrypto_hash(password)
}

#[cfg(not(target_family = "wasm"))]
pub fn hash_password(password: &str) -> Result<String, Error> {
    argon2_hash(password)
}
```

The same shape applies to `shared/client/dns.rs`: the system resolver on
native, the browser resolver on wasm. Keep these markers. Document the
difference in the item's doc comment.

A third kind is a capability difference that no move can remove:

```rust
// The wasm executor is single-threaded, so its future type drops `Send`.
#[cfg(target_family = "wasm")]
type ExchangeFuture = Pin<Box<dyn Future<Output = Result<Body>>>>;

#[cfg(not(target_family = "wasm"))]
type ExchangeFuture = Pin<Box<dyn Future<Output = Result<Body>> + Send>>;
```

Keep it, and add one comment that states the capability difference.

---

## Decision table

| Question                                                   | Answer |
|------------------------------------------------------------|--------|
| Does the marker delete the item on one target?             | Move the item to `native/` or `wasm/`. |
| Does the marker select a different body for the same item? | Keep it in `shared/`. |
| Does the marker only pick a `Send` bound or a type alias?  | Keep it in `shared/`. |
| Is the marker on a `mod` declaration or a `pub use` inside `shared/`? | Move the module and the re-export. |
| Is the marker on an item whose only caller is already target-gated? | Move both to `native/`. |
| Does the same platform-agnostic item exist in both `native/` and `wasm/`? | Move one copy to `shared/`. |

The last row is the opposite of a deleting marker. Two copies of the same
function, trait or type, one in `native/` and one in `wasm/`, are a
duplication defect. If the code touches no platform API, move it to `shared/`
and delete the copies. If one part touches a platform API, keep that part in
`native/` or `wasm/` and move the common part to `shared/`.

---

## Check

Run this before a commit:

```bash
rg -n 'cfg\((not\()?target_family = "wasm"' backends/*/src/shared --glob '*.rs'
```

Every hit must be a strategy marker or a capability difference. A hit with a
deleting marker is a finding: move the code.

A wasm build with `-D warnings` finds dead code behind a wrong marker, but it
does not find the layout defect. The gate lives in review, and in the rule
above.

---

## Why this matters

- `shared/` states one promise: this code is correct on every target. Markers
  break the promise and make the module unreadable.
- Native-only code in `shared/` pulls native dependencies into the wasm build
  graph, or needs an empty stub. A move removes the stub.
- The marker count grows by one per new caller. A move ends the growth.
