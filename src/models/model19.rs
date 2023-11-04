/// PPP Link
///
/// Include this model to configure a Point-to-Point Protocol link
#[derive(Debug)]
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
    pub pty: u16,
    /// Duplex
    ///
    /// Enumerated value.  Duplex mode
    pub dup: Option<u16>,
    /// Flow Control
    ///
    /// Flow Control Method
    pub flw: Option<u16>,
    /// Authentication
    ///
    /// Enumerated value.  Authentication method
    pub auth: Option<u16>,
    /// Username
    ///
    /// Username for authentication
    pub usrnam: Option<String>,
    /// Password
    ///
    /// Password for authentication
    pub pw: Option<String>,
}

#[allow(missing_docs)]

impl Model19 {
    pub const NAM: crate::PointDef<Self, String> = crate::PointDef::new(0, 4, true);
    pub const RTE: crate::PointDef<Self, u32> = crate::PointDef::new(4, 2, true);
    pub const BITS: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, true);
    pub const PTY: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, true);
    pub const DUP: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, true);
    pub const FLW: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, true);
    pub const AUTH: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const USRNAM: crate::PointDef<Self, String> = crate::PointDef::new(11, 12, false);
    pub const PW: crate::PointDef<Self, String> = crate::PointDef::new(23, 6, false);
}

impl crate::Model for Model19 {
    const ID: u16 = 19;
    const LENGTH: u16 = 30;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            nam: Self::NAM.from_data(data)?,
            rte: Self::RTE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            bits: Self::BITS
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            pty: Self::PTY
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dup: Self::DUP.from_data(data)?,
            flw: Self::FLW.from_data(data)?,
            auth: Self::AUTH.from_data(data)?,
            usrnam: Self::USRNAM.from_data(data)?,
            pw: Self::PW.from_data(data)?,
        })
    }
}
