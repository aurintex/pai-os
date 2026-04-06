//! MVP [`FlowRunner`](crate::ports::FlowRunner) — static request → inference → response.

use crate::domain::{DomainEvent, EventBus};
use crate::ports::{FlowError, FlowResult, FlowRunner, FlowType, InferencePort, SessionContext};
use std::sync::Arc;
use tokio::sync::mpsc::error::TrySendError;

/// Built-in flows only; user-defined scripting is out of scope (see issue #38).
pub struct HardcodedFlowRunner {
    inference: Arc<dyn InferencePort>,
    event_bus: EventBus,
}

impl HardcodedFlowRunner {
    pub fn new(inference: Arc<dyn InferencePort>, event_bus: EventBus) -> Self {
        Self {
            inference,
            event_bus,
        }
    }

    fn publish(&self, event: DomainEvent) -> Result<(), FlowError> {
        self.event_bus.try_publish(event).map_err(|e| match e {
            TrySendError::Full(_) => FlowError::EventBusFull,
            TrySendError::Closed(_) => FlowError::EventBusClosed,
        })
    }
}

impl FlowRunner for HardcodedFlowRunner {
    fn execute(
        &self,
        flow_type: FlowType,
        context: &SessionContext,
    ) -> Result<FlowResult, FlowError> {
        match flow_type {
            FlowType::InferenceEcho => {
                if context.prompt.trim().is_empty() {
                    return Err(FlowError::EmptyPrompt);
                }

                self.publish(DomainEvent::InferenceRequested {
                    prompt: context.prompt.clone(),
                })?;

                let response = self.inference.complete(&context.prompt)?;

                self.publish(DomainEvent::InferenceCompleted {
                    response: response.clone(),
                })?;

                Ok(FlowResult { response })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::SessionState;
    use crate::ports::InferenceError;

    #[derive(Debug)]
    struct EchoInference;

    impl InferencePort for EchoInference {
        fn complete(&self, prompt: &str) -> Result<String, InferenceError> {
            Ok(format!("echo:{prompt}"))
        }
    }

    #[tokio::test]
    async fn inference_echo_end_to_end() {
        let (bus, mut rx) = EventBus::channel(8);
        let runner = HardcodedFlowRunner::new(Arc::new(EchoInference), bus);
        let ctx = SessionContext::new(SessionState::Listening, "hi");
        let out = runner.execute(FlowType::InferenceEcho, &ctx).unwrap();
        assert_eq!(out.response, "echo:hi");

        match rx.recv().await {
            Some(DomainEvent::InferenceRequested { prompt }) => assert_eq!(prompt, "hi"),
            other => panic!("expected InferenceRequested, got {other:?}"),
        }
        match rx.recv().await {
            Some(DomainEvent::InferenceCompleted { response }) => assert_eq!(response, "echo:hi"),
            other => panic!("expected InferenceCompleted, got {other:?}"),
        }
    }
}
