use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use thiserror::Error;

use crate::{DecodeError, Group, Models};

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
    /// Get model address from discovered models struct
    fn addr(models: &Models) -> ModelAddr<Self>;
    /// Parse model data from a given u16 slice
    fn parse(data: &[u16]) -> Result<Self, ParseError<Self>>;
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

// The following impls are written manually as `#[derive(...)]` would
// add a `M: Clone`/`M: PartialEq`/... bound which is not needed as the
// model type is only used as a `PhantomData` marker.

impl<M: Model> Clone for ModelAddr<M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M: Model> Copy for ModelAddr<M> {}

impl<M: Model> PartialEq for ModelAddr<M> {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr && self.len == other.len
    }
}

impl<M: Model> Eq for ModelAddr<M> {}

impl<M: Model> Hash for ModelAddr<M> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.addr.hash(state);
        self.len.hash(state);
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
