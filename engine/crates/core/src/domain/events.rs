//! Domain events exchanged on the [`crate::domain::EventBus`].

/// Application-level events for orchestration and telemetry (in-process only).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainEvent {
    /// An inference request entered the static flow.
    InferenceRequested { prompt: String },
    /// Inference produced a text response.
    InferenceCompleted { response: String },
}
