//! High-level flows and MVP interaction with the session state machine.
//!
//! See [`interaction`] for the Interaction flow (UI events → session actions).

pub mod interaction;

pub use interaction::{apply_interaction_event, InteractionEvent};
