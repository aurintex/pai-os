//! [`DeviceControlPort`] — privileged device lifecycle operations.

use thiserror::Error;

/// Errors from a [`DeviceControlPort`] implementation.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DeviceControlError {
    /// The operation is not permitted in the current device state.
    #[error("operation not permitted in current device state")]
    NotPermitted,
    /// The underlying hardware driver returned an error.
    #[error("hardware driver error: {0}")]
    DriverError(String),
    /// The requested operation is not supported on this device.
    #[error("operation not supported on this device: {0}")]
    Unsupported(String),
    /// The operation timed out before completing.
    #[error("device control operation timed out")]
    Timeout,
}

/// Controls privileged device lifecycle operations (reboot, factory reset, etc.).
///
/// Implementations are provided by a platform adapter and injected at the composition root.
/// Core holds only this port interface and is unaware of the concrete hardware.
pub trait DeviceControlPort: Send + Sync {
    /// Trigger a clean system reboot.
    ///
    /// Returns `Ok(())` if the reboot sequence was successfully initiated.
    /// The caller should not expect to receive a response after this returns.
    fn reboot(&self) -> Result<(), DeviceControlError>;

    /// Erase all user data and restore factory defaults.
    ///
    /// This is a destructive, irreversible operation. The caller is responsible
    /// for any confirmation or safety gating before invoking this method.
    fn factory_reset(&self) -> Result<(), DeviceControlError>;

    /// Query the current device power state.
    ///
    /// Returns a [`DevicePowerState`] describing whether the device is running
    /// normally, in a low-power mode, or in a degraded state.
    fn power_state(&self) -> Result<DevicePowerState, DeviceControlError>;
}

/// Represents the power / operational state of the device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevicePowerState {
    /// Device is operating normally at full capacity.
    Normal,
    /// Device is in a low-power / sleep mode.
    LowPower,
    /// Device is in a degraded state (e.g. thermal throttling, partial hardware failure).
    Degraded,
}
