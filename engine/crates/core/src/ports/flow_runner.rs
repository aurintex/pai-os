//! [`FlowRunner`] port and flow metadata (see architecture docs: MVP static flows).

use crate::domain::{CoreError, SessionState};
use crate::ports::inference::InferenceError;
use thiserror::Error;

/// Static MVP flow kinds; extend as more hardcoded flows land.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlowType {
    /// Minimal path: prompt → inference → text response.
    InferenceEcho,
}

/// Per-invocation inputs for flow execution (session-scoped).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionContext {
    /// Snapshot of orchestrator state for the session.
    pub session_state: SessionState,
    /// Prompt text for inference-backed flows.
    pub prompt: String,
}

impl SessionContext {
    pub fn new(session_state: SessionState, prompt: impl Into<String>) -> Self {
        Self {
            session_state,
            prompt: prompt.into(),
        }
    }
}

/// Outcome of a completed flow step (MVP: single text result).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlowResult {
    pub response: String,
}

/// Errors while executing a flow.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum FlowError {
    #[error("flow type {0:?} is not supported by this runner")]
    UnsupportedFlowType(FlowType),
    #[error("inference error: {0}")]
    Inference(#[from] InferenceError),
    #[error("event bus is full; could not publish domain event")]
    EventBusFull,
    #[error("inference prompt was empty")]
    EmptyPrompt,
    #[error("session state does not allow this flow (state={0:?})")]
    SessionNotReady(SessionState),
    #[error(transparent)]
    State(#[from] CoreError),
}

/// Executes built-in flows behind a stable interface (MVP: [`crate::adapters::HardcodedFlowRunner`]).
pub trait FlowRunner: Send + Sync {
    fn execute(
        &self,
        flow_type: FlowType,
        context: &SessionContext,
    ) -> Result<FlowResult, FlowError>;
}
