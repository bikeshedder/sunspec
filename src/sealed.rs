use crate::{ModelInfo, ParseError};

/// Supertrait of all traits which must not be implemented outside of
/// this crate. As this trait is not nameable from outside of this crate
/// those traits can be extended without breaking compatibility.
pub trait Sealed {}

/// Implementation of [`ModelKind`](crate::ModelKind). It is not
/// nameable from outside of this crate, which keeps its methods private.
pub trait ModelKindImpl: Sized {
    /// Check whether a discovered model with the given information is
    /// of this kind.
    fn matches(info: &ModelInfo) -> bool;
    /// Parse the data of a discovered model with the given information.
    fn parse(info: &ModelInfo, data: &[u16]) -> Result<Self, ParseError>;
}
