use thiserror::Error;

/// This error is returned if a communication fails because of a timeout
/// or underlying modbus error.
#[derive(Error, Debug)]
pub enum CommunicationError {
    /// The operation timed out
    #[error("Timeout")]
    Timeout,
}
