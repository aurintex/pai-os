//! Core orchestrator domain types (SessionManager, EventBus, etc.).

mod error;
mod event_bus;
mod events;
mod session_manager;
mod session_state;
mod state_machine;

pub use error::CoreError;
pub use event_bus::EventBus;
pub use events::DomainEvent;
pub use session_manager::SessionManager;
pub use session_state::SessionState;
pub use state_machine::StateMachine;

#[cfg(test)]
pub(crate) use session_manager::session_manager_for_interaction_tests;
