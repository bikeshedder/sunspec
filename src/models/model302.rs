//! Irradiance Model
/// Irradiance Model
///
/// Include to support various irradiance measurements
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model302 {}
#[allow(missing_docs)]
impl Model302 {}
impl crate::Model for Model302 {
    const ID: u16 = 302;
    fn from_data(#[allow(unused)] data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {})
    }
}
