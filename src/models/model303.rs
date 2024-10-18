//! Back of Module Temperature Model
/// Back of Module Temperature Model
///
/// Include to support variable number of  back of module temperature measurements
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model303 {}
#[allow(missing_docs)]
impl Model303 {}
impl crate::Model for Model303 {
    const ID: u16 = 303;
    fn from_data(#[allow(unused)] data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {})
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m303
    }
}
