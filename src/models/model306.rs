//! Reference Point Model
/// Reference Point Model
///
/// Include to support a standard reference point
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model306 {
    /// GHI
    ///
    /// Global Horizontal Irradiance
    pub ghi: Option<u16>,
    /// Amps
    ///
    /// Current measurement at reference point
    pub a: Option<u16>,
    /// Voltage
    ///
    /// Voltage  measurement at reference point
    pub v: Option<u16>,
    /// Temperature
    ///
    /// Temperature measurement at reference point
    pub tmp: Option<u16>,
}
#[allow(missing_docs)]
impl Model306 {
    pub const GHI: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, false);
    pub const A: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, false);
    pub const V: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, false);
    pub const TMP: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, false);
}
impl crate::Model for Model306 {
    const ID: u16 = 306;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            ghi: Self::GHI.from_data(data)?,
            a: Self::A.from_data(data)?,
            v: Self::V.from_data(data)?,
            tmp: Self::TMP.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m306
    }
}
