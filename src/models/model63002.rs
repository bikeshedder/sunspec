//! SunSpec Test Model 2
/// SunSpec Test Model 2
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model63002 {
    #[allow(missing_docs)]
    pub repeating: Vec<Repeating>,
}
#[allow(missing_docs)]
impl Model63002 {}
static MODEL63002_FIELDS: &[crate::FieldInfo] = &[crate::FieldInfo {
    name: "repeating",
    label: "repeating",
    description: "",
    kind: crate::FieldKind::RepeatingGroup(<Repeating as crate::GroupMeta>::group_info),
}];
static MODEL63002_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "model_63002",
    label: "SunSpec Test Model 2",
    description: "",
    fields: MODEL63002_FIELDS,
};
impl crate::GroupMeta for Model63002 {
    fn group_info() -> &'static crate::GroupInfo {
        &MODEL63002_GROUP_INFO
    }
}
impl crate::Group for Model63002 {
    const LEN: u16 = 0;
}
impl Model63002 {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let (nested_data, repeating) = Repeating::parse_multiple(nested_data)?;
        Ok((nested_data, Self { repeating }))
    }
}
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Repeating {
    #[allow(missing_docs)]
    pub sunssf_1: Option<i16>,
    #[allow(missing_docs)]
    pub int16_1: Option<i16>,
    #[allow(missing_docs)]
    pub int16_2: Option<i16>,
    #[allow(missing_docs)]
    pub sunssf_2: Option<i16>,
}
#[allow(missing_docs)]
impl Repeating {
    pub const SUNSSF_1: crate::Point<Self, Option<i16>> = crate::Point::new(0, 1, false);
    pub const INT16_1: crate::Point<Self, Option<i16>> = crate::Point::new(1, 1, true);
    pub const INT16_2: crate::Point<Self, Option<i16>> = crate::Point::new(2, 1, false);
    pub const SUNSSF_2: crate::Point<Self, Option<i16>> = crate::Point::new(3, 1, false);
}
static REPEATING_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "sunssf_1",
        label: "sunssf_1",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "int16_1",
        label: "int16_1",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "int16_2",
        label: "int16_2",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "sunssf_2",
        label: "sunssf_2",
        description: "",
        kind: crate::FieldKind::Point,
    },
];
static REPEATING_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "repeating",
    label: "repeating",
    description: "",
    fields: REPEATING_FIELDS,
};
impl crate::GroupMeta for Repeating {
    fn group_info() -> &'static crate::GroupInfo {
        &REPEATING_GROUP_INFO
    }
}
impl crate::Group for Repeating {
    const LEN: u16 = 4;
}
impl Repeating {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                sunssf_1: Self::SUNSSF_1.from_data(data)?,
                int16_1: Self::INT16_1.from_data(data)?,
                int16_2: Self::INT16_2.from_data(data)?,
                sunssf_2: Self::SUNSSF_2.from_data(data)?,
            },
        ))
    }
    fn parse_multiple(data: &[u16]) -> Result<(&[u16], Vec<Self>), crate::DecodeError> {
        let group_len = usize::from(<Repeating as crate::Group>::LEN);
        if group_len == 0 {
            return Ok((data, Vec::new()));
        }
        if data.len() % group_len != 0 {
            return Err(crate::DecodeError::OutOfBounds);
        }
        let group_count = data.len() / group_len;
        let (data, groups) =
            (0..group_count).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Repeating::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
impl crate::Model for Model63002 {
    const ID: u16 = 63002;
    const NAME: &'static str = "model_63002";
    const LABEL: &'static str = "SunSpec Test Model 2";
    const DESCRIPTION: &'static str = "";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m63002
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
