//! Ports (traits) defined and consumed by the core orchestrator.

mod device_control;
mod flow_runner;
mod inference;
mod sensor_relay;
mod session_config;

pub use crate::types::{InferenceRequest, Token};
pub use device_control::{DeviceControlError, DeviceControlPort, DevicePowerState};
pub use flow_runner::{FlowError, FlowResult, FlowRunner, FlowType, SessionContext};
pub use inference::{InferenceError, InferencePort};
pub use sensor_relay::{SensorReading, SensorRelayError, SensorRelayPort};
pub use session_config::{ConfigEntry, SessionConfigError, SessionConfigPort};
