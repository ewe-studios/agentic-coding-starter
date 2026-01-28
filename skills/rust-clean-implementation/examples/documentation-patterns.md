# Documentation Patterns in Rust

## Purpose

This guide demonstrates how to write clear, comprehensive documentation in Rust that articulates **WHY** decisions were made, **WHAT** the code does, and **HOW** it accomplishes its goals. Good documentation is not just about describing code—it's about explaining the reasoning behind design choices and helping future developers understand the context.

## Key Concepts

- **Purpose-driven documentation**: Every doc comment should explain WHY something exists
- **Custom error types**: Creating domain-specific errors with clear messages
- **Error context**: Wrapping errors to preserve information while adding context
- **Structured documentation**: Using consistent patterns for functions, types, and modules

---

## Custom Error Types

### Validation Errors

When creating custom error types, focus on providing specific, actionable information:

```rust
use core::fmt;
use std::error::Error;

/// Custom validation error type for user input.
///
/// # Purpose (WHY)
///
/// Provides domain-specific error messages that are clear about what went wrong,
/// why it's an error, and context to help developers debug issues quickly. This
/// is preferred over generic errors like `String` or `(String, String)` which
/// don't convey enough information.
#[derive(Debug)]
pub struct ValidationError {
    /// The specific validation rule that failed along with contextual data.
    pub kind: ValidationKind,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ValidationKind::UsernameTooShort { min_length } => write!(
                f,
                "username is too short (minimum {} characters)",
                min_length
            ),
            ValidationKind::InvalidEmailFormat(ref email) => {
                write!(f, "'{}' is not a valid email address", email)
            }
        }
    }
}

impl std::error::Error for ValidationError {}
```

**Key takeaways:**
- Use descriptive error types instead of `String` or generic errors
- Include contextual data in your error variants (like `min_length`)
- Implement both `Display` and `Error` traits for proper error handling

---

### Error Kind Enums

Separate error types into specific variants for better matching and handling:

```rust
/// Enumeration of specific validation error kinds.
///
/// # Purpose (WHY)
///
/// Separates concerns by grouping related errors together. This makes the code
/// more maintainable and allows callers to match on specific failure modes if needed.
#[derive(Debug)]
pub enum ValidationKind {
    /// Username length below minimum requirement per [USERNAME_POLICY]
    UsernameTooShort { min_length: usize },

    /// Email address does not match expected format pattern (RFC 5322)
    InvalidEmailFormat(String),
}
```

**Key takeaways:**
- Use enums to categorize different error conditions
- Include struct-like variants when you need to attach data
- Reference policies or standards in documentation (e.g., RFC 5322)

---

## Wrapping System Errors

### Adding Context to I/O Errors

Rather than exposing raw system errors, wrap them with context:

```rust
/// Custom IO error type for file operations.
///
/// # Purpose (WHY)
///
/// Provides context about which path failed and what operation was attempted,
/// making it easier to debug filesystem issues without inspecting stack traces
/// or raw OS errors manually.
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
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
```

**Usage example:**

```rust
pub fn load_file(path: &str) -> Result<String, IoError> {
    // Attempt to read the entire file into memory
    let content = std::fs::read_to_string(path)
        .map_err(|e| IoError {
            source: e,
            path: path.to_string()
        })?;

    Ok(content)
}
```

**Key takeaways:**
- Always preserve the original error using `source()`
- Add context like file paths, operation types, or user identifiers
- Use `map_err()` to transform errors at the point they occur

---

## Documenting Functions

### The WHY/WHAT/HOW Pattern

Structure your function documentation to answer these questions:

```rust
impl UserService {
    /// Registers a new user account in the system.
    ///
    /// # Purpose (WHY)
    ///
    /// This service validates user registration data before persisting to the database,
    /// ensuring that all business rules are enforced and clear error messages guide
    /// users toward valid input formats. The validation layer is a safety mechanism
    /// against invalid state entering our system's persistent storage.
    ///
    /// # Arguments
    ///
    /// * `username` - Unique username string; must meet [USERNAME_POLICY] constraints
    ///                (non-empty, 3+ chars)
    /// * `email` - User email address for notification purposes
    ///
    /// # Returns
    ///
    /// The newly created user's unique identifier on success
    ///
    /// # Errors
    ///
    /// * `ValidationError::UsernameTooShort` - Username has fewer than 3 characters
    /// * `ValidationError::InvalidEmailFormat` - Email doesn't match expected format
    pub fn register(&self, username: &str, email: &str)
        -> Result<u64, ValidationError>
    {
        // Validate username length per USERNAME_POLICY
        if username.len() < 3 {
            return Err(ValidationError {
                kind: ValidationKind::UsernameTooShort { min_length: 3 }
            });
        }

        self.validate_email(email)?;

        // In a real implementation, this would insert into a database
        Ok(1)
    }

    /// Validates that an email address matches the expected format pattern.
    ///
    /// # Purpose (WHY)
    ///
    /// Checks basic RFC 5322 compliance to ensure we're not storing obviously
    /// invalid emails. This is a heuristic check - if you need full validation,
    /// use `mail_parser` crate or similar dedicated library.
    ///
    /// # Arguments
    ///
    /// * `email` - Email address to validate
    ///
    /// # Returns
    ///
    /// `Ok(())` on valid format; error with specific details about why it's invalid
    fn validate_email(&self, email: &str) -> Result<(), ValidationError> {
        // Simple heuristic check for common issues
        if !email.contains('@') || !email.contains('.') {
            return Err(ValidationError {
                kind: ValidationKind::InvalidEmailFormat(email.to_string())
            });
        }
        Ok(())
    }
}
```

