//! Ports (traits) defined and consumed by the core orchestrator.

mod flow_runner;
mod inference;

pub use flow_runner::{FlowError, FlowResult, FlowRunner, FlowType, SessionContext};
pub use inference::{InferenceError, InferencePort};
