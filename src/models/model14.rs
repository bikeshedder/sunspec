/// Proxy Server
///
/// Include this block to allow for a proxy server
#[derive(Debug)]
pub struct Model14 {
    /// name
    ///
    /// Interface name (8 chars)
    pub nam: Option<String>,
    /// Capabilities
    ///
    /// Bitmask value.  Proxy configuration capabilities
    pub cap: u16,
    /// Config
    ///
    /// Enumerated value.  Set proxy address type
    pub cfg: u16,
    /// Type
    ///
    /// Enumerate value.  Proxy server type
    pub typ: u16,
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
    pub const NAM: crate::PointDef<Self, String> = crate::PointDef::new(0, 4, true);
    pub const CAP: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const CFG: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, true);
    pub const TYP: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, true);
    pub const ADDR: crate::PointDef<Self, String> = crate::PointDef::new(7, 20, true);
    pub const PORT: crate::PointDef<Self, u16> = crate::PointDef::new(27, 1, true);
    pub const USER: crate::PointDef<Self, String> = crate::PointDef::new(28, 12, true);
    pub const PW: crate::PointDef<Self, String> = crate::PointDef::new(40, 12, true);
}

impl crate::Model for Model14 {
    const ID: u16 = 14;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            nam: Self::NAM.from_data(data)?,
            cap: Self::CAP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cfg: Self::CFG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            typ: Self::TYP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            addr: Self::ADDR
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            port: Self::PORT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            user: Self::USER.from_data(data)?,
            pw: Self::PW.from_data(data)?,
        })
    }
}
