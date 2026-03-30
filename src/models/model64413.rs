//! PV Curves
/// PV Curves
///
/// Current-Voltage and Power-Voltage Profiles for PV Simulation.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model64413 {
    /// IV length
    ///
    /// Number of points in the IV curve.
    pub iv_len: Option<u16>,
    /// POA Irradiance
    ///
    /// Plane of Array Irradiance
    pub irr: Option<u16>,
    #[allow(missing_docs)]
    pub irr_sf: Option<i16>,
}
#[allow(missing_docs)]
impl Model64413 {
    pub const IV_LEN: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, false);
    pub const IRR: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, false);
    pub const IRR_SF: crate::Point<Self, Option<i16>> = crate::Point::new(2, 1, false);
}
impl crate::Model for Model64413 {
    const ID: u16 = 64413;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            iv_len: Self::IV_LEN.from_data(data)?,
            irr: Self::IRR.from_data(data)?,
            irr_sf: Self::IRR_SF.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m64413
    }
}
