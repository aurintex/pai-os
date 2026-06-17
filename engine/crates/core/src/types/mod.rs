//! Shared domain types consumed by Core ports.

use tokio_util::sync::CancellationToken;

/// Parameters for a single inference request.
#[derive(Debug, Clone)]
pub struct InferenceRequest {
    /// The prompt text sent to the model.
    pub prompt: String,
    /// Optional model identifier; the adapter uses its default model when `None`.
    pub model_id: Option<String>,
    /// Sampling temperature (0.0–2.0). Higher values produce more varied output.
    pub temperature: Option<f32>,
    /// Nucleus-sampling probability mass (0.0–1.0).
    pub top_p: Option<f32>,
    /// Maximum tokens to generate. Adapter default applies when `None`.
    pub max_tokens: Option<u32>,
    /// Sequences that stop generation early (e.g., `["<|im_end|>"]`).
    pub stop_sequences: Vec<String>,
    /// Token that the caller cancels to abort the in-flight request.
    pub cancellation: CancellationToken,
}

impl InferenceRequest {
    /// Minimal request with all options at their defaults.
    pub fn simple(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            model_id: None,
            temperature: None,
            top_p: None,
            max_tokens: None,
            stop_sequences: Vec::new(),
            cancellation: CancellationToken::new(),
        }
    }
}

/// A single streamed token chunk from an inference adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub content: String,
}
