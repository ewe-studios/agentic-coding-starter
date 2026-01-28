# Security Best Practices for Rust

## Purpose

This guide demonstrates essential security patterns for writing secure Rust applications. Security is not just about using a safe language—it requires conscious decisions about input validation, secrets management, injection prevention, and secure coding practices. This guide shows you how to build security into your Rust applications from the ground up.

## Key Concepts

- **Input validation**: Whitelist-based validation to prevent malicious input
- **Secrets management**: Using the `secrecy` crate to protect sensitive data
- **SQL injection prevention**: Parameterized queries with SQLx
- **Command injection prevention**: Safe process execution patterns
- **Defense in depth**: Multiple layers of security controls

---

## Input Validation

### Whitelist-Based Validation

Always prefer whitelisting over blacklisting for input validation:

```rust
use std::error::Error;

/// Custom error type for security-related errors.
#[derive(Debug)]
pub enum SecurityError {
    /// The input exceeds maximum allowed length
    InputTooLong { max: usize, actual: usize },

    /// Invalid character found in sanitized input
    InvalidCharacter(char),

    /// Path does not resolve to an absolute location within expected directory tree
    InvalidPath,

    /// External command execution failed with error output
    CommandFailed(String),
}

impl std::fmt::Display for SecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputTooLong { max, actual } => {
                write!(f, "input exceeds maximum length {} (got {})", max, actual)
            }
            Self::InvalidCharacter(c) => {
                write!(f, "invalid character '{}' found", c)
            }
            Self::InvalidPath => {
                write!(f, "path must be absolute within allowed directory")
            }
            Self::CommandFailed(msg) => {
                write!(f, "command execution failed: {}", msg)
            }
        }
    }
}

impl std::error::Error for SecurityError {}

/// Maximum length of user input before sanitization
const MAX_INPUT_LENGTH: usize = 1024;

/// Validates untrusted input by checking length and character whitelist.
///
/// # Purpose (WHY)
///
/// Prevents various attacks including:
/// - DoS via large inputs (length check)
/// - Injection attacks (character whitelist)
/// - Buffer overflow in downstream processing
///
/// # Arguments
///
/// * `input` - Untrusted user input to validate
///
/// # Returns
///
/// Sanitized input on success
///
/// # Errors
///
/// * `SecurityError::InputTooLong` - Input exceeds MAX_INPUT_LENGTH
/// * `SecurityError::InvalidCharacter` - Input contains non-whitelisted characters
pub fn process_user_input(input: &str) -> Result<String, SecurityError> {
    // Length check - prevents buffer overflow and resource exhaustion
    if input.len() > MAX_INPUT_LENGTH {
        return Err(SecurityError::InputTooLong {
            max: MAX_INPUT_LENGTH,
            actual: input.len(),
        });
    }

    // Character validation using whitelist approach
    // Only allow alphanumeric, space, and common punctuation
    const VALID_CHARS: &str = "abcdefghijklmnopqrstuvwxyz\
                               ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                               0123456789 .,_-";

    for ch in input.chars() {
        if !VALID_CHARS.contains(ch) {
            return Err(SecurityError::InvalidCharacter(ch));
        }
    }

    // Sanitize by trimming whitespace
    Ok(input.trim().to_string())
}
```

**Key takeaways:**
- Always check input length to prevent DoS attacks
- Use whitelists (what's allowed) rather than blacklists (what's forbidden)
- Validate as early as possible in your application
- Return specific errors with context for debugging

---

### Testing Input Validation

Always test boundary conditions and malicious inputs:

```rust
#[cfg(test)]
mod security_tests {
    use super::*;

    #[test]
    fn test_input_validation_rejects_too_long() {
        let long_input = "a".repeat(2000);
        let result = process_user_input(&long_input);

        assert!(matches!(
            result,
            Err(SecurityError::InputTooLong { max: 1024, actual: 2000 })
        ));
    }

    #[test]
    fn test_input_validation_accepts_valid() {
        let input = "Hello World";
        assert!(process_user_input(input).is_ok());
    }

    #[test]
    fn test_input_validation_rejects_special_chars() {
        let input = "Hello<script>alert('xss')</script>";
        assert!(process_user_input(input).is_err());
    }

    #[test]
    fn test_input_validation_rejects_sql_injection() {
        let input = "'; DROP TABLE users; --";
        assert!(process_user_input(input).is_err());
    }
}
```

**Key takeaways:**
- Test with extremely long inputs
- Test with injection attempts (XSS, SQL, etc.)
- Test boundary conditions (exactly at max length)
- Test with valid inputs to ensure you don't break functionality

