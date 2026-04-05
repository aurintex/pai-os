//! Core domain errors.

use crate::domain::session_state::SessionState;
use thiserror::Error;

/// Errors from session orchestration and state transitions.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    /// The requested transition is not allowed from the current state.
    #[error("invalid session transition from {from:?} to {to:?}")]
    InvalidTransition {
        from: SessionState,
        to: SessionState,
    },
}
