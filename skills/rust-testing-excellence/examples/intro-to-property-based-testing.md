# Introduction to Property-Based Testing in Rust

**A Complete Guide from Beginner to Advanced**

---

## Table of Contents

1. [Beginner: Understanding Property-Based Testing](#beginner-understanding-property-based-testing)
2. [Intermediate: Writing Effective Properties](#intermediate-writing-effective-properties)
3. [Advanced: Strategies, Shrinking, and Complex Properties](#advanced-strategies-shrinking-and-complex-properties)

---

# Beginner: Understanding Property-Based Testing

## What is Property-Based Testing?

Property-based testing (PBT) is a testing approach where you define **properties** (invariants) that should always hold true for your code, and the testing framework automatically generates hundreds of test cases to verify those properties.

### Traditional Testing vs Property-Based Testing

**Traditional Example Testing:**
```rust
#[test]
fn test_reverse_string() {
    assert_eq!(reverse("hello"), "olleh");
    assert_eq!(reverse("rust"), "tsur");
    assert_eq!(reverse(""), "");
}
```

You manually write each test case. You might miss edge cases.

**Property-Based Testing:**
```rust
proptest! {
    #[test]
    fn test_reverse_twice_is_identity(s in ".*") {
        let reversed_twice = reverse(&reverse(&s));
        prop_assert_eq!(&reversed_twice, &s);
    }
}
```

The framework generates 100+ random strings automatically. You define the **property**: reversing twice gives you back the original.

---

## Why Use Property-Based Testing?

### Benefits

1. **Finds Edge Cases You Didn't Think Of**
   - Automatically tests with empty strings, unicode, special characters, large inputs
   - Discovers bugs in corner cases you wouldn't manually test

2. **Tests Invariants, Not Examples**
   - Instead of "reverse('hello') == 'olleh'"
   - You test "reverse(reverse(x)) == x for ALL x"

3. **Less Code, More Coverage**
   - One property can replace dozens of example tests
   - Framework generates hundreds of test cases

4. **Living Documentation**
   - Properties describe **what** your code does, not just **how**
   - "This function is reversible" is clearer than 10 example tests

### When to Use Property-Based Testing

✅ **Perfect for:**
- Mathematical operations (addition, sorting, hashing)
- Serialization/deserialization (JSON, binary formats)
- Parsers (should never panic on any input)
- Data transformations (encoding, compression)
- Reversible operations (encrypt/decrypt, compress/decompress)

❌ **Not ideal for:**
- Testing specific business logic with exact expected outputs
- Integration tests with external services
- UI behavior testing
- Tests where you need exact control over inputs

---

## Getting Started with Proptest

### 1. Add Proptest to Your Project

```toml
# Cargo.toml
[dev-dependencies]
proptest = "1.4"
```

### 2. Your First Property Test

Let's test a simple `add` function:

```rust
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_add_commutative(a in 0i32..1000, b in 0i32..1000) {
            // Property: addition is commutative (a + b == b + a)
            prop_assert_eq!(add(a, b), add(b, a));
        }
    }
}
```

**What's happening:**
- `a in 0i32..1000` - Generate random `i32` values between 0 and 1000
- `b in 0i32..1000` - Generate another random `i32`
- `prop_assert_eq!` - Assert the property holds
- Framework runs this test 256 times by default with different random inputs

### 3. Understanding Strategies

**Strategies** define how to generate test data.

```rust
proptest! {
    #[test]
    fn test_with_different_strategies(
        // Generate any i32
        any_int in any::<i32>(),

        // Generate i32 in range
        small_int in 0i32..100,

        // Generate strings matching regex
        name in "[a-zA-Z]{3,10}",

        // Generate strings of any length
        any_string in ".*",

        // Generate booleans
        flag in any::<bool>(),
    ) {
        // Your property tests here
    }
}
```

### 4. Basic Property Patterns

#### Pattern 1: Roundtrip Properties

**Test that encode/decode are inverses:**

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_base64_roundtrip(data in any::<Vec<u8>>()) {
        let encoded = base64_encode(&data);
        let decoded = base64_decode(&encoded).unwrap();
        prop_assert_eq!(data, decoded);
    }
}
```

#### Pattern 2: Invariant Properties

**Test that something never panics:**

```rust
proptest! {
    #[test]
    fn test_parser_never_panics(input in ".*") {
        // Should never panic, no matter what input we give it
        let _ = parse_json(&input);
        // If we reach here, it didn't panic - test passes
    }
}
```

#### Pattern 3: Oracle Properties

**Compare against a known correct implementation:**

```rust
proptest! {
    #[test]
    fn test_custom_sort_matches_std_sort(mut data in any::<Vec<i32>>()) {
        let mut expected = data.clone();

        my_sort(&mut data);      // Your implementation
        expected.sort();          // Standard library (oracle)

        prop_assert_eq!(data, expected);
    }
}
```

#### Pattern 4: Idempotence Properties

**Test that doing something twice gives same result:**

```rust
proptest! {
    #[test]
    fn test_normalize_idempotent(s in ".*") {
        let once = normalize(&s);
        let twice = normalize(&once);
        prop_assert_eq!(once, twice);
    }
}
```

---

## Common Beginner Mistakes

### Mistake 1: Testing Implementation, Not Properties

```rust
// BAD ❌ - This is just an example test with random inputs
proptest! {
    #[test]
    fn bad_test(x in 0i32..100) {
        prop_assert_eq!(add(x, 5), x + 5); // You're reimplementing add!
    }
}

// GOOD ✅ - Test a property
proptest! {
    #[test]
    fn good_test(x in 0i32..100, y in 0i32..100) {
        // Property: addition is commutative
        prop_assert_eq!(add(x, y), add(y, x));
    }
}
```

### Mistake 2: Not Handling Errors Properly

```rust
// BAD ❌ - unwrap can panic
proptest! {
    #[test]
    fn bad_test(data in any::<Vec<u8>>()) {
        let decoded = decode(&data).unwrap(); // May panic!
    }
}

// GOOD ✅ - Handle errors gracefully
proptest! {
    #[test]
    fn good_test(data in any::<Vec<u8>>()) {
        match decode(&data) {
            Ok(decoded) => {
                // Test properties of successful decode
                prop_assert!(decoded.len() <= data.len());
            },
            Err(_) => {
                // It's OK for some inputs to fail
                // Just don't panic
            }
        }
    }
}
```

### Mistake 3: Not Constraining Input Ranges

```rust
// BAD ❌ - May overflow
proptest! {
    #[test]
    fn bad_test(a in any::<i32>(), b in any::<i32>()) {
        prop_assert_eq!(add(a, b), a + b); // Can overflow!
    }
}

// GOOD ✅ - Constrain to safe ranges
proptest! {
    #[test]
    fn good_test(a in 0i32..1000, b in 0i32..1000) {
        prop_assert_eq!(add(a, b), a + b);
    }
}
```

---

## Beginner Exercise: Write Your First Property Tests

Try writing property tests for these functions:

```rust
// 1. Test that multiply is commutative
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// 2. Test that uppercase is idempotent
pub fn uppercase(s: &str) -> String {
    s.to_uppercase()
}

// 3. Test that reversing a vec twice gives original
pub fn reverse<T: Clone>(v: &[T]) -> Vec<T> {
    v.iter().rev().cloned().collect()
}
```

<details>
<summary>Click to see solutions</summary>

```rust
use proptest::prelude::*;

proptest! {
    // 1. Multiply is commutative
    #[test]
    fn test_multiply_commutative(a in 0i32..100, b in 0i32..100) {
        prop_assert_eq!(multiply(a, b), multiply(b, a));
    }

    // 2. Uppercase is idempotent
    #[test]
    fn test_uppercase_idempotent(s in ".*") {
        let once = uppercase(&s);
        let twice = uppercase(&once);
        prop_assert_eq!(once, twice);
    }

    // 3. Reverse twice gives original
    #[test]
    fn test_reverse_twice(v in any::<Vec<i32>>()) {
        let twice = reverse(&reverse(&v));
        prop_assert_eq!(v, twice);
    }
}
```

</details>

---

# Intermediate: Writing Effective Properties

## Advanced Strategies

### Custom Generators

Create custom strategies for your domain types:

```rust
use proptest::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub age: u8,
    pub email: String,
}

// Custom strategy for generating valid users
fn user_strategy() -> impl Strategy<Value = User> {
    (
        "[a-zA-Z]{3,20}",           // name: 3-20 letters
        18u8..100u8,                 // age: 18-99
        "[a-z]{3,10}@[a-z]{3,10}\\.com", // email
    ).prop_map(|(name, age, email)| User { name, age, email })
}

proptest! {
    #[test]
    fn test_user_serialization(user in user_strategy()) {
        let json = serde_json::to_string(&user).unwrap();
        let decoded: User = serde_json::from_string(&json).unwrap();
        prop_assert_eq!(user, decoded);
    }
}
```

### Combining Strategies

```rust
// Generate optional values
fn maybe_string() -> impl Strategy<Value = Option<String>> {
    prop::option::of(".*")
}

// Generate collections
fn small_vec() -> impl Strategy<Value = Vec<i32>> {
    prop::collection::vec(0i32..100, 0..20) // 0-20 elements
}

// Generate tuples
fn coordinate() -> impl Strategy<Value = (f64, f64)> {
    (-180.0..180.0, -90.0..90.0) // longitude, latitude
}

proptest! {
    #[test]
    fn test_with_combined_strategies(
        opt in maybe_string(),
        vec in small_vec(),
        coord in coordinate(),
    ) {
        // Test your properties
    }
}
```

---

## Thinking in Properties

### The Property Discovery Process

For any function, ask yourself:

1. **Metamorphic Properties** - "If I transform the input, how does the output transform?"
   ```rust
   // If I sort, then add element, then sort again...
   // Should be same as add element then sort once
   proptest! {
       #[test]
       fn test_sort_metamorphic(mut vec in any::<Vec<i32>>(), x in any::<i32>()) {
           vec.sort();
           vec.push(x);
           vec.sort();

           let mut vec2 = vec.clone();
           vec2.push(x);
           vec2.sort();

           prop_assert_eq!(vec, vec2);
       }
   }
   ```

2. **Invariant Properties** - "What must always be true about the output?"
   ```rust
   // Sorted output must have same length as input
   proptest! {
       #[test]
       fn test_sort_preserves_length(vec in any::<Vec<i32>>()) {
           let original_len = vec.len();
           let mut sorted = vec.clone();
           sorted.sort();
           prop_assert_eq!(sorted.len(), original_len);
       }
   }
   ```

3. **Inverse Properties** - "Can I undo the operation?"
   ```rust
   // encode then decode gives original
   proptest! {
       #[test]
       fn test_encoding_invertible(data in any::<Vec<u8>>()) {
           let encoded = encode(&data);
           let decoded = decode(&encoded).unwrap();
           prop_assert_eq!(data, decoded);
       }
   }
   ```

4. **Comparison Properties** - "Does it match a simpler/slower/known implementation?"
   ```rust
   // Fast path should match slow path
   proptest! {
       #[test]
       fn test_fast_path_matches_slow_path(data in any::<Vec<i32>>()) {
           prop_assert_eq!(fast_sort(&data), slow_but_correct_sort(&data));
       }
   }
   ```

---

## Testing Stateful Systems

### State Machine Testing

```rust
use proptest::prelude::*;

#[derive(Debug, Clone)]
enum BankAction {
    Deposit(u64),
    Withdraw(u64),
    CheckBalance,
}

fn action_strategy() -> impl Strategy<Value = BankAction> {
    prop_oneof![
        (1u64..1000).prop_map(BankAction::Deposit),
        (1u64..100).prop_map(BankAction::Withdraw),
        Just(BankAction::CheckBalance),
    ]
}

proptest! {
    #[test]
    fn test_bank_account_invariants(
        initial_balance in 0u64..10000,
        actions in prop::collection::vec(action_strategy(), 1..50)
    ) {
        let mut account = BankAccount::new(initial_balance);
        let mut expected_balance = initial_balance;

        for action in actions {
            match action {
                BankAction::Deposit(amount) => {
                    account.deposit(amount);
                    expected_balance += amount;
                }
                BankAction::Withdraw(amount) => {
                    if account.balance() >= amount {
                        account.withdraw(amount).unwrap();
                        expected_balance -= amount;
                    }
                }
                BankAction::CheckBalance => {
                    prop_assert_eq!(account.balance(), expected_balance);
                }
            }

            // Invariant: balance never goes negative
            prop_assert!(account.balance() >= 0);
        }
    }
}
```

---

## Handling Preconditions

Sometimes your properties only hold under certain conditions:

```rust
proptest! {
    #[test]
    fn test_division(a in any::<i32>(), b in any::<i32>()) {
        // Skip cases where b is 0 (precondition)
        prop_assume!(b != 0);

        let result = a / b;
        prop_assert_eq!(result * b, a);
    }
}
```

**Note:** Use `prop_assume!` sparingly - too many rejections slow down tests. Better to constrain your strategy:

```rust
// Better approach
proptest! {
    #[test]
    fn test_division(a in any::<i32>(), b in 1i32..1000) {
        // b is always non-zero by construction
        let result = a / b;
        prop_assert_eq!(result * b, a);
    }
}
```

---

## Debugging Failed Properties

When a property fails, proptest shows you the failing input:

```
thread 'tests::test_reverse_twice' panicked at 'Test failed: reverse(reverse("🦀")) != "🦀"'
```

### Understanding Shrinking

Proptest automatically **shrinks** failing cases to minimal examples:

```rust
// Original failing input: "aaa🦀bbb🎉ccc"
// Shrunk to: "🦀"
```

This makes debugging much easier - you get the simplest input that triggers the bug.

### Reproducing Failures

Save the seed to reproduce failures:

```bash
# Test failed with seed 1234567890
PROPTEST_SEED=1234567890 cargo test
```

Or in code:

```rust
proptest! {
    #![proptest_config(ProptestConfig {
        cases: 1000,  // Run more cases
        .. ProptestConfig::default()
    })]

    #[test]
    fn my_test(x in any::<i32>()) {
        // ...
    }
}
```

---

## Intermediate Exercise

Write property tests for this cache implementation:

```rust
pub struct Cache<K, V> {
    max_size: usize,
    data: HashMap<K, V>,
}

