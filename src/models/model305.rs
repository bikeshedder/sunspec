//! GPS

/// GPS
///
/// Include to support location measurements
#[derive(Debug)]
pub struct Model305 {
    /// Tm
    ///
    /// UTC 24 hour time stamp to millisecond hhmmss.sssZ format
    pub tm: Option<String>,
    /// Date
    ///
    /// UTC Date string YYYYMMDD format
    pub date: Option<String>,
    /// Location
    ///
    /// Location string (40 chars max)
    pub loc: Option<String>,
    /// Lat
    ///
    /// Latitude with seven degrees of precision
    pub lat: Option<i32>,
    /// Long
    ///
    /// Longitude with seven degrees of precision
    pub long: Option<i32>,
    /// Altitude
    ///
    /// Altitude measurement in meters
    pub alt: Option<i32>,
}

#[allow(missing_docs)]

impl Model305 {
    pub const TM: crate::PointDef<Self, Option<String>> = crate::PointDef::new(0, 6, false);
    pub const DATE: crate::PointDef<Self, Option<String>> = crate::PointDef::new(6, 4, false);
    pub const LOC: crate::PointDef<Self, Option<String>> = crate::PointDef::new(10, 20, false);
    pub const LAT: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(30, 2, false);
    pub const LONG: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(32, 2, false);
    pub const ALT: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(34, 2, false);
}

impl crate::Model for Model305 {
    const ID: u16 = 305;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            tm: Self::TM.from_data(data)?,
            date: Self::DATE.from_data(data)?,
            loc: Self::LOC.from_data(data)?,
            lat: Self::LAT.from_data(data)?,
            long: Self::LONG.from_data(data)?,
            alt: Self::ALT.from_data(data)?,
        })
    }
}
