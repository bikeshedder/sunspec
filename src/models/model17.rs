//! Serial Interface
/// Serial Interface
///
/// Include this model for serial interface configuration support
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model17 {
    /// Name
    ///
    /// Interface name (8 chars)
    pub nam: Option<String>,
    /// Rate
    ///
    /// Interface baud rate in bits per second
    pub rte: u32,
    /// Bits
    ///
    /// Number of data bits per character
    pub bits: u16,
    /// Parity
    ///
    /// Bitmask value.  Parity setting
    pub pty: Pty,
    /// Duplex
    ///
    /// Enumerated value.  Duplex mode
    pub dup: Option<Dup>,
    /// Flow Control
    ///
    /// Flow Control Method
    pub flw: Option<Flw>,
    /// Interface Type
    ///
    /// Enumerated value.  Interface type
    pub typ: Option<Typ>,
    /// Protocol
    ///
    /// Enumerated value. Serial protocol selection
    pub pcol: Option<Pcol>,
}
#[allow(missing_docs)]
impl Model17 {
    pub const NAM: crate::Point<Self, Option<String>> = crate::Point::new(0, 4, true);
    pub const RTE: crate::Point<Self, u32> = crate::Point::new(4, 2, true);
    pub const BITS: crate::Point<Self, u16> = crate::Point::new(6, 1, true);
    pub const PTY: crate::Point<Self, Pty> = crate::Point::new(7, 1, true);
    pub const DUP: crate::Point<Self, Option<Dup>> = crate::Point::new(8, 1, true);
    pub const FLW: crate::Point<Self, Option<Flw>> = crate::Point::new(9, 1, true);
    pub const TYP: crate::Point<Self, Option<Typ>> = crate::Point::new(10, 1, false);
    pub const PCOL: crate::Point<Self, Option<Pcol>> = crate::Point::new(11, 1, false);
}
static MODEL17_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "nam",
        label: "Name",
        description: "Interface name (8 chars)",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "rte",
        label: "Rate",
        description: "Interface baud rate in bits per second",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "bits",
        label: "Bits",
        description: "Number of data bits per character",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "pty",
        label: "Parity",
        description: "Bitmask value.  Parity setting",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dup",
        label: "Duplex",
        description: "Enumerated value.  Duplex mode",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "flw",
        label: "Flow Control",
        description: "Flow Control Method",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "typ",
        label: "Interface Type",
        description: "Enumerated value.  Interface type",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "pcol",
        label: "Protocol",
        description: "Enumerated value. Serial protocol selection",
        kind: crate::FieldKind::Point,
    },
];
static MODEL17_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "model_17",
    label: "Serial Interface",
    description: "Include this model for serial interface configuration support",
    fields: MODEL17_FIELDS,
};
impl crate::GroupMeta for Model17 {
    fn group_info() -> &'static crate::GroupInfo {
        &MODEL17_GROUP_INFO
    }
}
impl crate::Group for Model17 {
    const LEN: u16 = 12;
}
impl Model17 {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                nam: Self::NAM.from_data(data)?,
                rte: Self::RTE.from_data(data)?,
                bits: Self::BITS.from_data(data)?,
                pty: Self::PTY.from_data(data)?,
                dup: Self::DUP.from_data(data)?,
                flw: Self::FLW.from_data(data)?,
                typ: Self::TYP.from_data(data)?,
                pcol: Self::PCOL.from_data(data)?,
            },
        ))
    }
}
/// Parity
///
/// Bitmask value.  Parity setting
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Pty {
    #[allow(missing_docs)]
    None,
    #[allow(missing_docs)]
    Odd,
    #[allow(missing_docs)]
    Even,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Pty {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Odd,
            2 => Self::Even,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::None => 0,
            Self::Odd => 1,
            Self::Even => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Pty {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Duplex
///
/// Enumerated value.  Duplex mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Dup {
    #[allow(missing_docs)]
    Full,
    #[allow(missing_docs)]
    Half,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Dup {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Full,
            1 => Self::Half,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Full => 0,
            Self::Half => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Dup {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Flow Control
///
/// Flow Control Method
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Flw {
    #[allow(missing_docs)]
    None,
    #[allow(missing_docs)]
    Hw,
    #[allow(missing_docs)]
    Xonxoff,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Flw {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Hw,
            2 => Self::Xonxoff,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::None => 0,
            Self::Hw => 1,
            Self::Xonxoff => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Flw {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Interface Type
///
/// Enumerated value.  Interface type
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Typ {
    #[allow(missing_docs)]
    Unknown,
    #[allow(missing_docs)]
    Rs232,
    #[allow(missing_docs)]
    Rs485,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Typ {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Rs232,
            2 => Self::Rs485,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Unknown => 0,
            Self::Rs232 => 1,
            Self::Rs485 => 2,
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
/// Protocol
///
/// Enumerated value. Serial protocol selection
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Pcol {
    #[allow(missing_docs)]
    Unknown,
    #[allow(missing_docs)]
    Modbus,
    #[allow(missing_docs)]
    Vendor,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Pcol {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Modbus,
            2 => Self::Vendor,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Unknown => 0,
            Self::Modbus => 1,
            Self::Vendor => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Pcol {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
impl crate::Model for Model17 {
    const ID: u16 = 17;
    const NAME: &'static str = "model_17";
    const LABEL: &'static str = "Serial Interface";
    const DESCRIPTION: &'static str =
        "Include this model for serial interface configuration support";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m17
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