impl<K: Hash + Eq, V> Cache<K, V> {
    pub fn new(max_size: usize) -> Self { /* ... */ }
    pub fn insert(&mut self, key: K, value: V) -> Option<V> { /* ... */ }
    pub fn get(&self, key: &K) -> Option<&V> { /* ... */ }
    pub fn len(&self) -> usize { /* ... */ }
}
```

Properties to test:
1. Cache never exceeds max_size
2. Inserting then getting returns the inserted value
3. Length is always <= max_size

<details>
<summary>Click to see solution</summary>

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_cache_never_exceeds_max_size(
        max_size in 1usize..100,
        operations in prop::collection::vec((any::<String>(), any::<i32>()), 0..200)
    ) {
        let mut cache = Cache::new(max_size);

        for (key, value) in operations {
            cache.insert(key, value);
            prop_assert!(cache.len() <= max_size);
        }
    }

    #[test]
    fn test_insert_then_get(
        max_size in 10usize..100,
        key in ".*",
        value in any::<i32>()
    ) {
        let mut cache = Cache::new(max_size);
        cache.insert(key.clone(), value);

        prop_assert_eq!(cache.get(&key), Some(&value));
    }
}
```

</details>

---

# Advanced: Strategies, Shrinking, and Complex Properties

## Custom Shrinking Strategies

