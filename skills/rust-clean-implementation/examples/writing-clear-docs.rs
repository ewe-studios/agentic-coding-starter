// Example of WHY/WHAT/HOW documentation patterns and proper panic handling
//
// This file demonstrates:
// 1. Documentation with clear Purpose (WHY), Args, Returns, Panics sections
// 2. Result-based error handling using thiserror for libraries / anyhow for apps

use std::path::{Path, PathBuf};
use thiserror::Error;

/// # Purpose (WHY)
///
/// Validates and normalizes user input before database insertion.
/// Ensures data integrity by checking format constraints and business rules
/// as specified in the [UserInputSpecification] document.

impl UserValidator {
    /// Normalizes a username according to system policy requirements defined in RFC 1234.

    /// Args:
    ///
    /// * `username` - The raw username string provided by user, must be non-empty ASCII text

    /// Returns:
    ///
    /// A validated and normalized Username struct if all checks pass; error otherwise
    pub fn normalize_username(&self, username: &str) -> Result<Username> {
        // Implementation...
    }

    /// Validates an email address format.

    /// Args:
    ///
    /// * `email` - Email string to validate

    /// Returns:
    ///
    /// Parsed and validated Email struct if valid; error otherwise
    pub fn validate_email(&self, email: &str) -> Result<Email> {
        // Implementation...
    }
}

/// # Purpose (WHY)
///
/// Loads application configuration from disk. Ensures all required fields are present,
/// values follow business rules per [CONFIG_SCHEMA], and the structure is consistent
/// with expected settings.

impl ConfigLoader {
    /// Loads full application configuration file into memory for runtime use.
    ///
    /// The function performs three validation steps:
    /// 1. File existence and readability checks (see std::fs errors)
    /// 2. YAML parsing of content structure
    /// 3. Business rule validation per [Self::validate_schema]
    pub fn load(&self, config_path: &Path) -> Result<Configuration> {
        // Implementation...
    }
}

/// Custom error type for user input validation failures.
///
/// Each variant clearly documents:
/// - The specific invalid condition
/// - What the caller should fix to resolve it

#[derive(Error, Debug)]
pub enum ValidationError {
    /// Input contains characters not allowed per [USERNAME_PATTERN_REGEX]
    #[error("Username '{input}' contains disallowed character: {bad_char}")]
    DisallowedCharacter {
        input: String,
        bad_char: char,
    },

    /// Username length exceeds maximum allowed size
    #[error("Username too long: max_length={max}, actual={actual}")]
    TooLong {
        max_length: usize,
        actual: usize,
    },
}

/// Example of proper panic documentation with explicit preconditions

impl DataAccessor {
    /// Retrieves data by index from internal storage.

    /// Args:
    ///
    /// * `index` - Positional index to retrieve element at
    pub fn get_at_index(&self, index: usize) -> &T {
        // This implementation assumes valid input per precondition docs above.
        self.data[index]  // Panics if index >= len()
    }

    /// Attempts retrieval with bounds checking (never panics for invalid indices).

    /// Returns:
    ///
    /// Reference to element at position or None if out of bounds
    pub fn get_at_index_safe(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
}

// Example using anyhow for application-level error handling

use anyhow::{Context, Result};

/// Application configuration loader with context-rich errors.
///
/// This demonstrates proper propagation and attachment of context to
/// underlying I/O or parsing failures.

impl ConfigLoaderV2 {
    /// Loads config from YAML file with rich error messages at each failure point.

    pub async fn load(&self) -> Result<Configuration> {
        // Step 1: Read file - attach context for this operation
        let content = tokio::fs::read_to_string("config.yaml")
            .await
            .context("Failed to read configuration from config.yaml")?;

        // Step 2: Parse YAML structure
        serde_yaml::from_str::<RawConfig>(&content)
            .context("Configuration file is malformed and cannot be parsed as valid YAML")?;

        Ok(Configuration { /* ... */ })
    }
}