---

## Secrets Management

### Using the secrecy Crate

Never log or debug-print sensitive data. Use the `secrecy` crate:

```rust
use secrecy::{ExposeSecret, Secret};

/// Configuration containing sensitive data.
///
/// # Purpose (WHY)
///
/// Wraps all sensitive configuration in Secret<T> to prevent accidental
/// exposure via Debug output, logs, or error messages. Secrets must be
/// explicitly exposed when needed, making it obvious in code reviews.
struct SensitiveConfig {
    /// Database connection string with password
    database_url: Secret<String>,

    /// API keys for external services
    api_keys: Vec<Secret<String>>,

    /// Private key in PEM format
    private_key_pem: Secret<Vec<u8>>,
}

impl SensitiveConfig {
    /// Loads configuration from environment variables.
    ///
    /// # Purpose (WHY)
    ///
    /// Environment variables are preferred over config files for secrets
    /// because they're not checked into version control and can be managed
    /// by secret management systems (AWS Secrets Manager, HashiCorp Vault, etc.)
    ///
    /// # Errors
    ///
    /// Returns error if required environment variables are not set
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Load from environment variables (never hardcode!)
        let db_password = std::env::var("DATABASE_PASSWORD")
            .map_err(|_| "DATABASE_PASSWORD must be set")?;

        let api_key_1 = std::env::var("API_KEY_1").unwrap_or_default();
        let api_key_2 = std::env::var("API_KEY_2").unwrap_or_default();

        Ok(Self {
            database_url: Secret::new(db_password),
            api_keys: vec![
                Secret::new(api_key_1),
                Secret::new(api_key_2),
            ],
            private_key_pem: Secret::new(load_private_key()?),
        })
    }

    /// Makes an API request using a stored key.
    ///
    /// # Purpose (WHY)
    ///
    /// Demonstrates how to explicitly expose secrets only when needed
    /// for the actual operation. The secret is never stored in a
    /// non-Secret variable or logged.
    fn make_api_request(&self, key_idx: usize)
        -> Result<String, Box<dyn std::error::Error>>
    {
        // Explicitly expose secret only when needed
        let api_key = self.api_keys[key_idx].expose_secret();

        // Only log partial information - NEVER log the full secret!
        if api_key.len() >= 4 {
            println!("Using API key prefix: {}", &api_key[..4]);
        }

        // Use the API key for the actual request
        Ok(format!("Request made with key {}", key_idx))
    }

    /// Establishes a database connection.
    ///
    /// # Purpose (WHY)
    ///
    /// Shows how to use secrets with libraries that need the actual value.
    /// The exposure is explicit and localized to the minimum scope.
    fn make_database_connection(&self)
        -> Result<String, Box<dyn std::error::Error>>
    {
        // Expose only in the local scope where it's needed
        let db_url = self.database_url.expose_secret();

        // Use db_url with your database library
        Ok(format!("Connected to database"))
    }
}

fn load_private_key() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Load from a secure location, not from the repository
    std::fs::read("/etc/app/private.pem")
        .map_err(|e| format!("failed to load private key: {}", e).into())
}
```

**Key takeaways:**
- Wrap all secrets in `Secret<T>` from the `secrecy` crate
- Load secrets from environment variables, not config files
- Never log full secrets—log only non-sensitive prefixes if needed
- Use `expose_secret()` explicitly and only where necessary

---

## SQL Injection Prevention

### Parameterized Queries with SQLx

Always use parameterized queries. Never concatenate user input into SQL:

