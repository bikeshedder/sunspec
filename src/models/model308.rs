//! Mini Met Model
/// Mini Met Model
///
/// Include to support a few basic measurements
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model308 {
    /// GHI
    ///
    /// Global Horizontal Irradiance
    pub ghi: Option<u16>,
    /// Temp
    ///
    /// Back of module temperature measurement
    pub tmp_bom: Option<i16>,
    /// Ambient Temperature
    pub tmp_amb: Option<i16>,
    /// Wind Speed
    pub wnd_spd: Option<u16>,
}
#[allow(missing_docs)]
impl Model308 {
    pub const GHI: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(0, 1, false);
    pub const TMP_BOM: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(1, 1, false);
    pub const TMP_AMB: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(2, 1, false);
    pub const WND_SPD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(3, 1, false);
}
impl crate::Model for Model308 {
    const ID: u16 = 308;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            ghi: Self::GHI.from_data(data)?,
            tmp_bom: Self::TMP_BOM.from_data(data)?,
            tmp_amb: Self::TMP_AMB.from_data(data)?,
            wnd_spd: Self::WND_SPD.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m308
    }
}
