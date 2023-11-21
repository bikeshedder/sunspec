/// Base Met
///
/// Base Meteorological Model
///
/// Notes: This model supersedes model 301
#[derive(Debug)]
pub struct Model307 {
    /// Ambient Temperature
    pub tmpamb: Option<i16>,
    /// Relative Humidity
    pub rh: Option<i16>,
    /// Barometric Pressure
    pub pres: Option<i16>,
    /// Wind Speed
    pub wndspd: Option<i16>,
    /// Wind Direction
    pub wnddir: Option<i16>,
    /// Rainfall
    pub rain: Option<i16>,
    /// Snow Depth
    pub snw: Option<i16>,
    /// Precipitation Type
    ///
    /// Precipitation Type (WMO 4680 SYNOP code reference)
    pub ppt: Option<i16>,
    /// Electric Field
    pub elecfld: Option<i16>,
    /// Surface Wetness
    pub surwet: Option<i16>,
    /// Soil Wetness
    pub soilwet: Option<i16>,
}

#[allow(missing_docs)]

impl Model307 {
    pub const TMPAMB: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(0, 1, false);
    pub const RH: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(1, 1, false);
    pub const PRES: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(2, 1, false);
    pub const WNDSPD: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(3, 1, false);
    pub const WNDDIR: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(4, 1, false);
    pub const RAIN: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(5, 1, false);
    pub const SNW: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(6, 1, false);
    pub const PPT: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(7, 1, false);
    pub const ELECFLD: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(8, 1, false);
    pub const SURWET: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(9, 1, false);
    pub const SOILWET: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(10, 1, false);
}

impl crate::Model for Model307 {
    const ID: u16 = 307;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            tmpamb: Self::TMPAMB.from_data(data)?,
            rh: Self::RH.from_data(data)?,
            pres: Self::PRES.from_data(data)?,
            wndspd: Self::WNDSPD.from_data(data)?,
            wnddir: Self::WNDDIR.from_data(data)?,
            rain: Self::RAIN.from_data(data)?,
            snw: Self::SNW.from_data(data)?,
            ppt: Self::PPT.from_data(data)?,
            elecfld: Self::ELECFLD.from_data(data)?,
            surwet: Self::SURWET.from_data(data)?,
            soilwet: Self::SOILWET.from_data(data)?,
        })
    }
}