### Understanding Shrinking

When a test fails, proptest tries to find the **minimal** failing case through shrinking:

```rust
// Original failing input: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
// Shrunk to: vec![5]
```

### Custom Shrinking

```rust
use proptest::strategy::{Strategy, ValueTree};
use proptest::test_runner::TestRunner;

// Custom strategy with specific shrinking behavior
fn custom_even_number() -> impl Strategy<Value = i32> {
    (0i32..1000).prop_map(|x| x * 2)  // Only even numbers
        .prop_filter("must be divisible by 4", |x| x % 4 == 0)
}

proptest! {
    #[test]
    fn test_with_custom_strategy(n in custom_even_number()) {
        prop_assert!(n % 4 == 0);
    }
}
```

### Shrinking Complex Types

```rust
use proptest::prelude::*;

#[derive(Debug, Clone)]
struct Config {
    timeout: u64,
    retries: u32,
    buffer_size: usize,
}

// Custom strategy with coordinated shrinking
fn config_strategy() -> impl Strategy<Value = Config> {
    (
        1000u64..10000,    // timeout: 1-10 seconds
        1u32..10,          // retries: 1-10 times
        1024usize..8192,   // buffer_size: 1KB-8KB
    ).prop_map(|(timeout, retries, buffer_size)| Config {
        timeout,
        retries,
        buffer_size,
    })
}

proptest! {
    #[test]
    fn test_config_validation(config in config_strategy()) {
        prop_assert!(config.timeout >= 1000);
        prop_assert!(config.retries >= 1);
        prop_assert!(config.buffer_size >= 1024);
    }
}
```

