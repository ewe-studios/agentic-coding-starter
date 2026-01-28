# Basic Rust Implementation Template

## Purpose

This template provides a starting point for writing clean, idiomatic Rust code following the standards defined in the rust-clean-implementation skill. Use this as a foundation for new Rust projects and modules, ensuring consistency and best practices from the start.

## Key Concepts

- **Result types**: Using `Result<T, E>` for explicit error handling
- **Naming conventions**: Following Rust's snake_case and naming standards
- **Documentation**: Writing clear doc comments for public items
- **Type safety**: Leveraging Rust's type system for correctness
- **Error propagation**: Using the `?` operator for clean error flow

---

## Basic Template Structure

### Minimal Working Example

Start with this basic structure for any Rust file:

```rust
//! # Purpose (WHY) - [Module Name]
//!
//! This module [brief description of what it does and why it exists].
//! Copy this file as a starting point for new implementations.
//! Follow all standards from .agents/rules/*rust*.md

/// Example demonstrating clean, synchronous Rust implementation patterns.
///
/// # Purpose (WHY)
///
/// Demonstrates the basic structure and error handling patterns that
/// should be used throughout the codebase. This is a template for
/// creating new functions with proper documentation and error handling.
fn main() {
    // Use Result with ? operator for error handling
    match example_function() {
        Ok(result) => println!("Success: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}

/// Example function demonstrating proper error handling.
///
/// # Purpose (WHY)
///
/// Returns Result<T, E> to propagate errors explicitly rather than
/// using unwrap(). This makes error handling visible and forces
/// callers to handle potential failures.
///
/// # Returns
///
/// A success message if everything works correctly
///
/// # Errors
///
/// Returns a string error if something goes wrong
fn example_function() -> Result<String, String> {
    Ok("success".to_string())
}
```

**Key takeaways:**
- Use module-level doc comments (`//!`) to describe the file's purpose
- Every public function should have a doc comment
- Always use `Result` for functions that can fail
- Never use `unwrap()` or `expect()` in production code

---

## Function Documentation Template

### Standard Documentation Format

Use this format for all function documentation:

```rust
/// [Brief one-line description of what the function does]
///
/// # Purpose (WHY)
///
/// [Explain why this function exists, what problem it solves,
/// and any important context about design decisions]
///
/// # Arguments
///
/// * `param1` - [Description, including constraints and valid values]
/// * `param2` - [Description, including any references to related concepts]
///
/// # Returns
///
/// [What is returned on success, including any important details]
///
/// # Errors
///
/// * `ErrorType::Variant1` - [When and why this error occurs]
/// * `ErrorType::Variant2` - [When and why this error occurs]
///
/// # Panics
///
/// * [Conditions under which this function will panic, if any]
///
/// # Examples
///
/// ```
/// let result = my_function("input");
/// assert!(result.is_ok());
/// ```
fn my_function(param1: &str, param2: i32) -> Result<String, MyError> {
    // Implementation
    Ok(String::new())
}
```

---

## Error Handling Template

### Custom Error Type

Define custom error types for your module:

```rust
use std::fmt;

/// Custom error type for this module.
///
/// # Purpose (WHY)
///
/// Provides specific error variants for all possible failure modes
/// in this module, making error handling explicit and type-safe.
#[derive(Debug)]
pub enum MyError {
    /// Input validation failed
    InvalidInput { reason: String },

    /// Resource not found
    NotFound { id: u64 },

    /// Operation not permitted
    PermissionDenied,

    /// Wrapped I/O error
    Io(std::io::Error),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput { reason } => {
                write!(f, "invalid input: {}", reason)
            }
            Self::NotFound { id } => {
                write!(f, "resource with id {} not found", id)
            }
            Self::PermissionDenied => {
                write!(f, "operation not permitted")
            }
            Self::Io(e) => {
                write!(f, "I/O error: {}", e)
            }
        }
    }
}

impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

// Automatic conversion from std::io::Error
impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
```

**Key takeaways:**
- Use enums to represent different error conditions
- Always implement `Display` and `Error` traits
- Implement `From` for automatic error conversions
- Include context in error variants (like `id` or `reason`)

---

## Struct Template

### Data Types with Documentation

Define structs with proper documentation:

```rust
/// Represents a [what this struct models].
///
/// # Purpose (WHY)
///
/// [Explain why this struct exists and what abstraction it provides]
///
/// # Examples
///
/// ```
/// let user = User::new(1, "Alice".to_string());
/// println!("{}", user.name);
/// ```
#[derive(Debug, Clone)]
pub struct User {
    /// Unique identifier for this user
    pub id: u64,

