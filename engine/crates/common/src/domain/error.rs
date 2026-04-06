//! Error types for the `common` crate.

use std::path::PathBuf;

/// Errors from configuration loading and parsing.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("failed to read config file {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to parse {format} config {path}: {message}")]
    Parse {
        path: PathBuf,
        format: &'static str,
        message: String,
    },
}

impl ConfigError {
    pub(crate) fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        ConfigError::Io {
            path: path.into(),
            source,
        }
    }

    pub(crate) fn parse(
        path: impl Into<PathBuf>,
        format: &'static str,
        message: impl std::fmt::Display,
    ) -> Self {
        ConfigError::Parse {
            path: path.into(),
            format,
            message: message.to_string(),
        }
    }
}
