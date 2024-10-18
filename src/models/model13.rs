//! IPv6
/// IPv6
///
/// Include to support an IPv6 protocol stack on this interface
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model13 {
    /// Name
    ///
    /// Interface name
    pub nam: Option<String>,
    /// Config Status
    ///
    /// Enumerated value.  Configuration status
    pub cfg_st: CfgSt,
    /// Change Status
    ///
    /// Bitmask value.  A configuration change is pending
    pub chg_st: ChgSt,
    /// Config Capability
    ///
    /// Bitmask value. Identify capable sources of configuration
    pub cap: Cap,
    /// IPv6 Config
    ///
    /// Enumerated value.  Configuration method used.
    pub cfg: Cfg,
    /// Control
    ///
    /// Bitmask value.  Configure use of services
    pub ctl: Ctl,
    /// IP
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    pub addr: String,
    /// CIDR
    ///
    /// Classless Inter-Domain Routing Number
    pub cidr: Option<String>,
    /// Gateway
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    pub gw: Option<String>,
    /// DNS1
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    pub dns1: Option<String>,
    /// DNS2
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    pub dns2: Option<String>,
    /// NTP1
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    pub ntp1: Option<String>,
    /// NTP2
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    pub ntp2: Option<String>,
    /// Domain
    ///
    /// Domain name (24 chars max)
    pub dom_nam: Option<String>,
    /// Host Name
    ///
    /// Host name (24 chars max)
    pub host_nam: Option<String>,
}
#[allow(missing_docs)]
impl Model13 {
    pub const NAM: crate::Point<Self, Option<String>> = crate::Point::new(0, 4, true);
    pub const CFG_ST: crate::Point<Self, CfgSt> = crate::Point::new(4, 1, false);
    pub const CHG_ST: crate::Point<Self, ChgSt> = crate::Point::new(5, 1, false);
    pub const CAP: crate::Point<Self, Cap> = crate::Point::new(6, 1, false);
    pub const CFG: crate::Point<Self, Cfg> = crate::Point::new(7, 1, true);
    pub const CTL: crate::Point<Self, Ctl> = crate::Point::new(8, 1, true);
    pub const ADDR: crate::Point<Self, String> = crate::Point::new(9, 20, true);
    pub const CIDR: crate::Point<Self, Option<String>> = crate::Point::new(29, 20, true);
    pub const GW: crate::Point<Self, Option<String>> = crate::Point::new(49, 20, true);
    pub const DNS1: crate::Point<Self, Option<String>> = crate::Point::new(69, 20, true);
    pub const DNS2: crate::Point<Self, Option<String>> = crate::Point::new(89, 20, true);
    pub const NTP1: crate::Point<Self, Option<String>> = crate::Point::new(109, 20, true);
    pub const NTP2: crate::Point<Self, Option<String>> = crate::Point::new(129, 20, true);
    pub const DOM_NAM: crate::Point<Self, Option<String>> = crate::Point::new(149, 12, true);
    pub const HOST_NAM: crate::Point<Self, Option<String>> = crate::Point::new(161, 12, true);
}
impl crate::Model for Model13 {
    const ID: u16 = 13;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            nam: Self::NAM.from_data(data)?,
            cfg_st: Self::CFG_ST.from_data(data)?,
            chg_st: Self::CHG_ST.from_data(data)?,
            cap: Self::CAP.from_data(data)?,
            cfg: Self::CFG.from_data(data)?,
            ctl: Self::CTL.from_data(data)?,
            addr: Self::ADDR.from_data(data)?,
            cidr: Self::CIDR.from_data(data)?,
            gw: Self::GW.from_data(data)?,
            dns1: Self::DNS1.from_data(data)?,
            dns2: Self::DNS2.from_data(data)?,
            ntp1: Self::NTP1.from_data(data)?,
            ntp2: Self::NTP2.from_data(data)?,
            dom_nam: Self::DOM_NAM.from_data(data)?,
            host_nam: Self::HOST_NAM.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m13
    }
}
/// Config Status
///
/// Enumerated value.  Configuration status
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum CfgSt {
    #[allow(missing_docs)]
    NotConfigured = 0,
    #[allow(missing_docs)]
    ValidSetting = 1,
    #[allow(missing_docs)]
    ValidHw = 2,
}
impl crate::Value for CfgSt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<CfgSt> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                CfgSt::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
bitflags::bitflags! {
    #[doc = " Change Status"] #[doc = " "] #[doc =
    " Bitmask value.  A configuration change is pending"] #[derive(Copy, Clone, Debug,
    Eq, PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct ChgSt : u16 { #[allow(missing_docs)] const Pending
    = 1; }
}
impl crate::Value for ChgSt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<ChgSt> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535u16 {
            Ok(Some(ChgSt::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535u16.encode()
        }
    }
}
bitflags::bitflags! {
    #[doc = " Config Capability"] #[doc = " "] #[doc =
    " Bitmask value. Identify capable sources of configuration"] #[derive(Copy, Clone,
    Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct Cap : u16 { #[allow(missing_docs)] const Dhcp = 1;
    #[allow(missing_docs)] const Bootp = 2; #[allow(missing_docs)] const Zeroconf = 4;
    #[allow(missing_docs)] const Dns = 8; #[allow(missing_docs)] const CfgSettable = 16;
    #[allow(missing_docs)] const HwConfig = 32; #[allow(missing_docs)] const NtpClient =
    64; #[allow(missing_docs)] const ResetRequired = 128; }
}
impl crate::Value for Cap {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<Cap> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535u16 {
            Ok(Some(Cap::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535u16.encode()
        }
    }
}
/// IPv6 Config
///
/// Enumerated value.  Configuration method used.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Cfg {
    #[allow(missing_docs)]
    Static = 0,
    #[allow(missing_docs)]
    Dhcp = 1,
    #[allow(missing_docs)]
    Bootp = 2,
    #[allow(missing_docs)]
    Zeroconf = 3,
}
impl crate::Value for Cfg {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Cfg> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Cfg::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Control
///
/// Bitmask value.  Configure use of services
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Ctl {
    #[allow(missing_docs)]
    EnableDns = 0,
    #[allow(missing_docs)]
    EnableNtp = 1,
}
impl crate::Value for Ctl {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Ctl> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Ctl::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
