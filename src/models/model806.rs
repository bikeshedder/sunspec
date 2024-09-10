//! Flow Battery Model
/// Flow Battery Model
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model806 {
    /// Battery Points To Be Determined
    pub bat_tbd: u16,
}
#[allow(missing_docs)]
impl Model806 {
    pub const BAT_TBD: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
}
impl crate::Model for Model806 {
    const ID: u16 = 806;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            bat_tbd: Self::BAT_TBD.from_data(data)?,
        })
    }
}
