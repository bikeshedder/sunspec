//! GPS
/// Type alias for [`Location`].
pub type Model305 = Location;
/// GPS
///
/// Include to support location measurements
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Location {
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
impl Location {
    pub const TM: crate::Point<Self, Option<String>> = crate::Point::new(0, 6);
    pub const DATE: crate::Point<Self, Option<String>> = crate::Point::new(6, 4);
    pub const LOC: crate::Point<Self, Option<String>> = crate::Point::new(10, 20);
    pub const LAT: crate::Point<Self, Option<i32>> = crate::Point::new(30, 2);
    pub const LONG: crate::Point<Self, Option<i32>> = crate::Point::new(32, 2);
    pub const ALT: crate::Point<Self, Option<i32>> = crate::Point::new(34, 2);
}
impl crate::sealed::Sealed for Location {}
impl crate::Group for Location {
    const LEN: u16 = 36;
}
impl Location {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::ParseError> {
        let nested_data = data.get(36..).unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                tm: Self::TM.from_data(data)?,
                date: Self::DATE.from_data(data)?,
                loc: Self::LOC.from_data(data)?,
                lat: Self::LAT.from_data(data)?,
                long: Self::LONG.from_data(data)?,
                alt: Self::ALT.from_data(data)?,
            },
        ))
    }
}
impl From<Location> for crate::AnyModel {
    fn from(model: Location) -> Self {
        Self::M305(model)
    }
}
impl crate::Model for Location {
    const ID: u16 = 305;
    const NAME: &'static str = "location";
    const LABEL: &'static str = "GPS";
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
