//! [`SessionManager`] — central orchestration entry point (composition will inject ports later).

use crate::domain::error::CoreError;
use crate::domain::state_machine::StateMachine;
use crate::flows::interaction::{apply_interaction_event, InteractionEvent};

/// Coordinates session lifecycle; owns the global [`StateMachine`].
///
/// Future: EventBus, full `FlowRunner`, and capability ports attach here per architecture docs.
#[derive(Debug, Default)]
pub struct SessionManager {
    state_machine: StateMachine,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            state_machine: StateMachine::new(),
        }
    }

    pub fn state_machine(&self) -> &StateMachine {
        &self.state_machine
    }

    pub fn state_machine_mut(&mut self) -> &mut StateMachine {
        &mut self.state_machine
    }

    /// Handle a physical / HMI interaction (MVP Interaction flow).
    pub fn handle_interaction(&mut self, event: InteractionEvent) -> Result<(), CoreError> {
        apply_interaction_event(&mut self.state_machine, event)
    }
}
