/// Mini Met Model
///
/// Include to support a few basic measurements
#[derive(Debug)]
pub struct Model308 {
    /// GHI
    ///
    /// Global Horizontal Irradiance
    pub ghi: Option<u16>,
    /// Temp
    ///
    /// Back of module temperature measurement
    pub tmpbom: Option<i16>,
    /// Ambient Temperature
    pub tmpamb: Option<i16>,
    /// Wind Speed
    pub wndspd: Option<u16>,
}

#[allow(missing_docs)]

impl Model308 {
    pub const GHI: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(0, 1, false);
    pub const TMPBOM: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(1, 1, false);
    pub const TMPAMB: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(2, 1, false);
    pub const WNDSPD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(3, 1, false);
}

impl crate::Model for Model308 {
    const ID: u16 = 308;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            ghi: Self::GHI.from_data(data)?,
            tmpbom: Self::TMPBOM.from_data(data)?,
            tmpamb: Self::TMPAMB.from_data(data)?,
            wndspd: Self::WNDSPD.from_data(data)?,
        })
    }
}
