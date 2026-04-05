use std::{fmt::Debug, marker::PhantomData};

use thiserror::Error;

use crate::{DecodeError, Group, Models};

/// Static metadata for a generated SunSpec model type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModelInfo {
    /// SunSpec model ID.
    pub id: u16,
    /// Stable model name from the source JSON.
    pub name: &'static str,
    /// Human-readable model label.
    pub label: &'static str,
    /// Human-readable model description.
    pub description: &'static str,
}

/// A discovered model together with its static metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiscoveredModel {
    /// Static metadata for the discovered model.
    pub info: ModelInfo,
    /// The discovered address of this model.
    pub addr: u16,
    /// The discovered length of this model.
    pub len: u16,
}

/// Model data that decoded successfully but failed semantic validation.
#[derive(Debug, Error)]
#[error("Invalid point data")]
pub struct InvalidPointData<T: Debug> {
    /// The decoded model data.
    pub model: T,
}

/// Error returned while parsing a model from registers.
#[derive(Debug, Error)]
pub enum ParseError<T: Debug> {
    /// Register decoding failed before a full model value could be produced.
    #[error(transparent)]
    Decode(#[from] DecodeError),
    /// The model decoded successfully but contains invalid point data.
    #[error(transparent)]
    InvalidPointData(InvalidPointData<T>),
}

/// Every model implements this trait which contains methods
/// for accessing the address and parsing the model.
pub trait Model: Sized + Group + Debug {
    /// Model ID
    const ID: u16;
    /// Stable model name from the source JSON.
    const NAME: &'static str;
    /// Human-readable model label.
    const LABEL: &'static str;
    /// Human-readable model description.
    const DESCRIPTION: &'static str;
    /// Get model address from discovered models struct
    fn addr(models: &Models) -> ModelAddr<Self>;
    /// Parse model data from a given u16 slice
    fn parse(data: &[u16]) -> Result<Self, ParseError<Self>>;
    /// Static metadata for this model type.
    fn info() -> ModelInfo {
        ModelInfo {
            id: Self::ID,
            name: Self::NAME,
            label: Self::LABEL,
            description: Self::DESCRIPTION,
        }
    }
}

/// This structure is used to store the address of
/// models after a successful model discovery.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct ModelAddr<M: Model> {
    /// The discovered address of this model.
    pub addr: u16,
    /// The discovered length of this model. A length of
    /// 0 indicates that the model is unsupported.
    pub len: u16,
    model: PhantomData<M>,
}

impl<M: Model> ModelAddr<M> {
    /// Set the address of a discovered model
    pub fn set_addr(&mut self, addr: u16, len: u16) {
        self.addr = addr;
        self.len = len;
    }
}

impl<M: Model> Default for ModelAddr<M> {
    fn default() -> Self {
        Self {
            addr: Default::default(),
            len: Default::default(),
            model: Default::default(),
        }
    }
}
