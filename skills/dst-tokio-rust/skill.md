---
name: "Deterministic Simulation Testing (Tokio/Rust)"
description: "Implement DST for distributed systems in Rust - controlling time, entropy, network, and execution for reproducible testing"
approved: No
created: 2026-01-19
license: "MIT"
metadata:
  author: "Main Agent"
  version: "1.0"
  last_updated: "2026-01-19"
  tags:
    - rust
    - tokio
    - testing
    - distributed-systems
    - simulation
    - determinism
tools:
  - Rust
  - Cargo
  - turmoil
  - madsim
  - proptest
files:
  - dst-rust-guide.md: "Main tutorial on DST concepts and using existing libraries"
  - dst-internals-guide.md: "Deep dive on building your own DST framework from scratch"
  - examples/basic-dst-setup.rs: "Complete working example of DST test setup"
---

# Deterministic Simulation Testing (Rust)

## Overview

Deterministic Simulation Testing (DST) is a testing methodology where distributed systems run in a fully controlled, reproducible environment. Every source of non-determinism (time, randomness, network, I/O) is replaced with simulated, seedable alternatives.

This skill covers both using existing DST libraries (turmoil, madsim) and building your own DST infrastructure from scratch.

**Key insight**: If you control all sources of non-determinism and use a seed, running the same test twice with the same seed produces identical behavior - making distributed system bugs reproducible.

## When to Use This Skill

- **Testing distributed systems** with complex failure modes
- **Reproducing intermittent failures** that are hard to debug
- **Verifying consensus protocols** under network partitions
- **Running thousands of test scenarios** quickly (simulated time)
- **Building custom simulation infrastructure** for non-standard I/O patterns

## Prerequisites

- **Rust knowledge**: async/await, futures, traits
- **Tokio familiarity**: Runtime, spawning tasks, channels
- **Distributed systems basics**: CAP theorem, consensus, replication

## Attached Scripts and Code

### Skill Usage Type

**EDUCATIONAL** - Study the examples and patterns, implement fresh in your project. The code examples are reference implementations.

### File Documentation

#### Guide: dst-rust-guide.md
**Purpose**: Main tutorial covering DST concepts, four pillars of determinism, and using existing libraries
**Language**: Markdown with Rust examples
**Usage**: STUDY for understanding, apply patterns to your project

#### Guide: dst-internals-guide.md
**Purpose**: Deep dive into building your own DST framework from scratch
**Language**: Markdown with complete Rust implementations
**Usage**: STUDY to understand internals, copy patterns when building custom infrastructure

#### Example: examples/basic-dst-setup.rs
**Purpose**: Working example of a complete DST test setup
**Language**: Rust
**Usage**: REFERENCE implementation, adapt to your specific needs

## Core Concepts

### The Four Pillars of Determinism

| Pillar | Problem | Solution |
|--------|---------|----------|
| **Execution** | Thread scheduling is non-deterministic | Single-threaded async executor |
| **Entropy** | RNGs use system entropy | Seeded PRNG throughout |
| **Time** | Real clocks advance unpredictably | Simulated clock, manual advancement |
| **I/O** | Network has variable latency/loss | In-memory simulated network |

### Key Libraries

| Library | Purpose | Use When |
|---------|---------|----------|
| **turmoil** | Network simulation for Tokio | Standard TCP/UDP patterns |
| **madsim** | Full libc interception | Need to catch all entropy/time sources |
| **proptest** | Property-based input generation | Generating test scenarios |

## Step-by-Step Guide

### Step 1: Add Dependencies

```toml
[features]
default = []
simulation = ["turmoil"]

[dependencies]
tokio = { version = "1", features = ["full"] }
rand = "0.8"

[dev-dependencies]
turmoil = "0.6"
```

### Step 2: Configure Single-Threaded Runtime

```rust
// For Tokio tests
#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn my_simulation_test() {
    // Time doesn't advance automatically
    // Single thread eliminates scheduler noise
}
```

### Step 3: Basic Turmoil Setup

```rust
use turmoil::Builder;
use std::time::Duration;

#[test]
fn test_with_turmoil() {
    let mut sim = Builder::new()
        .simulation_duration(Duration::from_secs(60))
        .build();

    sim.host("server", || async {
        let listener = turmoil::net::TcpListener::bind("0.0.0.0:8080").await?;
        // Server logic...
        Ok(())
    });

    sim.client("client", async {
        let stream = turmoil::net::TcpStream::connect("server:8080").await?;
        // Client logic...
        Ok(())
    });

    sim.run().unwrap();
}
```

### Step 4: Seed Management

```rust
fn run_with_seed<F>(test_fn: F)
where
    F: FnOnce(u64) + std::panic::UnwindSafe
{
    let seed: u64 = std::env::var("TEST_SEED")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| rand::random());

    println!("TEST_SEED={}", seed);

    let result = std::panic::catch_unwind(|| test_fn(seed));

    if result.is_err() {
        eprintln!("Reproduce with: TEST_SEED={} cargo test", seed);
        std::panic::resume_unwind(result.unwrap_err());
    }
}
```

