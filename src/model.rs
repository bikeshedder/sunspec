use thiserror::Error;

use crate::{point::ReadPointError, CommunicationError};

/// Every model implements this trait which contains methods
/// for accessing
pub trait Model: Sized {
    /// Model ID
    const ID: u16;
    /// Parse model points from a given u16 slice
    fn from_data(data: &[u16]) -> Result<Self, ReadModelError>;
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
