//! Cellular Link
/// Cellular Link
///
/// Include this model to support a cellular interface link
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model18 {
    /// Name
    ///
    /// Interface name
    pub nam: Option<String>,
    /// IMEI
    ///
    /// International Mobile Equipment Identifier for the interface
    pub imei: Option<u32>,
    /// APN
    ///
    /// Access Point Name for the interface
    pub apn: Option<String>,
    /// Number
    ///
    /// Phone number for the interface
    pub num: Option<String>,
    /// PIN
    ///
    /// Personal Identification Number for the interface
    pub pin: Option<String>,
}
#[allow(missing_docs)]
impl Model18 {
    pub const NAM: crate::Point<Self, Option<String>> = crate::Point::new(0, 4, true);
    pub const IMEI: crate::Point<Self, Option<u32>> = crate::Point::new(4, 2, true);
    pub const APN: crate::Point<Self, Option<String>> = crate::Point::new(6, 4, true);
    pub const NUM: crate::Point<Self, Option<String>> = crate::Point::new(10, 6, true);
    pub const PIN: crate::Point<Self, Option<String>> = crate::Point::new(16, 6, true);
}
static MODEL18_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "nam",
        label: "Name",
        description: "Interface name",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "imei",
        label: "IMEI",
        description: "International Mobile Equipment Identifier for the interface",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "apn",
        label: "APN",
        description: "Access Point Name for the interface",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "num",
        label: "Number",
        description: "Phone number for the interface",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "pin",
        label: "PIN",
        description: "Personal Identification Number for the interface",
        kind: crate::FieldKind::Point,
    },
];
static MODEL18_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "model_18",
    label: "Cellular Link",
    description: "Include this model to support a cellular interface link",
    fields: MODEL18_FIELDS,
};
impl crate::GroupMeta for Model18 {
    fn group_info() -> &'static crate::GroupInfo {
        &MODEL18_GROUP_INFO
    }
}
impl crate::Group for Model18 {
    const LEN: u16 = 22;
}
impl Model18 {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                nam: Self::NAM.from_data(data)?,
                imei: Self::IMEI.from_data(data)?,
                apn: Self::APN.from_data(data)?,
                num: Self::NUM.from_data(data)?,
                pin: Self::PIN.from_data(data)?,
            },
        ))
    }
}
impl crate::Model for Model18 {
    const ID: u16 = 18;
    const NAME: &'static str = "model_18";
    const LABEL: &'static str = "Cellular Link";
    const DESCRIPTION: &'static str = "Include this model to support a cellular interface link";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m18
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