---

## Recursive and Tree-Like Structures

### Testing Recursive Data Structures

```rust
use proptest::prelude::*;

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

fn expr_strategy() -> impl Strategy<Value = Expr> {
    let leaf = any::<i32>().prop_map(Expr::Num);

    leaf.prop_recursive(
        8,  // Max depth
        256, // Max nodes
        10,  // Expected branch size
        |inner| {
            prop_oneof![
                (inner.clone(), inner.clone())
                    .prop_map(|(l, r)| Expr::Add(Box::new(l), Box::new(r))),
                (inner.clone(), inner.clone())
                    .prop_map(|(l, r)| Expr::Mul(Box::new(l), Box::new(r))),
            ]
        },
    )
}

fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Num(n) => *n as i64,
        Expr::Add(l, r) => eval(l) + eval(r),
        Expr::Mul(l, r) => eval(l) * eval(r),
    }
}

proptest! {
    #[test]
    fn test_expr_evaluation(expr in expr_strategy()) {
        // Property: Evaluation should never panic
        let _ = eval(&expr);
    }

    #[test]
    fn test_expr_distributive(
        a in expr_strategy(),
        b in expr_strategy(),
        c in expr_strategy(),
    ) {
        // Property: a * (b + c) == (a * b) + (a * c)
        let left = Expr::Mul(
            Box::new(a.clone()),
            Box::new(Expr::Add(Box::new(b.clone()), Box::new(c.clone())))
        );

        let right = Expr::Add(
            Box::new(Expr::Mul(Box::new(a.clone()), Box::new(b))),
            Box::new(Expr::Mul(Box::new(a), Box::new(c)))
        );

        prop_assert_eq!(eval(&left), eval(&right));
    }
}
```

