//! Inference boundary for the core orchestrator (implemented by inference adapters).

use crate::types::{InferenceRequest, Token};
use thiserror::Error;
use tokio::sync::mpsc::Receiver;

/// Errors from an [`InferencePort`] implementation.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum InferenceError {
    /// Backend failed to produce a completion.
    #[error("inference failed: {0}")]
    Failed(String),
    /// Request was cancelled via the [`InferenceRequest::cancellation`] token.
    #[error("inference was cancelled")]
    Cancelled,
    /// Requested model is not loaded or does not exist.
    #[error("model not found: {0}")]
    ModelNotFound(String),
    /// Input prompt exceeds the model's context window.
    #[error("prompt exceeds context window: {0} tokens")]
    ContextTooLong(usize),
    /// The backend has exhausted its available resources (memory, threads, etc.).
    #[error("inference backend resources exhausted")]
    ResourceExhausted,
}

/// Drives token-streaming LLM inference. Core depends on this port, not on concrete crates.
///
/// # Async approach: AFIT (async fn in trait, stabilised Rust 1.75)
///
/// AFIT is used here instead of `async_trait` because all concrete adapters are monomorphised
/// at the composition root via generics — no `Box<dyn InferencePort>` or `Arc<dyn InferencePort>`.
/// This avoids a runtime allocation per call and keeps the trait zero-cost.
/// Trade-off: the trait is NOT dyn-compatible; callers must use a type parameter (`I: InferencePort`).
///
/// The `async_fn_in_trait` lint is suppressed: all callers are in-crate generics, not
/// external trait users, so the missing `Send` bound on the opaque future is intentional.
#[allow(async_fn_in_trait)]
pub trait InferencePort: Send + Sync {
    /// Start streaming inference for the given request.
    ///
    /// Returns a channel receiver that yields token chunks as they become available.
    /// The channel closes when generation completes or the cancellation token fires.
    async fn infer(
        &self,
        req: InferenceRequest,
    ) -> Result<Receiver<Result<Token, InferenceError>>, InferenceError>;
}
