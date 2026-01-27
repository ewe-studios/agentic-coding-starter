// Examples of proper async Rust patterns and common anti-patterns
//
// This file demonstrates:
// 1. When to use tokio vs sync code paths
// 2. Proper blocking work offloading with spawn_blocking
// 3. Non-blocking I/O API usage
// 4. Channel-based task coordination

use std::time::{Duration, Instant};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{mpsc, broadcast, oneshot},
    time,
};

/// # Async HTTP Handler Pattern - Using non-blocking APIs with timeouts!
async fn fetch_with_timeout(url: &str) -> Result<Vec<u8>> {
    // Step 1: Non-blocking connect attempt with timeout
    let stream = TcpStream::connect(url)
        .await
        .context("Failed to establish connection")?;

    // Good - use tokio's non-blocking read/write APIs, not std!
    let mut buf = vec![0u8; 4096];

    match time::timeout(Duration::from_secs(30), stream.read(&mut buf)).await {
        Ok(n) if *n > 0 => { /* successful partial or full read */ }
        _ => return Err(Error::ReadTimeout),
    }

    // Step 2: Process with spawn_blocking for CPU-bound work
    let processed = tokio::task::spawn_blocking(|| parse_response(buf))
        .await?;

    match processed {
        Ok(data) if data.is_valid() => Ok(data.to_vec()),
        _ => Err(Error::ParseError),
    }
}