---

## Performance Testing with Properties

### Testing Time Complexity

```rust
use std::time::Instant;
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_sort_is_nlogn(n in 100usize..10000) {
        let data: Vec<i32> = (0..n as i32).collect();

        let start = Instant::now();
        let mut sorted = data.clone();
        sorted.sort();
        let duration = start.elapsed();

        // Property: O(n log n) means duration should grow slower than O(n²)
        // If n doubles, time should grow by ~2.1x, not 4x
        // This is a loose approximation - real benchmarks are better
        let expected_max = Duration::from_micros(n as u64 * 10);
        prop_assert!(duration < expected_max);
    }
}
```

**Note:** For serious performance testing, use `criterion` benchmarks instead of property tests.

---

## Advanced Patterns

### 1. Differential Testing

Compare two implementations:

```rust
proptest! {
    #[test]
    fn test_fast_matches_slow(input in ".*") {
        let fast_result = fast_json_parser(&input);
        let slow_result = slow_json_parser(&input);

        match (fast_result, slow_result) {
            (Ok(fast), Ok(slow)) => prop_assert_eq!(fast, slow),
            (Err(_), Err(_)) => {}, // Both failed is OK
            _ => prop_assert!(false, "One succeeded, one failed"),
        }
    }
}
```

### 2. Model-Based Testing

