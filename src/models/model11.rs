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
    pub cfgst: u16,
    /// Link State
    ///
    /// Enumerated value. State information for this interface
    pub st: u16,
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
    pub frcspd: Option<u16>,
}

#[allow(missing_docs)]

impl Model11 {
    pub const SPD: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const CFGST: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const ST: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const MAC: crate::PointDef<Self, String> = crate::PointDef::new(3, 3, false);
    pub const NAM: crate::PointDef<Self, String> = crate::PointDef::new(7, 4, true);
    pub const CTL: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, true);
    pub const FRCSPD: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, true);
}

impl crate::Model for Model11 {
    const ID: u16 = 11;
    const LENGTH: u16 = 13;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            spd: Self::SPD
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cfgst: Self::CFGST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            st: Self::ST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            mac: Self::MAC.from_data(data)?,
            nam: Self::NAM.from_data(data)?,
            ctl: Self::CTL.from_data(data)?,
            frcspd: Self::FRCSPD.from_data(data)?,
        })
    }
}
