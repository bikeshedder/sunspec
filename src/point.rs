use std::marker::PhantomData;

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
    pub fn from_data(&self, data: &[u16]) -> Result<T, DecodeError> {
        let slice = data
            .get(self.offset as usize..(self.offset as usize + self.length as usize))
            .ok_or(DecodeError::OutOfBounds)?;
        let value = T::decode(slice)?;
        Ok(value)
    }
}

impl<M: Model, T: Value> Copy for PointDef<M, T> {}

impl<M: Model, T: Value> Clone for PointDef<M, T> {
    fn clone(&self) -> Self {
        *self
    }
}
