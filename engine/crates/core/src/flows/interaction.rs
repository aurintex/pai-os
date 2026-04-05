//! MVP **Interaction** flow: physical UI events → session actions.
//!
//! Maps to [Core: MVP Flows — Interaction](https://docs.aurintex.com/architecture/modules/core/#mvp-flows-flows-module)
//! (e.g. button short press → start listening). Full `FlowRunner`, Voice, and EventBus are separate work.

use crate::domain::{CoreError, SessionState, StateMachine};

/// Physical / HMI events that the composition root will eventually receive from the peripherals layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InteractionEvent {
    /// Short press: request listen / wake path — transitions toward [`SessionState::Listening`] when valid.
    ///
    /// Typical mapping: idle device → arm listening (wake word, VAD, etc. in later layers).
    ShortPressStartListen,
    /// User or system interrupt: tear down in-flight work to a safe session state (see [`StateMachine::interrupt`]).
    Interrupt,
}

/// Apply a single interaction event to the global [`StateMachine`].
pub fn apply_interaction_event(
    machine: &mut StateMachine,
    event: InteractionEvent,
) -> Result<(), CoreError> {
    match event {
        InteractionEvent::Interrupt => {
            machine.interrupt();
            Ok(())
        }
        InteractionEvent::ShortPressStartListen => machine.transition_to(SessionState::Listening),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_press_idle_to_listening() {
        let mut sm = StateMachine::new();
        apply_interaction_event(&mut sm, InteractionEvent::ShortPressStartListen).unwrap();
        assert_eq!(sm.state(), SessionState::Listening);
    }

    #[test]
    fn short_press_idempotent_while_listening() {
        let mut sm = StateMachine::new();
        apply_interaction_event(&mut sm, InteractionEvent::ShortPressStartListen).unwrap();
        apply_interaction_event(&mut sm, InteractionEvent::ShortPressStartListen).unwrap();
        assert_eq!(sm.state(), SessionState::Listening);
    }

    #[test]
    fn interrupt_from_processing_via_event() {
        let mut sm = StateMachine::new();
        sm.transition_to(SessionState::Listening).unwrap();
        sm.transition_to(SessionState::Recording).unwrap();
        sm.transition_to(SessionState::Processing).unwrap();
        apply_interaction_event(&mut sm, InteractionEvent::Interrupt).unwrap();
        assert_eq!(sm.state(), SessionState::Listening);
    }

    #[test]
    fn short_press_from_error_is_rejected() {
        let mut sm = StateMachine::new();
        sm.transition_to(SessionState::Error).unwrap();
        assert_eq!(
            apply_interaction_event(&mut sm, InteractionEvent::ShortPressStartListen),
            Err(CoreError::InvalidTransition {
                from: SessionState::Error,
                to: SessionState::Listening,
            })
        );
    }

    #[test]
    fn session_manager_delegates_handle_interaction() {
        use crate::domain::SessionManager;

        let mut mgr = SessionManager::new();
        mgr.handle_interaction(InteractionEvent::ShortPressStartListen)
            .unwrap();
        assert_eq!(mgr.state_machine().state(), SessionState::Listening);
    }
}