```rust
use sqlx::{PgPool, Row, FromRow};

#[derive(Debug)]
pub enum DatabaseError {
    /// Query execution failed
    QueryFailed(String),

    /// No user found with given identifier
    UserNotFound,
}

impl std::error::Error for DatabaseError {}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueryFailed(msg) => write!(f, "database query failed: {}", msg),
            Self::UserNotFound => write!(f, "user not found"),
        }
    }
}

#[derive(Debug, FromRow)]
struct User {
    id: i64,
    username: String,
    email: String,
}

/// Retrieves a user by ID safely using parameterized queries.
///
/// # Purpose (WHY)
///
/// Demonstrates SQL injection prevention using SQLx's parameterized queries.
/// The database driver handles all escaping automatically, making SQL injection
/// impossible regardless of user input.
///
/// # Arguments
///
/// * `pool` - Database connection pool
/// * `user_id` - User ID to look up
///
/// # Returns
///
/// User record if found
///
/// # Errors
///
/// * `DatabaseError::UserNotFound` - No user with given ID
/// * `DatabaseError::QueryFailed` - Database query failed
async fn get_user_safe(pool: &PgPool, user_id: i64)
    -> Result<User, DatabaseError>
{
    // GOOD: Use parameterized queries
    // SQLx handles escaping automatically - SQL injection is impossible
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, email FROM users WHERE id = $1"
    )
    .bind(user_id)  // Parameter is bound safely
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => DatabaseError::UserNotFound,
        _ => DatabaseError::QueryFailed(e.to_string()),
    })?;

    Ok(user)
}

/// Searches for users by username safely.
///
/// # Purpose (WHY)
///
/// Shows how to use LIKE queries safely with parameterized queries.
/// Even pattern matching is safe when parameters are bound.
async fn search_users_by_username(pool: &PgPool, search_term: &str)
    -> Result<Vec<User>, DatabaseError>
{
    // GOOD: Even LIKE patterns are safe with parameterized queries
    let pattern = format!("%{}%", search_term);

    let users = sqlx::query_as::<_, User>(
        "SELECT id, username, email FROM users WHERE username LIKE $1"
    )
    .bind(&pattern)  // SQLx escapes the pattern safely
    .fetch_all(pool)
    .await
    .map_err(|e| DatabaseError::QueryFailed(e.to_string()))?;

    Ok(users)
}

/// BAD EXAMPLE - DO NOT USE
///
/// # Warning
///
/// This function is vulnerable to SQL injection and should NEVER be used.
/// It's included only to demonstrate what NOT to do.
#[allow(dead_code)]
async fn get_user_vulnerable(pool: &PgPool, user_id: &str)
    -> Result<User, DatabaseError>
{
    // BAD: String concatenation allows SQL injection!
    // If user_id is "1 OR 1=1", this returns all users!
    let query = format!(
        "SELECT id, username, email FROM users WHERE id = {}",
        user_id  // NEVER do this!
    );

    let user = sqlx::query_as::<_, User>(&query)
        .fetch_one(pool)
        .await
        .map_err(|e| DatabaseError::QueryFailed(e.to_string()))?;

    Ok(user)
}
```

**Key takeaways:**
- Always use parameterized queries with `$1`, `$2`, etc.
- Use `.bind()` to bind parameters—never string concatenation
- SQLx handles all escaping automatically
- This works for all query types, including LIKE patterns

---

## Command Injection Prevention

### Safe Process Execution

When executing external commands, use argument arrays and validate paths:

```rust
use std::process::Command;
use std::path::PathBuf;

/// Runs an external command safely on a validated file.
///
/// # Purpose (WHY)
///
/// Demonstrates safe command execution by:
/// 1. Validating file paths before use (canonicalization)
/// 2. Using argument arrays instead of shell interpolation
/// 3. Never invoking a shell with user input
///
/// # Arguments
///
/// * `file_path` - Path to process (must exist and be a regular file)
///
/// # Returns
///
/// Command stdout on success
///
/// # Errors
///
/// * `SecurityError::InvalidPath` - Path doesn't exist or isn't a file
/// * `SecurityError::CommandFailed` - Command execution failed
pub fn run_command_safe(file_path: &str)
    -> Result<Vec<u8>, SecurityError>
{
    // Step 1: Validate and canonicalize the path
    // This resolves symlinks and ensures the path is absolute
    let resolved = PathBuf::from(file_path)
        .canonicalize()
        .map_err(|_| SecurityError::InvalidPath)?;

    // Step 2: Verify it's a file (not a directory or special file)
    if !resolved.is_file() {
        return Err(SecurityError::InvalidPath);
    }

    // Step 3: Use argument array - NEVER shell interpolation!
    // Each argument is passed directly to the process, no shell involved
    let output = Command::new("process-tool")
        .arg(&resolved)  // Safe: passed directly to argv[1]
        .output()
        .map_err(|e| SecurityError::CommandFailed(e.to_string()))?;

    // Step 4: Check exit status
    if !output.status.success() {
        return Err(SecurityError::CommandFailed(
            format!("command exited with status: {}", output.status)
        ));
    }

    Ok(output.stdout)
}

/// Runs a command with multiple validated arguments.
///
/// # Purpose (WHY)
///
/// Shows how to pass multiple arguments safely. Each argument is
/// a separate element in the argument array, preventing shell
/// interpretation entirely.
pub fn run_command_with_args(
    file_path: &str,
    output_format: &str,
) -> Result<Vec<u8>, SecurityError> {
    // Validate file path
    let resolved = PathBuf::from(file_path)
        .canonicalize()
        .map_err(|_| SecurityError::InvalidPath)?;

    // Validate format (whitelist)
    if !["json", "xml", "csv"].contains(&output_format) {
        return Err(SecurityError::InvalidPath);
    }

    // Each argument is separate - no shell interpretation possible
    let output = Command::new("converter")
        .arg("--input")
        .arg(&resolved)
        .arg("--format")
        .arg(output_format)
        .output()
        .map_err(|e| SecurityError::CommandFailed(e.to_string()))?;

    if !output.status.success() {
        return Err(SecurityError::CommandFailed(
            String::from_utf8_lossy(&output.stderr).to_string()
        ));
    }

    Ok(output.stdout)
}

/// BAD EXAMPLE - DO NOT USE
///
/// # Warning
///
/// This function is vulnerable to command injection and should NEVER be used.
/// It's included only to demonstrate what NOT to do.
#[allow(dead_code)]
fn run_command_vulnerable(command: &str, arg1: &str)
    -> Result<Vec<u8>, SecurityError>
{
    // BAD: Shell is invoked with user input!
    // If arg1 is "; rm -rf /", it will be executed!
    let output = Command::new("sh")
        .arg("-c")  // Invokes shell - DANGEROUS!
        .arg(format!("{} {}", command, arg1))  // Shell interprets this!
        .output()
        .map_err(|e| SecurityError::CommandFailed(e.to_string()))?;

    Ok(output.stdout)
}
```