    /// User's display name
    pub name: String,

    /// User's email address (must be validated before use)
    email: String,
}

impl User {
    /// Creates a new User instance.
    ///
    /// # Purpose (WHY)
    ///
    /// Provides a constructor that validates input and ensures
    /// the User is in a valid state from creation.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique user identifier
    /// * `name` - User's display name (must not be empty)
    /// * `email` - User's email (will be validated)
    ///
    /// # Returns
    ///
    /// A new User instance if validation passes
    ///
    /// # Errors
    ///
    /// * `MyError::InvalidInput` - Name is empty or email is invalid
    pub fn new(id: u64, name: String, email: String) -> Result<Self, MyError> {
        // Validate inputs
        if name.is_empty() {
            return Err(MyError::InvalidInput {
                reason: "name cannot be empty".to_string(),
            });
        }

        if !email.contains('@') {
            return Err(MyError::InvalidInput {
                reason: "invalid email format".to_string(),
            });
        }

        Ok(Self { id, name, email })
    }

    /// Gets the user's email address.
    ///
    /// # Purpose (WHY)
    ///
    /// Provides controlled access to the private email field,
    /// ensuring it can't be modified without validation.
    pub fn email(&self) -> &str {
        &self.email
    }
}
```

**Key takeaways:**
- Document each field's purpose and constraints
- Use `pub` for public fields, private for encapsulation
- Provide constructor methods with validation
- Add getter methods for private fields when needed

---

## Testing Template

### Unit Test Structure

Include tests for all functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests [specific behavior being validated].
    ///
    /// # Purpose (WHY)
    ///
    /// Ensures [what guarantee this test provides]
    #[test]
    fn test_user_creation_success() {
        let user = User::new(
            1,
            "Alice".to_string(),
            "alice@example.com".to_string(),
        );

        assert!(user.is_ok());
        let user = user.unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Alice");
        assert_eq!(user.email(), "alice@example.com");
    }

    /// Tests that empty names are rejected.
    #[test]
    fn test_user_creation_empty_name() {
        let result = User::new(
            1,
            String::new(),
            "alice@example.com".to_string(),
        );

        assert!(result.is_err());
        match result {
            Err(MyError::InvalidInput { reason }) => {
                assert!(reason.contains("name"));
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    /// Tests that invalid emails are rejected.
    #[test]
    fn test_user_creation_invalid_email() {
        let result = User::new(1, "Alice".to_string(), "not-an-email".to_string());

        assert!(result.is_err());
        match result {
            Err(MyError::InvalidInput { reason }) => {
                assert!(reason.contains("email"));
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    /// Tests error display formatting.
    #[test]
    fn test_error_display() {
        let err = MyError::NotFound { id: 42 };
        let message = err.to_string();

        assert!(message.contains("42"));
        assert!(message.contains("not found"));
    }
}
```

**Key takeaways:**
- Test both success and failure cases
- Use descriptive test names that explain what's being tested
- Document complex tests with comments
- Test edge cases and boundary conditions

---

## Module Organization

### File Structure

Organize your code into logical modules:

```rust
//! # Purpose (WHY) - User Management Module
//!
//! This module handles user creation, validation, and management.
//! It provides a clean API for working with user accounts while
//! enforcing all business rules and validation constraints.

// Re-export public types
pub use error::UserError;
pub use user::User;

// Private modules
mod error;
mod user;
mod validation;

/// Module-level configuration constants.
///
/// # Purpose (WHY)
///
/// Centralizes all configuration values to make them easy to find
/// and modify. These should be overridable via environment variables
/// in production.
pub mod config {
    /// Minimum username length
    pub const MIN_USERNAME_LENGTH: usize = 3;

    /// Maximum username length
    pub const MAX_USERNAME_LENGTH: usize = 30;

    /// Email validation regex pattern
    pub const EMAIL_PATTERN: &str = r"^[^\s@]+@[^\s@]+\.[^\s@]+$";
}
```

**Key takeaways:**
- Use module-level doc comments to describe the module's purpose
- Re-export public types for clean API
- Keep implementation details in private modules
- Use a `config` module for constants

---

## Complete Example

### Putting It All Together

Here's a complete example combining all the templates:

