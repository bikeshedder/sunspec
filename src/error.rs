use std::error::Error;

use thiserror::Error;

/// This error is returned if a communication fails because of a timeout
/// or underlying modbus error.
#[derive(Error, Debug)]
pub enum CommunicationError {
    /// The operation timed out
    #[error("Timeout")]
    Timeout,
    /// Implementation specific modbus error
    #[error("Modbus")]
    Modbus(Box<dyn Error>),
}

impl CommunicationError {
    /// Create communication error from implementation specific modbus error
    pub fn from_modbus(error: impl Error + 'static) -> Self {
        Self::Modbus(Box::new(error))
    }
}