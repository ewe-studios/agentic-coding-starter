# Iterator Patterns in Rust

## Purpose

This guide demonstrates clean, efficient iterator patterns in Rust. Iterators are one of Rust's most powerful features, enabling expressive, zero-cost abstractions for data processing. Understanding when to use iterator combinators versus collecting into collections is essential for writing performant, idiomatic Rust code.

## Key Concepts

- **Iterator combinators**: Chaining operations like `map`, `filter`, `take` for elegant data processing
- **Lazy evaluation**: Iterators don't compute until consumed, enabling short-circuit optimization
- **Custom iterators**: Implementing the `Iterator` trait for your own types
- **Avoiding unnecessary allocations**: Using iterators directly instead of collecting into vectors
- **Trait implementations**: Implementing `Display`, `From`, and other traits correctly

---

## Iterator Combinators

### Chaining Operations

Iterator combinators allow you to build complex data transformations from simple operations:

```rust
/// Processes numbers using efficient iterator combinators.
///
/// # Purpose (WHY)
///
/// Demonstrates the power of iterator chaining. Each operation
/// (filter, map, take) is applied lazily without intermediate
/// allocations. Only the final `collect()` allocates memory.
///
/// # Arguments
///
/// * `numbers` - Slice of numbers to process
///
/// # Returns
///
/// Vec containing up to 10 positive numbers, each doubled
pub fn process_numbers(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()                      // Iterate over slice
        .filter(|&&x| x > 0)        // Keep only positive numbers
        .map(|&x| x * 2)            // Double each value
        .take(10)                   // Take first 10 (short-circuits!)
        .collect()                  // Collect into Vec
}
```

**Key takeaways:**
- Chain operations for readability and performance
- Operations are lazy—they don't run until `collect()` or similar
- `take(n)` short-circuits, stopping iteration early
- Only one allocation at the end with `collect()`

---

### Short-Circuit Evaluation

Use methods like `any`, `all`, and `find` to avoid unnecessary work:

```rust
/// Checks if any number is even using short-circuit evaluation.
///
/// # Purpose (WHY)
///
/// Demonstrates how `any()` stops as soon as it finds a match,
/// avoiding unnecessary iteration. This is much more efficient
/// than collecting and checking length.
///
/// # Arguments
///
/// * `numbers` - Slice of numbers to check
///
/// # Returns
///
/// true if at least one even number exists
pub fn check_any_even(numbers: &[i32]) -> bool {
    // GOOD ✅: Use any() - stops at first match, no allocation
    numbers.iter().any(|&x| x % 2 == 0)
}

/// BAD EXAMPLE - DO NOT USE
///
/// This example shows what NOT to do: creating unnecessary collections.
#[allow(dead_code)]
fn check_any_even_bad(numbers: &[i32]) -> bool {
    // BAD ❌: Filters entire iterator, collects into Vec, checks length
    // This allocates memory and processes every element!
    numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .collect::<Vec<_>>()
        .len()
        > 0
}
```

**Key takeaways:**
- Use `any()`, `all()`, `find()` for early termination
- Avoid unnecessary `collect()` calls
- Short-circuiting methods are both cleaner and faster

---

### Avoiding Unnecessary Collections

Don't collect into a Vec when you can iterate directly:

```rust
/// Prints uppercase names directly from iterator.
///
/// # Purpose (WHY)
///
/// Demonstrates streaming iteration without intermediate allocation.
/// The `map()` creates an iterator that lazily transforms each name,
/// and the for loop consumes it one item at a time.
///
/// # Arguments
///
/// * `names` - Slice of names to process
pub fn uppercase_names(names: &[String]) {
    // GOOD ✅: Iterate directly - no intermediate Vec
    for name in names.iter().map(|s| s.to_uppercase()) {
        println!("{}", name);
    }
}

/// BAD EXAMPLE - DO NOT USE
///
/// Creates an unnecessary intermediate collection.
#[allow(dead_code)]
fn uppercase_names_bad(names: &[String]) {
    // BAD ❌: Allocates a Vec when not needed
    let uppercase_names: Vec<String> = names
        .iter()
        .map(|s| s.to_uppercase())
        .collect();  // Unnecessary allocation!

    for name in &uppercase_names {
        println!("{}", name);
    }
}
```

**Key takeaways:**
- Only `collect()` when you need to store results
- Use iterators directly in for loops when possible
- Map operations are lazy and efficient

---

### Partitioning Data

Split data into multiple collections efficiently:

```rust
/// Splits numbers into even and odd vectors.
///
/// # Purpose (WHY)
///
/// Uses `partition()` to separate items into two collections
/// in a single pass. This is more efficient than filtering
/// twice or manually building two vectors.
///
/// # Arguments
///
/// * `numbers` - Slice of numbers to split
///
/// # Returns
///
/// Tuple of (even_numbers, odd_numbers)
pub fn split_even_odd(numbers: &[i32]) -> (Vec<i32>, Vec<i32>) {
    numbers
        .iter()
        .copied()  // Copy i32 values (cheap)
        .partition(|&x| x % 2 == 0)  // True goes to first Vec, false to second
}
```