Use a simpler model to verify complex implementation:

```rust
use std::collections::HashMap;

// Simple model (slow but obviously correct)
struct SimpleCache {
    data: Vec<(String, i32)>,
    max_size: usize,
}

impl SimpleCache {
    fn insert(&mut self, key: String, value: i32) {
        // Remove old entry if exists
        self.data.retain(|(k, _)| k != &key);
        // Add new entry
        self.data.push((key, value));
        // Keep only last max_size entries
        if self.data.len() > self.max_size {
            self.data.remove(0);
        }
    }

    fn get(&self, key: &str) -> Option<i32> {
        self.data.iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| *v)
    }
}

// Fast implementation (complex but efficient)
struct FastCache {
    data: HashMap<String, i32>,
    order: Vec<String>,
    max_size: usize,
}

proptest! {
    #[test]
    fn test_fast_cache_matches_simple_model(
        max_size in 1usize..20,
        operations in prop::collection::vec(
            (any::<String>(), any::<i32>()),
            0..100
        )
    ) {
        let mut simple = SimpleCache { data: vec![], max_size };
        let mut fast = FastCache::new(max_size);

        for (key, value) in operations {
            simple.insert(key.clone(), value);
            fast.insert(key.clone(), value);

            // After each operation, both should agree
            prop_assert_eq!(simple.get(&key), fast.get(&key));
        }
    }
}
```

### 3. Fuzzing Integration

Use proptest as a fuzzer:

```rust
// In your fuzz target (fuzz/fuzz_target_1.rs)
#![no_main]
use libfuzzer_sys::fuzz_target;
use proptest::prelude::*;

fuzz_target!(|data: &[u8]| {
    // Convert fuzz input to your domain type
    if let Ok(s) = std::str::from_utf8(data) {
        // Property: parser never panics
        let _ = my_parser(s);
    }
});
```

---

## Configuration and Tuning

### Global Configuration

```rust
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 10000,  // Run 10,000 test cases (default: 256)
        max_shrink_iters: 10000,  // Shrink up to 10k times
        timeout: 5000,  // Timeout after 5 seconds
        .. ProptestConfig::default()
    })]

    #[test]
    fn intensive_test(x in any::<i32>()) {
        // This test runs 10,000 times
    }
}
```

### Per-Test Configuration

```rust
proptest! {
    #[test]
    fn quick_test(x in any::<i32>()) {
        // Uses default 256 cases
    }

    #[test]
    #[proptest(cases = 10000)]
    fn thorough_test(x in any::<i32>()) {
        // Runs 10,000 cases
    }
}
```

---

## Real-World Example: Testing a URL Parser

