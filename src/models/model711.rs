/// DER Frequency Droop
///
/// DER Frequency Droop model.
#[derive(Debug)]
pub struct Model711 {
    /// DER Frequency Droop Module Enable
    ///
    /// DER Frequency-Watt (Frequency-Droop) control enable.
    pub ena: u16,
    /// Set Active Control Request
    ///
    /// Set active control. 0 = No active control.
    pub adptctlreq: u16,
    /// Set Active Control Result
    ///
    /// Result of last set active control operation.
    pub adptctlrslt: u16,
    /// Stored Control Count
    ///
    /// Number of stored controls supported.
    pub nctl: u16,
    /// Reversion Timeout
    ///
    /// Reversion time in seconds.  0 = No reversion time.
    pub rvrttms: Option<u32>,
    /// Reversion Time Left
    ///
    /// Reversion time remaining in seconds.
    pub rvrtrem: Option<u32>,
    /// Reversion Control
    ///
    /// Default control after reversion timeout.
    pub rvrtctl: Option<u16>,
    /// Deadband Scale Factor
    ///
    /// Deadband scale factor.
    pub db_sf: i16,
    /// Frequency Change Scale Factor
    ///
    /// Frequency change scale factor.
    pub k_sf: i16,
    /// Open-Loop Scale Factor
    ///
    /// Open loop response time scale factor.
    pub rsptms_sf: i16,
}

#[allow(missing_docs)]

impl Model711 {
    pub const ENA: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const ADPTCTLREQ: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const ADPTCTLRSLT: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const NCTL: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const RVRTTMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(4, 2, true);
    pub const RVRTREM: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(6, 2, false);
    pub const RVRTCTL: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(8, 1, true);
    pub const DB_SF: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const K_SF: crate::PointDef<Self, i16> = crate::PointDef::new(10, 1, false);
    pub const RSPTMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
}

impl crate::Model for Model711 {
    const ID: u16 = 711;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            ena: Self::ENA.from_data(data)?,
            adptctlreq: Self::ADPTCTLREQ.from_data(data)?,
            adptctlrslt: Self::ADPTCTLRSLT.from_data(data)?,
            nctl: Self::NCTL.from_data(data)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rvrtrem: Self::RVRTREM.from_data(data)?,
            rvrtctl: Self::RVRTCTL.from_data(data)?,
            db_sf: Self::DB_SF.from_data(data)?,
            k_sf: Self::K_SF.from_data(data)?,
            rsptms_sf: Self::RSPTMS_SF.from_data(data)?,
        })
    }
}
