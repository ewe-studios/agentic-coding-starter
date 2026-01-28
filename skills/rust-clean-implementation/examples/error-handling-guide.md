# Error Handling Guide for Rust

## Purpose

This guide provides a comprehensive approach to error handling in Rust, demonstrating how to create custom error types, handle errors gracefully, and write clean, maintainable code. Proper error handling is crucial for building robust applications that fail gracefully and provide clear feedback to users and developers.

## Key Concepts

- **Custom error types**: Creating domain-specific errors with meaningful messages
- **Error propagation**: Using the `?` operator for clean error flow
- **Error conversion**: Implementing `From` trait for automatic conversions
- **Result types**: Leveraging Rust's type system for explicit error handling
- **derive_more**: Using macros to reduce boilerplate

---

## Basic Error Types

### Defining Custom Errors

Start with a simple enum that represents all possible error conditions:

```rust
use core::fmt;
use std::error::Error;

/// Domain-specific validation errors.
///
/// # Purpose (WHY)
///
/// Groups related validation failures into a single type that can be
/// matched and handled specifically by calling code. This provides
/// better type safety than using strings or generic error types.
#[derive(Debug)]
pub enum ValidationError {
    /// Username length below minimum requirement per [USERNAME_POLICY]
    UsernameTooShort { min_length: usize },

    /// Email address does not match expected format pattern (RFC 5322)
    InvalidEmailFormat(String),
}
```

**Key takeaways:**
- Use enums to represent different error conditions
- Include context data as struct fields (like `min_length`)
- Document each variant with its specific meaning

---

### Implementing Display

Make your errors user-friendly by implementing `Display`:

```rust
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::UsernameTooShort { min_length } => {
                write!(
                    f,
                    "username is too short (minimum {} characters)",
                    min_length
                )
            }
            ValidationError::InvalidEmailFormat(email) => {
                write!(f, "'{}' is not a valid email address", email)
            }
        }
    }
}

impl std::error::Error for ValidationError {}
```

**Key takeaways:**
- Provide clear, actionable error messages
- Include relevant context from the error variant
- Always implement both `Display` and `Error` traits

---

## Using derive_more for Less Boilerplate

The `derive_more` crate can reduce boilerplate code:

```rust
use derive_more::{Display, From};

/// Domain-specific validation errors with automatic Display implementation.
///
/// # Purpose (WHY)
///
/// Using derive_more reduces boilerplate while maintaining the same
/// functionality. The Display implementation is generated based on
/// the structure of each variant.
#[derive(Debug, Display, From)]
pub enum ValidationError {
    /// Username length below minimum requirement
    #[display(fmt = "username is too short (minimum {} characters)", min_length)]
    UsernameTooShort { min_length: usize },

    /// Email address does not match expected format
    #[display(fmt = "'{}' is not a valid email address", _0)]
    InvalidEmailFormat(String),
}

impl std::error::Error for ValidationError {}
```

**Key takeaways:**
- `derive_more::Display` generates `Display` implementation from attributes
- Use `#[display(fmt = "...")]` to customize messages
- `#[from(ignore)]` skips automatic `From` implementations when needed

---

## Error Propagation with ?

### Basic Usage

The `?` operator makes error propagation clean and readable:

```rust
pub struct UserService {
    // Database connection or other state
}

impl UserService {
    /// Registers a new user account.
    ///
    /// # Purpose (WHY)
    ///
    /// The validation layer is a safety mechanism against invalid state
    /// entering persistent storage. This ensures all business rules are
    /// enforced before data reaches the database.
    ///
    /// # Arguments
    ///
    /// * `username` - Unique username string; must meet [USERNAME_POLICY] constraints
    /// * `email` - User email address for notification purposes
    ///
    /// # Returns
    ///
    /// The newly created user's unique identifier on success
    ///
    /// # Errors
    ///
    /// * `ValidationError::UsernameTooShort` - Username has fewer than 3 characters
    /// * `ValidationError::InvalidEmailFormat` - Email format is invalid
    pub fn register(&self, username: &str, email: &str)
        -> Result<u64, ValidationError>
    {
        // Validate username length per USERNAME_POLICY
        // (minimum 3 characters for usability and spam prevention)
        if username.len() < 3 {
            return Err(ValidationError::UsernameTooShort { min_length: 3 });
        }

        // The ? operator propagates errors automatically
        self.validate_email(email)?;

        // In a real implementation, insert into database
        Ok(1)
    }

    /// Validates email format.
    ///
    /// # Arguments
    ///
    /// * `email` - Email address to validate
    ///
    /// # Returns
    ///
    /// `Ok(())` on valid format; error with specific details otherwise
    fn validate_email(&self, email: &str) -> Result<(), ValidationError> {
        // Simple heuristic check for common issues
        if !email.contains('@') || !email.contains('.') {
            return Err(ValidationError::InvalidEmailFormat(email.to_string()));
        }
        Ok(())
    }
}
```

