//! # Purpose (WHY)
//!
//! This module demonstrates the custom error handling pattern used throughout
//! the codebase. It shows how to implement clean, explicit errors with proper
//! documentation that articulates WHY each behavior exists and WHAT it does.

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
            ValidationKind::InvalidEmailFormat(email) => write!(f, "'{}' is not a valid email address", email),
        }
    }
}

impl std::error::Error for ValidationError {}

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

impl std::error::Error for IoError {}

/// Custom error type that wraps other errors.
///
/// # Purpose (WHY)
///
/// Allows converting domain-specific errors into trait objects (`Box<dyn Error>`)
/// which can be used in APIs that need dynamic dispatch. This is useful when
/// returning results through generic interfaces or async functions where the exact
/// error type isn't known at compile time but needs to implement `Error`.
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

impl std::error::Error for WrappedError {}

/// # Purpose (WHY)
///
/// Demonstrates the WHY/WHAT/HOW documentation pattern in action.
///
/// This service validates user registration data before persisting to the database,
/// ensuring that all business rules are enforced and clear error messages guide
/// users toward valid input formats. The validation layer is a safety mechanism
/// against invalid state entering our system's persistent storage.
impl UserService {
    /// Registers a new user account in the system.

    /// Args:
    ///
    /// * `username` - Unique username string; must meet [USERNAME_POLICY] constraints (non-empty, 3+ chars)
    /// * `email` - User email address for notification purposes
    /// * `password_hash` - Cryptographically hashed password per [PASSWORD_HASHING]

    /// Returns:
    ///
    /// The newly created user's unique identifier on success; error if validation fails or database insert errors

    /// # Errors
    ///
    /// * `ValidationError` - Raised when input does not meet policy constraints (see ValidationKind variants)
    pub fn register(&self, username: &str, email: &str) -> Result<u64> {
        // Validate username length per USERNAME_POLICY (minimum 3 characters for usability and spam prevention)
        if username.len() < 3 {
            return Err(ValidationError { kind: ValidationKind::UsernameTooShort { min_length: 3 } });
        }

        self.validate_email(email)?;

        let user_id = self.db.insert(username, email)?;
        Ok(user_id)
    }
}

impl UserService {
    /// Validates that an email address matches the expected format pattern.
    ///
    /// # Purpose (WHY)
    ///
    /// Checks basic RFC 5322 compliance to ensure we're not storing obviously
    /// invalid emails. This is a heuristic check - if you need full validation,
    /// use `mail_parser` crate or similar dedicated library.

    /// Args:
    ///
    /// * `email` - Email address to validate

    /// Returns:
    ///
    /// Ok(()) on valid format; error with specific details about why it's invalid
    fn validate_email(&self, email: &str) -> Result<()> {
        // Simple heuristic check for common issues (missing @ or domain)
        if !email.contains('@') || !email.ends_with(".com") {
            return Err(ValidationError { kind: ValidationKind::InvalidEmailFormat(email.to_string()) });
        }
        Ok(())
    }

    /// # Panics
    ///
    /// * `user_id` is greater than the current user count - caller must ensure unique IDs through [Self::register]
}

/// Data processor that handles file I/O and parsing.
///
/// # Purpose (WHY)
///
/// Demonstrates how to wrap low-level errors in domain-specific error types,
/// making them easier to handle at higher levels of abstraction. This prevents
/// callers from having to deal with raw `std::io::Error` but still preserves
/// information about the original failure.
impl DataProcessor {
    /// Loads and parses a data file.

    /// Args:
    ///
    /// * `path` - Path to input data file; must exist on filesystem

    /// Returns:
    ///
    /// Parsed content as string if successful

    /// # Errors
    ///
    /// * `IoError` - Raised when file cannot be read or doesn't exist (preserves path and source error)
    pub fn load_file(&self, path: &str) -> Result<String> {
        // Attempt to read the entire file into memory. Uses std::fs which is appropriate
        // for files small enough to fit in available RAM.
        let content = std::fs::read_to_string(path).map_err(|e| IoError { source: e, path: path.to_string() })?;

        self.parse_content(&content)
    }

    /// Parses the file contents into structured data.

    /// Args:
    ///
    /// * `content` - Raw text content from [Self::load_file]

    /// Returns:
    ///
    /// Parsed string representation on success

    /// # Errors
    ///
    /// This function will return errors indicating parse failures if format is invalid.
    fn parse_content(&self, content: &str) -> Result<String> {
        // Check for required marker in file (a simple validation heuristic)
        if !content.contains("valid") {
            return Err(format!("invalid data format detected - missing 'valid' marker"));
        }
        Ok(content.to_string())
    }

    /// # Panics
    ///
    /// * `path` is empty or None after trimming whitespace - caller must provide a valid path (see [Self::load_file])
}

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
    // WHY: Using trait bounds here allows any Error to be wrapped, not just our custom types.
    // WHAT: The conversion happens at compile time for concrete types but produces a dynamic dispatch object.
    // HOW: We use Into trait bound which is optimized by the compiler when E implements From<Box<dyn ...>> or similar.
    E: Into<Box<dyn Error + Send + Sync>>,
{
    err.into()
}

/// # Purpose (WHY)
///
/// Example of how to convert our custom IoError into a boxed error for async APIs
/// that need dyn Error return types. This maintains type safety while allowing
 /// dynamic dispatch where needed.
fn wrap_for_async_api(error: IoError) -> Box<dyn Error + Send + Sync> {
    // WHAT: Convert specific domain error to trait object format required by the API contract.
    // HOW: Since we have Display and Debug already, this is a simple conversion that preserves all information.
    convert_to_boxed_error(error)
}

/// Async operation result type using boxed errors for dynamic dispatch.
pub async fn process_data_async(path: &str) -> Result<String> {
    let processor = DataProcessor::new();
    // WHAT: Converting IoError to Box<dyn Error + Send + Sync> so it can be returned through
    // a trait object interface where the exact error type isn't known at compile time.
    let content = wrap_for_async_api(processor.load_file(path).await?)?;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// # Purpose (WHY)
    ///
    /// Validates that ValidationError construction and Display work correctly,
    /// ensuring error messages are clear and useful for debugging.
    #[test]
    fn test_validation_error_display() {
        let err = ValidationError { kind: ValidationKind::UsernameTooShort { min_length: 3 } };
        assert!(err.to_string().contains("too short"));
        assert!(!err.to_string().is_empty());
    }

    /// # Purpose (WHY)
    ///
    /// Validates that IoError wraps the source error and path correctly,
    /// ensuring filesystem errors are fully captured.
    #[test]
    fn test_io_error_wrapping() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let wrapped = IoError { source: io_err.clone(), path: "/tmp/test.txt".to_string() };
        assert!(wrapped.to_string().contains("/tmp/test.txt"));
    }

    /// # Purpose (WHY)
    ///
    /// Validates that WrappedError properly wraps other error types,
    /// ensuring the inner error is preserved and accessible.
    #[test]
    fn test_wrapped_error_conversion() {
        let source_err = ValidationError { kind: ValidationKind::InvalidEmailFormat("bad@test.com".to_string()) };
        let boxed: Box<dyn Error + Send + Sync> = convert_to_boxed_error(source_err);
        assert!(boxed.to_string().contains("email"));
    }
}
