use std::marker::PhantomData;

use crate::{sealed::Sealed, value::Value, Group, ParseError};

/// The access of a point, which is either [`ReadOnly`] or [`ReadWrite`].
///
/// This trait is sealed and cannot be implemented outside of this crate.
pub trait Access: Sealed {
    /// Whether points with this access can be written.
    const WRITABLE: bool;
}

/// Access of points which can only be read.
#[derive(Debug)]
pub enum ReadOnly {}

/// Access of points which can be read and written.
#[derive(Debug)]
pub enum ReadWrite {}

impl Sealed for ReadOnly {}
impl Sealed for ReadWrite {}

impl Access for ReadOnly {
    const WRITABLE: bool = false;
}

impl Access for ReadWrite {
    const WRITABLE: bool = true;
}

/// Definition of a point
///
/// The type parameter `A` is the [`Access`] of the point. Only points
/// with [`ReadWrite`] access can be written.
#[derive(Debug)]
pub struct Point<G: Group, T: Value, A: Access = ReadOnly> {
    /// Offset within the model
    pub offset: u16,
    /// Length of the data
    pub length: u16,
    model: PhantomData<G>,
    point_type: PhantomData<T>,
    access: PhantomData<A>,
}

impl<G: Group, T: Value, A: Access> Point<G, T, A> {
    /// Create new point definition
    pub const fn new(offset: u16, length: u16) -> Self {
        Self {
            offset,
            length,
            model: PhantomData,
            point_type: PhantomData,
            access: PhantomData,
        }
    }
    /// Is this point writable?
    pub const fn writable(&self) -> bool {
        A::WRITABLE
    }
    /// Parse the value of this point from the data of its group
    pub fn from_data(&self, data: &[u16]) -> Result<T, ParseError> {
        let slice = data
            .get(self.offset as usize..(self.offset as usize + self.length as usize))
            .ok_or(ParseError::TooShort)?;
        let value = T::decode(slice)?;
        Ok(value)
    }
}
