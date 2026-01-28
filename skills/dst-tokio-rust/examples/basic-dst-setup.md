# Deterministic Simulation Testing - Basic Setup

## Purpose

This guide demonstrates a complete DST (Deterministic Simulation Testing) setup using turmoil for testing distributed Rust systems. You'll learn how to create reproducible tests that can reliably expose race conditions and network issues.

## Prerequisites

- Rust with tokio
- Understanding of async/await
- Basic distributed systems concepts

## Key Concepts

1. **Seed Management** - Reproducible test runs
2. **Deterministic Hashing** - Consistent HashMap behavior
3. **Simulated Networks** - In-memory TCP simulation
4. **Fault Injection** - Network partitions and latency
5. **Event Capture** - Verifying determinism

---

## Setup

### Cargo.toml Configuration

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

### Running Tests

```bash
# With random seed
cargo test --features simulation

# With specific seed for reproduction
TEST_SEED=12345 cargo test --features simulation
```

---

## 1. Deterministic HashMap

Standard Rust HashMaps use randomized hashing for security. In DST, we need deterministic hashing:

```rust
use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};

/// A hasher that produces deterministic results (no randomization)
#[derive(Clone, Default)]
pub struct DeterministicHasher {
    state: u64,
}

impl Hasher for DeterministicHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        // FNV-1a algorithm (deterministic)
        const FNV_PRIME: u64 = 1099511628211;
        for byte in bytes {
            self.state ^= *byte as u64;
            self.state = self.state.wrapping_mul(FNV_PRIME);
        }
    }
}

/// BuildHasher that creates DeterministicHasher instances
#[derive(Clone, Default)]
pub struct DeterministicBuildHasher;

impl BuildHasher for DeterministicBuildHasher {
    type Hasher = DeterministicHasher;

    fn build_hasher(&self) -> Self::Hasher {
        // FNV-1a offset basis
        DeterministicHasher {
            state: 14695981039346656037,
        }
    }
}

/// Type alias for deterministic HashMap
pub type DetHashMap<K, V> = HashMap<K, V, DeterministicBuildHasher>;

/// Create a new deterministic HashMap
pub fn det_hash_map<K, V>() -> DetHashMap<K, V> {
    HashMap::with_hasher(DeterministicBuildHasher)
}
```

**Why this matters:** Regular HashMaps have non-deterministic iteration order, which can cause flaky tests when order matters.

---

## 2. Test Seed Management

Manage seeds to make failures reproducible:

```rust
/// Get test seed from environment or generate random one
fn get_test_seed() -> u64 {
    std::env::var("TEST_SEED")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| rand::random())
}

/// Run a test with seed management and failure reproduction instructions
fn run_simulation_test<F>(test_name: &str, test_fn: F)
where
    F: FnOnce(u64) + std::panic::UnwindSafe,
{
    let seed = get_test_seed();
    println!("=== {} ===", test_name);
    println!("TEST_SEED={}", seed);

    let result = std::panic::catch_unwind(|| {
        test_fn(seed);
    });

    if result.is_err() {
        eprintln!("\n=== TEST FAILED ===");
        eprintln!("To reproduce:");
        eprintln!("  TEST_SEED={} cargo test {} --features simulation",
            seed, test_name);
        std::panic::resume_unwind(result.unwrap_err());
    }
}
```

**Usage:**

```rust
#[test]
fn test_my_feature() {
    run_simulation_test("test_my_feature", |seed| {
        let mut rng = StdRng::seed_from_u64(seed);
        // Test logic using seeded RNG...
    });
}
```

---

## 3. Simple Echo Server Test

Basic turmoil test demonstrating client-server communication:

```rust
use turmoil::Builder;
use std::time::Duration;

#[test]
fn test_echo_server() {
    run_simulation_test("test_echo_server", |_seed| {
        let mut sim = Builder::new()
            .simulation_duration(Duration::from_secs(60))
            .build();

        // Echo server
        sim.host("server", || async {
            let listener = turmoil::net::TcpListener::bind("0.0.0.0:8080").await?;
            println!("[server] Listening on :8080");

            let (mut socket, addr) = listener.accept().await?;
            println!("[server] Connection from {}", addr);

            let mut buf = [0u8; 1024];
            loop {
                let n = socket.read(&mut buf).await?;
                if n == 0 {
                    break;
                }
                socket.write_all(&buf[..n]).await?;
            }

            Ok(())
        });

        // Client
        sim.client("client", async {
            // Small delay to ensure server is ready
            tokio::time::sleep(Duration::from_millis(10)).await;

            let mut socket = turmoil::net::TcpStream::connect("server:8080").await?;
            println!("[client] Connected to server");

            socket.write_all(b"hello world").await?;

            let mut buf = [0u8; 1024];
            let n = socket.read(&mut buf).await?;

            assert_eq!(&buf[..n], b"hello world", "Echo mismatch!");
            println!("[client] Echo verified!");

            Ok(())
        });

        sim.run().expect("Simulation failed");
    });
}
```

**Key points:**
- Server and client run as separate "hosts" in the simulation
- Network communication is in-memory but behaves like real TCP
- Time advances in simulation only (no actual waiting)

