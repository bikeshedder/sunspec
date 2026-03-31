//! GPS
/// Type alias for [`Location`].
pub type Model305 = Location;
/// GPS
///
/// Include to support location measurements
#[derive(Debug)]
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
    pub const TM: crate::Point<Self, Option<String>> = crate::Point::new(0, 6, false);
    pub const DATE: crate::Point<Self, Option<String>> = crate::Point::new(6, 4, false);
    pub const LOC: crate::Point<Self, Option<String>> = crate::Point::new(10, 20, false);
    pub const LAT: crate::Point<Self, Option<i32>> = crate::Point::new(30, 2, false);
    pub const LONG: crate::Point<Self, Option<i32>> = crate::Point::new(32, 2, false);
    pub const ALT: crate::Point<Self, Option<i32>> = crate::Point::new(34, 2, false);
}
static LOCATION_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "tm",
        label: "Tm",
        description: "UTC 24 hour time stamp to millisecond hhmmss.sssZ format",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "date",
        label: "Date",
        description: "UTC Date string YYYYMMDD format",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "loc",
        label: "Location",
        description: "Location string (40 chars max)",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "lat",
        label: "Lat",
        description: "Latitude with seven degrees of precision",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "long",
        label: "Long",
        description: "Longitude with seven degrees of precision",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "alt",
        label: "Altitude",
        description: "Altitude measurement in meters",
        kind: crate::FieldKind::Point,
    },
];
static LOCATION_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "location",
    label: "GPS",
    description: "Include to support location measurements",
    fields: LOCATION_FIELDS,
};
impl crate::GroupMeta for Location {
    fn group_info() -> &'static crate::GroupInfo {
        &LOCATION_GROUP_INFO
    }
}
impl crate::Group for Location {
    const LEN: u16 = 36;
}
impl Location {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
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
impl crate::Model for Location {
    const ID: u16 = 305;
    const NAME: &'static str = "location";
    const LABEL: &'static str = "GPS";
    const DESCRIPTION: &'static str = "Include to support location measurements";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m305
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
