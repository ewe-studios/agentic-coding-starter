---
name: "Rust Testing Excellence"
description: "Writing proper, clear and correct Rust tests. Understanding that useless tests with unvalidated values or muted variables are bad; knowing what really good tests look like."
approved: No
created: 2026-01-27
license: "MIT"
metadata:
  author: "Main Agent"
  version: "1.0"
  last_updated: "2026-01-27"
  tags:
    - rust
    - testing
    - unit-tests
tools:
  - Rust
  - Cargo test
files:
  - examples/bad-test-patterns.rs: "Examples of what NOT to do in tests"
  - examples/good-test-structure.rs: "Example of proper test structure and validation"
assets:
---
# Testing Excellence in Rust

## Overview

Good testing requires more than just calling functions with arguments. This skill covers how to write **meaningful, validated** tests that actually verify behavior rather than just asserting side effects.

A useless test has one or more of these flaws:
- No actual assertions (just calls a function and checks it doesn't panic)
- Uses hardcoded/muted variables without validating inputs
- Tests implementation details instead of observable behavior

Good tests validate both **correctness** (the result is what we expect) and **robustness** (error paths work correctly).

## When to Use This Skill

- Writing new unit tests for Rust modules
- Reviewing existing test suites for quality issues
- Implementing integration or property-based tests
- Refactoring code while ensuring proper test coverage remains intact

## Prerequisites

- Understanding of `#[cfg(test)]` module structure and where to place different types of tests (unit, integration, benchmarks)
- Familiarity with the Rust testing framework macros: `assert_eq!`, `assert!`, `unwrap()`
- Basic knowledge of async/await for tokio-based tests if applicable

## Core Concepts

### The Three Test Validations

Every meaningful test must validate:

1. **Input Validation** - Verify inputs are handled correctly (valid and invalid cases)
2. **Output Verification** - Confirm the output matches expected behavior
3. **Error Path Testing** - Ensure error conditions produce appropriate results/errors

### Good Tests Validate Input, Not Just Output

```rust
// BAD: No validation of input or assertions about what happens with bad data
#[test]
fn test_process() {
    let result = process("valid_input").unwrap(); // Assumes success!
}

// GOOD: Validates both valid and invalid inputs produce correct results/errors
#[test]
fn test_process_validation() {
    assert!(process("valid_input")?.is_some());
    assert_eq!(
        process(None),
        Err(Error::MissingInput)
    );
}
```

### Meaningful Assertions

Assertions should verify observable behavior, not implementation details:

```rust
// GOOD: Asserts the actual output value is correct
assert_eq!(calculate(2 + 3), Some(10));

// BAD: Tests an internal detail (implementation leakage)
let state = mock_obj.state();
assert_eq!(state.last_call_args(), "expected");
```

## Step-by-Step Guide

### Step 1: Place tests in the Correct Location

```rust
#[cfg(test)]
mod tests {
    // Unit tests go here, inside #[cfg(test)] module of crate being tested
}

// Integration tests at ./tests/ directory (project root)
fn main() -> std::io::Result<()> { /* ... */ }
```

### Step 2: Feature-Gate Test Modules

Use descriptive names and feature flags rather than individual cfg attributes:

```rust
#[cfg(test)]
mod input_validation_tests {
    #[test]
    fn test_empty_input_returns_error() {}
}

// Use a module-level gate instead of cluttering each test:
impl MyModule {
    #[cfg(feature = "validation-tests")]
    mod validation_suite { /* ... */ }
}
```

### Step 3: Test Both Valid and Invalid Inputs

```rust
#[test]
fn test_user_validation_valid_cases() {
    for name in ["alice", "bob"] {
        let user = User::new(name);
        assert!(user.is_valid());
        assert_eq!(user.name(), name.to_string());
    }
}

#[test]
fn test_user_validation_invalid_cases() {
    // Test empty, whitespace-only, and oversized inputs
    assert!(User::new("").is_err());  // Empty fails

    for invalid in ["", "   ", str_repeat(1000)] {  // Whitespace only
        let result = User::new(invalid);
        if cfg!(test) {
            println!("Test input: {:?}", invalid);  // Document what we're testing!
        }
        assert!(result.is_err());
    }

    for i in [101, 102] {  // Out of range values
        assert_eq!(
            get_value(i),
            Err(Error::InvalidRange)
        );
    }
}
```

### Step 4: Use Property-Based Testing for Edge Cases

```rust
#[cfg(test)]
mod proptest_tests {
    use proptest::prelude::*;

    proptest! {
        // Test that valid input always produces a result within bounds
        #[test]
        fn test_valid_inputs_produce_valid_outputs(
            name in "[a-zA-Z]+".prop(),
            value in 0i32..100,
        ) {
            let user = User::new(&name);
            prop_assert!(user.is_ok());
            assert_eq!(
                process_value(user.unwrap(), value),
                Ok(value)
            );
        }
    }

    // Test that certain properties hold for all inputs
    #[test]
    fn test_idempotency(input in "[a-zA-Z]+".prop()) {
        let first = compute_hash(&input);
        let second = compute_hash(&input);  // Should produce same result

        prop_assert_eq!(first, second,
            "Hash computation should be deterministic for the same input");
    }
}
```

### Step 5: Document Test Inputs and Expected Behavior

```rust
#[test]
fn test_parse_invalid_json() {
    let inputs = vec![
        ("{invalid json}", Error::ParseError),
        (r#"{"key": "value""#, Error::Incomplete),  // Missing closing brace
        ("", None),                                // Empty string returns error or empty result?
    ];

    for (input, expected) in inputs {
        let result = parse_json(input);
        assert_eq!(
            result.err(),
            Some(expected),
            "Input {:?} should produce {}",
            input,
            expected  // Document what we're validating!
        );
    }
}
```

## Common Patterns

### Pattern 1: Test Helper Functions for Reusable Assertions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn assert_valid_user(name: &str) -> User {
        let user = User::new(name).expect("Test setup failed");
        assert!(user.is_valid(), "User {} should be valid", name);
        user
    }

    #[test]
    fn test_normal_cases() {
        for name in ["alice", "bob"] {
            _ = assert_valid_user(name);  // Reusable validation helper!
        }
    }
}
```

### Pattern 2: Test Error Messages Are Descriptive

```rust
#[test]
fn test_error_messages_include_context() {
    let result = process_data("");
    if let Err(e) = result {
        // Verify error message contains relevant context, not just "error"
        assert!(
            e.to_string().contains("empty input"),
            "Error should explain the problem: {}",
            e
        );
    } else {
        panic!("Expected an error for empty input");
    }
}
```

### Pattern 3: Async Test with Proper Setup

```rust
#[tokio::test]
async fn test_async_operation_with_timeout() {
    let future = async_task();
    tokio::time::timeout(Duration::from_millis(100), future)
        .await
        .expect("Operation should complete within timeout");

    // Verify the result is correct, not just that it didn't panic or time out!
}
```

## Pitfalls to Avoid

### ❌ Bad: Tests Without Assertions (Muted Variables)

```rust
// BAD: Just calling function with no assertions about what happens after
#[test]
fn test_process() {
    let input = "valid_input";
    process(input).unwrap();  // Assumes success, doesn't verify anything!
}
```

### ❌ Bad: Hardcoded Input Without Validation

```rust
let data = vec![1usize; 1000];  // Magic number without explanation!

