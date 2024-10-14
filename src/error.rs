use thiserror::Error;

#[cfg(feature = "tokio-modbus")]
use crate::tokio_modbus::TokioModbusError;

/// This error is returned if a communication fails because of a timeout
/// or underlying modbus error.
#[derive(Error, Debug)]
pub enum CommunicationError {
    /// The operation timed out
    #[error("Timeout")]
    Timeout,
    /// Implementation specific modbus error
    #[cfg(feature = "tokio-modbus")]
    #[error("Modbus")]
    Modbus(#[from] TokioModbusError),
}

#[cfg(feature = "tokio-modbus")]
impl CommunicationError {
    pub(crate) fn from_modbus_error(e: tokio_modbus::Error) -> Self {
        Self::Modbus(TokioModbusError::Error(e))
    }
    pub(crate) fn from_modbus_exception(e: tokio_modbus::Exception) -> Self {
        Self::Modbus(TokioModbusError::Exception(e))
    }
}