**Key takeaways:**
- Always use `Command::new(program).arg(arg1).arg(arg2)`
- Never use `sh -c` with user input
- Canonicalize and validate file paths before use
- Whitelist allowed values for any user-controlled arguments

---

## Path Traversal Prevention

### Validating File Paths

Prevent directory traversal attacks by validating paths:

```rust
use std::path::{Path, PathBuf};

/// Validates that a path is within an allowed directory.
///
/// # Purpose (WHY)
///
/// Prevents path traversal attacks (e.g., "../../etc/passwd")
/// by ensuring all paths resolve to locations within the
/// allowed base directory.
///
/// # Arguments
///
/// * `user_provided_path` - Path from user input
/// * `allowed_base` - Base directory that all paths must be under
///
/// # Returns
///
/// Canonicalized path if valid
///
/// # Errors
///
/// * `SecurityError::InvalidPath` - Path is outside allowed directory
pub fn validate_path(
    user_provided_path: &str,
    allowed_base: &Path,
) -> Result<PathBuf, SecurityError> {
    // Canonicalize the base directory
    let base = allowed_base
        .canonicalize()
        .map_err(|_| SecurityError::InvalidPath)?;

    // Build the full path
    let full_path = base.join(user_provided_path);

    // Canonicalize to resolve . and .. components
    let canonical = full_path
        .canonicalize()
        .map_err(|_| SecurityError::InvalidPath)?;

    // Verify the canonical path starts with the base
    if !canonical.starts_with(&base) {
        return Err(SecurityError::InvalidPath);
    }

    Ok(canonical)
}

#[cfg(test)]
mod path_tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_validate_path_accepts_valid() {
        let temp_dir = std::env::temp_dir();
        let test_dir = temp_dir.join("test_security");
        fs::create_dir_all(&test_dir).unwrap();

        let result = validate_path("file.txt", &test_dir);
        assert!(result.is_ok());

        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_validate_path_rejects_traversal() {
        let temp_dir = std::env::temp_dir();
        let test_dir = temp_dir.join("test_security");
        fs::create_dir_all(&test_dir).unwrap();

        let result = validate_path("../../etc/passwd", &test_dir);
        assert!(result.is_err());

        fs::remove_dir_all(&test_dir).ok();
    }
}
```

**Key takeaways:**
- Always canonicalize paths to resolve `.` and `..`
- Check that the canonical path starts with your allowed base
- Never trust user-provided paths without validation

---

## Summary

Building secure Rust applications requires:

1. **Input Validation**: Use whitelists and length checks for all user input
2. **Secrets Management**: Use `secrecy` crate to protect sensitive data
3. **SQL Safety**: Always use parameterized queries, never string concatenation
4. **Command Safety**: Use argument arrays, never invoke shells with user input
5. **Path Safety**: Canonicalize and validate all file paths
6. **Defense in Depth**: Layer multiple security controls

Security is not a feature you add at the end—it must be built into your application from the start. By following these patterns, you'll create Rust applications that are resistant to common attacks and vulnerabilities.

**Remember**: Security is a journey, not a destination. Stay informed about new vulnerabilities and best practices, and always assume user input is malicious until proven otherwise.