**Key takeaways:**
- Use `?` to propagate errors up the call stack
- Return specific error types rather than generic ones
- Validate early and fail fast with clear messages

---

## Wrapping System Errors

### Adding Context to I/O Errors

Wrap standard library errors to add domain-specific context:

```rust
/// Custom IO error type for file operations.
///
/// # Purpose (WHY)
///
/// Provides context about which path failed and what operation was attempted,
/// making it easier to debug filesystem issues without inspecting stack traces.
#[derive(Debug)]
pub struct IoError {
    /// The specific underlying I/O error from the system call
    pub source: std::io::Error,

    /// Path that was being accessed when the error occurred
    pub path: String,
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to access '{}': {}", self.path, self.source)
    }
}

impl std::error::Error for IoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}
```

**Usage with map_err:**

```rust
pub struct DataProcessor;

impl DataProcessor {
    /// Loads and parses a data file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to input data file; must exist on filesystem
    ///
    /// # Returns
    ///
    /// Parsed content as string if successful
    ///
    /// # Errors
    ///
    /// * `IoError` - File cannot be read or doesn't exist
    pub fn load_file(&self, path: &str) -> Result<String, IoError> {
        // Use map_err to add context at the error site
        let content = std::fs::read_to_string(path)
            .map_err(|e| IoError {
                source: e,
                path: path.to_string()
            })?;

        self.parse_content(&content)
    }

    /// Parses the file contents into structured data.
    ///
    /// # Arguments
    ///
    /// * `content` - Raw text content from [Self::load_file]
    ///
    /// # Returns
    ///
    /// Parsed string representation on success
    fn parse_content(&self, content: &str) -> Result<String, IoError> {
        // Check for required marker in file (a simple validation heuristic)
        if !content.contains("valid") {
            // For demonstration - in real code, use a proper ParseError type
            return Err(IoError {
                source: std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "missing 'valid' marker"
                ),
                path: "(content)".to_string(),
            });
        }
        Ok(content.to_string())
    }
}
```

**Key takeaways:**
- Use `map_err` to transform errors and add context
- Preserve the original error with the `source()` method
- Add relevant information like file paths or operation types

---

## Unified Error Types

### Combining Multiple Error Sources

Create a unified error type that can represent errors from multiple sources:

```rust
use derive_more::From;

/// Unified error type for the entire application.
///
/// # Purpose (WHY)
///
/// Provides a single error type that can represent all possible failures
/// in the application, making it easier to return errors from functions
/// that might fail in multiple ways.
#[derive(Debug, Display, From)]
pub enum AppError {
    /// Validation error from user input
    #[display(fmt = "validation error: {}", _0)]
    Validation(ValidationError),

    /// I/O error from file operations
    #[display(fmt = "I/O error: {}", _0)]
    Io(IoError),

    /// Database error
    #[display(fmt = "database error: {}", _0)]
    Database(String),

    /// Generic error for other cases
    #[display(fmt = "error: {}", _0)]
    Other(String),
}

impl std::error::Error for AppError {}
```

**Using the unified error type:**

```rust
pub struct Application {
    user_service: UserService,
    data_processor: DataProcessor,
}

impl Application {
    /// Processes user registration and loads their data file.
    ///
    /// # Purpose (WHY)
    ///
    /// Demonstrates handling multiple error types in a single function
    /// by using a unified error type. The `From` trait automatically
    /// converts specific errors to AppError.
    pub fn process_user_registration(
        &self,
        username: &str,
        email: &str,
        data_file: &str,
    ) -> Result<String, AppError> {
        // ValidationError automatically converts to AppError via From trait
        let user_id = self.user_service.register(username, email)?;

        // IoError automatically converts to AppError via From trait
        let data = self.data_processor.load_file(data_file)?;

        Ok(format!("User {} registered with data: {}", user_id, data))
    }
}
```

**Key takeaways:**
- Use `derive_more::From` to automatically implement conversions
- Create unified error types for application boundaries
- The `?` operator uses `From` to convert errors automatically

