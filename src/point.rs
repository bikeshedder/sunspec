use std::marker::PhantomData;

use crate::{
    value::{DecodeError, FixedSize, Value},
    Group,
};

/// Definition of a point
#[derive(Debug)]
pub struct Point<G: Group, T: Value> {
    /// Offset within the model
    pub offset: u16,
    /// Length of the data
    pub length: u16,
    /// Is this point writable?
    pub writable: bool,
    model: PhantomData<G>,
    point_type: PhantomData<T>,
}

impl<G: Group, T: Value> Point<G, T> {
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
    pub fn from_data(&self, data: &[u16]) -> Result<T, DecodeError> {
        let slice = data
            .get(self.offset as usize..(self.offset as usize + self.length as usize))
            .ok_or(DecodeError::OutOfBounds)?;
        let value = T::decode(slice)?;
        Ok(value)
    }
}

impl<G: Group, T: FixedSize> Point<G, T> {
    /// Check whether a decoded point value is invalid.
    pub fn is_invalid(&self, value: &T) -> bool {
        value.is_invalid()
    }
}
