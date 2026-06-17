//! MVP [`FlowRunner`](crate::ports::FlowRunner) — static request → inference → response.

use crate::domain::{DomainEvent, EventBus};
use crate::ports::{
    FlowError, FlowResult, FlowRunner, FlowType, InferenceError, InferencePort, InferenceRequest,
    SessionContext,
};
use std::sync::Arc;
use tokio::sync::mpsc::error::TrySendError;

/// Built-in flows only; user-defined scripting is out of scope (see issue #38).
///
/// `I` is monomorphised at the composition root — one concrete inference adapter per build.
/// Using a generic instead of `Arc<dyn InferencePort>` avoids a vtable indirection and is the
/// preferred pattern when the type is known at compile time (see architecture docs).
pub struct HardcodedFlowRunner<I: InferencePort> {
    inference: Arc<I>,
    event_bus: EventBus,
}

impl<I: InferencePort> HardcodedFlowRunner<I> {
    pub fn new(inference: Arc<I>, event_bus: EventBus) -> Self {
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

impl<I: InferencePort + 'static> FlowRunner for HardcodedFlowRunner<I> {
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

                let req = InferenceRequest::simple(context.prompt.clone());

                // Bridge: async InferencePort → sync FlowRunner.
                // block_in_place moves the calling worker off the thread pool so that
                // Handle::current().block_on(...) can drive the async future to completion.
                // Both the engine binary and tests use the multi-thread Tokio runtime, which
                // is required for block_in_place to work correctly.
                let response = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(async {
                        let mut rx = self.inference.infer(req).await?;
                        let mut s = String::new();
                        while let Some(tok) = rx.recv().await {
                            s.push_str(&tok?.content);
                        }
                        Ok::<String, InferenceError>(s)
                    })
                })?;

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
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use crate::domain::SessionState;
    use crate::types::Token;
    use tokio::sync::mpsc;

    #[derive(Debug)]
    struct EchoInference;

    impl InferencePort for EchoInference {
        async fn infer(
            &self,
            req: InferenceRequest,
        ) -> Result<mpsc::Receiver<Result<Token, InferenceError>>, InferenceError> {
            let (tx, rx) = mpsc::channel(1);
            let content = format!("echo:{}", req.prompt);
            // try_send completes synchronously; no spawned task needed for the stub
            tx.try_send(Ok(Token { content }))
                .map_err(|_| InferenceError::Failed("channel send failed".into()))?;
            Ok(rx)
        }
    }

    // multi_thread required: execute() bridges sync→async via block_in_place,
    // which needs at least 2 threads in the runtime.
    #[tokio::test(flavor = "multi_thread")]
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