---

## 4. Network Partition Test

Test a distributed system's behavior under network partitions:

```rust
use std::sync::{Arc, Mutex};

/// Shared state for tracking node status
#[derive(Debug, Clone, Default)]
struct ClusterState {
    heartbeats: DetHashMap<String, u64>,
    leader: Option<String>,
}

#[test]
fn test_survives_partition() {
    run_simulation_test("test_survives_partition", |seed| {
        let mut rng = StdRng::seed_from_u64(seed);
        let state = Arc::new(Mutex::new(ClusterState::default()));

        let mut sim = Builder::new()
            .simulation_duration(Duration::from_secs(120))
            .min_message_latency(Duration::from_millis(1))
            .max_message_latency(Duration::from_millis(50))
            .build();

        // Create 3-node cluster
        let nodes = vec!["node-a", "node-b", "node-c"];

        for node_name in &nodes {
            let name = node_name.to_string();
            let state = state.clone();

            sim.host(node_name, move || {
                let name = name.clone();
                let state = state.clone();

                async move {
                    // Simple heartbeat loop
                    let mut tick = 0u64;
                    loop {
                        tokio::time::sleep(Duration::from_millis(100)).await;
                        tick += 1;

                        // Record heartbeat
                        {
                            let mut s = state.lock().unwrap();
                            s.heartbeats.insert(name.clone(), tick);
                        }

                        if tick % 10 == 0 {
                            println!("[{}] Heartbeat #{}", name, tick);
                        }

                        if tick > 500 {
                            break;
                        }
                    }

                    Ok(())
                }
            });
        }

        // Run for 2 seconds normally
        for _ in 0..20 {
            sim.step().unwrap();
        }

        // Partition a random node
        let victim_idx = rng.gen_range(0..nodes.len());
        let victim = nodes[victim_idx];
        let other = nodes[(victim_idx + 1) % nodes.len()];

        println!("\n=== PARTITIONING {} from {} ===\n", victim, other);
        sim.partition(victim, other);

        // Run during partition
        for _ in 0..50 {
            sim.step().unwrap();
        }

        // Heal partition
        println!("\n=== HEALING PARTITION ===\n");
        sim.repair(victim, other);

        // Run to completion
        sim.run().expect("Simulation failed after partition");

        // Verify all nodes made progress
        let final_state = state.lock().unwrap();
        for node in &nodes {
            let heartbeats = final_state.heartbeats.get(*node).copied().unwrap_or(0);
            assert!(
                heartbeats > 100,
                "{} should have made progress (got {} heartbeats)",
                node,
                heartbeats
            );
        }
    });
}
```

**Key techniques:**
- Seeded RNG for reproducible random partition selection
- Deterministic HashMap for consistent state tracking
- Step-by-step simulation control
- Partition injection and healing

---

## 5. Determinism Verification

Verify that same seed produces identical behavior:

```rust
#[derive(Debug, Clone, PartialEq)]
enum Event {
    Started { node: String, time_ms: u64 },
    MessageSent { from: String, to: String, seq: u64 },
    MessageReceived { from: String, to: String, seq: u64 },
    Completed { node: String, time_ms: u64 },
}

#[test]
fn test_determinism_verification() {
    let seed = 42u64;

    println!("Running simulation twice with seed {}", seed);

    let result1 = run_and_capture_events(seed);
    let result2 = run_and_capture_events(seed);

    assert_eq!(
        result1.len(),
        result2.len(),
        "Event count mismatch: {} vs {}",
        result1.len(),
        result2.len()
    );

    for (i, (e1, e2)) in result1.iter().zip(result2.iter()).enumerate() {
        assert_eq!(
            e1, e2,
            "Event mismatch at index {}:\n  Run 1: {:?}\n  Run 2: {:?}",
            i, e1, e2
        );
    }

    println!("Determinism verified! {} events matched.", result1.len());
}

fn run_and_capture_events(seed: u64) -> Vec<Event> {
    let events = Arc::new(Mutex::new(Vec::new()));

    let mut sim = Builder::new()
        .simulation_duration(Duration::from_secs(10))
        .build();

    // Set up client and server that record events
    // (Full implementation omitted for brevity)

    sim.run().expect("Simulation failed");

    Arc::try_unwrap(events).unwrap().into_inner().unwrap()
}
```

**Verification strategy:**
1. Run simulation twice with same seed
2. Capture all events in both runs
3. Compare events element-by-element
4. Any difference indicates non-determinism

---

## 6. Simulated Time (Tokio-Only)

Test time-dependent code without actual waiting:

```rust
#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn test_simulated_time() {
    let start = tokio::time::Instant::now();

    // This doesn't actually wait 1 hour - time is simulated
    tokio::time::sleep(Duration::from_secs(3600)).await;

    let elapsed = start.elapsed();
    assert_eq!(elapsed, Duration::from_secs(3600));

    println!("Simulated 1 hour of time instantly!");
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn test_timeout_behavior() {
    use tokio::time::timeout;

    // Test that a slow operation times out
    let slow_op = async {
        tokio::time::sleep(Duration::from_secs(10)).await;
        42
    };

    let result = timeout(Duration::from_secs(5), slow_op).await;

    assert!(result.is_err(), "Should have timed out");
    println!("Timeout behavior verified!");
}
```