#[test]
fn test_large_data() {
    let result = heavy_operation(&data).unwrap();  // Assumes success for ALL inputs?
    assert!(result.is_ok());
}
```

### ❌ Bad: Testing Implementation Details

```rust
// BAD: Tests specific implementation (how, not what)
let internal_state = mock_obj.get_internal_map();
assert_eq!(*internal_state.last_key(), "expected");

// GOOD: Test observable behavior only
mock_obj.process("input");
assert!(mock_obj.final_result().contains("output"));
```

### ❌ Bad: No Error Path Testing

```rust
#[test]
fn test_valid_input() {
    assert_ok!(process(valid_data));  // Only tests success path!
}
// Never tested what happens with None, "", or invalid values...
```

## Examples

**Good Test Structure Example:**

```rust
use std::collections::HashMap;

/// # Good Unit Test Pattern
#[cfg(test)]
mod input_validation_tests {
    use super::*;

    #[test]
    fn test_valid_inputs_produce_correct_output() {
        let cases = vec![
            ("alice", 30, "valid@example.com"),
            ("bob", 25, "another@test.org"),
        ];

        for (name, age, email) in cases {
            assert!(validate_user_input(name, age, email).is_ok(),
                "Input ({}, {}, {}) should be valid",
                name,
                age
            );

            let user = validate_user_input(name, age, email).unwrap();
            assert_eq!(&user.name(), name);
        }
    }

    #[test]
    fn test_invalid_inputs_produce_errors() {
        // Test empty input case
        assert!(validate_user_input("", 30, "email@example.com").is_err());

        // Test out-of-range age with specific error type
        let result = validate_age(200).unwrap();
        match result {
            UserError::InvalidAge => { /* correct */ }
            _ => panic!("Expected InvalidAge for age=200"),
        }

        // Test malformed email produces appropriate parse error
        assert!(validate_email("not_an_email").is_err());
    }
}
```

## References

- [`rust.md`](../stacks/rust.md) - Comprehensive Rust testing conventions (MANDATORY reading)
- The `proptest` crate documentation: https://docs.rs/proptest/latest/
- Official Rust Testing Guide: https://doc.rust-lang.org/book/ch11-00-testing.html
- Tokio async test patterns if using tokio

---

*Created: 2026-01-27*
