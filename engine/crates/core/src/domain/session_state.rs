//! Session lifecycle states for the engine orchestrator.
//!
//! Values match the architecture documentation (`architecture/modules/core`).

/// Global session lifecycle for the core orchestrator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionState {
    /// No active user session or capture.
    Idle,
    /// Waiting for wake word or listen trigger.
    Listening,
    /// Capturing audio/video for a session.
    Recording,
    /// Running inference (STT, LLM, vision, etc.).
    Processing,
    /// Generating output (e.g. TTS) to the user.
    Responding,
    /// Unrecoverable or operator-visible failure; requires explicit recovery.
    Error,
}

impl SessionState {
    /// Returns true if an interrupt should tear down in-flight work and move to a safe state.
    pub fn is_active(self) -> bool {
        matches!(
            self,
            SessionState::Listening
                | SessionState::Recording
                | SessionState::Processing
                | SessionState::Responding
        )
    }
}
