//! Global session [`StateMachine`](crate::domain::state_machine::StateMachine).
//!
//! Enforces valid transitions and interrupt handling described in the architecture docs.

use crate::domain::error::CoreError;
use crate::domain::session_state::SessionState;

/// Owns [`SessionState`] and validates transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateMachine {
    state: SessionState,
}

impl StateMachine {
    /// Starts in [`SessionState::Idle`].
    pub fn new() -> Self {
        Self {
            state: SessionState::Idle,
        }
    }

    /// Current session state.
    pub fn state(&self) -> SessionState {
        self.state
    }

    /// Move to `to` if allowed; otherwise returns [`CoreError::InvalidTransition`].
    pub fn transition_to(&mut self, to: SessionState) -> Result<(), CoreError> {
        if !is_valid_transition(self.state, to) {
            return Err(CoreError::InvalidTransition {
                from: self.state,
                to,
            });
        }
        self.state = to;
        Ok(())
    }

    /// Roll back in-flight work to a safe state (Listening preferred; Idle when already Listening).
    ///
    /// From active capture/inference/response states, transitions to [`SessionState::Listening`].
    /// Idempotent when already `Idle`, `Listening`, or `Error`.
    pub fn interrupt(&mut self) {
        match self.state {
            SessionState::Recording | SessionState::Processing | SessionState::Responding => {
                self.state = SessionState::Listening;
            }
            SessionState::Listening => {
                self.state = SessionState::Idle;
            }
            SessionState::Idle | SessionState::Error => {}
        }
    }
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

fn is_valid_transition(from: SessionState, to: SessionState) -> bool {
    use SessionState::*;
    match (from, to) {
        // Idempotent / no-op
        (a, b) if a == b => true,

        (Idle, Listening) | (Idle, Error) => true,
        (Listening, Idle)
        | (Listening, Recording)
        | (Listening, Processing)
        | (Listening, Error) => true,
        (Recording, Listening)
        | (Recording, Processing)
        | (Recording, Idle)
        | (Recording, Error) => true,
        (Processing, Responding)
        | (Processing, Listening)
        | (Processing, Idle)
        | (Processing, Error) => true,
        (Responding, Idle) | (Responding, Listening) | (Responding, Error) => true,
        (Error, Idle) => true,

        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idle_to_listening_ok() {
        let mut sm = StateMachine::new();
        assert!(sm.transition_to(SessionState::Listening).is_ok());
        assert_eq!(sm.state(), SessionState::Listening);
    }

    #[test]
    fn idle_to_recording_invalid() {
        let mut sm = StateMachine::new();
        assert_eq!(
            sm.transition_to(SessionState::Recording),
            Err(CoreError::InvalidTransition {
                from: SessionState::Idle,
                to: SessionState::Recording,
            })
        );
    }

    #[test]
    fn interrupt_from_processing_goes_to_listening() {
        let mut sm = StateMachine::new();
        sm.transition_to(SessionState::Listening).unwrap();
        sm.transition_to(SessionState::Recording).unwrap();
        sm.transition_to(SessionState::Processing).unwrap();
        sm.interrupt();
        assert_eq!(sm.state(), SessionState::Listening);
    }

    #[test]
    fn interrupt_from_listening_goes_to_idle() {
        let mut sm = StateMachine::new();
        sm.transition_to(SessionState::Listening).unwrap();
        sm.interrupt();
        assert_eq!(sm.state(), SessionState::Idle);
    }
}
