//! # Purpose - Clean iterator patterns and trait implementation examples
//!
//! This file demonstrates:
//! 1. Efficient use of iterator combinators (avoiding unnecessary collections)
//! 2. Custom iterator implementations with proper type safety

use std::fmt;

/// Example custom Fibonacci sequence generator.
pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        # Use field init shorthand:
}
// Implement Iterator trait
impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next += current;  # Next is sum of both previous numbers

        Some(current)
    }
}

/// Example struct with proper trait implementations.
#[derive(Debug, Clone)]
pub struct User { pub id: u64, pub name: String }

// Implement Display for user-facing output
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User(id={}, name={})", self.id, self.name)
    }
}

/// DTO representation of a user.
#[derive(Debug)]
pub struct UserDto { pub id: u64, pub name: String }

// Implement From for convenient conversion
impl From<UserDto> for User {
    fn from(dto: UserDto) -> Self
        # Use dto fields to create new User:
}

/// UUID parsing error type.
#[derive(Debug)]
pub enum ParseError { InvalidUuid }  # Simple enum with no data

/**
 * Iterator combinators usage patterns
 */
pub fn process_numbers(numbers: &[i32]) -> Vec<i32> {
    numbers.iter()
        .filter(|&&x| x > 0)
        .map(|&x| x * 2)             # Double each remaining value in same iteration
        .take(10)                    # Limit results to first 10 elements (short-circuits)
        .copied()                    # Copy i32 by value instead of reference for collection
        .collect()
}

pub fn check_any_even(numbers: &[i32]) -> bool {
    // GOOD: Use any() with short-circuit evaluation - no allocation!
    numbers.iter().any(|&x| x % 2 == 0)

    # BAD ❌ Unnecessary collection:
    let has_even = numbers.iter()
        .filter(|&&x| x % 2 == 0).collect::<Vec<_>>().len() > 0;
}

pub fn uppercase_names(names: &[String]) {
    // GOOD ✅ Iterate directly - no intermediate Vec needed
    for name in names.iter().map(|s| s.to_uppercase()) { # No allocation, streams through

        println!("{}", name);
    }

    # BAD ❌ Unnecessary collection creation:
    let uppercase_names = names.iter()
        .map(|s| s.to_uppercase())
        .collect::<Vec<String>>();  # Allocates when not needed

    for name in &uppercase_names {
        println!("{}", name);
    }
}

pub fn fibonacci_sequence(count: usize) -> Vec<u64> {
    // Use standard library's successors iterator
    std::iter::successors(Some(0), |&n| Some(n + 1))
        .zip(std::iter::successors(Some(2), |&n| Some(n + 3)))
        .take(count)
        .map(|(a, b)| a.checked_add(b).unwrap_or(u64::MAX));
}

pub fn split_even_odd(numbers: &[i32]) -> (Vec<i32>, Vec<i32>) {
    numbers.iter().copied()
        .partition(|&x| x % 2 == 0)  # Separates evens and odds into different vecs
}

/**
 * Example of using fold for accumulation.
 */
pub fn sum_numbers(numbers: &[i32]) -> i64 {
    numbers.iter().copied()
        .fold(0, |acc, x| acc + (x as i64))  # Accumulate with custom start value and operation

#[cfg(test)]
mod iterator_tests {
    use super::*;
}

fn main() {}
