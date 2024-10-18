//! Proxy Server
/// Proxy Server
///
/// Include this block to allow for a proxy server
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model14 {
    /// name
    ///
    /// Interface name (8 chars)
    pub nam: Option<String>,
    /// Capabilities
    ///
    /// Bitmask value.  Proxy configuration capabilities
    pub cap: Cap,
    /// Config
    ///
    /// Enumerated value.  Set proxy address type
    pub cfg: u16,
    /// Type
    ///
    /// Enumerate value.  Proxy server type
    pub typ: Typ,
    /// Address
    ///
    /// IPv4 or IPv6 proxy hostname or dotted address (40 chars)
    pub addr: String,
    /// Port
    ///
    /// Proxy port number
    pub port: u16,
    /// Username
    ///
    /// Proxy user name
    pub user: Option<String>,
    /// Password
    ///
    /// Proxy password
    pub pw: Option<String>,
}
#[allow(missing_docs)]
impl Model14 {
    pub const NAM: crate::PointDef<Self, Option<String>> = crate::PointDef::new(0, 4, true);
    pub const CAP: crate::PointDef<Self, Cap> = crate::PointDef::new(4, 1, true);
    pub const CFG: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, true);
    pub const TYP: crate::PointDef<Self, Typ> = crate::PointDef::new(6, 1, true);
    pub const ADDR: crate::PointDef<Self, String> = crate::PointDef::new(7, 20, true);
    pub const PORT: crate::PointDef<Self, u16> = crate::PointDef::new(27, 1, true);
    pub const USER: crate::PointDef<Self, Option<String>> = crate::PointDef::new(28, 12, true);
    pub const PW: crate::PointDef<Self, Option<String>> = crate::PointDef::new(40, 12, true);
}
impl crate::Model for Model14 {
    const ID: u16 = 14;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            nam: Self::NAM.from_data(data)?,
            cap: Self::CAP.from_data(data)?,
            cfg: Self::CFG.from_data(data)?,
            typ: Self::TYP.from_data(data)?,
            addr: Self::ADDR.from_data(data)?,
            port: Self::PORT.from_data(data)?,
            user: Self::USER.from_data(data)?,
            pw: Self::PW.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m14
    }
}
bitflags::bitflags! {
    #[doc = " Capabilities"] #[doc = " "] #[doc =
    " Bitmask value.  Proxy configuration capabilities"] #[derive(Copy, Clone, Debug, Eq,
    PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct Cap : u16 { #[allow(missing_docs)] const NoProxy =
    1; #[allow(missing_docs)] const Ipv4Proxy = 2; #[allow(missing_docs)] const Ipv6Proxy
    = 4; }
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
bitflags::bitflags! {
    #[doc = " Type"] #[doc = " "] #[doc = " Enumerate value.  Proxy server type"]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct Typ : u16 {}
}
impl crate::Value for Typ {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<Typ> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535u16 {
            Ok(Some(Typ::from_bits_retain(value)))
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
