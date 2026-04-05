//! Reference Point Model
/// Type alias for [`RefPoint`].
pub type Model306 = RefPoint;
/// Reference Point Model
///
/// Include to support a standard reference point
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct RefPoint {
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
impl RefPoint {
    pub const GHI: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, false);
    pub const A: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, false);
    pub const V: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, false);
    pub const TMP: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, false);
}
static REF_POINT_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "ghi",
        label: "GHI",
        description: "Global Horizontal Irradiance",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "a",
        label: "Amps",
        description: "Current measurement at reference point",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "v",
        label: "Voltage",
        description: "Voltage  measurement at reference point",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tmp",
        label: "Temperature",
        description: "Temperature measurement at reference point",
        kind: crate::FieldKind::Point,
    },
];
static REF_POINT_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "ref_point",
    label: "Reference Point Model",
    description: "Include to support a standard reference point",
    fields: REF_POINT_FIELDS,
};
impl crate::GroupMeta for RefPoint {
    fn group_info() -> &'static crate::GroupInfo {
        &REF_POINT_GROUP_INFO
    }
}
impl crate::Group for RefPoint {
    const LEN: u16 = 4;
}
impl RefPoint {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                ghi: Self::GHI.from_data(data)?,
                a: Self::A.from_data(data)?,
                v: Self::V.from_data(data)?,
                tmp: Self::TMP.from_data(data)?,
            },
        ))
    }
}
impl crate::Model for RefPoint {
    const ID: u16 = 306;
    const NAME: &'static str = "ref_point";
    const LABEL: &'static str = "Reference Point Model";
    const DESCRIPTION: &'static str = "Include to support a standard reference point";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m306
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
