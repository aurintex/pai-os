//! # pai-core
//!
//! Central orchestrator domain for the paiOS engine: [`SessionState`](domain::SessionState),
//! [`StateMachine`](domain::StateMachine), [`SessionManager`](domain::SessionManager), MVP
//! [`flows::interaction`](flows), and (later) hexagonal ports.
//!
//! **Architecture:** [Core module](https://docs.aurintex.com/architecture/modules/core/) —
//! SessionManager, state machine, flows, EventBus, and saga rollback are described there; only part
//! of that surface is implemented so far (session lifecycle + Interaction flow helpers).
//!
//! The composition root is [`pai-engine`](https://github.com/aurintex/pai-os/tree/main/engine/pai-engine);
//! it is the only place that should construct adapters and inject them into core.

pub mod adapters;
pub mod domain;
pub mod flows;
pub mod ports;
