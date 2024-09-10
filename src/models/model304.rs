//! Inclinometer Model
/// Inclinometer Model
///
/// Include to support orientation measurements
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model304 {}
#[allow(missing_docs)]
impl Model304 {}
impl crate::Model for Model304 {
    const ID: u16 = 304;
    fn from_data(#[allow(unused)] data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {})
    }
}
