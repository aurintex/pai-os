//! Bounded in-process event bus using [`tokio::sync::mpsc`].

use crate::domain::events::DomainEvent;
use tokio::sync::mpsc;

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

    /// Non-blocking publish; returns the event back if the buffer is full.
    pub fn try_publish(&self, event: DomainEvent) -> Result<(), DomainEvent> {
        self.sender.try_send(event).map_err(|e| e.into_inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