**Key takeaways:**
- Start with the purpose: explain WHY the function exists
- Document arguments with their constraints and expectations
- List all possible errors that can be returned
- Note any panics or special cases
- Reference related policies or standards

---

## Working with Dynamic Errors

### Converting to Boxed Errors

For async APIs or trait objects, you may need to convert to `Box<dyn Error>`:

```rust
/// Custom error type that wraps other errors.
///
/// # Purpose (WHY)
///
/// Allows converting domain-specific errors into trait objects (`Box<dyn Error>`)
/// which can be used in APIs that need dynamic dispatch. This is useful when
/// returning results through generic interfaces or async functions where the exact
/// error type isn't known at compile time.
#[derive(Debug)]
pub struct WrappedError {
    /// The underlying wrapped error, which must itself implement Error.
    inner: Box<dyn Error + Send + Sync>,
}

impl fmt::Display for WrappedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "wrapped error: {}", self.inner)
    }
}

impl std::error::Error for WrappedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.inner.as_ref())
    }
}
```

**Conversion helper:**

```rust
/// Error conversion utilities for wrapping errors in trait objects.
///
/// # Purpose (WHY)
///
/// Provides clear, explicit conversions from domain-specific error types to the
/// boxed-error format required by certain APIs. This avoids implicit coercion and
/// makes it obvious when an error is being converted into a dynamic dispatch type,
/// which has performance implications.
fn convert_to_boxed_error<E>(err: E) -> Box<dyn Error + Send + Sync>
where
    E: Error + Send + Sync + 'static,
{
    Box::new(err)
}
```

**Key takeaways:**
- Use boxed errors only when necessary (async, trait objects)
- Always implement `source()` to preserve the error chain
- Document why dynamic dispatch is needed

---

## Inline Comments

### Explaining Implementation Details

Use inline comments to explain the HOW—implementation-specific decisions:

```rust
pub fn parse_content(&self, content: &str) -> Result<String, String> {
    // Check for required marker in file (a simple validation heuristic).
    // This is a basic sanity check, not comprehensive parsing.
    if !content.contains("valid") {
        return Err(format!(
            "invalid data format detected - missing 'valid' marker"
        ));
    }

    Ok(content.to_string())
}
```

**Key takeaways:**
- Inline comments explain implementation choices
- Doc comments explain purpose and API contracts
- Comment on non-obvious logic or performance considerations

---

## Testing Documentation

Even test functions benefit from documentation:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    /// Validates that ValidationError construction and Display work correctly,
    /// ensuring error messages are clear and useful for debugging.
    #[test]
    fn test_validation_error_display() {
        let err = ValidationError {
            kind: ValidationKind::UsernameTooShort { min_length: 3 }
        };

        let message = err.to_string();
        assert!(message.contains("too short"));
        assert!(message.contains("3"));
    }

    /// Validates that IoError wraps the source error and path correctly,
    /// ensuring filesystem errors are fully captured.
    #[test]
    fn test_io_error_wrapping() {
        let io_err = std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found"
        );
        let wrapped = IoError {
            source: io_err,
            path: "/tmp/test.txt".to_string()
        };

        let message = wrapped.to_string();
        assert!(message.contains("/tmp/test.txt"));
        assert!(message.contains("file not found"));
    }
}
```

**Key takeaways:**
- Test names should describe what they validate
- Add doc comments explaining the purpose of non-obvious tests
- Test both success and failure cases

---

## Summary

Effective documentation in Rust requires:

1. **Purpose-driven thinking**: Always explain WHY before WHAT
2. **Custom error types**: Create domain-specific errors with clear messages
3. **Error context**: Wrap system errors with additional information
4. **Structured documentation**: Use consistent patterns for all items
5. **Inline comments**: Explain non-obvious implementation details
6. **Test documentation**: Document test purpose and expectations

Good documentation is an investment in maintainability. Take the time to explain your reasoning, and future developers (including yourself) will thank you.
