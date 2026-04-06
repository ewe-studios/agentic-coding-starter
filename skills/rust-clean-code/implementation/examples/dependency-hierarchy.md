# Dependency Hierarchy: Project First, Stdlib Second, External Last

This document explains the critical principle of checking project modules before adding dependencies.

## The Dependency Hierarchy

```
1. Project modules/crates (FIRST)
   ↓ Can't fulfill need?
2. Rust stdlib (SECOND)
   ↓ Can't fulfill need?
3. External crates (LAST RESORT)
```

## Process: Building Blocks Before Dependencies

**MANDATORY steps before adding external dependency:**

1. **Search project codebase** - Does a module already provide this?
2. **Check building blocks** - Can we compose existing project types?
3. **Try stdlib** - Does standard library provide primitives?
4. **Create project module** - Build on existing foundation if possible
5. **External dependency** - Only when truly necessary

## Import Ordering: Top-Level Only 🚨

**MOST IMPORTANT RULE: All imports must be at the top level of the file.**

**❌ BAD - Imports scattered throughout file:**
```rust
//! Module documentation.

use std::path::Path;

pub struct Foo { ... }

impl Foo {
    pub fn method(&self) {
        use some::external::crate::Thing;  // BAD: inline import
        Thing::new()
    }
}

pub fn helper() {
    use another::module::Helper;  // BAD: function-level import
    Helper::run()
}
```

**✅ GOOD - All imports at top:**
```rust
//! Module documentation.

use another::module::Helper;
use some::external::crate::Thing;
use std::path::Path;

pub struct Foo { ... }

impl Foo {
    pub fn method(&self) {
        Thing::new()
    }
}

pub fn helper() {
    Helper::run()
}
```

**Import Order Convention:**
1. Project modules (`crate::`, `foundation_*::`)
2. Rust stdlib (`std::`, `core::`)
3. External crates (`serde::`, `tokio::`, etc.)

**Why Top-Level Imports Matter:**
- **Visibility**: All dependencies visible at a glance
- **Consistency**: Matches Rust community conventions
- **Maintainability**: Easy to spot unused imports
- **Compilation**: Enables better compiler optimizations
- **Code Review**: Clear understanding of dependencies

## Example: HTTP Test Server

**❌ BAD - Immediate external dependency:**
```toml
[dev-dependencies]
axum = "0.7"  # External framework
hyper = "1.0"  # External HTTP
```

**✅ GOOD - Use project building blocks:**
```rust
// Project already has:
// - wire::simple_http::HttpRequestReader
// - wire::simple_http::SimpleOutgoingResponse
// - wire::simple_http::RenderHttp trait + Http11 impl
// - std::net::TcpListener (stdlib)

// So we create: foundation_core/src/testing/http_server.rs
use crate::wire::simple_http::{HttpRequestReader, SimpleOutgoingResponse, Http11};
use std::net::TcpListener;
use std::thread;

pub struct TestHttpServer {
    listener: TcpListener,
    // ... implementation using existing building blocks
}

impl TestHttpServer {
    pub fn start() -> Self {
        // Compose existing project types
    }
}
```

## Benefits of Project-First Approach

1. **Consistency** - Uses patterns already established in codebase
2. **Less bloat** - No redundant dependencies
3. **Better integration** - Works naturally with existing types
4. **Maintainability** - One less external dependency to track
5. **Learning** - Understand project architecture better

## How to Search Project Codebase

```bash
# Search for HTTP types
rg "struct.*Http" --type rust

# Search for error types
rg "enum.*Error" --type rust

# Search for traits
rg "trait.*" --type rust

# Find modules by name
fd -e rs | grep pattern
```

## When External Dependencies ARE Appropriate

✅ **Use external crates when:**
- Functionality is complex and well-tested (e.g., `serde`, `tokio`)
- Project doesn't have related infrastructure
- Stdlib doesn't provide the functionality
- Creating from scratch would take significant time
- The crate is widely used and well-maintained

❌ **Avoid external crates when:**
- Project already has similar types
- Stdlib can do it with minimal code
- You're only using 5% of the crate's functionality
- The crate has many transitive dependencies
- You can compose from existing project building blocks

## Checklist Before Adding Dependencies

- [ ] Searched project codebase for existing types/traits
- [ ] Checked if stdlib provides the functionality
- [ ] Considered composing from existing project types
- [ ] Verified external crate is truly necessary
- [ ] Checked crate maintenance status and popularity
- [ ] Reviewed transitive dependencies (`cargo tree`)
- [ ] **All imports placed at top of file (no inline imports)**
