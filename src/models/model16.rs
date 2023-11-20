/// Simple IP Network
///
/// Include this model for a simple IPv4 network stack
#[derive(Debug)]
pub struct Model16 {
    /// Name
    ///
    /// Interface name.  (8 chars)
    pub nam: Option<String>,
    /// Config
    ///
    /// Enumerated value.  Force IPv4 configuration method
    pub cfg: u16,
    /// Control
    ///
    /// Bitmask value Configure use of services
    pub ctl: u16,
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
    pub lnkctl: Option<u16>,
}

#[allow(missing_docs)]

impl Model16 {
    pub const NAM: crate::PointDef<Self, String> = crate::PointDef::new(0, 4, true);
    pub const CFG: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const CTL: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, true);
    pub const ADDR: crate::PointDef<Self, String> = crate::PointDef::new(6, 8, true);
    pub const MSK: crate::PointDef<Self, String> = crate::PointDef::new(14, 8, true);
    pub const GW: crate::PointDef<Self, String> = crate::PointDef::new(22, 8, true);
    pub const DNS1: crate::PointDef<Self, String> = crate::PointDef::new(30, 8, true);
    pub const DNS2: crate::PointDef<Self, String> = crate::PointDef::new(38, 8, true);
    pub const MAC: crate::PointDef<Self, String> = crate::PointDef::new(46, 4, false);
    pub const LNKCTL: crate::PointDef<Self, u16> = crate::PointDef::new(50, 1, true);
}

impl crate::Model for Model16 {
    const ID: u16 = 16;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            nam: Self::NAM.from_data(data)?,
            cfg: Self::CFG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ctl: Self::CTL
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            addr: Self::ADDR
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            msk: Self::MSK
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            gw: Self::GW.from_data(data)?,
            dns1: Self::DNS1.from_data(data)?,
            dns2: Self::DNS2.from_data(data)?,
            mac: Self::MAC.from_data(data)?,
            lnkctl: Self::LNKCTL.from_data(data)?,
        })
    }
}
