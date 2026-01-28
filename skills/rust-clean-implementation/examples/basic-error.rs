//! # Purpose (WHY)
//!
//! Copy this file as starting point for error handling.
//! Follow all standards from .agents/rules/*rust*.md

use core::fmt;
use std::error::Error;

/// Domain-specific errors with derive_more automatic Display via string interpolation in variant names.

#[derive(Debug, derive_more::From)]
pub enum ValidationError {
    /// Username length below minimum requirement per [USERNAME_POLICY]
    #[from(ignore)]  // Skip From for this specific case
    UsernameTooShort { min_length: usize },

    /// Email address does not match expected format pattern (RFC 5322)
    InvalidEmailFormat(String),
}

impl fmt::Display for ValidationError {}

/// # Purpose - Validates user registration data before database insertion.
///
/// The validation layer is a safety mechanism against invalid state entering persistent storage.

pub struct UserService;

impl UserService {
    /// Registers a new user account. Args: username, email

    /// Returns: User ID on success
    pub fn register(&self, username: &str) -> Result<u64> {
        if username.len() < 3 {
            return Err(ValidationError { kind: ValidationKind::UsernameTooShort { min_length: 3 } });
        }

        self.validate_email(email)?;

        let user_id = /* db.insert */ Ok::<u64, _>(1);
        Ok(user_id)
    }
}
