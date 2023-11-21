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
    pub adpt_ctl_req: u16,
    /// Set Active Control Result
    ///
    /// Result of last set active control operation.
    pub adpt_ctl_rslt: u16,
    /// Stored Control Count
    ///
    /// Number of stored controls supported.
    pub n_ctl: u16,
    /// Reversion Timeout
    ///
    /// Reversion time in seconds.  0 = No reversion time.
    pub rvrt_tms: Option<u32>,
    /// Reversion Time Left
    ///
    /// Reversion time remaining in seconds.
    pub rvrt_rem: Option<u32>,
    /// Reversion Control
    ///
    /// Default control after reversion timeout.
    pub rvrt_ctl: Option<u16>,
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
    pub rsp_tms_sf: i16,
}

#[allow(missing_docs)]

impl Model711 {
    pub const ENA: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const ADPT_CTL_REQ: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const ADPT_CTL_RSLT: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const N_CTL: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const RVRT_TMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(4, 2, true);
    pub const RVRT_REM: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(6, 2, false);
    pub const RVRT_CTL: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(8, 1, true);
    pub const DB_SF: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const K_SF: crate::PointDef<Self, i16> = crate::PointDef::new(10, 1, false);
    pub const RSP_TMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
}

impl crate::Model for Model711 {
    const ID: u16 = 711;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            ena: Self::ENA.from_data(data)?,
            adpt_ctl_req: Self::ADPT_CTL_REQ.from_data(data)?,
            adpt_ctl_rslt: Self::ADPT_CTL_RSLT.from_data(data)?,
            n_ctl: Self::N_CTL.from_data(data)?,
            rvrt_tms: Self::RVRT_TMS.from_data(data)?,
            rvrt_rem: Self::RVRT_REM.from_data(data)?,
            rvrt_ctl: Self::RVRT_CTL.from_data(data)?,
            db_sf: Self::DB_SF.from_data(data)?,
            k_sf: Self::K_SF.from_data(data)?,
            rsp_tms_sf: Self::RSP_TMS_SF.from_data(data)?,
        })
    }
}
