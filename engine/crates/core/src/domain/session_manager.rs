//! [`SessionManager`] — central orchestration entry point (composition injects ports).

use crate::domain::error::CoreError;
use crate::domain::state_machine::StateMachine;
use crate::domain::{EventBus, SessionState};
use crate::flows::interaction::{apply_interaction_event, InteractionEvent};
use crate::ports::{FlowError, FlowRunner, FlowType, SessionContext};
use std::sync::Arc;
use tracing::warn;

/// Coordinates session lifecycle; owns the global [`StateMachine`] and delegates flows to [`FlowRunner`].
pub struct SessionManager {
    state_machine: StateMachine,
    flow_runner: Arc<dyn FlowRunner>,
    event_bus: EventBus,
}

impl SessionManager {
    pub fn new(flow_runner: Arc<dyn FlowRunner>, event_bus: EventBus) -> Self {
        Self {
            state_machine: StateMachine::new(),
            flow_runner,
            event_bus,
        }
    }

    pub fn state_machine(&self) -> &StateMachine {
        &self.state_machine
    }

    pub fn state_machine_mut(&mut self) -> &mut StateMachine {
        &mut self.state_machine
    }

    pub fn event_bus(&self) -> &EventBus {
        &self.event_bus
    }

    pub fn flow_runner(&self) -> &Arc<dyn FlowRunner> {
        &self.flow_runner
    }

    /// Handle a physical / HMI interaction (MVP Interaction flow).
    pub fn handle_interaction(&mut self, event: InteractionEvent) -> Result<(), CoreError> {
        apply_interaction_event(&mut self.state_machine, event)
    }

    /// Run the static inference echo flow: prompt → inference → response, with state transitions.
    pub fn handle_flow_request(&mut self, prompt: &str) -> Result<String, FlowError> {
        match self.state_machine.state() {
            SessionState::Idle => {
                self.state_machine.transition_to(SessionState::Listening)?;
            }
            SessionState::Listening => {}
            other => return Err(FlowError::SessionNotReady(other)),
        }

        self.state_machine.transition_to(SessionState::Processing)?;

        let ctx = SessionContext::new(self.state_machine.state(), prompt.to_string());
        let result = self.flow_runner.execute(FlowType::InferenceEcho, &ctx);

        match result {
            Ok(flow) => {
                self.state_machine.transition_to(SessionState::Responding)?;
                self.state_machine.transition_to(SessionState::Listening)?;
                Ok(flow.response)
            }
            Err(e) => {
                if let Err(transition_err) = self.state_machine.transition_to(SessionState::Error) {
                    warn!(
                        flow_error = ?e,
                        transition_error = ?transition_err,
                        "failed to transition to Error state after flow failure"
                    );
                }
                Err(e)
            }
        }
    }
}

#[cfg(test)]
pub(crate) fn session_manager_for_interaction_tests() -> SessionManager {
    use crate::adapters::HardcodedFlowRunner;
    use crate::ports::{InferenceError, InferencePort};

    #[derive(Debug)]
    struct StubInference;

    impl InferencePort for StubInference {
        fn complete(&self, _prompt: &str) -> Result<String, InferenceError> {
            Ok(String::new())
        }
    }

    let (event_bus, _rx) = EventBus::channel(8);
    let runner = Arc::new(HardcodedFlowRunner::new(
        Arc::new(StubInference),
        event_bus.clone(),
    ));
    SessionManager::new(runner, event_bus)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::HardcodedFlowRunner;
    use crate::domain::SessionState;
    use crate::ports::{InferenceError, InferencePort};

    #[derive(Debug)]
    struct EchoInference;

    impl InferencePort for EchoInference {
        fn complete(&self, prompt: &str) -> Result<String, InferenceError> {
            Ok(format!("echo:{prompt}"))
        }
    }

    #[test]
    fn handle_flow_request_happy_path() {
        let (bus, mut rx) = EventBus::channel(8);
        let runner = Arc::new(HardcodedFlowRunner::new(
            Arc::new(EchoInference),
            bus.clone(),
        ));
        let mut mgr = SessionManager::new(runner, bus);
        let out = mgr.handle_flow_request("hello").unwrap();
        assert_eq!(out, "echo:hello");
        assert_eq!(mgr.state_machine().state(), SessionState::Listening);

        let e1 = rx.blocking_recv().expect("event 1");
        let e2 = rx.blocking_recv().expect("event 2");
        assert!(matches!(
            e1,
            crate::domain::DomainEvent::InferenceRequested { .. }
        ));
        assert!(matches!(
            e2,
            crate::domain::DomainEvent::InferenceCompleted { .. }
        ));
    }
}
