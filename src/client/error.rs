use std::error::Error;

use thiserror::Error;

use crate::DecodeError;

/// This error is returned if a communication fails because of a timeout
/// or underlying modbus error.
#[derive(Error, Debug)]
pub enum CommunicationError {
    /// The operation timed out
    #[error("Timeout")]
    Timeout,
    /// Implementation specific modbus error
    #[error("Modbus")]
    Modbus(Box<dyn Error + Send + Sync>),
}

impl CommunicationError {
    /// Create communication error from implementation specific modbus error
    pub fn from_modbus(error: impl Error + Send + Sync + 'static) -> Self {
        Self::Modbus(Box::new(error))
    }
}

/// This error is returned if there was an error loading the
/// requested model.
#[derive(Debug, Error)]
pub enum ReadModelError {
    /// Some error occured while communicating via the modbus. This
    /// error is implementation specific.
    #[error("Modbus: {0}")]
    Communication(#[from] CommunicationError),
    /// The reading of a point within the model failed. Please see the
    /// encapsulated `ReadPointError` for further details.
    #[error("Reading point failed")]
    Point(#[from] ReadPointError),
}

/// This error is returned if there was an error while
/// reading data from a point.
#[derive(Debug, Error)]
pub enum ReadPointError {
    /// Communication error.
    #[error("Communication error")]
    Communication(#[from] CommunicationError),
    /// The decoding of the point data failed.
    #[error("Decode error")]
    DecodeError(#[from] DecodeError),
    /// The point is mandatory but value is missing.
    #[error("Missing mandatory value")]
    MissingMandatoryValue,
}

/// This error is returned if there was an error while
/// writing data to a point.
#[derive(Debug, Error)]
pub enum WritePointError {
    /// Communication error.
    #[error("Communication error")]
    Communication(#[from] CommunicationError),
    /// The encoded value was too large for the point.
    #[error("Encoded value too large for point")]
    ValueTooLarge,
}
