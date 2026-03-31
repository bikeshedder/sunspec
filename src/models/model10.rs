//! Communication Interface Header
/// Communication Interface Header
///
/// To be included first for a complete interface description
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model10 {
    /// Interface Status
    ///
    /// Overall interface status
    pub st: St,
    /// Interface Control
    ///
    /// Overall interface control (TBD)
    pub ctl: Option<u16>,
    /// Physical Access Type
    ///
    /// Enumerated value.  Type of physical media
    pub typ: Option<Typ>,
}
#[allow(missing_docs)]
impl Model10 {
    pub const ST: crate::Point<Self, St> = crate::Point::new(0, 1, false);
    pub const CTL: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, true);
    pub const TYP: crate::Point<Self, Option<Typ>> = crate::Point::new(2, 1, false);
}
static MODEL10_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "st",
        label: "Interface Status",
        description: "Overall interface status",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "ctl",
        label: "Interface Control",
        description: "Overall interface control (TBD)",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "typ",
        label: "Physical Access Type",
        description: "Enumerated value.  Type of physical media",
        kind: crate::FieldKind::Point,
    },
];
static MODEL10_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "model_10",
    label: "Communication Interface Header",
    description: "To be included first for a complete interface description",
    fields: MODEL10_FIELDS,
};
impl crate::GroupMeta for Model10 {
    fn group_info() -> &'static crate::GroupInfo {
        &MODEL10_GROUP_INFO
    }
}
impl crate::Group for Model10 {
    const LEN: u16 = 4;
}
impl Model10 {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                st: Self::ST.from_data(data)?,
                ctl: Self::CTL.from_data(data)?,
                typ: Self::TYP.from_data(data)?,
            },
        ))
    }
}
/// Interface Status
///
/// Overall interface status
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum St {
    #[allow(missing_docs)]
    Down,
    #[allow(missing_docs)]
    Up,
    #[allow(missing_docs)]
    Fault,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for St {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Down,
            1 => Self::Up,
            2 => Self::Fault,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Down => 0,
            Self::Up => 1,
            Self::Fault => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for St {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Physical Access Type
///
/// Enumerated value.  Type of physical media
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Typ {
    #[allow(missing_docs)]
    Unknown,
    #[allow(missing_docs)]
    Internal,
    #[allow(missing_docs)]
    TwistedPair,
    #[allow(missing_docs)]
    Fiber,
    #[allow(missing_docs)]
    Wireless,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Typ {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Internal,
            2 => Self::TwistedPair,
            3 => Self::Fiber,
            4 => Self::Wireless,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Unknown => 0,
            Self::Internal => 1,
            Self::TwistedPair => 2,
            Self::Fiber => 3,
            Self::Wireless => 4,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Typ {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
impl crate::Model for Model10 {
    const ID: u16 = 10;
    const NAME: &'static str = "model_10";
    const LABEL: &'static str = "Communication Interface Header";
    const DESCRIPTION: &'static str = "To be included first for a complete interface description";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m10
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
