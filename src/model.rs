use std::fmt::Debug;

use thiserror::Error;

use crate::{sealed::Sealed, AnyModel, DecodeError, Group, ModelInfo};

/// Error returned while parsing a model from registers.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum ParseError {
    /// The data ended before all points and groups of the model were
    /// read.
    #[error("Model data too short")]
    TooShort,
    /// The data of a repeating group is not a multiple of the group
    /// length.
    #[error("Invalid length of repeating group")]
    InvalidGroupLength,
    /// A point value could not be decoded.
    #[error(transparent)]
    Decode(#[from] DecodeError),
}

/// Every model implements this trait which contains information about
/// the model and a method for parsing it.
///
/// This trait is sealed and cannot be implemented outside of this crate.
pub trait Model: Sealed + Sized + Group + Debug + Into<AnyModel> {
    /// Model ID
    const ID: u16;
    /// Name of the model as defined by the SunSpec specification
    const NAME: &'static str;
    /// Label of the model as defined by the SunSpec specification
    const LABEL: &'static str;
    /// Information about this model which is also available at runtime
    const INFO: ModelInfo = ModelInfo::of::<Self>();
    /// Parse model data from a given u16 slice
    fn parse(data: &[u16]) -> Result<Self, ParseError>;
}