```rust
//! # Purpose (WHY) - User Registration Service
//!
//! This module provides user registration functionality with proper
//! validation, error handling, and security best practices.

use std::fmt;

/// Result type alias for this module.
type Result<T> = std::result::Result<T, UserError>;

/// Custom error type for user operations.
#[derive(Debug)]
pub enum UserError {
    /// Username too short
    UsernameTooShort { min_length: usize, actual: usize },

    /// Invalid email format
    InvalidEmail(String),

    /// User already exists
    DuplicateUser { username: String },
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UsernameTooShort { min_length, actual } => {
                write!(
                    f,
                    "username too short: minimum {} chars, got {}",
                    min_length, actual
                )
            }
            Self::InvalidEmail(email) => {
                write!(f, "invalid email format: {}", email)
            }
            Self::DuplicateUser { username } => {
                write!(f, "user '{}' already exists", username)
            }
        }
    }
}

impl std::error::Error for UserError {}

/// Represents a registered user.
#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
    email: String,
}

/// Service for managing user registration.
pub struct UserService {
    next_id: u64,
    // In a real implementation, this would be a database connection
}

impl UserService {
    /// Creates a new UserService instance.
    pub fn new() -> Self {
        Self { next_id: 1 }
    }

    /// Registers a new user with validation.
    ///
    /// # Purpose (WHY)
    ///
    /// Validates all input according to business rules before creating
    /// a user account. This ensures no invalid state enters the system.
    ///
    /// # Arguments
    ///
    /// * `username` - Desired username (must be 3-30 characters)
    /// * `email` - User's email address (must be valid format)
    ///
    /// # Returns
    ///
    /// The newly created User instance
    ///
    /// # Errors
    ///
    /// * `UserError::UsernameTooShort` - Username is too short
    /// * `UserError::InvalidEmail` - Email format is invalid
    pub fn register(&mut self, username: String, email: String) -> Result<User> {
        // Validate username length
        if username.len() < 3 {
            return Err(UserError::UsernameTooShort {
                min_length: 3,
                actual: username.len(),
            });
        }

        // Validate email format
        if !email.contains('@') || !email.contains('.') {
            return Err(UserError::InvalidEmail(email));
        }

        // Create user
        let user = User {
            id: self.next_id,
            username,
            email,
        };

        self.next_id += 1;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_valid_user() {
        let mut service = UserService::new();
        let result = service.register(
            "alice".to_string(),
            "alice@example.com".to_string(),
        );

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.username, "alice");
        assert_eq!(user.id, 1);
    }

    #[test]
    fn test_register_short_username() {
        let mut service = UserService::new();
        let result = service.register("ab".to_string(), "ab@example.com".to_string());

        assert!(matches!(
            result,
            Err(UserError::UsernameTooShort { min_length: 3, actual: 2 })
        ));
    }

    #[test]
    fn test_register_invalid_email() {
        let mut service = UserService::new();
        let result = service.register("alice".to_string(), "not-an-email".to_string());

        assert!(matches!(result, Err(UserError::InvalidEmail(_))));
    }
}

fn main() {
    let mut service = UserService::new();

    match service.register("alice".to_string(), "alice@example.com".to_string()) {
        Ok(user) => println!("Created user: {} (id: {})", user.username, user.id),
        Err(e) => eprintln!("Failed to create user: {}", e),
    }
}
```

---

## Quick Reference Checklist

When creating new Rust code, ensure:

- [ ] Module-level doc comment (`//!`) explaining purpose
- [ ] All public items have doc comments
- [ ] Functions return `Result<T, E>` for operations that can fail
- [ ] Custom error types with `Display` and `Error` implementations
- [ ] Input validation with clear error messages
- [ ] No `unwrap()` or `expect()` in production code
- [ ] Tests for both success and failure cases
- [ ] Following snake_case naming conventions
- [ ] Struct fields documented with their purpose
- [ ] Constants defined in a `config` module or at module level

---

## Summary

This template provides:

1. **Structure**: Consistent organization for Rust code
2. **Documentation**: Clear patterns for documenting code
3. **Error handling**: Type-safe error handling with `Result`
4. **Validation**: Input validation at boundaries
5. **Testing**: Comprehensive test coverage
6. **Type safety**: Leveraging Rust's type system

Copy this template when starting new Rust files, and adapt it to your specific needs while maintaining these core principles. Remember: good code is not just about working—it's about being clear, maintainable, and robust.