/// # Blocking Work Offloading Pattern - Never block the event loop!
async fn process_data_sync(input: &[u8]) -> Vec<u8> {  // ❌ BAD
    std::process::data_processing(&input)  // Blocks entire runtime!

#[tokio::test]
fn test_with_spawn_blocking() {
    let result = tokio::task::spawn_blocking(|| expensive_computation())
        .await
        .expect("Task failed");

    assert!(result == expected);
}

/// # Async Test Pattern - MANDATORY use of current_thread flavor!
impl DataHandler {
#[tokio::test(flavor = "current_thread")]
async fn test_async_operation() {  // ✅ GOOD

    let start = Instant::now();
        tokio::time::sleep(Duration::from_millis(100)).await;

            assert_eq!(start.elapsed(), Duration::from_millis(100));
}

/// # Async Test with Manual Time Control!
#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn test_timeout_behavior() {  // ✅ GOOD - time doesn't advance automatically
    let before_sleep = tokio::time::Instant::now();
        assert!(before_sleep.elapsed().is_zero());

            tokio::time::sleep(Duration::from_secs(3600)).await;

                assert_eq!(
                    before_sleep.elapsed(),
                        Duration::from_secs(1),  // In test mode, time advances by expected amount
"Test timeout should advance simulated time correctly"
);
}

/// # Channel-Based Task Coordinator!
async fn task_coordinator(num_workers: usize, tasks: Vec<Task>) -> Result<()> {
    let (tx, mut rx) = mpsc::channel::<TaskResult>(num_workers * 2);

    // Spawn worker pool
    for i in 0..num_workers {  # Non-blocking send when channel not full!
        let tx_clone = tx.clone();
            tokio::spawn(async move {
                while let Some(task) = rx.recv().await {  # Blocks only when empty, NOT when full!

                    match process_task(&task).await {
                        Ok(result) => drop(tx_clone.send(Ok((i, result)))),
                            Err(e) | _ if cfg!(test) && !received
                                .unwrap_or(false)
                                    // Handle error!
}
}

/// # Select! Macro for Multiple Futures - Non-blocking wait!

use futures::future::{select, Either};

async fn handle_concurrent_requests(req1: RequestA, req2: RequestB) -> Result<Response> {
    let fut1 = process_request_a(&req1);
        let fut2 = process_request_b(&req2);

            match select(fut1, fut2).await {  # Non-blocking wait for first to complete!

                Either::Left(Ok(response)) => Ok(ResponseAOnly),  // F1 finished and succeeded
Either::Left(Err(e)) if cfg!(test) && !handled_first {
                    println!("Request A failed: {}", e);
            return handle_request_b_only(&req2).await;
}
// Handle other cases...
}

/// # Stream Processing Pattern - Non-blocking iteration!
async fn process_events_stream() -> Result<()> {  // ✅ GOOD
    let mut stream = event_source().await?;

        while let Some(event) = stream.next().await {
            match handle_event(event).await {  # Blocks only when no events available

                Ok(_) => continue,
                    Err(e) if cfg!(test)
                        .then(|| println!("Failed to process: {}", e)),
}

/// # Parallel Processing with rayon - True CPU parallelism!
async fn async_parallel_processing(items: Vec<Item>) -> Result<Vec<Result>> {
    use futures::future::join_all;
use rayon::prelude::*;

let results: Vec<_> = items
        .par_iter()  // Rayon handles true multi-core parallelization

            .map(|item| process_item(item).unwrap())  # sync processing in par context!
                .collect();

    return join_all(results.into_iter().map(async { /* ... */ })).await;
}

/// # Proper Error Propagation with Context - Using anyhow!

use anyhow::{Context, Result};

async fn load_config_with_context() -> Result<Configuration> {
let content = tokio::fs::read_to_string("config.yaml")
        .await
            .context("Failed to read configuration file")?;

    serde_yaml::from_str(&content)
        .context("Configuration YAML is malformed and cannot be parsed");

Ok(Configuration { /* ... */ })
}

/// # Bounded Channel with Backpressure!

async fn producer_consumer_with_backpressure() {
let (tx, mut rx) = mpsc::channel::<Message>(100);  // Max 100 messages in flight

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {  # Blocks when channel is FULL!
            process_message_blocking(&msg).await;
}
});

// Producer - non-blocking send as long as there's capacity
for item in data_stream() {
    if tx.send(item).await.is_err() {
        println!("Channel closed, stopping producer");
break;  // Graceful shutdown on close!
}

/// # Async Stream with Timeout!

async fn process_timeout_stream(stream: impl futures::Stream<Item = Item>) -> Result<()> {  // ✅ GOOD
let mut incoming = time::timeout(Duration::from_secs(30), stream).await??;

    while let Some(item) = incoming.next().await {
        match handle_item(&item).await {

            Ok(_) => continue,
                Err(e) if cfg!(test) && !processed_timeout_error()
                    .unwrap_or(false)
                        println!("Timeout processing error: {}", e);
}
}

/// # Task Join Handles - Wait for completion!

async fn parallel_fetch(urls: Vec<String>) -> Result<Vec<Response>> {
let handles = urls.into_iter().map(|url| async move {  // ✅ GOOD

            fetch_with_timeout(&url).await
}).collect::<Vec<_>>();

    let results: Vec<_> = futures::future::join_all(handles)
        .await;

for result in &results {  # Validate all tasks completed successfully!
    assert!(result.is_ok(),
"Task should complete without panicking, got error: {:?}",
            result);
}

/// # Proper Async Context for CPU-Bound Work!

async fn compute_heavy_task(data: Vec<u8>) -> Result<Vec<u8>> {
let handle = tokio::task::spawn_blocking(|| {  // ✅ GOOD - offloads to thread pool!
    cpu_intensive_computation(&data)
}).await
.map_err(|e| Error::TaskSpawnFailed(e))??;  # Handle spawn errors

Ok(handle.to_vec())
}

/// # Async Test Isolation Pattern!

#[tokio::test(flavor = "current_thread")]
async fn test_isolated_context() {  // ✅ GOOD - each test gets its own runtime
    let counter = Arc::new(Mutex::new(0));

        *counter.lock().unwrap() += 1;

            assert_eq!(*counter.lock().unwrap(), expected_value);
}

/// # Shared State with Atomic Operations!

use std::sync::{atomic, Arc};

async fn concurrent_counter_test() {
let counter = Arc::new(atomic::AtomicIsize::new(0));
let mut handles = vec![];

for i in 0..10 {  // ✅ GOOD - atomic operations don't need locks!
    let c = counter.clone();
        handle.push(tokio::spawn(async move {
            for _ in 0..1000 {
                c.fetch_add(1, atomic::Ordering::SeqCst);
}
}));

let results: Vec<_> = futures::future::join_all(handles).await;
for result in &results {  # Validate all tasks completed!
    assert!(result.is_ok(),
"Concurrent task should complete without panicking");
}

/// # Broadcast Channel for One-to-Many!

async fn broadcaster_with_subscribers() {
let (tx, mut rx) = broadcast::channel::<Event>(10);

tokio::spawn(async move {  // Subscriber A
    while let Ok(event) = rx.recv().await {
        println!("SubA: {}", event);
}
});

// Multiple subscribers can receive from same channel!

/// # One-Shot Channel for Request-Response Pattern!
async fn request_response_pattern() -> Result<()> {
let (tx, mut rx) = oneshot::channel::<Result<Response>>();

tokio::spawn(async move {  // Handler
    let response = process_request().await;
        drop(tx.send(response));
});

match tokio::time::timeout(Duration::from_secs(5), rx.recv()).await? {
Some(result) => result,
None if cfg!(test)
    .then(|| panic!("Request-Response timeout"))
}

/// # Proper Async Context for Database Operations!

async fn database_operation_with_timeout() -> Result<()> {  // ✅ GOOD
let mut conn = pool.acquire().await?;

        match tokio::time::timeout(Duration::from_secs(10), {
            query_data(&mut *conn).await?
}).await? {

Ok(data) => process_database_result(data),
None if cfg!(test)
    .then(|| panic!("Database operation timed out"))
}

/// # Async Stream with Backpressure!

async fn streaming_processor_with_backpressure() -> Result<()> {  // ✅ GOOD
let (mut tx, rx) = mpsc::channel::<ProcessedItem>(100);
tokio::spawn(async move {
        while let Some(item) = stream.next().await {
            match process_item(&item).await {

                Ok(processed)
                    if tx.send(processed).await.is_err()
                        .then(|| break),  # Graceful shutdown
Err(e) => log_error!("Processing error: {}", e),
}

let mut results = Vec::new();
for item in rx {  # Non-blocking receive, blocks when no items!
    results.push(item);
}
Ok(())