**Benefits:**
- Tests run instantly (simulated time)
- Fully deterministic
- No flaky timeouts
- Easy to test multi-hour scenarios

---

## 7. Seeded Random Sequences

Verify determinism of random number generation:

```rust
#[test]
fn test_seeded_random_determinism() {
    let seed = 12345u64;

    // Run twice with same seed
    let values1 = generate_random_sequence(seed, 10);
    let values2 = generate_random_sequence(seed, 10);

    assert_eq!(values1, values2, "Random sequences should match with same seed");
    println!("Values: {:?}", values1);

    // Different seed should give different values
    let values3 = generate_random_sequence(seed + 1, 10);
    assert_ne!(values1, values3, "Different seeds should give different values");
}

fn generate_random_sequence(seed: u64, count: usize) -> Vec<u64> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..count).map(|_| rng.gen()).collect()
}
```

---

## 8. Fault Scenario Generator

Generate complex fault scenarios deterministically:

```rust
#[derive(Debug, Clone)]
enum Fault {
    Partition { node_a: String, node_b: String },
    Repair { node_a: String, node_b: String },
    Latency { node: String, delay_ms: u64 },
}

struct FaultScenarioGenerator {
    rng: StdRng,
    nodes: Vec<String>,
}

impl FaultScenarioGenerator {
    fn new(seed: u64, nodes: Vec<String>) -> Self {
        FaultScenarioGenerator {
            rng: StdRng::seed_from_u64(seed),
            nodes,
        }
    }

    fn generate_scenario(&mut self, num_faults: usize) -> Vec<(Duration, Fault)> {
        let mut scenario = Vec::new();
        let mut time = Duration::from_secs(5);

        for _ in 0..num_faults {
            let fault = self.generate_fault();
            scenario.push((time, fault));
            time += Duration::from_secs(self.rng.gen_range(2..10));
        }

        scenario
    }

    fn generate_fault(&mut self) -> Fault {
        let fault_type = self.rng.gen_range(0..3);

        match fault_type {
            0 => {
                let i = self.rng.gen_range(0..self.nodes.len());
                let j = (i + 1) % self.nodes.len();
                Fault::Partition {
                    node_a: self.nodes[i].clone(),
                    node_b: self.nodes[j].clone(),
                }
            }
            1 => {
                let i = self.rng.gen_range(0..self.nodes.len());
                let j = (i + 1) % self.nodes.len();
                Fault::Repair {
                    node_a: self.nodes[i].clone(),
                    node_b: self.nodes[j].clone(),
                }
            }
            _ => {
                let i = self.rng.gen_range(0..self.nodes.len());
                Fault::Latency {
                    node: self.nodes[i].clone(),
                    delay_ms: self.rng.gen_range(100..1000),
                }
            }
        }
    }
}

#[test]
fn test_generated_fault_scenarios() {
    let seed = get_test_seed();
    println!("Generating fault scenario with seed {}", seed);

    let nodes = vec!["alpha".into(), "beta".into(), "gamma".into()];
    let mut generator = FaultScenarioGenerator::new(seed, nodes.clone());

    let scenario = generator.generate_scenario(5);

    for (time, fault) in &scenario {
        println!("At {:?}: {:?}", time, fault);
    }

    // Verify determinism
    let mut generator2 = FaultScenarioGenerator::new(seed, nodes);
    let scenario2 = generator2.generate_scenario(5);

    for ((t1, f1), (t2, f2)) in scenario.iter().zip(scenario2.iter()) {
        assert_eq!(t1, t2);
        assert_eq!(format!("{:?}", f1), format!("{:?}", f2));
    }

    println!("Fault scenario generation is deterministic!");
}
```

---

## Summary

This guide covered:

1. ✅ **Deterministic HashMap** - Consistent iteration order
2. ✅ **Seed Management** - Reproducible test runs
3. ✅ **Echo Server Test** - Basic turmoil usage
4. ✅ **Network Partitions** - Fault injection
5. ✅ **Determinism Verification** - Event comparison
6. ✅ **Simulated Time** - Fast time-dependent tests
7. ✅ **Seeded RNG** - Reproducible randomness
8. ✅ **Fault Scenarios** - Complex deterministic scenarios

## Key Takeaways

- **Always use seeds** - Makes failures reproducible
- **Avoid real time** - Use tokio::time, not std::time
- **Deterministic data structures** - Custom HashMap when order matters
- **Capture events** - Verify identical behavior across runs
- **Test failure modes** - Partitions, latency, message loss

## References

- [Turmoil Documentation](https://docs.rs/turmoil)
- [Tokio Time Testing](https://docs.rs/tokio/latest/tokio/time/index.html)
- [TigerBeetle DST Blog Post](https://tigerbeetle.com/blog)

---

*This guide is part of the DST Tokio Rust skill*
*Last Updated: 2026-01-28*
