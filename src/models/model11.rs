//! Ethernet Link Layer
/// Ethernet Link Layer
///
/// Include to support a wired ethernet port
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model11 {
    /// Ethernet Link Speed
    ///
    /// Interface speed in Mb/s
    pub spd: u16,
    /// Interface Status Flags
    ///
    /// Bitmask values Interface flags.
    pub cfg_st: CfgSt,
    /// Link State
    ///
    /// Enumerated value. State information for this interface
    pub st: St,
    /// MAC
    ///
    /// IEEE MAC address of this interface
    pub mac: Option<String>,
    /// Name
    ///
    /// Interface name (8 chars)
    pub nam: Option<String>,
    /// Control
    ///
    /// Control flags
    pub ctl: Option<Ctl>,
    /// Forced Speed
    ///
    /// Forced interface speed in Mb/s when AUTO is disabled
    pub frc_spd: Option<u16>,
}
#[allow(missing_docs)]
impl Model11 {
    pub const SPD: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
    pub const CFG_ST: crate::Point<Self, CfgSt> = crate::Point::new(1, 1, false);
    pub const ST: crate::Point<Self, St> = crate::Point::new(2, 1, false);
    pub const MAC: crate::Point<Self, Option<String>> = crate::Point::new(3, 4, false);
    pub const NAM: crate::Point<Self, Option<String>> = crate::Point::new(7, 4, true);
    pub const CTL: crate::Point<Self, Option<Ctl>> = crate::Point::new(11, 1, true);
    pub const FRC_SPD: crate::Point<Self, Option<u16>> = crate::Point::new(12, 1, true);
}
impl crate::Group for Model11 {
    const LEN: u16 = 13;
}
impl Model11 {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        Ok((
            nested_data,
            Self {
                spd: Self::SPD.from_data(data)?,
                cfg_st: Self::CFG_ST.from_data(data)?,
                st: Self::ST.from_data(data)?,
                mac: Self::MAC.from_data(data)?,
                nam: Self::NAM.from_data(data)?,
                ctl: Self::CTL.from_data(data)?,
                frc_spd: Self::FRC_SPD.from_data(data)?,
            },
        ))
    }
}
bitflags::bitflags! {
    #[doc = " Interface Status Flags"] #[doc = " "] #[doc =
    " Bitmask values Interface flags."] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct CfgSt : u16 { #[allow(missing_docs)] const Link = 1; #[allow(missing_docs)]
    const FullDuplex = 2; #[allow(missing_docs)] const AutoNeg1 = 4;
    #[allow(missing_docs)] const AutoNeg2 = 8; #[allow(missing_docs)] const AutoNeg3 =
    16; #[allow(missing_docs)] const ResetRequired = 32; #[allow(missing_docs)] const
    HwFault = 64; }
}
impl crate::Value for CfgSt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for CfgSt {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::from_bits_retain(65535u16);
    fn is_invalid(&self) -> bool {
        self.bits() == 65535u16
    }
}
/// Link State
///
/// Enumerated value. State information for this interface
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum St {
    #[allow(missing_docs)]
    Unknown,
    #[allow(missing_docs)]
    Enabled,
    #[allow(missing_docs)]
    Disabled,
    #[allow(missing_docs)]
    Testing,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for St {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Enabled,
            2 => Self::Disabled,
            3 => Self::Testing,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Unknown => 0,
            Self::Enabled => 1,
            Self::Disabled => 2,
            Self::Testing => 3,
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
bitflags::bitflags! {
    #[doc = " Control"] #[doc = " "] #[doc = " Control flags"] #[derive(Copy, Clone,
    Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct Ctl : u16 { #[allow(missing_docs)] const Auto = 1;
    #[allow(missing_docs)] const FullDuplex = 2; }
}
impl crate::Value for Ctl {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for Ctl {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::from_bits_retain(65535u16);
    fn is_invalid(&self) -> bool {
        self.bits() == 65535u16
    }
}
impl crate::Model for Model11 {
    const ID: u16 = 11;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m11
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
