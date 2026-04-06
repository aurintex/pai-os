//! # pai-core
//!
//! Central orchestrator domain for the paiOS engine: [`SessionState`](domain::SessionState),
//! [`StateMachine`](domain::StateMachine), [`SessionManager`](domain::SessionManager), MVP
//! [`flows::interaction`](flows), [`EventBus`](domain::EventBus), [`FlowRunner`](ports::FlowRunner),
//! and [`HardcodedFlowRunner`](adapters::HardcodedFlowRunner).
//!
//! **Architecture:** [Core module](https://docs.aurintex.com/architecture/modules/core/) —
//! SessionManager, state machine, flows, EventBus, and saga rollback are described there; the MVP
//! static inference flow and bounded event bus are wired; saga rollback is still deferred.
//!
//! The composition root is [`pai-engine`](https://github.com/aurintex/pai-os/tree/main/engine/pai-engine);
//! it is the only place that should construct adapters and inject them into core.

pub mod adapters;
pub mod domain;
pub mod flows;
pub mod ports;