**Key takeaways:**
- `partition()` splits into two collections in one pass
- More efficient than filtering twice
- Works with any predicate function

---

### Folding and Accumulation

Use `fold()` for custom accumulation logic:

```rust
/// Sums numbers with custom accumulation.
///
/// # Purpose (WHY)
///
/// Demonstrates `fold()` for accumulation with a custom starting value
/// and operation. While `sum()` exists for this specific case, `fold()`
/// is useful for more complex accumulations.
///
/// # Arguments
///
/// * `numbers` - Slice of numbers to sum
///
/// # Returns
///
/// Sum of all numbers as i64
pub fn sum_numbers(numbers: &[i32]) -> i64 {
    numbers
        .iter()
        .copied()
        .fold(0, |acc, x| acc + (x as i64))  // Start at 0, add each element
}

/// Finds the maximum value with fold.
///
/// # Purpose (WHY)
///
/// Shows a more complex fold operation. While `max()` exists,
/// this demonstrates how fold can implement custom logic.
pub fn find_max(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        return None;
    }

    Some(numbers.iter().copied().fold(i32::MIN, |max, x| {
        if x > max { x } else { max }
    }))
}
```

**Key takeaways:**
- `fold()` takes an initial value and accumulation function
- Useful for custom reduction operations
- Consider using specialized methods like `sum()`, `max()` when available

---

## Custom Iterators

### Implementing the Iterator Trait

Create your own iterators by implementing the `Iterator` trait:

```rust
use std::fmt;

/// Custom Fibonacci sequence generator.
///
/// # Purpose (WHY)
///
/// Demonstrates how to implement a custom iterator that generates
/// values on-demand. This is memory-efficient because it doesn't
/// store the entire sequence, only the current state.
pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    /// Creates a new Fibonacci iterator starting at 0, 1.
    pub fn new() -> Self {
        Self { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        // Calculate next values
        self.curr = self.next;
        self.next = current + self.next;

        Some(current)
    }
}

/// Usage example: Generate first 10 Fibonacci numbers.
pub fn fibonacci_sequence(count: usize) -> Vec<u64> {
    Fibonacci::new()
        .take(count)
        .collect()
}
```

**Key takeaways:**
- Implement `Iterator` by defining `type Item` and `next()`
- Store minimal state needed to generate next item
- Return `None` when iteration is complete (or `Some` for infinite iterators)
- Automatically get all iterator combinators for free!

---

### Iterator with Transformation

Create iterators that transform data on the fly:

```rust
/// Custom iterator that yields squared values.
///
/// # Purpose (WHY)
///
/// Demonstrates wrapping an existing iterator and transforming
/// its values. This is more efficient than collecting and
/// mapping in a separate step.
pub struct Squared<I> {
    inner: I,
}

impl<I> Squared<I> {
    pub fn new(inner: I) -> Self {
        Self { inner }
    }
}

impl<I> Iterator for Squared<I>
where
    I: Iterator<Item = i32>,
{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|x| x * x)
    }
}

/// Usage example.
pub fn squared_numbers(numbers: &[i32]) -> Vec<i32> {
    Squared::new(numbers.iter().copied())
        .take(5)
        .collect()
}
```

**Key takeaways:**
- You can wrap existing iterators to transform them
- Use generic type parameters to work with any iterator
- Leverage `map()` on `Option` for clean transformations

---

## Trait Implementations

### Display Trait

Implement `Display` for user-facing output:

```rust
/// Example user struct with proper trait implementations.
///
/// # Purpose (WHY)
///
/// Demonstrates proper separation between `Debug` (for developers)
/// and `Display` (for users). Debug is auto-derived, Display is
/// implemented manually for a nice user-facing format.
#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User #{}: {}", self.id, self.name)
    }
}
```

**Key takeaways:**
- `Debug` is for developers (use `#[derive(Debug)]`)
- `Display` is for users (implement manually)
- Use `write!` macro to format output

---

### From Trait for Conversions

Implement `From` for type conversions:

```rust
/// Data Transfer Object for User.
///
/// # Purpose (WHY)
///
/// DTOs are used for serialization/API boundaries. The From trait
/// provides a standard way to convert between DTO and domain types.
#[derive(Debug)]
pub struct UserDto {
    pub id: u64,
    pub name: String,
}

impl From<UserDto> for User {
    fn from(dto: UserDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
        }
    }
}

/// Usage example.
pub fn convert_dto(dto: UserDto) -> User {
    // Can use into() due to From implementation
    dto.into()
}
```

**Key takeaways:**
- Implement `From` instead of `Into` (you get both)
- Use for infallible conversions
- Consider `TryFrom` for fallible conversions

---

## Practical Examples

### Chaining Multiple Operations

Real-world example with multiple transformations:

