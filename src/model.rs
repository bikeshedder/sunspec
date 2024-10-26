use std::marker::PhantomData;

use crate::{DecodeError, Models};

/// Every model implements this trait which contains methods
/// for accessing
pub trait Model: Sized {
    /// Model ID
    const ID: u16;
    /// Model length
    const LEN: u16;
    /// Parse model points from a given u16 slice
    fn from_data(data: &[u16]) -> Result<Self, DecodeError>;
    /// Get model address from discovered models struct
    fn addr(models: &Models) -> ModelAddr<Self>;
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
