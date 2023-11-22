//! Ethernet Link Layer

/// Ethernet Link Layer
///
/// Include to support a wired ethernet port
#[derive(Debug)]
pub struct Model11 {
    /// Ethernet Link Speed
    ///
    /// Interface speed in Mb/s
    pub spd: u16,
    /// Interface Status Flags
    ///
    /// Bitmask values Interface flags.
    pub cfg_st: u16,
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
    pub ctl: Option<u16>,
    /// Forced Speed
    ///
    /// Forced interface speed in Mb/s when AUTO is disabled
    pub frc_spd: Option<u16>,
}

#[allow(missing_docs)]

impl Model11 {
    pub const SPD: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const CFG_ST: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const ST: crate::PointDef<Self, St> = crate::PointDef::new(2, 1, false);
    pub const MAC: crate::PointDef<Self, Option<String>> = crate::PointDef::new(3, 4, false);
    pub const NAM: crate::PointDef<Self, Option<String>> = crate::PointDef::new(7, 4, true);
    pub const CTL: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(11, 1, true);
    pub const FRC_SPD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(12, 1, true);
}

impl crate::Model for Model11 {
    const ID: u16 = 11;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            spd: Self::SPD.from_data(data)?,
            cfg_st: Self::CFG_ST.from_data(data)?,
            st: Self::ST.from_data(data)?,
            mac: Self::MAC.from_data(data)?,
            nam: Self::NAM.from_data(data)?,
            ctl: Self::CTL.from_data(data)?,
            frc_spd: Self::FRC_SPD.from_data(data)?,
        })
    }
}

#[doc = "Link State\n\nEnumerated value. State information for this interface"]
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum :: FromRepr)]
#[repr(u16)]
pub enum St {
    #[doc = ""]
    Unknown = 0,
    #[doc = ""]
    Enabled = 1,
    #[doc = ""]
    Disabled = 2,
    #[doc = ""]
    Testing = 3,
}
impl crate::Value for St {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<St> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                St::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
            ))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535.encode()
        }
    }
}
