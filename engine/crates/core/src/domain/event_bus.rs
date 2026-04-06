//! Bounded in-process event bus using [`tokio::sync::mpsc`].

use crate::domain::events::DomainEvent;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TrySendError;

/// Single-consumer bounded channel for [`DomainEvent`] delivery.
///
/// Producers clone [`EventBus::sender`]. Exactly one task should consume the
/// [`mpsc::Receiver`] returned from [`EventBus::channel`].
#[derive(Clone, Debug)]
pub struct EventBus {
    sender: mpsc::Sender<DomainEvent>,
}

impl EventBus {
    /// Creates a bus and the sole receiver. `capacity` is the mpsc queue bound.
    pub fn channel(capacity: usize) -> (Self, mpsc::Receiver<DomainEvent>) {
        let (tx, rx) = mpsc::channel(capacity);
        (Self { sender: tx }, rx)
    }

    /// Cloneable handle for publishers (fan-in to the single consumer).
    pub fn sender(&self) -> mpsc::Sender<DomainEvent> {
        self.sender.clone()
    }

    /// Non-blocking publish. On failure, use [`TrySendError::into_inner`] to recover the event.
    pub fn try_publish(&self, event: DomainEvent) -> Result<(), TrySendError<DomainEvent>> {
        self.sender.try_send(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc::error::TrySendError;

    #[tokio::test]
    async fn try_publish_returns_full_when_buffer_saturated() {
        let (bus, mut rx) = EventBus::channel(1);
        bus.try_publish(DomainEvent::InferenceRequested {
            prompt: "first".into(),
        })
        .unwrap();

        let ev = DomainEvent::InferenceRequested {
            prompt: "second".into(),
        };
        let err = bus.try_publish(ev.clone()).unwrap_err();
        let got = match err {
            TrySendError::Full(e) => e,
            other => panic!("expected Full, got {other:?}"),
        };
        assert_eq!(got, ev);

        rx.recv().await;
    }

    #[tokio::test]
    async fn send_and_receive_round_trip() {
        let (bus, mut rx) = EventBus::channel(4);
        let prompt = "hello";
        bus.try_publish(DomainEvent::InferenceRequested {
            prompt: prompt.into(),
        })
        .unwrap();

        match rx.recv().await {
            Some(DomainEvent::InferenceRequested { prompt: p }) => assert_eq!(p, prompt),
            other => panic!("unexpected event: {other:?}"),
        }
    }

    #[tokio::test]
    async fn sender_clone_delivers_to_same_receiver() {
        let (bus, mut rx) = EventBus::channel(2);
        let s2 = bus.sender();
        s2.try_send(DomainEvent::InferenceCompleted {
            response: "ok".into(),
        })
        .unwrap();

        match rx.recv().await {
            Some(DomainEvent::InferenceCompleted { response }) => assert_eq!(response, "ok"),
            other => panic!("unexpected event: {other:?}"),
        }
    }
}
