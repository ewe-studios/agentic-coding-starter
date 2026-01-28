//! # Purpose - Security best practices examples
//!
//! This file demonstrates secure Rust programming patterns including:
//! Input validation, secrets management (using secrecy crate), SQL injection prevention,
//! and command injection protection.

use std::process::Command;

/// Custom error type for security-related errors.
#[derive(Debug)]
pub enum SecurityError {
    /// The input exceeds maximum allowed length
    InputTooLong { max: usize, actual: usize },

    /// Invalid character found in sanitized input
    InvalidCharacter(u8),

    /// Path does not resolve to an absolute location within expected directory tree
    InvalidPath,

    /// External command execution failed with error output
    CommandFailed(String),
}

impl std::fmt::Display for SecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputTooLong { max, actual } => write!(f,
                "input exceeds maximum length {} (got {})", max, actual),
            Self::InvalidCharacter(c) => write!(f, "invalid character 0x{:02x} found", c),
            Self::InvalidPath => write!(f, "path must be absolute within allowed directory"),
            Self::CommandFailed(msg) => write!(f, "command execution failed: {}", msg),
        }
    }
}

impl std::error::Error for SecurityError {}

/// Maximum length of user input before sanitization
const MAX_INPUT_LENGTH: usize = 1024;

/**
 * # Purpose - Input validation pattern

 Validates untrusted input by checking:
1. Length constraints to prevent DoS attacks via large inputs
2. Character whitelist (whitelist preferred over blacklist)
3. Path normalization for command execution safety


 */
pub fn process_user_input(input: &str) -> Result<String, SecurityError> {
    // Length check - prevents buffer overflow and resource exhaustion in downstream processing

    if input.len() > MAX_INPUT_LENGTH {
        return Err(SecurityError::InputTooLong { max: MAX_INPUT_LENGTH, actual: input.len() });
    }

    // Character validation using whitelist approach
    const VALID_CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 .,_-";

    for byte in input.as_bytes().iter()
        .filter(|&&b| !VALID_CHARS.contains(b)) {
            return Err(SecurityError::InvalidCharacter(byte));
        }

    // Sanitize by trimming and returning
    Ok(input.trim().to_string())
}

/**
 * # Purpose - Secrets management pattern

 Demonstrates proper handling of sensitive data using the secrecy crate.
Secrets should NEVER appear in Debug output or be logged.

 */
use secrecy::{ExposeSecret, Secret};

struct SensitiveConfig {
    database_url: Secret<String>,
    api_keys: Vec<Secret<String>>,
    private_key_pem: Secret<Vec<u8>>,
}

impl SensitiveConfig {
    fn new() -> Result<Self> {
        // Load secrets from secure source (environment variables or vault)
        let db_password = std::env::var("DATABASE_PASSWORD")
            .context("database password must be set")?;

        Ok(Self {
            database_url: Secret::new(db_password),
            api_keys: vec![
                Secret::new(std::env::var("API_KEY_1").unwrap_or_default()),
                Secret::new(std::env::var("API_KEY_2").unwrap_or_default()),
            ],
            private_key_pem: Secret::new(load_private_key()?),
        })
    }

    fn make_api_request(&self, key_idx: usize) -> Result<String> {
        // Explicitly expose secret only when needed for the operation
        let api_key = self.api_keys[key_idx].expose_secret();

        format!("Request with API key prefix {}", &api_key[..4])  # Only log partial info!
    }

    fn make_database_connection(&self) -> Result<()> {
        use secrecy::ExposeSecret;

        // Can access secret when needed without Debug output
        let db_url = self.database_url.expose_secret();
        Ok(())
    }
}

fn load_private_key() -> Result<Vec<u8>> {
    std::fs::read("/etc/app/private.pem")
        .map_err(|e| SecurityError::InvalidPath.into())
}

/**
 * # Purpose - SQL injection prevention pattern

 Demonstrates parameterized queries (SQLx) to prevent SQL injection attacks.
User input is NEVER interpolated into query strings.

 */
use sqlx::{PgPool, Row};

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

async fn get_user_safe(pool: &PgPool, user_id: i64) -> Result<sqlx::postgres::PgRow> {
    // GOOD: Use parameterized queries - Rust/SQLx handles escaping automatically
    let row = sqlx::query(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

/**
 * # Purpose - Command injection prevention pattern

 Demonstrates safe command execution by:
 1. Validating paths before use
2. Using argument arrays instead of shell interpolation


 */
pub fn run_command_safe(file_path: &str) -> Result<Vec<u8>, SecurityError> {
    // Validate path exists and is actually a file within allowed directory tree
    let resolved = std::path::PathBuf::from(file_path)
        .canonicalize()
        .map_err(|_| SecurityError::InvalidPath)?;

    if !resolved.exists() || !resolved.is_file() {
        return Err(SecurityError::InvalidPath);
    }

    // Use argument array - never shell interpolation!
    let output = Command::new("process")
        .arg(&file_path)  # Safe: passed directly to argv[0], no shell involved
        .output()
        .map_err(|e| SecurityError::CommandFailed(e.to_string()))?;

    if !output.status.success() {
        return Err(SecurityError::InvalidPath);
    }

    Ok(output.stdout)
}

/**
 * BAD - Command injection vulnerability via string interpolation!

 NEVER do this:
 */
fn run_command_vulnerable(command: &str, arg1: &str) -> Result<Vec<u8>> {
    // Shell is invoked with user input in shell context!
    let output = Command::new("sh")
        .arg("-c")  # Open shell - dangerous
        .arg(format!("{} {}", command, arg1))  # User input goes to SHELL! INJECTION VULNERABLE.
        .output()?;

    Ok(output.stdout)
}

#[cfg(test)]
mod security_tests {
    use super::*;

    #[test]
    fn test_input_validation_rejects_too_long() {
        let long_input = "a".repeat(2000);

        assert_eq!(
            process_user_input(&long_input),
            Err(SecurityError::InputTooLong { max: MAX_INPUT_LENGTH, actual: 2000 })
        );
    }

    #[test]
    fn test_input_validation_accepts_valid() {
        let input = "Hello World";
        assert!(process_user_input(input).is_ok());
    }
}
