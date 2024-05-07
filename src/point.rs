use std::{io, marker::PhantomData};

use thiserror::Error;

use crate::{
    model::Model,
    value::{DecodeError, Value},
};

/// Definition of a point
#[derive(Debug)]
pub struct PointDef<M: Model, T: Value> {
    /// Offset within the model
    pub offset: u16,
    /// Length of the data
    pub length: u16,
    /// Is this point writable?
    pub writable: bool,
    model: PhantomData<M>,
    point_type: PhantomData<T>,
}

impl<M: Model, T: Value> PointDef<M, T> {
    /// Create new point definition
    pub const fn new(offset: u16, length: u16, writable: bool) -> Self {
        Self {
            offset,
            length,
            writable,
            model: PhantomData,
            point_type: PhantomData,
        }
    }
    /// Parse value from given data
    pub fn from_data(&self, data: &[u16]) -> Result<T, ReadPointError> {
        let slice = data
            .get(self.offset as usize..(self.offset as usize + self.length as usize))
            .ok_or(ReadPointError::DecodeError(DecodeError::OutOfBounds))?;
        let value = T::decode(slice)?;
        Ok(value)
    }
}

/// This error is returned if there was an error while
/// reading data from a point.
#[derive(Debug, Error)]
pub enum ReadPointError {
    /// I/O error occured. Please note that all errors returned by `tokio-modbus`
    /// are stored inside this I/O error.
    #[error("I/O error")]
    IO(#[from] io::Error),
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
    /// I/O error occured. Please note that all errors returned by `tokio-modbus`
    /// are stored inside this I/O error.
    #[error("I/O error")]
    IO(#[from] io::Error),
    /// The encoded value was too large for the point.
    #[error("Encoded value too large for point")]
    ValueTooLarge,
}