---

## Error Handling Patterns

### Early Return Pattern

Validate input and return early on errors:

```rust
pub fn process_input(input: &str) -> Result<String, ValidationError> {
    // Check 1: Not empty
    if input.is_empty() {
        return Err(ValidationError::InvalidEmailFormat(
            "input cannot be empty".to_string()
        ));
    }

    // Check 2: Length constraints
    if input.len() < 3 {
        return Err(ValidationError::UsernameTooShort { min_length: 3 });
    }

    // Check 3: Format validation
    if !input.contains('@') {
        return Err(ValidationError::InvalidEmailFormat(input.to_string()));
    }

    // All checks passed
    Ok(input.trim().to_string())
}
```

**Key takeaways:**
- Fail fast with early returns
- Validate in order of cheapest to most expensive checks
- Provide specific error types for each validation

---

### Chaining Operations

Chain multiple fallible operations cleanly:

```rust
pub fn process_user_data(user_id: u64, data_path: &str)
    -> Result<String, AppError>
{
    let processor = DataProcessor;

    // Chain multiple operations that can fail
    let content = processor.load_file(data_path)?;
    let parsed = processor.parse_content(&content)?;

    Ok(format!("Processed data for user {}: {}", user_id, parsed))
}
```

**Key takeaways:**
- Use `?` to chain operations that return `Result`
- Each operation can fail independently
- Errors are automatically converted via `From` trait

---

## Testing Error Handling

### Testing Error Cases

Always test both success and failure paths:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_with_valid_input() {
        let service = UserService;
        let result = service.register("alice", "alice@example.com");
        assert!(result.is_ok());
    }

    #[test]
    fn test_register_with_short_username() {
        let service = UserService;
        let result = service.register("ab", "alice@example.com");

        assert!(result.is_err());
        let err = result.unwrap_err();

        // Check that we got the right error variant
        match err {
            ValidationError::UsernameTooShort { min_length } => {
                assert_eq!(min_length, 3);
            }
            _ => panic!("Expected UsernameTooShort error"),
        }
    }

    #[test]
    fn test_register_with_invalid_email() {
        let service = UserService;
        let result = service.register("alice", "not-an-email");

        assert!(result.is_err());
        match result.unwrap_err() {
            ValidationError::InvalidEmailFormat(email) => {
                assert_eq!(email, "not-an-email");
            }
            _ => panic!("Expected InvalidEmailFormat error"),
        }
    }

    #[test]
    fn test_error_display() {
        let err = ValidationError::UsernameTooShort { min_length: 3 };
        let message = err.to_string();

        assert!(message.contains("too short"));
        assert!(message.contains("3"));
    }
}
```

**Key takeaways:**
- Test all error conditions, not just success cases
- Use pattern matching to verify error variants
- Test error messages for clarity and usefulness

---

## Common Pitfalls to Avoid

### ❌ Don't Use unwrap() in Production Code

```rust
// BAD: This will panic if the file doesn't exist
let content = std::fs::read_to_string(path).unwrap();

// GOOD: Handle the error properly
let content = std::fs::read_to_string(path)
    .map_err(|e| IoError { source: e, path: path.to_string() })?;
```

### ❌ Don't Use Generic String Errors

```rust
// BAD: String errors lose type information
fn validate(input: &str) -> Result<(), String> {
    if input.is_empty() {
        return Err("input is empty".to_string());
    }
    Ok(())
}

// GOOD: Use custom error types
fn validate(input: &str) -> Result<(), ValidationError> {
    if input.is_empty() {
        return Err(ValidationError::InvalidEmailFormat(
            "input is empty".to_string()
        ));
    }
    Ok(())
}
```

### ❌ Don't Ignore Errors

```rust
// BAD: Silently ignoring errors
let _ = do_something_that_might_fail();

// GOOD: Handle or propagate errors
do_something_that_might_fail()?;
```

---

## Summary

Effective error handling in Rust involves:

1. **Custom error types**: Use enums to represent specific error conditions
2. **Clear messages**: Implement `Display` with helpful, actionable messages
3. **Error context**: Wrap system errors with domain-specific information
4. **Type safety**: Use `Result<T, E>` for explicit error handling
5. **Error propagation**: Use `?` operator for clean error flow
6. **Testing**: Test both success and failure paths thoroughly

By following these patterns, you'll create robust applications that fail gracefully and provide clear feedback when things go wrong. Remember: good error handling is not just about catching errors—it's about guiding users and developers toward solutions.