```rust
#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub amount: f64,
    pub status: OrderStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderStatus {
    Pending,
    Completed,
    Cancelled,
}

/// Calculates total revenue from completed orders.
///
/// # Purpose (WHY)
///
/// Demonstrates real-world iterator usage: filtering by status,
/// extracting amounts, and summing. This is done in one efficient
/// pass without intermediate allocations.
pub fn calculate_revenue(orders: &[Order]) -> f64 {
    orders
        .iter()
        .filter(|order| order.status == OrderStatus::Completed)
        .map(|order| order.amount)
        .sum()  // Specialized method for numeric types
}

/// Finds high-value pending orders.
///
/// # Purpose (WHY)
///
/// Shows how to combine filtering conditions and limit results.
/// Only collects the final filtered set, not intermediate results.
pub fn high_value_pending_orders(orders: &[Order], threshold: f64) -> Vec<Order> {
    orders
        .iter()
        .filter(|order| {
            order.status == OrderStatus::Pending
                && order.amount >= threshold
        })
        .cloned()  // Clone the orders for the Vec
        .take(10)  // Limit to first 10
        .collect()
}
```

**Key takeaways:**
- Combine multiple filters for complex conditions
- Use `sum()` for numeric accumulation
- `cloned()` is needed when collecting owned values from references

---

### Working with Results

Handle errors in iterators gracefully:

```rust
/// Parses a list of number strings.
///
/// # Purpose (WHY)
///
/// Demonstrates error handling in iterators. The `collect()` method
/// can collect into `Result<Vec<_>, _>`, stopping at the first error.
pub fn parse_numbers(inputs: &[&str]) -> Result<Vec<i32>, std::num::ParseIntError> {
    inputs
        .iter()
        .map(|s| s.parse::<i32>())
        .collect()  // Collects into Result<Vec<i32>, ParseIntError>
}

/// Filters out invalid numbers instead of failing.
///
/// # Purpose (WHY)
///
/// Shows how to use `filter_map` to skip errors rather than
/// propagating them. Useful when you want best-effort processing.
pub fn parse_numbers_lenient(inputs: &[&str]) -> Vec<i32> {
    inputs
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())  // Keep only successful parses
        .collect()
}
```

**Key takeaways:**
- `collect()` can turn `Iterator<Result<T, E>>` into `Result<Vec<T>, E>`
- Use `filter_map` with `.ok()` to skip errors
- Choose fail-fast vs. lenient based on requirements

---

## Performance Tips

### When to Collect

```rust
/// Examples of when to collect vs. when to iterate directly.

// ✅ COLLECT: You need the data multiple times
pub fn reuse_data(numbers: &[i32]) -> (i32, i32) {
    let positives: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x > 0)
        .copied()
        .collect();

    let sum: i32 = positives.iter().sum();
    let count = positives.len();
    (sum, count)
}

// ✅ NO COLLECT: Single pass, used once
pub fn single_pass(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .filter(|&&x| x > 0)
        .map(|&x| x * 2)
        .sum()  // No collect needed!
}

// ✅ COLLECT: Need to return owned data
pub fn return_filtered(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .filter(|&&x| x > 0)
        .copied()
        .collect()  // Necessary to return Vec
}
```

**Key takeaways:**
- Collect when you need data multiple times
- Collect when returning owned data from a function
- Skip collect for single-pass operations
- Skip collect when just printing or checking conditions

---

## Testing Iterators

Always test your iterator implementations:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_first_five() {
        let fib: Vec<u64> = Fibonacci::new().take(5).collect();
        assert_eq!(fib, vec![0, 1, 1, 2, 3]);
    }

    #[test]
    fn test_process_numbers() {
        let numbers = vec![-2, -1, 0, 1, 2, 3, 4, 5];
        let result = process_numbers(&numbers);

        assert_eq!(result, vec![2, 4, 6, 8, 10]);  // First 5 positives, doubled
    }

    #[test]
    fn test_any_even() {
        assert!(check_any_even(&[1, 2, 3]));
        assert!(!check_any_even(&[1, 3, 5]));
        assert!(!check_any_even(&[]));
    }

    #[test]
    fn test_split_even_odd() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let (evens, odds) = split_even_odd(&numbers);

        assert_eq!(evens, vec![2, 4, 6]);
        assert_eq!(odds, vec![1, 3, 5]);
    }
}
```

**Key takeaways:**
- Test edge cases (empty inputs, single items)
- Test the happy path with expected outputs
- Test custom iterators with known sequences

---

## Summary

Mastering iterator patterns in Rust involves:

1. **Iterator combinators**: Use `map`, `filter`, `take` for expressive data processing
2. **Lazy evaluation**: Operations don't run until consumed, enabling optimization
3. **Short-circuit evaluation**: Use `any`, `all`, `find` to stop early
4. **Avoid unnecessary allocations**: Iterate directly instead of collecting
5. **Custom iterators**: Implement `Iterator` trait for your own types
6. **Proper trait implementations**: Implement `Display`, `From`, and others correctly
7. **Error handling**: Use `collect()` with `Result` or `filter_map` with `.ok()`

By leveraging iterators effectively, you'll write code that is both elegant and performant, taking full advantage of Rust's zero-cost abstractions.
