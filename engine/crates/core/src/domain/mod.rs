//! Core orchestrator domain types (SessionManager, EventBus, etc.).

mod error;
mod session_manager;
mod session_state;
mod state_machine;

pub use error::CoreError;
pub use session_manager::SessionManager;
pub use session_state::SessionState;
pub use state_machine::StateMachine;
