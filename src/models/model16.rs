//! Simple IP Network
/// Simple IP Network
///
/// Include this model for a simple IPv4 network stack
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model16 {
    /// Name
    ///
    /// Interface name.  (8 chars)
    pub nam: Option<String>,
    /// Config
    ///
    /// Enumerated value.  Force IPv4 configuration method
    pub cfg: Cfg,
    /// Control
    ///
    /// Bitmask value Configure use of services
    pub ctl: Ctl,
    /// Address
    ///
    /// IP address
    pub addr: String,
    /// Netmask
    ///
    /// Netmask
    pub msk: String,
    /// Gateway
    ///
    /// Gateway IP address
    pub gw: Option<String>,
    /// DNS1
    ///
    /// 32 bit IP address of DNS server
    pub dns1: Option<String>,
    /// DNS2
    ///
    /// 32 bit IP address of DNS server
    pub dns2: Option<String>,
    /// MAC
    ///
    /// IEEE MAC address of this interface
    pub mac: Option<String>,
    /// Link Control
    ///
    /// Bitmask value.  Link control flags
    pub lnk_ctl: Option<LnkCtl>,
}
#[allow(missing_docs)]
impl Model16 {
    pub const NAM: crate::PointDef<Self, Option<String>> = crate::PointDef::new(0, 4, true);
    pub const CFG: crate::PointDef<Self, Cfg> = crate::PointDef::new(4, 1, false);
    pub const CTL: crate::PointDef<Self, Ctl> = crate::PointDef::new(5, 1, true);
    pub const ADDR: crate::PointDef<Self, String> = crate::PointDef::new(6, 8, true);
    pub const MSK: crate::PointDef<Self, String> = crate::PointDef::new(14, 8, true);
    pub const GW: crate::PointDef<Self, Option<String>> = crate::PointDef::new(22, 8, true);
    pub const DNS1: crate::PointDef<Self, Option<String>> = crate::PointDef::new(30, 8, true);
    pub const DNS2: crate::PointDef<Self, Option<String>> = crate::PointDef::new(38, 8, true);
    pub const MAC: crate::PointDef<Self, Option<String>> = crate::PointDef::new(46, 4, false);
    pub const LNK_CTL: crate::PointDef<Self, Option<LnkCtl>> = crate::PointDef::new(50, 1, true);
}
impl crate::Model for Model16 {
    const ID: u16 = 16;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            nam: Self::NAM.from_data(data)?,
            cfg: Self::CFG.from_data(data)?,
            ctl: Self::CTL.from_data(data)?,
            addr: Self::ADDR.from_data(data)?,
            msk: Self::MSK.from_data(data)?,
            gw: Self::GW.from_data(data)?,
            dns1: Self::DNS1.from_data(data)?,
            dns2: Self::DNS2.from_data(data)?,
            mac: Self::MAC.from_data(data)?,
            lnk_ctl: Self::LNK_CTL.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m16
    }
}
/// Config
///
/// Enumerated value.  Force IPv4 configuration method
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Cfg {
    #[allow(missing_docs)]
    Static = 0,
    #[allow(missing_docs)]
    Dhcp = 1,
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
bitflags::bitflags! {
    #[doc = " Control"] #[doc = " "] #[doc = " Bitmask value Configure use of services"]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct Ctl : u16 {
    #[allow(missing_docs)] const EnableDns = 1; #[allow(missing_docs)] const EnableNtp =
    2; }
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
impl crate::Value for Option<Ctl> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535u16 {
            Ok(Some(Ctl::from_bits_retain(value)))
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
    #[doc = " Link Control"] #[doc = " "] #[doc = " Bitmask value.  Link control flags"]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct LnkCtl : u16 {
    #[allow(missing_docs)] const Autonegotiate = 1; #[allow(missing_docs)] const
    FullDuplex = 2; #[allow(missing_docs)] const Force10mb = 4; #[allow(missing_docs)]
    const Force100mb = 8; #[allow(missing_docs)] const Force1gb = 16; }
}
impl crate::Value for LnkCtl {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<LnkCtl> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535u16 {
            Ok(Some(LnkCtl::from_bits_retain(value)))
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