### Step 5: Fault Injection

```rust
// Network partition
sim.partition("node-a", "node-b");

// Heal partition
sim.repair("node-a", "node-b");

// Latency injection
let mut sim = Builder::new()
    .min_message_latency(Duration::from_millis(100))
    .max_message_latency(Duration::from_millis(500))
    .build();
```

## Common Patterns

### Pattern 1: Deterministic HashMap

```rust
use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};

#[derive(Clone, Default)]
pub struct DeterministicBuildHasher;

impl BuildHasher for DeterministicBuildHasher {
    type Hasher = DeterministicHasher;
    fn build_hasher(&self) -> Self::Hasher {
        DeterministicHasher { state: 14695981039346656037 }
    }
}

type DetHashMap<K, V> = HashMap<K, V, DeterministicBuildHasher>;
```

### Pattern 2: Simulated Time Only

```rust
#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn test_timeout_behavior() {
    let start = tokio::time::Instant::now();

    // This doesn't actually wait
    tokio::time::sleep(Duration::from_secs(3600)).await;

    // But an hour has "passed"
    assert_eq!(start.elapsed(), Duration::from_secs(3600));
}
```

### Pattern 3: Abstract I/O Layer

```rust
#[cfg(not(feature = "simulation"))]
pub use tokio::net::{TcpListener, TcpStream};

#[cfg(feature = "simulation")]
pub use turmoil::net::{TcpListener, TcpStream};
```

## Pitfalls to Avoid

### Pitfall 1: Using std::time instead of tokio::time
```rust
// BAD - bypasses simulated time
std::time::Instant::now()
std::thread::sleep(duration)

// GOOD - uses simulated time
tokio::time::Instant::now()
tokio::time::sleep(duration).await
```

### Pitfall 2: HashMap iteration order dependencies
```rust
// BAD - order is non-deterministic
for (k, v) in hashmap.iter() {
    process_in_order(k, v); // Order affects result
}

// GOOD - use BTreeMap or sort first
let mut items: Vec<_> = hashmap.iter().collect();
items.sort_by_key(|(k, _)| *k);
```

### Pitfall 3: Third-party crate entropy
```rust
// BAD - uuid uses system entropy internally
let id = uuid::Uuid::new_v4();

// GOOD - generate from seeded RNG
let id = uuid::Uuid::from_u128(rng.gen());
```

## Examples

### Example 1: Complete DST Test

```rust
use turmoil::Builder;
use std::time::Duration;
use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;

#[test]
fn test_consensus_under_partition() {
    let seed: u64 = std::env::var("TEST_SEED")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(42);

    println!("TEST_SEED={}", seed);
    let mut rng = StdRng::seed_from_u64(seed);

    let mut sim = Builder::new()
        .simulation_duration(Duration::from_secs(120))
        .build();

    // Create 3-node consensus cluster
    for i in 0..3 {
        let node_name = format!("node-{}", i);
        sim.host(&node_name, || async move {
            // Consensus participant logic
            Ok(())
        });
    }

    // Run for 10 seconds normally
    sim.run_until(Duration::from_secs(10));

    // Random partition
    let victim = rng.gen_range(0..3);
    sim.partition(&format!("node-{}", victim), &format!("node-{}", (victim + 1) % 3));

    // Run during partition
    sim.run_until(Duration::from_secs(30));

    // Heal
    sim.repair(&format!("node-{}", victim), &format!("node-{}", (victim + 1) % 3));

    // Verify recovery
    sim.run().unwrap();
}
```

### Example 2: Determinism Verification

```rust
#[test]
fn verify_determinism() {
    let seed = 12345u64;

    let result1 = run_and_capture(seed);
    let result2 = run_and_capture(seed);

    assert_eq!(result1, result2,
        "Non-determinism detected! Check for unseeded RNG or real time.");
}

fn run_and_capture(seed: u64) -> Vec<Event> {
    let events = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    // Run simulation, capture events
    // Return events for comparison
    events.lock().unwrap().clone()
}
```

## Script Reference

| File | Language | Usage Type | Purpose |
|------|----------|------------|---------|
| dst-rust-guide.md | Markdown | EDUCATIONAL | Main DST concepts and library usage |
| dst-internals-guide.md | Markdown | EDUCATIONAL | Building custom DST infrastructure |
| examples/basic-dst-setup.rs | Rust | EDUCATIONAL | Working reference implementation |

## References

- [TigerBeetle Blog: A Descent Into the Vortex](https://tigerbeetle.com/blog)
- [S2.dev: Deterministic simulation testing for async Rust](https://s2.dev/blog)
- [turmoil GitHub](https://github.com/tokio-rs/turmoil)
- [madsim GitHub](https://github.com/madsim-rs/madsim)
- [FoundationDB Testing Talk](https://www.youtube.com/watch?v=4fFDFbi3toc)

---
*Created: 2026-01-19*
*Last Updated: 2026-01-19*
