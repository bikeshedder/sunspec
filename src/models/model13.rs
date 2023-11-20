/// IPv6
///
/// Include to support an IPv6 protocol stack on this interface
#[derive(Debug)]
pub struct Model13 {
    /// Name
    ///
    /// Interface name
    pub nam: Option<String>,
    /// Config Status
    ///
    /// Enumerated value.  Configuration status
    pub cfgst: u16,
    /// Change Status
    ///
    /// Bitmask value.  A configuration change is pending
    pub chgst: u16,
    /// Config Capability
    ///
    /// Bitmask value. Identify capable sources of configuration
    pub cap: u16,
    /// IPv6 Config
    ///
    /// Enumerated value.  Configuration method used.
    pub cfg: u16,
    /// Control
    ///
    /// Bitmask value.  Configure use of services
    pub ctl: u16,
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
    pub domnam: Option<String>,
    /// Host Name
    ///
    /// Host name (24 chars max)
    pub hostnam: Option<String>,
}

#[allow(missing_docs)]

impl Model13 {
    pub const NAM: crate::PointDef<Self, String> = crate::PointDef::new(0, 4, true);
    pub const CFGST: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const CHGST: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const CAP: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const CFG: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, true);
    pub const CTL: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, true);
    pub const ADDR: crate::PointDef<Self, String> = crate::PointDef::new(9, 20, true);
    pub const CIDR: crate::PointDef<Self, String> = crate::PointDef::new(29, 20, true);
    pub const GW: crate::PointDef<Self, String> = crate::PointDef::new(49, 20, true);
    pub const DNS1: crate::PointDef<Self, String> = crate::PointDef::new(69, 20, true);
    pub const DNS2: crate::PointDef<Self, String> = crate::PointDef::new(89, 20, true);
    pub const NTP1: crate::PointDef<Self, String> = crate::PointDef::new(109, 20, true);
    pub const NTP2: crate::PointDef<Self, String> = crate::PointDef::new(129, 20, true);
    pub const DOMNAM: crate::PointDef<Self, String> = crate::PointDef::new(149, 12, true);
    pub const HOSTNAM: crate::PointDef<Self, String> = crate::PointDef::new(161, 12, true);
}

impl crate::Model for Model13 {
    const ID: u16 = 13;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            nam: Self::NAM.from_data(data)?,
            cfgst: Self::CFGST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            chgst: Self::CHGST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cap: Self::CAP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cfg: Self::CFG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ctl: Self::CTL
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            addr: Self::ADDR
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cidr: Self::CIDR.from_data(data)?,
            gw: Self::GW.from_data(data)?,
            dns1: Self::DNS1.from_data(data)?,
            dns2: Self::DNS2.from_data(data)?,
            ntp1: Self::NTP1.from_data(data)?,
            ntp2: Self::NTP2.from_data(data)?,
            domnam: Self::DOMNAM.from_data(data)?,
            hostnam: Self::HOSTNAM.from_data(data)?,
        })
    }
}