```rust
use proptest::prelude::*;
use url::Url;

// Strategy for generating valid URLs
fn url_strategy() -> impl Strategy<Value = String> {
    (
        prop_oneof!["http://", "https://"],
        "[a-z]{3,10}",  // domain
        prop_oneof!["", ":[0-9]{2,5}"],  // optional port
        prop_oneof!["", "/[a-z/]{0,20}"],  // optional path
        prop_oneof!["", "\\?[a-z]+=\\[a-z]+"],  // optional query
    ).prop_map(|(scheme, domain, port, path, query)| {
        format!("{}{}{}{}{}", scheme, domain, port, path, query)
    })
}

proptest! {
    // Property 1: Parsing then serializing gives equivalent URL
    #[test]
    fn test_url_roundtrip(url_str in url_strategy()) {
        if let Ok(url) = Url::parse(&url_str) {
            let serialized = url.as_str();
            let reparsed = Url::parse(serialized).unwrap();
            prop_assert_eq!(url, reparsed);
        }
    }

    // Property 2: Parser never panics
    #[test]
    fn test_url_parser_never_panics(s in ".*") {
        let _ = Url::parse(&s);
    }

    // Property 3: Valid URLs have required components
    #[test]
    fn test_url_has_scheme(url_str in url_strategy()) {
        if let Ok(url) = Url::parse(&url_str) {
            prop_assert!(url.scheme() == "http" || url.scheme() == "https");
        }
    }
}
```

---

## Advanced Exercise: Implement and Test a LRU Cache

Implement an LRU (Least Recently Used) cache and write comprehensive property tests:

**Requirements:**
```rust
pub struct LruCache<K, V> {
    capacity: usize,
    // Your implementation
}

impl<K: Hash + Eq + Clone, V: Clone> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self;
    pub fn get(&mut self, key: &K) -> Option<&V>;
    pub fn put(&mut self, key: K, value: V);
    pub fn len(&self) -> usize;
}
```

**Properties to test:**
1. Never exceeds capacity
2. Most recently used items are retained
3. Get updates access order
4. Putting same key updates value without increasing size

<details>
<summary>Click to see property tests</summary>

```rust
use proptest::prelude::*;
use std::collections::VecDeque;

// Simple model for verification
struct SimpleLru<K, V> {
    capacity: usize,
    items: VecDeque<(K, V)>,
}

impl<K: Eq + Clone, V: Clone> SimpleLru<K, V> {
    fn new(capacity: usize) -> Self {
        SimpleLru { capacity, items: VecDeque::new() }
    }

    fn get(&mut self, key: &K) -> Option<V> {
        if let Some(pos) = self.items.iter().position(|(k, _)| k == key) {
            let item = self.items.remove(pos).unwrap();
            self.items.push_back(item.clone());
            Some(item.1)
        } else {
            None
        }
    }

    fn put(&mut self, key: K, value: V) {
        // Remove if exists
        self.items.retain(|(k, _)| k != &key);
        // Add to back
        self.items.push_back((key, value));
        // Evict oldest if over capacity
        if self.items.len() > self.capacity {
            self.items.pop_front();
        }
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

#[derive(Debug, Clone)]
enum CacheOp {
    Get(String),
    Put(String, i32),
}

fn cache_op_strategy() -> impl Strategy<Value = CacheOp> {
    prop_oneof![
        "[a-z]{1,3}".prop_map(CacheOp::Get),
        ("[a-z]{1,3}", any::<i32>()).prop_map(|(k, v)| CacheOp::Put(k, v)),
    ]
}

proptest! {
    #[test]
    fn test_lru_matches_model(
        capacity in 1usize..20,
        operations in prop::collection::vec(cache_op_strategy(), 0..100)
    ) {
        let mut simple = SimpleLru::new(capacity);
        let mut lru = LruCache::new(capacity);

        for op in operations {
            match op {
                CacheOp::Get(key) => {
                    let simple_result = simple.get(&key);
                    let lru_result = lru.get(&key).cloned();
                    prop_assert_eq!(simple_result, lru_result);
                }
                CacheOp::Put(key, value) => {
                    simple.put(key.clone(), value);
                    lru.put(key, value);
                }
            }

            // Invariants
            prop_assert!(lru.len() <= capacity);
            prop_assert_eq!(lru.len(), simple.len());
        }
    }

    #[test]
    fn test_lru_never_exceeds_capacity(
        capacity in 1usize..50,
        operations in prop::collection::vec(
            ("[a-z]{1,5}", any::<i32>()),
            0..200
        )
    ) {
        let mut lru = LruCache::new(capacity);

        for (key, value) in operations {
            lru.put(key, value);
            prop_assert!(lru.len() <= capacity);
        }
    }

    #[test]
    fn test_lru_get_updates_order(
        capacity in 5usize..10,
        key in "[a-z]{1,3}",
        value in any::<i32>(),
        other_ops in prop::collection::vec(
            ("[a-z]{1,3}", any::<i32>()),
            5..10
        )
    ) {
        let mut lru = LruCache::new(capacity);

        // Insert target item
        lru.put(key.clone(), value);

        // Insert enough items to evict if not accessed
        for (k, v) in other_ops {
            lru.put(k, v);
        }

        // Access target - should move to front
        lru.get(&key);

        // Fill cache again
        for i in 0..capacity {
            lru.put(format!("x{}", i), i as i32);
        }

        // Target should still be there since we accessed it
        prop_assert_eq!(lru.get(&key), Some(&value));
    }
}
```

