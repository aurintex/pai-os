//! [`SensorRelayPort`] — start/stop sensor data relay and subscribe to readings.

use thiserror::Error;

/// Errors from a [`SensorRelayPort`] implementation.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SensorRelayError {
    /// The requested sensor does not exist or is not registered.
    #[error("sensor not found: {0}")]
    SensorNotFound(String),
    /// The relay is already active for this sensor.
    #[error("relay already active for sensor: {0}")]
    AlreadyActive(String),
    /// The relay is not active for this sensor; cannot stop what is not running.
    #[error("relay not active for sensor: {0}")]
    NotActive(String),
    /// A hardware or driver error occurred while reading sensor data.
    #[error("sensor hardware error for '{sensor}': {reason}")]
    HardwareError {
        /// Identifier of the affected sensor.
        sensor: String,
        /// Human-readable description of the hardware fault.
        reason: String,
    },
}

/// A single reading delivered from a sensor relay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SensorReading {
    /// Identifier of the sensor that produced this reading (e.g. `"microphone"`, `"imu"`).
    pub sensor_id: String,
    /// Raw payload bytes from the sensor (format is sensor-specific).
    pub payload: Vec<u8>,
    /// Monotonic timestamp in milliseconds since relay start.
    pub timestamp_ms: u64,
}

impl SensorReading {
    /// Create a new [`SensorReading`].
    pub fn new(sensor_id: impl Into<String>, payload: Vec<u8>, timestamp_ms: u64) -> Self {
        Self {
            sensor_id: sensor_id.into(),
            payload,
            timestamp_ms,
        }
    }
}

/// Starts and stops sensor data relays and delivers readings to the orchestrator.
///
/// Implementations are provided by a platform adapter and injected at the composition root.
/// Core holds only this port interface and is unaware of the concrete sensor hardware.
#[allow(async_fn_in_trait)]
pub trait SensorRelayPort: Send + Sync {
    /// Begin relaying data from the specified sensor.
    ///
    /// Returns `Ok(())` when the relay is successfully started.
    /// Returns [`SensorRelayError::AlreadyActive`] if the relay is already running.
    async fn start_relay(&self, sensor_id: &str) -> Result<(), SensorRelayError>;

    /// Stop an active relay for the specified sensor.
    ///
    /// Returns `Ok(())` when the relay is successfully stopped.
    /// Returns [`SensorRelayError::NotActive`] if no relay is running for this sensor.
    async fn stop_relay(&self, sensor_id: &str) -> Result<(), SensorRelayError>;

    /// Poll for the next available reading from the specified sensor.
    ///
    /// Returns `Ok(Some(SensorReading))` when a reading is available,
    /// `Ok(None)` when the relay is idle or no new data has arrived,
    /// or an error if the sensor encounters a hardware fault.
    async fn next_reading(
        &self,
        sensor_id: &str,
    ) -> Result<Option<SensorReading>, SensorRelayError>;
}
