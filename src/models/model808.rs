//! Flow Battery Module Model
/// Flow Battery Module Model
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model808 {
    /// Module Points To Be Determined
    pub module_tbd: u16,
}
#[allow(missing_docs)]
impl Model808 {
    pub const MODULE_TBD: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
}
impl crate::Model for Model808 {
    const ID: u16 = 808;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            module_tbd: Self::MODULE_TBD.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m808
    }
}
