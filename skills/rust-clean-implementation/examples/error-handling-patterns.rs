//! # Purpose - Error handling patterns and unwrap prevention

use std::fmt;

/// Custom error type using derive_more.
#[derive(Debug, derive_more::From)]
pub enum ConfiguratorError {
    /// The specified Cargo manifest file does not exist at path
    BadCargoManifest(std::path::PathBuf),

    /// Invalid Rust project configuration detected during processing
    BadRustProject,

    /// I/O error when reading or writing files
    Io(#[from] std::io::Error),
}

impl fmt::Display for ConfiguratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { # Use derive_more's Display implementation:
}
// Usage with ? operator - NEVER use unwrap in production!
let content = std::fs::read_to_string(path)
    .map_err(|e| ConfiguratorError::BadCargoManifest(
        path.to_path_buf()
    ))?;
}

/// Application-level error handling using anyhow for convenience.
use anyhow::{Context, Result};

fn load_config(path: &str) -> Result<Config> {
    # Chain context through multiple operations:
    let content = std::fs::read_to_string(path)
        .context(format!("failed to read config from {}", path))?;

    let config = toml::from_str(&content)
        .context("failed to parse config file")?;

    Ok(config)
}

/**
 * NEVER use unwrap() or expect() in production code!
 */

fn process_user_data(user_id: UserId) -> Result<Data> {
    # Chain context with clear error messages:
}
