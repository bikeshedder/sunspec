//! Inclinometer Model
/// Type alias for [`Inclinometer`].
pub type Model304 = Inclinometer;
/// Inclinometer Model
///
/// Include to support orientation measurements
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Inclinometer {
    #[allow(missing_docs)]
    pub incl: Vec<Incl>,
}
#[allow(missing_docs)]
impl Inclinometer {}
static INCLINOMETER_FIELDS: &[crate::FieldInfo] = &[crate::FieldInfo {
    name: "incl",
    label: "incl",
    description: "",
    kind: crate::FieldKind::RepeatingGroup(<Incl as crate::GroupMeta>::group_info),
}];
static INCLINOMETER_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "inclinometer",
    label: "Inclinometer Model",
    description: "Include to support orientation measurements",
    fields: INCLINOMETER_FIELDS,
};
impl crate::GroupMeta for Inclinometer {
    fn group_info() -> &'static crate::GroupInfo {
        &INCLINOMETER_GROUP_INFO
    }
}
impl crate::Group for Inclinometer {
    const LEN: u16 = 0;
}
impl Inclinometer {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let (nested_data, incl) = Incl::parse_multiple(nested_data)?;
        Ok((nested_data, Self { incl }))
    }
}
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Incl {
    /// X
    ///
    /// X-Axis inclination
    pub inclx: i32,
    /// Y
    ///
    /// Y-Axis inclination
    pub incly: Option<i32>,
    /// Z
    ///
    /// Z-Axis inclination
    pub inclz: Option<i32>,
}
#[allow(missing_docs)]
impl Incl {
    pub const INCLX: crate::Point<Self, i32> = crate::Point::new(0, 2, false);
    pub const INCLY: crate::Point<Self, Option<i32>> = crate::Point::new(2, 2, false);
    pub const INCLZ: crate::Point<Self, Option<i32>> = crate::Point::new(4, 2, false);
}
static INCL_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "inclx",
        label: "X",
        description: "X-Axis inclination",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "incly",
        label: "Y",
        description: "Y-Axis inclination",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "inclz",
        label: "Z",
        description: "Z-Axis inclination",
        kind: crate::FieldKind::Point,
    },
];
static INCL_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "incl",
    label: "incl",
    description: "",
    fields: INCL_FIELDS,
};
impl crate::GroupMeta for Incl {
    fn group_info() -> &'static crate::GroupInfo {
        &INCL_GROUP_INFO
    }
}
impl crate::Group for Incl {
    const LEN: u16 = 6;
}
impl Incl {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                inclx: Self::INCLX.from_data(data)?,
                incly: Self::INCLY.from_data(data)?,
                inclz: Self::INCLZ.from_data(data)?,
            },
        ))
    }
    fn parse_multiple(data: &[u16]) -> Result<(&[u16], Vec<Self>), crate::DecodeError> {
        let group_len = usize::from(<Incl as crate::Group>::LEN);
        if group_len == 0 {
            return Ok((data, Vec::new()));
        }
        if data.len() % group_len != 0 {
            return Err(crate::DecodeError::OutOfBounds);
        }
        let group_count = data.len() / group_len;
        let (data, groups) =
            (0..group_count).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Incl::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
impl crate::Model for Inclinometer {
    const ID: u16 = 304;
    const NAME: &'static str = "inclinometer";
    const LABEL: &'static str = "Inclinometer Model";
    const DESCRIPTION: &'static str = "Include to support orientation measurements";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m304
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