</details>

---

## Best Practices Summary

### Do's ✅

1. **Start Simple** - Begin with basic roundtrip and invariant properties
2. **Test Properties, Not Examples** - Think about what must always be true
3. **Constrain Inputs Appropriately** - Use ranges to avoid overflow/underflow
4. **Use Model-Based Testing** - Compare against a simpler correct implementation
5. **Test State Machines** - Generate sequences of operations
6. **Handle Errors Gracefully** - Don't unwrap in property tests
7. **Document Your Properties** - Explain what invariant you're testing
8. **Use Shrinking** - Rely on proptest to minimize failing cases

### Don'ts ❌

1. **Don't Reimplement the Function** - Test properties, not exact outputs
2. **Don't Ignore Failed Assumptions** - Too many `prop_assume!` calls slow tests
3. **Don't Test Business Logic** - Properties work best for mathematical/structural properties
4. **Don't Forget Edge Cases** - Empty inputs, overflow, unicode, etc.
5. **Don't Skip Error Paths** - Test both success and failure cases
6. **Don't Use Exact Equality for Floats** - Use approximate equality
7. **Don't Generate Unlimited Data** - Constrain collection sizes

---

## Further Reading

- [Proptest Book](https://altsysrq.github.io/proptest-book/intro.html)
- [Property-Based Testing Paper](https://www.cs.tufts.edu/~nr/cs257/archive/john-hughes/quick.pdf)
- [Choosing Properties for Property-Based Testing](https://fsharpforfunandprofit.com/posts/property-based-testing-2/)
- [QuickCheck (Original Haskell Library)](https://hackage.haskell.org/package/QuickCheck)

---

## Cheat Sheet

```rust
// Basic proptest setup
use proptest::prelude::*;

proptest! {
    #[test]
    fn my_test(
        // Common strategies
        any_int in any::<i32>(),
        range_int in 0i32..100,
        string in ".*",
        regex_string in "[a-z]{3,10}",
        vec in prop::collection::vec(any::<i32>(), 0..20),
        optional in prop::option::of(any::<i32>()),
    ) {
        // Assertions
        prop_assert!(condition);
        prop_assert_eq!(left, right);
        prop_assert_ne!(left, right);

        // Assumptions (skip test if false)
        prop_assume!(x != 0);
    }
}

// Custom strategies
fn my_strategy() -> impl Strategy<Value = MyType> {
    (any::<i32>(), "[a-z]+".prop_map(|name, age| MyType { name, age })
}

// Configuration
proptest! {
    #![proptest_config(ProptestConfig {
        cases: 1000,
        .. ProptestConfig::default()
    })]
}
```

---

**Happy Property Testing! 🦀**
