//! PPP Link
/// PPP Link
///
/// Include this model to configure a Point-to-Point Protocol link
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model19 {
    /// Name
    ///
    /// Interface name
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
    /// Authentication
    ///
    /// Enumerated value.  Authentication method
    pub auth: Option<Auth>,
    /// Username
    ///
    /// Username for authentication
    pub usr_nam: Option<String>,
    /// Password
    ///
    /// Password for authentication
    pub pw: Option<String>,
}
#[allow(missing_docs)]
impl Model19 {
    pub const NAM: crate::Point<Self, Option<String>> = crate::Point::new(0, 4, true);
    pub const RTE: crate::Point<Self, u32> = crate::Point::new(4, 2, true);
    pub const BITS: crate::Point<Self, u16> = crate::Point::new(6, 1, true);
    pub const PTY: crate::Point<Self, Pty> = crate::Point::new(7, 1, true);
    pub const DUP: crate::Point<Self, Option<Dup>> = crate::Point::new(8, 1, true);
    pub const FLW: crate::Point<Self, Option<Flw>> = crate::Point::new(9, 1, true);
    pub const AUTH: crate::Point<Self, Option<Auth>> = crate::Point::new(10, 1, false);
    pub const USR_NAM: crate::Point<Self, Option<String>> = crate::Point::new(11, 12, false);
    pub const PW: crate::Point<Self, Option<String>> = crate::Point::new(23, 6, false);
}
impl crate::Group for Model19 {
    const LEN: u16 = 30;
}
impl Model19 {
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
                auth: Self::AUTH.from_data(data)?,
                usr_nam: Self::USR_NAM.from_data(data)?,
                pw: Self::PW.from_data(data)?,
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
/// Authentication
///
/// Enumerated value.  Authentication method
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Auth {
    #[allow(missing_docs)]
    None,
    #[allow(missing_docs)]
    Pap,
    #[allow(missing_docs)]
    Chap,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Auth {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Pap,
            2 => Self::Chap,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::None => 0,
            Self::Pap => 1,
            Self::Chap => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Auth {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
impl crate::Model for Model19 {
    const ID: u16 = 19;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m19
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
