//! [`SessionConfigPort`] — get/set configuration for the current session.

use thiserror::Error;

/// Errors from a [`SessionConfigPort`] implementation.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SessionConfigError {
    /// The requested configuration key does not exist.
    #[error("configuration key not found: {0}")]
    KeyNotFound(String),
    /// The supplied value is invalid for the given key.
    #[error("invalid value for key '{key}': {reason}")]
    InvalidValue {
        /// The configuration key that was being set.
        key: String,
        /// Human-readable description of why the value was rejected.
        reason: String,
    },
    /// Persisting the configuration change failed.
    #[error("failed to persist configuration: {0}")]
    PersistError(String),
    /// Reading the stored configuration failed.
    #[error("failed to read configuration: {0}")]
    ReadError(String),
}

/// A single configuration entry returned by [`SessionConfigPort::get_config`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigEntry {
    /// The configuration key (e.g. `"inference.max_tokens"`).
    pub key: String,
    /// The serialised string value for the key.
    pub value: String,
}

impl ConfigEntry {
    /// Create a new [`ConfigEntry`] from `key` and `value`.
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

/// Reads and writes session-scoped configuration values.
///
/// Implementations are provided by a config adapter and injected at the composition root.
/// Core holds only this port interface and is unaware of the underlying storage mechanism.
pub trait SessionConfigPort: Send + Sync {
    /// Retrieve the current value for a configuration key.
    ///
    /// Returns `Ok(ConfigEntry)` if the key exists, or
    /// [`SessionConfigError::KeyNotFound`] when it does not.
    fn get_config(&self, key: &str) -> Result<ConfigEntry, SessionConfigError>;

    /// Persist a new value for a configuration key.
    ///
    /// Validates the value before writing; returns
    /// [`SessionConfigError::InvalidValue`] if validation fails.
    fn set_config(&self, key: &str, value: &str) -> Result<(), SessionConfigError>;

    /// List all configuration entries visible to this session.
    ///
    /// Returns an empty `Vec` when no configuration has been stored yet.
    fn list_config(&self) -> Result<Vec<ConfigEntry>, SessionConfigError>;
}
