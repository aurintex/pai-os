//! Configuration loading port.

use crate::domain::ConfigError;
use serde::de::DeserializeOwned;
use std::path::Path;

/// Loads typed configuration values from a backing store (files, remote, etc.).
pub trait ConfigProvider {
    /// Deserializes configuration from `path` into `T`.
    fn load<T: DeserializeOwned>(&self, path: &Path) -> Result<T, ConfigError>;
}
