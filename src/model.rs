use std::io;

use thiserror::Error;
use tokio_modbus::{client::Context, prelude::Reader};

use crate::discovery::ModelAddr;
use crate::point::ReadPointError;

/// Every model implements this trait which contains methods
/// for accessing
pub trait Model: Sized {
    /// Model ID
    const ID: u16;
    /// Length of the model
    const LENGTH: u16;
    /// Parse model points from a given u16 slice
    fn from_data(data: &[u16]) -> Result<Self, ReadModelError>;
}

/// Read model data from modbus
pub async fn read_model<M: Model>(
    ctx: &mut Context,
    addr: &ModelAddr<M>,
) -> Result<M, ReadModelError> {
    let data = ctx.read_holding_registers(addr.addr, addr.len).await?;
    M::from_data(&data)
}

/// This error is returned if there was an error loading the
/// requested model.
#[derive(Debug, Error)]
pub enum ReadModelError {
    /// I/O error occured. Please note that all errors returned by `tokio-modbus`
    /// are stored inside this I/O error.
    #[error("I/O error")]
    IO(#[from] io::Error),
    /// The reading of a point within the model failed. Please see the
    /// encapsulated `ReadPointError` for further details.
    #[error("Reading point failed")]
    Point(#[from] ReadPointError),
}
