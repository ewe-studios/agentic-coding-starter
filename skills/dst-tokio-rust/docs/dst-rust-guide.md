# Deterministic Simulation Testing in Rust

A comprehensive guide to implementing deterministic simulation testing (DST) for distributed systems in Rust.

---

## Table of Contents

1. [What is DST?](#what-is-dst)
2. [The Four Pillars of Determinism](#the-four-pillars-of-determinism)
3. [Setting Up Your First DST](#setting-up-your-first-dst)
4. [Controlling Non-Determinism Sources](#controlling-non-determinism-sources)
5. [Fault Injection & Chaos Testing](#fault-injection--chaos-testing)
6. [CI Integration & Seed Management](#ci-integration--seed-management)
7. [Common Pitfalls & Debugging](#common-pitfalls--debugging)
8. [Rust DST Ecosystem](#rust-dst-ecosystem)
9. [References](#references)

---

## What is DST?

### Definition

**Deterministic Simulation Testing (DST)** is a testing methodology where distributed systems are executed in a fully controlled, reproducible environment. Every source of non-determinism (time, randomness, network, I/O) is replaced with simulated, seedable alternatives, allowing exact reproduction of any test run.

### The Core Insight

> "If you control all sources of non-determinism and use a seed, running the same test twice with the same seed produces identical behavior."

This means that when a test fails in CI, you can reproduce the *exact* failure locally by using the same seed.

### Benefits for Distributed Systems

1. **Reproducible failures**: Every bug can be reproduced exactly, eliminating "works on my machine" problems
2. **Time compression**: Simulate hours/days of operation in seconds (no real waiting)
3. **Exhaustive fault testing**: Inject network partitions, crashes, and latency deterministically
4. **Coverage amplification**: Run thousands of scenarios that would be impractical with real systems
5. **Confidence in correctness**: Test invariants across many simulated executions

### Prior Art: Who Uses DST?

| System | Approach | Result |
|--------|----------|--------|
| **FoundationDB** | Original pioneers of simulation testing | Achieved "no data-losing bugs in production" for years |
| **TigerBeetle** | Multi-layered "defense-in-depth" with Vortex simulation | Mission-critical financial ledger |
| **RisingWave** | MadSim for async Rust simulation | Stream processing database |
| **S2.dev** | mad-turmoil hybrid approach | Storage infrastructure |

---

## The Four Pillars of Determinism

For a test to be deterministic, you must control **all** sources of non-determinism. These fall into four categories:

### 1. Execution - Single-Threaded Control

**Problem**: Thread schedulers are non-deterministic. Running the same multi-threaded code twice may interleave operations differently.

**Solution**: Run all simulation code on a single thread. Use cooperative multitasking (async/await) instead of OS threads.

```rust
// Use single-threaded Tokio runtime
let rt = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap();
```

**Key insight**: Async executors give you deterministic scheduling because *you* control when tasks yield.

### 2. Entropy - Seeded Randomness

**Problem**: Random number generators use system entropy by default, producing different values each run.

**Solution**: Seed all RNGs with known values derived from a test seed.

```rust
use rand::{SeedableRng, rngs::StdRng};

fn create_rng(test_seed: u64) -> StdRng {
    StdRng::seed_from_u64(test_seed)
}
```

**Hidden traps**:
- `HashMap` uses randomized hashing for DOS protection
- Third-party libraries may use their own RNGs
- `getrandom`/`getentropy` syscalls bypass your seeded RNG

### 3. Time - Simulated Clocks

**Problem**: `std::time::Instant::now()` reads the system clock, which advances non-deterministically between runs.

**Solution**: Use simulated time that only advances when you tell it to.

```rust
// Tokio's paused time mode
let rt = tokio::runtime::Builder::new_current_thread()
    .enable_time()
    .start_paused(true)  // Time doesn't advance automatically
    .build()
    .unwrap();

// Advance time manually
tokio::time::advance(Duration::from_secs(60)).await;
```

**Critical**: Use `tokio::time::Instant`, NOT `std::time::Instant`.

### 4. I/O - Simulated Networks and Storage

**Problem**: Real network I/O has variable latency, packet loss, and ordering that differs between runs.

**Solution**: Replace all I/O with in-memory simulations.

```rust
// With turmoil, you simulate a network in-process
use turmoil::Builder;

let mut sim = Builder::new().build();

sim.host("server", || async {
    // Server code runs in simulated network
});

sim.host("client", || async {
    // Client code runs in same simulated network
});

sim.run();  // Deterministic execution
```

---

## Setting Up Your First DST

### Step 1: Cargo.toml Configuration

Use feature flags to switch between real and simulated I/O:

```toml
[package]
name = "my-distributed-system"
version = "0.1.0"
edition = "2021"

[features]
default = []
simulation = ["turmoil", "dep:rand"]

[dependencies]
tokio = { version = "1", features = ["full"] }
rand = { version = "0.8", optional = true }

[dev-dependencies]
turmoil = "0.6"
rand = "0.8"
```

### Step 2: Abstract Your I/O Layer

Create traits that can be implemented by both real and simulated backends:

```rust
// src/io.rs
use std::future::Future;
use std::net::SocketAddr;

pub trait NetworkTransport: Send + Sync + 'static {
    fn bind(&self, addr: SocketAddr) -> impl Future<Output = Result<(), Error>> + Send;
    fn connect(&self, addr: SocketAddr) -> impl Future<Output = Result<Connection, Error>> + Send;
    fn send(&self, conn: &Connection, data: &[u8]) -> impl Future<Output = Result<(), Error>> + Send;
    fn recv(&self, conn: &Connection) -> impl Future<Output = Result<Vec<u8>, Error>> + Send;
}

// Real implementation uses tokio::net
#[cfg(not(feature = "simulation"))]
pub struct TokioTransport;

// Simulated implementation uses turmoil
#[cfg(feature = "simulation")]
pub struct TurmoilTransport;
```

### Step 3: Basic Turmoil Test Setup

```rust
// tests/simulation.rs
use turmoil::Builder;
use std::time::Duration;

#[test]
fn test_client_server_communication() {
    let mut sim = Builder::new()
        .simulation_duration(Duration::from_secs(60))
        .build();

    // Set up server
    sim.host("server", || async {
        let listener = turmoil::net::TcpListener::bind("0.0.0.0:8080").await?;
        let (mut socket, _) = listener.accept().await?;

        let mut buf = [0u8; 1024];
        let n = socket.read(&mut buf).await?;
        assert_eq!(&buf[..n], b"hello");

        socket.write_all(b"world").await?;
        Ok(())
    });

    // Set up client
    sim.client("client", async {
        let mut socket = turmoil::net::TcpStream::connect("server:8080").await?;
        socket.write_all(b"hello").await?;

        let mut buf = [0u8; 1024];
        let n = socket.read(&mut buf).await?;
        assert_eq!(&buf[..n], b"world");
        Ok(())
    });

    // Run deterministically
    sim.run().unwrap();
}
```

### Step 4: Running with Seeds

```rust
#[test]
fn test_with_seed() {
    // Use environment variable or default seed
    let seed: u64 = std::env::var("TEST_SEED")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(42);

    println!("Running with seed: {}", seed);

    let mut sim = Builder::new()
        .tick_duration(Duration::from_millis(1))
        .build();

    // Seed-derived RNG for any randomness needed
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    // Your test logic here, using `rng` for any random decisions

    sim.run().unwrap();
}
```

---

## Controlling Non-Determinism Sources

### Controlling HashMap Randomization

`HashMap` uses randomized hashing by default (DOS protection). This affects iteration order.

**Solution 1: Seeded RandomState**

```rust
use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};

/// A deterministic hasher for simulation testing
#[derive(Clone)]
pub struct DeterministicHasher {
    state: u64,
}

impl Hasher for DeterministicHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.state = self.state.wrapping_mul(31).wrapping_add(*byte as u64);
        }
    }
}

#[derive(Clone)]
pub struct DeterministicBuildHasher;

impl BuildHasher for DeterministicBuildHasher {
    type Hasher = DeterministicHasher;

    fn build_hasher(&self) -> Self::Hasher {
        DeterministicHasher { state: 0 }
    }
}

// Usage
type DetHashMap<K, V> = HashMap<K, V, DeterministicBuildHasher>;

fn create_map<K, V>() -> DetHashMap<K, V> {
    HashMap::with_hasher(DeterministicBuildHasher)
}
```

**Solution 2: BTreeMap (ordered, deterministic)**

```rust
use std::collections::BTreeMap;

// BTreeMap iteration order is deterministic (sorted by key)
let mut map: BTreeMap<String, i32> = BTreeMap::new();
```

### Controlling Time

**Never use these in simulation code:**
```rust
// BAD - non-deterministic
std::time::Instant::now()
std::time::SystemTime::now()
std::thread::sleep(duration)
```

**Always use these instead:**
```rust
// GOOD - deterministic with paused time
tokio::time::Instant::now()
tokio::time::sleep(duration).await
tokio::time::timeout(duration, future).await
```

**Setup for paused time:**
```rust
#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn test_with_simulated_time() {
    let start = tokio::time::Instant::now();

    // This doesn't actually wait - time is paused
    tokio::time::sleep(Duration::from_secs(3600)).await;

    // An hour has "passed" in simulated time
    assert_eq!(start.elapsed(), Duration::from_secs(3600));

    // But real time elapsed is milliseconds
}
```

### Intercepting libc Calls (Advanced)

For libraries that call `getrandom()` or `clock_gettime()` directly:

**Using MadSim's approach:**

```rust
// madsim intercepts these at the libc level
// In Cargo.toml:
// [target.'cfg(madsim)'.dependencies]
// madsim = "0.2"

#[cfg(madsim)]
use madsim::rand::random;

#[cfg(not(madsim))]
use rand::random;
```

**Using mad-turmoil (S2's hybrid):**

mad-turmoil combines turmoil's network simulation with madsim's syscall interception:

```toml
[dev-dependencies]
mad-turmoil = "0.1"  # Hypothetical - check actual crate name
```

### Feature Flag Pattern

```rust
// src/time.rs

#[cfg(not(feature = "simulation"))]
pub use std::time::Instant;

#[cfg(feature = "simulation")]
pub use tokio::time::Instant;

// src/rand.rs

#[cfg(not(feature = "simulation"))]
pub fn random_bytes(buf: &mut [u8]) {
    getrandom::getrandom(buf).unwrap();
}

#[cfg(feature = "simulation")]
pub fn random_bytes(buf: &mut [u8]) {
    // Use thread-local seeded RNG
    SIMULATION_RNG.with(|rng| {
        rng.borrow_mut().fill_bytes(buf);
    });
}
```

---

## Fault Injection & Chaos Testing

### Network Partitions

```rust
use turmoil::Builder;

#[test]
fn test_survives_partition() {
    let mut sim = Builder::new().build();

    sim.host("node-a", || async { /* ... */ });
    sim.host("node-b", || async { /* ... */ });
    sim.host("node-c", || async { /* ... */ });

    // Run for a bit normally
    sim.run_until(|| async {
        tokio::time::sleep(Duration::from_secs(5)).await;
    });

    // Partition node-a from others
    sim.partition("node-a", "node-b");
    sim.partition("node-a", "node-c");

    // Run during partition
    sim.run_until(|| async {
        tokio::time::sleep(Duration::from_secs(10)).await;
    });

    // Heal partition
    sim.repair("node-a", "node-b");
    sim.repair("node-a", "node-c");

    // Verify system recovers
    sim.run().unwrap();
}
```

### Latency Injection

```rust
use turmoil::{Builder, ToIpAddr};
use std::time::Duration;

#[test]
fn test_handles_slow_network() {
    let mut sim = Builder::new()
        // Add 100ms latency to all links
        .min_message_latency(Duration::from_millis(100))
        .max_message_latency(Duration::from_millis(200))
        .build();

    // Test that timeouts and retries work correctly
    sim.host("server", || async { /* ... */ });
    sim.client("client", async { /* ... */ });

    sim.run().unwrap();
}
```

### Process Crashes and Restarts

```rust
#[test]
fn test_crash_recovery() {
    let mut sim = Builder::new().build();

    // Track server state externally (simulates persistent storage)
    let state = Arc::new(Mutex::new(Vec::new()));
    let state_clone = state.clone();

    sim.host("server", move || {
        let state = state_clone.clone();
        async move {
            // Recover state on startup
            let recovered = state.lock().unwrap().clone();

            // Server logic...

            Ok(())
        }
    });

    // Run for a bit
    sim.run_until(|| async {
        tokio::time::sleep(Duration::from_secs(5)).await;
    });

    // Crash the server
    sim.crash("server");

    // Restart it
    let state_clone = state.clone();
    sim.host("server", move || {
        let state = state_clone.clone();
        async move {
            // Should recover from crash
            Ok(())
        }
    });

    sim.run().unwrap();
}
```

### Systematic Fault Exploration

```rust
#[test]
fn test_all_partition_scenarios() {
    let nodes = vec!["a", "b", "c"];

    // Test all possible 2-node partitions
    for i in 0..nodes.len() {
        for j in (i+1)..nodes.len() {
            let mut sim = Builder::new().build();

            for node in &nodes {
                sim.host(*node, || async { /* ... */ });
            }

            // Create specific partition
            sim.partition(nodes[i], nodes[j]);

            // Verify invariants hold
            sim.run().unwrap();
        }
    }
}
```

---

## CI Integration & Seed Management

### Capturing Seeds on Failure

```rust
use std::panic;

fn run_simulation_test<F>(test_fn: F)
where
    F: FnOnce(u64) + panic::UnwindSafe
{
    let seed: u64 = std::env::var("TEST_SEED")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| rand::random());

    println!("=== Running with seed: {} ===", seed);

    let result = panic::catch_unwind(|| {
        test_fn(seed);
    });

    if result.is_err() {
        eprintln!("=== TEST FAILED ===");
        eprintln!("To reproduce: TEST_SEED={} cargo test", seed);
        panic::resume_unwind(result.unwrap_err());
    }
}

#[test]
fn my_simulation_test() {
    run_simulation_test(|seed| {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut sim = Builder::new().build();

        // Test logic using seeded rng...

        sim.run().unwrap();
    });
}
```

### CI Configuration

```yaml
# .github/workflows/simulation.yml
name: Simulation Tests

on: [push, pull_request]

jobs:
  simulation:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        seed: [1, 2, 3, 4, 5]  # Run with multiple seeds
    steps:
      - uses: actions/checkout@v4

      - name: Run simulation tests
        env:
          TEST_SEED: ${{ matrix.seed }}
        run: cargo test --features simulation --release

      - name: Upload failure artifacts
        if: failure()
        uses: actions/upload-artifact@v4
        with:
          name: simulation-failure-seed-${{ matrix.seed }}
          path: target/simulation-logs/
```

### Meta-Test: Verifying Determinism

```rust
/// This test verifies that our simulation is actually deterministic
#[test]
fn test_determinism() {
    let seed = 12345u64;

    // Run simulation twice with same seed
    let result1 = run_and_capture(seed);
    let result2 = run_and_capture(seed);

    // Results must be identical
    assert_eq!(result1, result2,
        "Simulation is non-deterministic! Check for unseeded RNG or real time usage.");
}

fn run_and_capture(seed: u64) -> Vec<Event> {
    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = events.clone();

    let mut sim = Builder::new().build();

    sim.host("node", move || {
        let events = events_clone.clone();
        async move {
            // Record all significant events
            events.lock().unwrap().push(Event::Started);
            // ... more logic
            Ok(())
        }
    });

    sim.run().unwrap();

    Arc::try_unwrap(events).unwrap().into_inner().unwrap()
}
```

---

## Common Pitfalls & Debugging

### Identifying Non-Determinism Sources

**Symptom**: Test passes sometimes, fails others, or results differ between runs with same seed.

**Debugging checklist:**

1. **HashMap iteration**
   ```rust
   // Look for any .iter() on HashMap where order matters
   for (k, v) in hashmap.iter() {  // Non-deterministic order!
       process(k, v);
   }

   // Fix: Use BTreeMap or collect and sort
   let mut entries: Vec<_> = hashmap.iter().collect();
   entries.sort_by_key(|(k, _)| *k);
   ```

2. **Thread IDs / PIDs**
   ```rust
   // These are non-deterministic
   std::thread::current().id()
   std::process::id()
   ```

3. **System time in logs/packets**
   ```rust
   // Look for chrono or time crate usage
   chrono::Utc::now()  // Non-deterministic!
   ```

4. **Floating point ordering**
   ```rust
   // f64::NAN comparisons and HashMap with f64 keys cause issues
   ```

### Third-Party Library Audit

Some crates use RNG internally:

| Crate | Issue | Mitigation |
|-------|-------|------------|
| `uuid` | Generates random UUIDs | Use `uuid::Uuid::from_u128(seeded_value)` |
| `tempfile` | Random temp paths | Use deterministic paths in tests |
| `hyper` | Connection pool randomness | May need custom connectors |

### Platform-Specific Issues

```rust
// Path separators differ
#[cfg(windows)]
const SEP: char = '\\';
#[cfg(not(windows))]
const SEP: char = '/';

// Endianness in serialization
// Always use explicit endianness
buf.write_u32::<LittleEndian>(value)?;  // Not native!
```

### Debugging Strategy

1. **Add event logging**
   ```rust
   #[derive(Debug, Clone, PartialEq)]
   enum Event {
       MessageSent { from: String, to: String, seq: u64 },
       MessageReceived { from: String, to: String, seq: u64 },
       StateChange { node: String, old: State, new: State },
   }
   ```

2. **Compare event logs between runs**
   ```bash
   TEST_SEED=123 cargo test 2>&1 | tee run1.log
   TEST_SEED=123 cargo test 2>&1 | tee run2.log
   diff run1.log run2.log
   ```

3. **Binary search for non-determinism**
   - Comment out half your code
   - If still non-deterministic, bug is in remaining half
   - Repeat until found

---

## Rust DST Ecosystem

### turmoil

**Purpose**: Simulated networking for Tokio applications.

**Strengths**:
- Drop-in replacement for `tokio::net`
- Built-in fault injection (partitions, latency)
- Active development, good documentation

**Limitations**:
- Only handles networking, not time or RNG
- Requires Tokio (won't work with async-std)

**Repo**: https://github.com/tokio-rs/turmoil

### madsim

**Purpose**: Full simulation runtime with libc interception.

**Strengths**:
- Intercepts `getrandom`, `clock_gettime` at libc level
- Comprehensive - handles time, RNG, and network
- Used by RisingWave in production

**Limitations**:
- Requires compiling with special cfg flags
- May conflict with other libc wrappers
- Linux-focused

**Repo**: https://github.com/madsim-rs/madsim

### mad-turmoil (Hybrid)

**Purpose**: Combines turmoil's network simulation with madsim's syscall interception.

**Strengths**:
- Best of both worlds
- Catches non-determinism from libraries using libc directly

**Repo**: Part of S2.dev's stack (check their GitHub)

### proptest

**Purpose**: Property-based testing with shrinking.

**Complementary to DST**: Use proptest to generate inputs, DST to simulate execution.

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_consensus_with_random_faults(
        seed in any::<u64>(),
        num_nodes in 3..10usize,
        faults in prop::collection::vec(any::<Fault>(), 0..5),
    ) {
        let mut sim = create_simulation(seed, num_nodes);

        for fault in faults {
            sim.inject_fault(fault);
        }

        sim.run().unwrap();
        assert!(sim.verify_consensus());
    }
}
```

---

## Testing Strategy

### Defense-in-Depth (TigerBeetle Approach)

Don't rely on DST alone. Layer multiple testing strategies:

| Layer | Purpose | Example |
|-------|---------|---------|
| **Unit tests** | Verify individual functions | Test message serialization |
| **Property tests** | Find edge cases via random inputs | Proptest on state machine |
| **DST** | Verify distributed invariants | Consensus despite partitions |
| **Integration tests** | Real network, limited scenarios | Docker compose tests |
| **Chaos engineering** | Production-like failures | Kubernetes pod killing |

### When to Use DST vs Traditional Tests

**Use DST when**:
- Testing distributed system invariants
- Verifying behavior under network failures
- Need to test many scenarios quickly
- Debugging hard-to-reproduce issues

**Use traditional tests when**:
- Testing pure functions
- Verifying API contracts
- Testing third-party integrations (use mocks)
- Performance benchmarking (DST has overhead)

### DST Test Organization

```
tests/
├── unit/           # Fast, no simulation
│   └── *.rs
├── simulation/     # DST tests
│   ├── mod.rs
│   ├── consensus_test.rs
│   ├── replication_test.rs
│   └── partition_test.rs
└── integration/    # Real network tests
    └── *.rs
```

---

## References

### Essential Reading

- **TigerBeetle Blog**: "A Descent Into the Vortex" - [link]
  - Defense-in-depth testing philosophy
  - Vortex simulation architecture

- **S2.dev Blog**: "Deterministic simulation testing for async Rust" - [link]
  - Practical Tokio/turmoil implementation
  - mad-turmoil hybrid approach

- **FoundationDB Talk**: "Testing Distributed Systems w/ Deterministic Simulation"
  - Origin of the DST concept
  - Why FoundationDB never lost data

### Libraries and Tools

| Library | Purpose | Link |
|---------|---------|------|
| turmoil | Network simulation for Tokio | https://github.com/tokio-rs/turmoil |
| madsim | Full libc-intercepting simulation | https://github.com/madsim-rs/madsim |
| proptest | Property-based testing | https://github.com/proptest-rs/proptest |
| loom | Concurrency testing (different approach) | https://github.com/tokio-rs/loom |

### Systems Using DST

- **FoundationDB** (Apple) - Key-value store
- **TigerBeetle** - Financial ledger
- **RisingWave** - Stream processing
- **S2.dev** - Storage infrastructure

---

*Created: 2026-01-19*
*Last Updated: 2026-01-19*
