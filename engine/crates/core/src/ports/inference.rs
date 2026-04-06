//! Inference boundary for the core orchestrator (implemented by inference adapters).

use thiserror::Error;

/// Errors from an [`InferencePort`] implementation.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum InferenceError {
    /// Backend failed to produce a completion.
    #[error("inference failed: {0}")]
    Failed(String),
}

/// Drives text completions (LLM / edge inference). Core depends on this port, not on concrete crates.
pub trait InferencePort: Send + Sync {
    /// Run a single prompt/turn and return model text.
    fn complete(&self, prompt: &str) -> Result<String, InferenceError>;
}
