use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    marker::PhantomData,
    num::NonZeroU16,
};

use thiserror::Error;

use crate::{sealed::Sealed, AnyModel, DecodeError, Group, ModelInfo, Models};

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

impl<T: Debug> ParseError<T> {
    /// Convert the model contained in this error using the given function.
    ///
    /// This is used to convert the error of a specific model into the
    /// error of [`AnyModel`].
    pub fn map_model<U: Debug>(self, f: impl FnOnce(T) -> U) -> ParseError<U> {
        match self {
            Self::Decode(error) => ParseError::Decode(error),
            Self::InvalidPointData(InvalidPointData { model }) => {
                ParseError::InvalidPointData(InvalidPointData { model: f(model) })
            }
        }
    }
}

/// Every model implements this trait which contains methods
/// for accessing the address and parsing the model.
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
    /// Get model address from discovered models struct. Returns `None`
    /// if the model was not discovered.
    fn addr(models: &Models) -> Option<ModelAddr<Self>>;
    /// Parse model data from a given u16 slice
    fn parse(data: &[u16]) -> Result<Self, ParseError<Self>>;
}

/// This structure is used to store the address of
/// models after a successful model discovery.
///
/// The type parameter `M` is the model this address belongs to.
/// [`ModelInfo::addr`](crate::ModelInfo::addr) returns a
/// `ModelAddr<AnyModel>` for models which are only known at runtime.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct ModelAddr<M> {
    // A model can never start at address 0 as it is always preceded by
    // the SunS identifier. Using `NonZeroU16` makes
    // `Option<ModelAddr<M>>` the same size as `ModelAddr<M>`.
    addr: NonZeroU16,
    len: u16,
    model: PhantomData<M>,
}

impl<M> ModelAddr<M> {
    /// Create the address of a discovered model
    // Only used by the generated `Models::set_addr`, which is empty when
    // no models are enabled via Cargo features.
    #[allow(dead_code)]
    pub(crate) const fn new(addr: NonZeroU16, len: u16) -> Self {
        Self {
            addr,
            len,
            model: PhantomData,
        }
    }
    /// The discovered address of the first register following the
    /// model id and length registers.
    pub const fn addr(&self) -> u16 {
        self.addr.get()
    }
    /// The discovered length of the model. This is the number of
    /// registers following the model id and length registers.
    // This is not a collection, so `is_empty` would make no sense.
    #[allow(clippy::len_without_is_empty)]
    pub const fn len(&self) -> u16 {
        self.len
    }
    /// Change the model type of this address.
    pub(crate) fn cast<N>(self) -> ModelAddr<N> {
        ModelAddr {
            addr: self.addr,
            len: self.len,
            model: PhantomData,
        }
    }
}

// The following impls are written manually as `#[derive(...)]` would
// add a `M: Clone`/`M: PartialEq`/... bound which is not needed as the
// model type is only used as a `PhantomData` marker.

impl<M> Clone for ModelAddr<M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M> Copy for ModelAddr<M> {}

impl<M> PartialEq for ModelAddr<M> {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr && self.len == other.len
    }
}

impl<M> Eq for ModelAddr<M> {}

impl<M> Hash for ModelAddr<M> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.addr.hash(state);
        self.len.hash(state);
    }
}
