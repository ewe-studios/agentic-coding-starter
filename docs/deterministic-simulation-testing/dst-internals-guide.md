# Building Your Own DST Framework in Rust

A deep dive into implementing deterministic simulation testing infrastructure from scratch, understanding how turmoil, madsim, and similar tools work internally.

---

## Table of Contents

1. [Why Build Your Own?](#why-build-your-own)
2. [Architecture Overview](#architecture-overview)
3. [Building a Simulation Runtime](#building-a-simulation-runtime)
4. [Implementing Simulated Time](#implementing-simulated-time)
5. [Building a Simulated Network](#building-a-simulated-network)
6. [Controlling Entropy](#controlling-entropy)
7. [The Simulation Executor](#the-simulation-executor)
8. [Fault Injection System](#fault-injection-system)
9. [libc Interception (Advanced)](#libc-interception-advanced)
10. [Putting It All Together](#putting-it-all-together)

---

## Why Build Your Own?

### When to Use Existing Libraries

| Use turmoil/madsim when... | Build your own when... |
|---------------------------|------------------------|
| Standard networking patterns | Custom I/O abstractions |
| Tokio-based systems | Non-Tokio async runtimes |
| Quick setup needed | Learning/education |
| Active maintenance required | Custom fault models |

### What You'll Learn

Building your own DST framework teaches:
- Deep understanding of async execution
- How time simulation works
- Network abstraction patterns
- The mechanics of determinism

---

## Architecture Overview

A DST framework consists of these core components:

```
┌────────────────────────────────────────────────────────┐
│                    Simulation Runtime                   │
├─────────────┬─────────────┬─────────────┬─────────────┤
│  Simulated  │  Simulated  │   Seeded    │   Fault     │
│    Time     │   Network   │    RNG      │  Injector   │
├─────────────┴─────────────┴─────────────┴─────────────┤
│              Single-Threaded Executor                   │
├───────────────────────────────────────────────────────┤
│                    Your Application                     │
└────────────────────────────────────────────────────────┘
```

### Key Insight

The simulation runtime intercepts all non-deterministic operations:
- **Time queries** → return simulated clock
- **Sleep/timeout** → register with time wheel, yield
- **Network I/O** → route through simulated network
- **Random** → use seeded PRNG

---

## Building a Simulation Runtime

### Core Types

```rust
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::Reverse;
use std::time::Duration;

/// Unique identifier for each simulated node
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NodeId(pub u64);

/// Simulated time in nanoseconds since simulation start
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct SimTime(pub u64);

impl SimTime {
    pub fn from_duration(d: Duration) -> Self {
        SimTime(d.as_nanos() as u64)
    }

    pub fn as_duration(&self) -> Duration {
        Duration::from_nanos(self.0)
    }

    pub fn add(&self, d: Duration) -> Self {
        SimTime(self.0 + d.as_nanos() as u64)
    }
}

/// The main simulation state
pub struct Simulation {
    /// Current simulated time
    current_time: SimTime,

    /// Nodes in the simulation
    nodes: HashMap<NodeId, Node>,

    /// Pending timer events (min-heap by wake time)
    timers: BinaryHeap<Reverse<TimerEntry>>,

    /// Network simulation
    network: SimulatedNetwork,

    /// Global seeded RNG
    rng: rand::rngs::StdRng,

    /// Next node ID to assign
    next_node_id: u64,
}

struct Node {
    id: NodeId,
    name: String,
    /// Tasks belonging to this node
    tasks: VecDeque<Box<dyn FnMut(&mut SimContext) -> Poll<()>>>,
    /// Node-local RNG (derived from global seed + node id)
    rng: rand::rngs::StdRng,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct TimerEntry {
    wake_time: SimTime,
    node_id: NodeId,
    task_id: u64,
}
```

### Simulation Context

Each node's code receives a context for interacting with the simulation:

```rust
use std::task::Poll;

/// Context passed to node code, providing simulation primitives
pub struct SimContext<'a> {
    /// Current node's ID
    pub node_id: NodeId,

    /// Access to simulated time
    time: &'a SimTime,

    /// Timer registration
    timers: &'a mut BinaryHeap<Reverse<TimerEntry>>,

    /// Network handle
    network: &'a mut SimulatedNetwork,

    /// Node-local RNG
    rng: &'a mut rand::rngs::StdRng,
}

impl<'a> SimContext<'a> {
    /// Get current simulated time
    pub fn now(&self) -> SimTime {
        *self.time
    }

    /// Schedule a timer (like tokio::time::sleep)
    pub fn sleep(&mut self, duration: Duration) -> SleepFuture {
        let wake_time = self.time.add(duration);
        SleepFuture { wake_time }
    }

    /// Get a random value using the seeded RNG
    pub fn random<T>(&mut self) -> T
    where
        rand::distributions::Standard: rand::distributions::Distribution<T>,
    {
        use rand::Rng;
        self.rng.gen()
    }
}
```

---

## Implementing Simulated Time

### The Time Wheel

Real systems use time wheels for efficient timer management. Here's a simplified version:

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

/// A timer wheel for managing scheduled wakeups
pub struct TimeWheel {
    /// Min-heap of pending timers
    timers: BinaryHeap<Reverse<Timer>>,

    /// Current simulated time
    now: SimTime,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Timer {
    deadline: SimTime,
    id: TimerId,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TimerId(u64);

impl TimeWheel {
    pub fn new() -> Self {
        TimeWheel {
            timers: BinaryHeap::new(),
            now: SimTime(0),
        }
    }

    /// Register a timer, returns ID for cancellation
    pub fn schedule(&mut self, delay: Duration) -> TimerId {
        static NEXT_ID: std::sync::atomic::AtomicU64 =
            std::sync::atomic::AtomicU64::new(0);

        let id = TimerId(NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed));
        let deadline = self.now.add(delay);

        self.timers.push(Reverse(Timer { deadline, id }));
        id
    }

    /// Get next timer deadline (if any)
    pub fn next_deadline(&self) -> Option<SimTime> {
        self.timers.peek().map(|Reverse(t)| t.deadline)
    }

    /// Advance time to next deadline, return expired timer IDs
    pub fn advance_to_next(&mut self) -> Vec<TimerId> {
        let mut expired = Vec::new();

        if let Some(next_time) = self.next_deadline() {
            self.now = next_time;

            // Pop all timers at this exact time
            while let Some(Reverse(timer)) = self.timers.peek() {
                if timer.deadline == self.now {
                    let Reverse(timer) = self.timers.pop().unwrap();
                    expired.push(timer.id);
                } else {
                    break;
                }
            }
        }

        expired
    }

    /// Advance time by a specific amount
    pub fn advance_by(&mut self, duration: Duration) -> Vec<TimerId> {
        let target = self.now.add(duration);
        let mut all_expired = Vec::new();

        while let Some(next) = self.next_deadline() {
            if next <= target {
                all_expired.extend(self.advance_to_next());
            } else {
                break;
            }
        }

        self.now = target;
        all_expired
    }

    pub fn now(&self) -> SimTime {
        self.now
    }
}
```

### Sleep Future Implementation

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::cell::RefCell;
use std::rc::Rc;

/// Thread-local simulation state for the current node
thread_local! {
    static SIM_STATE: RefCell<Option<Rc<RefCell<NodeSimState>>>> = RefCell::new(None);
}

struct NodeSimState {
    time_wheel: TimeWheel,
    pending_wakers: HashMap<TimerId, Waker>,
}

pub struct SleepFuture {
    timer_id: Option<TimerId>,
    deadline: SimTime,
}

impl SleepFuture {
    pub fn new(duration: Duration) -> Self {
        SIM_STATE.with(|state| {
            let state = state.borrow();
            let state = state.as_ref().expect("not in simulation context");
            let mut state = state.borrow_mut();

            let timer_id = state.time_wheel.schedule(duration);
            let deadline = state.time_wheel.now().add(duration);

            SleepFuture {
                timer_id: Some(timer_id),
                deadline,
            }
        })
    }
}

impl Future for SleepFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        SIM_STATE.with(|state| {
            let state = state.borrow();
            let state = state.as_ref().expect("not in simulation context");
            let mut state = state.borrow_mut();

            if state.time_wheel.now() >= self.deadline {
                Poll::Ready(())
            } else {
                // Register waker so executor knows to poll us after timer fires
                if let Some(timer_id) = self.timer_id {
                    state.pending_wakers.insert(timer_id, cx.waker().clone());
                }
                Poll::Pending
            }
        })
    }
}

/// Simulated sleep function (replaces tokio::time::sleep)
pub async fn sleep(duration: Duration) {
    SleepFuture::new(duration).await
}
```

### Instant Replacement

```rust
/// Simulated Instant (replaces std::time::Instant)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Instant(SimTime);

impl Instant {
    pub fn now() -> Self {
        SIM_STATE.with(|state| {
            let state = state.borrow();
            let state = state.as_ref().expect("not in simulation context");
            Instant(state.borrow().time_wheel.now())
        })
    }

    pub fn elapsed(&self) -> Duration {
        Self::now().0.as_duration() - self.0.as_duration()
    }

    pub fn duration_since(&self, earlier: Instant) -> Duration {
        self.0.as_duration() - earlier.0.as_duration()
    }
}

impl std::ops::Add<Duration> for Instant {
    type Output = Instant;

    fn add(self, rhs: Duration) -> Self::Output {
        Instant(self.0.add(rhs))
    }
}
```

---

## Building a Simulated Network

### Network Model

```rust
use std::collections::{HashMap, VecDeque};
use std::net::SocketAddr;

/// Simulated network with configurable behavior
pub struct SimulatedNetwork {
    /// Node name -> address mapping
    dns: HashMap<String, SocketAddr>,

    /// Messages in flight
    in_flight: VecDeque<InFlightMessage>,

    /// Active connections
    connections: HashMap<ConnectionId, Connection>,

    /// Listening sockets
    listeners: HashMap<SocketAddr, ListenerState>,

    /// Network conditions
    config: NetworkConfig,

    /// Partition state: (node_a, node_b) -> true if partitioned
    partitions: HashMap<(NodeId, NodeId), bool>,

    /// RNG for latency/loss decisions
    rng: rand::rngs::StdRng,
}

#[derive(Clone)]
pub struct NetworkConfig {
    /// Min latency for message delivery
    pub min_latency: Duration,
    /// Max latency for message delivery
    pub max_latency: Duration,
    /// Probability of packet loss (0.0 - 1.0)
    pub packet_loss: f64,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig {
            min_latency: Duration::from_micros(100),
            max_latency: Duration::from_millis(10),
            packet_loss: 0.0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ConnectionId(u64);

struct InFlightMessage {
    from: NodeId,
    to: NodeId,
    connection: ConnectionId,
    data: Vec<u8>,
    delivery_time: SimTime,
}

struct Connection {
    id: ConnectionId,
    local_addr: SocketAddr,
    remote_addr: SocketAddr,
    local_node: NodeId,
    remote_node: NodeId,
    /// Pending received data
    recv_buffer: VecDeque<u8>,
    /// Is connection closed?
    closed: bool,
}

struct ListenerState {
    node: NodeId,
    /// Pending connection requests
    pending_accepts: VecDeque<ConnectionId>,
}
```

### Network Operations

```rust
impl SimulatedNetwork {
    pub fn new(seed: u64) -> Self {
        use rand::SeedableRng;
        SimulatedNetwork {
            dns: HashMap::new(),
            in_flight: VecDeque::new(),
            connections: HashMap::new(),
            listeners: HashMap::new(),
            config: NetworkConfig::default(),
            partitions: HashMap::new(),
            rng: rand::rngs::StdRng::seed_from_u64(seed),
        }
    }

    /// Register a hostname -> address mapping
    pub fn register_host(&mut self, name: &str, addr: SocketAddr) {
        self.dns.insert(name.to_string(), addr);
    }

    /// Resolve a hostname to address
    pub fn resolve(&self, name: &str) -> Option<SocketAddr> {
        self.dns.get(name).copied()
    }

    /// Create a listening socket
    pub fn bind(&mut self, node: NodeId, addr: SocketAddr) -> Result<(), NetworkError> {
        if self.listeners.contains_key(&addr) {
            return Err(NetworkError::AddressInUse);
        }

        self.listeners.insert(addr, ListenerState {
            node,
            pending_accepts: VecDeque::new(),
        });
        Ok(())
    }

    /// Initiate a connection
    pub fn connect(
        &mut self,
        from_node: NodeId,
        from_addr: SocketAddr,
        to_addr: SocketAddr,
        current_time: SimTime,
    ) -> Result<ConnectionId, NetworkError> {
        // Find which node owns the destination
        let to_node = self.listeners
            .get(&to_addr)
            .map(|l| l.node)
            .ok_or(NetworkError::ConnectionRefused)?;

        // Check for partition
        if self.is_partitioned(from_node, to_node) {
            return Err(NetworkError::NetworkUnreachable);
        }

        // Create connection
        static NEXT_CONN: std::sync::atomic::AtomicU64 =
            std::sync::atomic::AtomicU64::new(0);
        let conn_id = ConnectionId(
            NEXT_CONN.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        );

        let conn = Connection {
            id: conn_id,
            local_addr: from_addr,
            remote_addr: to_addr,
            local_node: from_node,
            remote_node: to_node,
            recv_buffer: VecDeque::new(),
            closed: false,
        };

        self.connections.insert(conn_id, conn);

        // Queue the connection for the listener
        if let Some(listener) = self.listeners.get_mut(&to_addr) {
            listener.pending_accepts.push_back(conn_id);
        }

        Ok(conn_id)
    }

    /// Send data over a connection
    pub fn send(
        &mut self,
        conn_id: ConnectionId,
        data: Vec<u8>,
        current_time: SimTime,
    ) -> Result<(), NetworkError> {
        use rand::Rng;

        let conn = self.connections
            .get(&conn_id)
            .ok_or(NetworkError::ConnectionClosed)?;

        if conn.closed {
            return Err(NetworkError::ConnectionClosed);
        }

        // Check for partition
        if self.is_partitioned(conn.local_node, conn.remote_node) {
            return Err(NetworkError::NetworkUnreachable);
        }

        // Simulate packet loss
        if self.rng.gen::<f64>() < self.config.packet_loss {
            // Packet "lost" - silently dropped
            return Ok(());
        }

        // Calculate delivery time with random latency
        let latency_range = self.config.max_latency - self.config.min_latency;
        let latency = self.config.min_latency
            + Duration::from_nanos(
                self.rng.gen_range(0..latency_range.as_nanos() as u64)
            );
        let delivery_time = current_time.add(latency);

        self.in_flight.push_back(InFlightMessage {
            from: conn.local_node,
            to: conn.remote_node,
            connection: conn_id,
            data,
            delivery_time,
        });

        Ok(())
    }

    /// Process messages that have reached their delivery time
    pub fn deliver_messages(&mut self, current_time: SimTime) {
        // Sort in_flight by delivery time and deliver all due messages
        let mut still_in_flight = VecDeque::new();

        while let Some(msg) = self.in_flight.pop_front() {
            if msg.delivery_time <= current_time {
                // Deliver the message
                if let Some(conn) = self.connections.get_mut(&msg.connection) {
                    if !conn.closed && !self.is_partitioned(msg.from, msg.to) {
                        conn.recv_buffer.extend(msg.data);
                    }
                }
            } else {
                still_in_flight.push_back(msg);
            }
        }

        self.in_flight = still_in_flight;
    }

    /// Create a network partition between two nodes
    pub fn partition(&mut self, a: NodeId, b: NodeId) {
        // Partition is symmetric
        self.partitions.insert((a, b), true);
        self.partitions.insert((b, a), true);
    }

    /// Heal a network partition
    pub fn repair(&mut self, a: NodeId, b: NodeId) {
        self.partitions.remove(&(a, b));
        self.partitions.remove(&(b, a));
    }

    fn is_partitioned(&self, a: NodeId, b: NodeId) -> bool {
        *self.partitions.get(&(a, b)).unwrap_or(&false)
    }

    /// Get next message delivery time (for time advancement)
    pub fn next_delivery_time(&self) -> Option<SimTime> {
        self.in_flight.iter().map(|m| m.delivery_time).min()
    }
}

#[derive(Debug)]
pub enum NetworkError {
    AddressInUse,
    ConnectionRefused,
    ConnectionClosed,
    NetworkUnreachable,
}
```

### TCP-like Socket API

```rust
/// Simulated TcpListener
pub struct TcpListener {
    addr: SocketAddr,
    node: NodeId,
}

impl TcpListener {
    pub async fn bind(addr: &str) -> Result<Self, NetworkError> {
        let addr: SocketAddr = addr.parse().expect("invalid address");
        let node = current_node_id();

        with_network(|net| net.bind(node, addr))?;

        Ok(TcpListener { addr, node })
    }

    pub async fn accept(&self) -> Result<(TcpStream, SocketAddr), NetworkError> {
        loop {
            // Check for pending connections
            let maybe_conn = with_network(|net| {
                if let Some(listener) = net.listeners.get_mut(&self.addr) {
                    listener.pending_accepts.pop_front()
                } else {
                    None
                }
            });

            if let Some(conn_id) = maybe_conn {
                let remote_addr = with_network(|net| {
                    net.connections.get(&conn_id).map(|c| c.remote_addr)
                }).unwrap();

                return Ok((TcpStream { conn_id }, remote_addr));
            }

            // Yield and wait for connections
            yield_now().await;
        }
    }
}

/// Simulated TcpStream
pub struct TcpStream {
    conn_id: ConnectionId,
}

impl TcpStream {
    pub async fn connect(addr: &str) -> Result<Self, NetworkError> {
        let addr: SocketAddr = addr.parse().expect("invalid address");
        let node = current_node_id();
        let local_addr = generate_ephemeral_addr();
        let current_time = current_sim_time();

        let conn_id = with_network(|net| {
            net.connect(node, local_addr, addr, current_time)
        })?;

        Ok(TcpStream { conn_id })
    }

    pub async fn write(&mut self, data: &[u8]) -> Result<usize, NetworkError> {
        let current_time = current_sim_time();
        with_network(|net| {
            net.send(self.conn_id, data.to_vec(), current_time)
        })?;
        Ok(data.len())
    }

    pub async fn read(&mut self, buf: &mut [u8]) -> Result<usize, NetworkError> {
        loop {
            let data = with_network(|net| {
                if let Some(conn) = net.connections.get_mut(&self.conn_id) {
                    if conn.closed && conn.recv_buffer.is_empty() {
                        return Err(NetworkError::ConnectionClosed);
                    }

                    let to_read = buf.len().min(conn.recv_buffer.len());
                    if to_read > 0 {
                        for i in 0..to_read {
                            buf[i] = conn.recv_buffer.pop_front().unwrap();
                        }
                        return Ok(to_read);
                    }
                }
                Ok(0)
            })?;

            if data > 0 {
                return Ok(data);
            }

            // Yield and wait for data
            yield_now().await;
        }
    }
}

// Helper functions (implementation details)
fn current_node_id() -> NodeId {
    // Get from thread-local simulation state
    todo!()
}

fn current_sim_time() -> SimTime {
    // Get from thread-local time wheel
    todo!()
}

fn with_network<T, F: FnOnce(&mut SimulatedNetwork) -> T>(f: F) -> T {
    // Access thread-local network state
    todo!()
}

async fn yield_now() {
    // Yield control back to executor
    std::future::pending::<()>().await;
}

fn generate_ephemeral_addr() -> SocketAddr {
    use std::net::{IpAddr, Ipv4Addr};
    static PORT: std::sync::atomic::AtomicU16 =
        std::sync::atomic::AtomicU16::new(49152);
    let port = PORT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port)
}
```

---

## Controlling Entropy

### Seeded RNG Distribution

```rust
use rand::{SeedableRng, RngCore};
use rand::rngs::StdRng;

/// RNG manager that derives node-local RNGs from a global seed
pub struct EntropyManager {
    /// Master seed for the entire simulation
    master_seed: u64,
    /// RNGs keyed by (node_id, purpose)
    rngs: HashMap<(NodeId, &'static str), StdRng>,
}

impl EntropyManager {
    pub fn new(seed: u64) -> Self {
        EntropyManager {
            master_seed: seed,
            rngs: HashMap::new(),
        }
    }

    /// Get or create an RNG for a specific node and purpose
    pub fn get_rng(&mut self, node: NodeId, purpose: &'static str) -> &mut StdRng {
        self.rngs.entry((node, purpose)).or_insert_with(|| {
            // Derive sub-seed from master seed + node + purpose
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use std::hash::Hasher;
            hasher.write_u64(self.master_seed);
            hasher.write_u64(node.0);
            hasher.write(purpose.as_bytes());
            StdRng::seed_from_u64(hasher.finish())
        })
    }

    /// Get master seed (for logging/reproduction)
    pub fn master_seed(&self) -> u64 {
        self.master_seed
    }
}
```

### Deterministic HashMap

```rust
use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher, Hash};

/// A hasher that produces deterministic results (no randomization)
#[derive(Clone, Default)]
pub struct DeterministicHasher {
    state: u64,
}

impl Hasher for DeterministicHasher {
    fn finish(&self) -> u64 {
        // FNV-1a finalization
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
        DeterministicHasher { state: 14695981039346656037 }
    }
}

/// Type alias for deterministic HashMap
pub type DetHashMap<K, V> = HashMap<K, V, DeterministicBuildHasher>;

/// Type alias for deterministic HashSet
pub type DetHashSet<T> = std::collections::HashSet<T, DeterministicBuildHasher>;

/// Create a new deterministic HashMap
pub fn det_hash_map<K, V>() -> DetHashMap<K, V> {
    HashMap::with_hasher(DeterministicBuildHasher)
}

/// Create a new deterministic HashSet
pub fn det_hash_set<T>() -> DetHashSet<T> {
    std::collections::HashSet::with_hasher(DeterministicBuildHasher)
}
```

### Seeded UUID Generation

```rust
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// Generate UUIDs deterministically from a seeded RNG
pub struct UuidGenerator {
    rng: StdRng,
}

impl UuidGenerator {
    pub fn new(seed: u64) -> Self {
        UuidGenerator {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    /// Generate a v4-like UUID (random, but from seeded RNG)
    pub fn generate(&mut self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        self.rng.fill(&mut bytes);

        // Set version (4) and variant bits per RFC 4122
        bytes[6] = (bytes[6] & 0x0f) | 0x40;  // Version 4
        bytes[8] = (bytes[8] & 0x3f) | 0x80;  // Variant 1

        bytes
    }

    /// Format as string
    pub fn generate_string(&mut self) -> String {
        let bytes = self.generate();
        format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5],
            bytes[6], bytes[7],
            bytes[8], bytes[9],
            bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
        )
    }
}
```

---

## The Simulation Executor

### Single-Threaded Async Executor

```rust
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::cell::RefCell;
use std::rc::Rc;

/// A minimal single-threaded async executor for simulation
pub struct SimExecutor {
    /// Tasks ready to be polled
    ready_queue: Rc<RefCell<VecDeque<TaskId>>>,
    /// All tasks
    tasks: HashMap<TaskId, Task>,
    /// Next task ID
    next_task_id: u64,
}

type TaskId = u64;

struct Task {
    id: TaskId,
    future: Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>>>>,
    node: NodeId,
}

impl SimExecutor {
    pub fn new() -> Self {
        SimExecutor {
            ready_queue: Rc::new(RefCell::new(VecDeque::new())),
            tasks: HashMap::new(),
            next_task_id: 0,
        }
    }

    /// Spawn a task for a specific node
    pub fn spawn<F>(&mut self, node: NodeId, future: F) -> TaskId
    where
        F: Future<Output = Result<(), Box<dyn std::error::Error>>> + 'static,
    {
        let id = self.next_task_id;
        self.next_task_id += 1;

        self.tasks.insert(id, Task {
            id,
            future: Box::pin(future),
            node,
        });

        self.ready_queue.borrow_mut().push_back(id);
        id
    }

    /// Poll all ready tasks once
    pub fn poll_ready(&mut self) -> bool {
        let mut made_progress = false;

        while let Some(task_id) = self.ready_queue.borrow_mut().pop_front() {
            if let Some(task) = self.tasks.get_mut(&task_id) {
                // Create waker for this task
                let waker = self.create_waker(task_id);
                let mut cx = Context::from_waker(&waker);

                // Poll the future
                match task.future.as_mut().poll(&mut cx) {
                    Poll::Ready(result) => {
                        // Task completed
                        self.tasks.remove(&task_id);
                        if let Err(e) = result {
                            eprintln!("Task {} failed: {}", task_id, e);
                        }
                    }
                    Poll::Pending => {
                        // Task yielded, will be re-queued when woken
                    }
                }

                made_progress = true;
            }
        }

        made_progress
    }

    /// Check if all tasks are complete
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    fn create_waker(&self, task_id: TaskId) -> Waker {
        let ready_queue = self.ready_queue.clone();

        // Create a raw waker that puts the task back on the ready queue
        fn clone_fn(ptr: *const ()) -> RawWaker {
            let data = unsafe { &*(ptr as *const WakerData) };
            let cloned = Box::new(WakerData {
                task_id: data.task_id,
                ready_queue: data.ready_queue.clone(),
            });
            RawWaker::new(Box::into_raw(cloned) as *const (), &VTABLE)
        }

        fn wake_fn(ptr: *const ()) {
            let data = unsafe { Box::from_raw(ptr as *mut WakerData) };
            data.ready_queue.borrow_mut().push_back(data.task_id);
        }

        fn wake_by_ref_fn(ptr: *const ()) {
            let data = unsafe { &*(ptr as *const WakerData) };
            data.ready_queue.borrow_mut().push_back(data.task_id);
        }

        fn drop_fn(ptr: *const ()) {
            unsafe { drop(Box::from_raw(ptr as *mut WakerData)) };
        }

        static VTABLE: RawWakerVTable = RawWakerVTable::new(
            clone_fn,
            wake_fn,
            wake_by_ref_fn,
            drop_fn,
        );

        struct WakerData {
            task_id: TaskId,
            ready_queue: Rc<RefCell<VecDeque<TaskId>>>,
        }

        let data = Box::new(WakerData { task_id, ready_queue });
        let raw_waker = RawWaker::new(Box::into_raw(data) as *const (), &VTABLE);
        unsafe { Waker::from_raw(raw_waker) }
    }
}
```

### The Main Simulation Loop

```rust
/// Complete simulation runner
pub struct Simulation {
    executor: SimExecutor,
    time_wheel: TimeWheel,
    network: SimulatedNetwork,
    entropy: EntropyManager,
    nodes: HashMap<NodeId, NodeInfo>,
    next_node_id: u64,
}

struct NodeInfo {
    name: String,
}

impl Simulation {
    pub fn new(seed: u64) -> Self {
        Simulation {
            executor: SimExecutor::new(),
            time_wheel: TimeWheel::new(),
            network: SimulatedNetwork::new(seed),
            entropy: EntropyManager::new(seed),
            nodes: HashMap::new(),
            next_node_id: 0,
        }
    }

    /// Add a node to the simulation
    pub fn host<F, Fut>(&mut self, name: &str, factory: F)
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error>>> + 'static,
    {
        let node_id = NodeId(self.next_node_id);
        self.next_node_id += 1;

        self.nodes.insert(node_id, NodeInfo {
            name: name.to_string(),
        });

        // Register in DNS
        let addr = format!("{}:0", name).parse().unwrap();
        self.network.register_host(name, addr);

        // Spawn the node's main task
        let future = factory();
        self.executor.spawn(node_id, future);
    }

    /// Run simulation until all tasks complete or timeout
    pub fn run(&mut self) -> Result<(), SimulationError> {
        let max_time = SimTime::from_duration(Duration::from_secs(3600)); // 1 hour max

        while !self.executor.is_empty() {
            // 1. Poll all ready tasks
            while self.executor.poll_ready() {
                // Keep polling while making progress
            }

            if self.executor.is_empty() {
                break;
            }

            // 2. Determine next event time
            let next_timer = self.time_wheel.next_deadline();
            let next_network = self.network.next_delivery_time();

            let next_event = match (next_timer, next_network) {
                (Some(t), Some(n)) => Some(t.min(n)),
                (Some(t), None) => Some(t),
                (None, Some(n)) => Some(n),
                (None, None) => None,
            };

            match next_event {
                Some(event_time) if event_time <= max_time => {
                    // 3. Advance time
                    self.time_wheel.advance_by(
                        event_time.as_duration() - self.time_wheel.now().as_duration()
                    );

                    // 4. Deliver network messages
                    self.network.deliver_messages(self.time_wheel.now());

                    // 5. Fire timers (wake sleeping tasks)
                    // This would wake tasks via their wakers
                }
                _ => {
                    // Deadlock or timeout
                    return Err(SimulationError::Deadlock);
                }
            }
        }

        Ok(())
    }

    /// Inject a network partition
    pub fn partition(&mut self, node_a: &str, node_b: &str) {
        let id_a = self.find_node(node_a);
        let id_b = self.find_node(node_b);
        if let (Some(a), Some(b)) = (id_a, id_b) {
            self.network.partition(a, b);
        }
    }

    /// Repair a network partition
    pub fn repair(&mut self, node_a: &str, node_b: &str) {
        let id_a = self.find_node(node_a);
        let id_b = self.find_node(node_b);
        if let (Some(a), Some(b)) = (id_a, id_b) {
            self.network.repair(a, b);
        }
    }

    fn find_node(&self, name: &str) -> Option<NodeId> {
        self.nodes.iter()
            .find(|(_, info)| info.name == name)
            .map(|(id, _)| *id)
    }
}

#[derive(Debug)]
pub enum SimulationError {
    Deadlock,
    Timeout,
    TaskFailed(String),
}
```

---

## Fault Injection System

### Fault Types

```rust
/// Types of faults that can be injected
#[derive(Clone, Debug)]
pub enum Fault {
    /// Network partition between nodes
    Partition { nodes: Vec<String> },

    /// Increased latency
    Latency { target: String, delay: Duration },

    /// Packet loss
    PacketLoss { target: String, rate: f64 },

    /// Node crash
    Crash { node: String },

    /// Node restart after crash
    Restart { node: String },

    /// Disk full simulation
    DiskFull { node: String },

    /// Clock skew
    ClockSkew { node: String, offset: Duration },
}

/// Fault scheduler for time-based fault injection
pub struct FaultScheduler {
    /// Scheduled faults: (time, fault)
    scheduled: BinaryHeap<Reverse<ScheduledFault>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct ScheduledFault {
    time: SimTime,
    fault: Fault,
}

impl Ord for Fault {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare by debug string for ordering (simple approach)
        format!("{:?}", self).cmp(&format!("{:?}", other))
    }
}

impl PartialOrd for Fault {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Fault {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

impl Eq for Fault {}

impl FaultScheduler {
    pub fn new() -> Self {
        FaultScheduler {
            scheduled: BinaryHeap::new(),
        }
    }

    /// Schedule a fault to occur at a specific simulation time
    pub fn schedule(&mut self, time: SimTime, fault: Fault) {
        self.scheduled.push(Reverse(ScheduledFault { time, fault }));
    }

    /// Schedule a fault after a delay from now
    pub fn schedule_after(&mut self, current: SimTime, delay: Duration, fault: Fault) {
        self.schedule(current.add(delay), fault);
    }

    /// Get faults that should trigger at or before the given time
    pub fn pop_due(&mut self, current_time: SimTime) -> Vec<Fault> {
        let mut due = Vec::new();

        while let Some(Reverse(sf)) = self.scheduled.peek() {
            if sf.time <= current_time {
                let Reverse(sf) = self.scheduled.pop().unwrap();
                due.push(sf.fault);
            } else {
                break;
            }
        }

        due
    }

    /// Get the next scheduled fault time
    pub fn next_time(&self) -> Option<SimTime> {
        self.scheduled.peek().map(|Reverse(sf)| sf.time)
    }
}
```

### Chaos Testing Patterns

```rust
/// Generate random fault scenarios based on seed
pub struct ChaosGenerator {
    rng: StdRng,
    node_names: Vec<String>,
}

impl ChaosGenerator {
    pub fn new(seed: u64, nodes: Vec<String>) -> Self {
        ChaosGenerator {
            rng: StdRng::seed_from_u64(seed),
            node_names: nodes,
        }
    }

    /// Generate a random fault
    pub fn generate_fault(&mut self) -> Fault {
        use rand::Rng;

        let fault_type = self.rng.gen_range(0..5);

        match fault_type {
            0 => {
                // Random partition
                let n = self.rng.gen_range(1..=self.node_names.len());
                let nodes: Vec<_> = self.node_names
                    .choose_multiple(&mut self.rng, n)
                    .cloned()
                    .collect();
                Fault::Partition { nodes }
            }
            1 => {
                // Latency spike
                let target = self.node_names.choose(&mut self.rng).unwrap().clone();
                let delay = Duration::from_millis(self.rng.gen_range(100..5000));
                Fault::Latency { target, delay }
            }
            2 => {
                // Packet loss
                let target = self.node_names.choose(&mut self.rng).unwrap().clone();
                let rate = self.rng.gen_range(0.01..0.5);
                Fault::PacketLoss { target, rate }
            }
            3 => {
                // Node crash
                let node = self.node_names.choose(&mut self.rng).unwrap().clone();
                Fault::Crash { node }
            }
            _ => {
                // Clock skew
                let node = self.node_names.choose(&mut self.rng).unwrap().clone();
                let offset = Duration::from_millis(self.rng.gen_range(0..60000));
                Fault::ClockSkew { node, offset }
            }
        }
    }

    /// Generate a scenario with multiple faults
    pub fn generate_scenario(&mut self, num_faults: usize, duration: Duration) -> Vec<(SimTime, Fault)> {
        use rand::Rng;

        (0..num_faults)
            .map(|_| {
                let time = SimTime::from_duration(
                    Duration::from_nanos(self.rng.gen_range(0..duration.as_nanos() as u64))
                );
                let fault = self.generate_fault();
                (time, fault)
            })
            .collect()
    }
}
```

---

## libc Interception (Advanced)

### Why libc Interception?

Some libraries call libc functions directly, bypassing your Rust abstractions:

- `getrandom()` / `getentropy()` - System entropy
- `clock_gettime()` - System time
- `gettimeofday()` - Wall clock time

### Using `LD_PRELOAD` Approach

Create a shared library that intercepts these calls:

```rust
// lib_intercept.rs (compiled as cdylib)

use std::ffi::c_int;
use std::sync::atomic::{AtomicU64, Ordering};

// Simulated time (nanoseconds since epoch)
static SIMULATED_TIME: AtomicU64 = AtomicU64::new(0);

// Simulated entropy counter
static ENTROPY_COUNTER: AtomicU64 = AtomicU64::new(0x1234567890abcdef);

/// Set the simulated time (called from test harness)
#[no_mangle]
pub extern "C" fn sim_set_time(nanos: u64) {
    SIMULATED_TIME.store(nanos, Ordering::SeqCst);
}

/// Set the entropy seed (called from test harness)
#[no_mangle]
pub extern "C" fn sim_set_entropy_seed(seed: u64) {
    ENTROPY_COUNTER.store(seed, Ordering::SeqCst);
}

// Intercept clock_gettime
#[no_mangle]
pub extern "C" fn clock_gettime(clk_id: c_int, tp: *mut libc::timespec) -> c_int {
    if tp.is_null() {
        return -1;
    }

    let nanos = SIMULATED_TIME.load(Ordering::SeqCst);
    let secs = nanos / 1_000_000_000;
    let nsecs = nanos % 1_000_000_000;

    unsafe {
        (*tp).tv_sec = secs as i64;
        (*tp).tv_nsec = nsecs as i64;
    }

    0
}

// Intercept getrandom
#[no_mangle]
pub extern "C" fn getrandom(buf: *mut u8, buflen: usize, _flags: u32) -> isize {
    if buf.is_null() {
        return -1;
    }

    // Generate deterministic "random" bytes
    for i in 0..buflen {
        let counter = ENTROPY_COUNTER.fetch_add(1, Ordering::SeqCst);
        // Simple PRNG based on counter
        let byte = (counter.wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407) >> 56) as u8;
        unsafe {
            *buf.add(i) = byte;
        }
    }

    buflen as isize
}

// Intercept getentropy (macOS)
#[no_mangle]
pub extern "C" fn getentropy(buf: *mut u8, buflen: usize) -> c_int {
    if getrandom(buf, buflen, 0) < 0 {
        -1
    } else {
        0
    }
}
```

### Build and Usage

```bash
# Cargo.toml for intercept library
[lib]
crate-type = ["cdylib"]

# Build
cargo build --release

# Run tests with interception
LD_PRELOAD=./target/release/libintercept.so cargo test
```

### Rust Symbol Override (madsim approach)

MadSim uses conditional compilation instead of LD_PRELOAD:

```rust
// In your crate, under cfg(madsim)

#[cfg(madsim)]
mod time {
    pub fn now() -> std::time::Instant {
        // Return simulated time from madsim runtime
        madsim::time::Instant::now().into()
    }
}

#[cfg(not(madsim))]
mod time {
    pub fn now() -> std::time::Instant {
        std::time::Instant::now()
    }
}
```

---

## Putting It All Together

### Complete Example: Building a Mini DST Framework

```rust
// src/lib.rs - A minimal but complete DST framework

pub mod time;
pub mod network;
pub mod entropy;
pub mod executor;
pub mod simulation;

// Re-exports
pub use simulation::Simulation;
pub use time::{Instant, sleep};
pub use network::{TcpListener, TcpStream};
pub use entropy::{det_hash_map, det_hash_set};
```

### Usage Example

```rust
use my_dst_framework::{Simulation, TcpListener, TcpStream, sleep};
use std::time::Duration;

#[test]
fn test_echo_server() {
    let seed = std::env::var("TEST_SEED")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(42);

    println!("Running with seed: {}", seed);

    let mut sim = Simulation::new(seed);

    // Echo server
    sim.host("server", || async {
        let listener = TcpListener::bind("0.0.0.0:8080").await?;

        loop {
            let (mut stream, _) = listener.accept().await?;
            let mut buf = [0u8; 1024];
            let n = stream.read(&mut buf).await?;
            stream.write(&buf[..n]).await?;
        }
    });

    // Client
    sim.host("client", || async {
        sleep(Duration::from_millis(100)).await;

        let mut stream = TcpStream::connect("server:8080").await?;
        stream.write(b"hello world").await?;

        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf).await?;
        assert_eq!(&buf[..n], b"hello world");

        Ok(())
    });

    sim.run().expect("Simulation failed");
}

#[test]
fn test_survives_partition() {
    let mut sim = Simulation::new(42);

    sim.host("node-a", || async {
        // ... consensus participant
        Ok(())
    });

    sim.host("node-b", || async {
        // ... consensus participant
        Ok(())
    });

    sim.host("node-c", || async {
        // ... consensus participant
        Ok(())
    });

    // After 5 seconds, partition node-a
    sim.schedule_fault(
        Duration::from_secs(5),
        Fault::Partition { nodes: vec!["node-a".into()] }
    );

    // After 15 seconds, heal partition
    sim.schedule_fault(
        Duration::from_secs(15),
        Fault::Repair { nodes: vec!["node-a".into()] }
    );

    sim.run().expect("Consensus failed during partition");
}
```

### Project Structure

```
my-distributed-system/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Application logic
│   ├── consensus.rs     # Consensus protocol
│   └── storage.rs       # Storage layer
└── tests/
    ├── simulation/
    │   ├── mod.rs
    │   ├── framework/   # Your custom DST framework
    │   │   ├── mod.rs
    │   │   ├── time.rs
    │   │   ├── network.rs
    │   │   ├── entropy.rs
    │   │   ├── executor.rs
    │   │   └── simulation.rs
    │   └── tests/
    │       ├── consensus_test.rs
    │       └── partition_test.rs
    └── integration/
        └── real_network_test.rs
```

---

## Key Takeaways

### What We Built

1. **Simulated Time** - TimeWheel + Sleep futures
2. **Simulated Network** - In-memory message passing with configurable delays
3. **Seeded Entropy** - Deterministic RNG distribution
4. **Single-Threaded Executor** - Cooperative scheduling
5. **Fault Injection** - Partitions, latency, crashes

### Design Principles

| Principle | Why |
|-----------|-----|
| **Single-threaded** | Eliminates scheduler non-determinism |
| **Explicit time advancement** | Time only moves when you say |
| **Seeded everything** | Same seed = same execution |
| **Abstracted I/O** | Can swap real ↔ simulated |

### When to Extend vs. Use Libraries

**Build your own when:**
- You need custom network models (e.g., RDMA simulation)
- Your runtime isn't Tokio (async-std, smol, embassy)
- You want to learn how it all works
- You need specific fault models

**Use turmoil/madsim when:**
- Standard TCP/UDP simulation is sufficient
- You want maintained, tested code
- Quick integration is priority
- You're using Tokio

---

## References

- [MadSim Source](https://github.com/madsim-rs/madsim) - Study the libc override techniques
- [Turmoil Source](https://github.com/tokio-rs/turmoil) - Clean network simulation patterns
- [async-task crate](https://docs.rs/async-task) - Task spawning primitives
- [futures-rs](https://github.com/rust-lang/futures-rs) - Future utilities
- [Rust Async Book](https://rust-lang.github.io/async-book/) - Understanding async internals

---

*Created: 2026-01-19*
*Last Updated: 2026-01-19*
